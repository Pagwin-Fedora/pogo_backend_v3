use crate::task::*;
use async_trait::async_trait;
use sqlx::postgres::PgExecutor;
use sqlx::{query, query_as};
use uuid::Uuid;

struct TaskV1Fragment {
    id: Uuid,
    title: String,
    body: String,
    progress: f32,
}

struct ParentChild {
    parent: Uuid,
    child: Uuid,
}

impl From<&TaskV1> for TaskV1Fragment {
    fn from(t: &TaskV1) -> Self {
        Self {
            id: t.id,
            title: t.title.clone(),
            body: t.body.clone(),
            progress: t.progress,
        }
    }
}

struct Id {
    id: Option<Uuid>,
}
impl From<Id> for Uuid {
    fn from(i: Id) -> Self {
        i.id.unwrap_or(Uuid::from_u128(0))
    }
}

#[async_trait]
impl TaskEncoder for sqlx::Pool<sqlx::postgres::Postgres> {
    type Identifier = Uuid;
    type DecodingError = sqlx::Error;
    type EncodingError = sqlx::Error;
    type IdentityFetchError = sqlx::Error;
    async fn encode_task(
        &mut self,
        task: TaskVersioning,
        login: &str,
    ) -> Result<Self::Identifier, Self::EncodingError> {
        match task {
            TaskVersioning::V1(task) => {
                let TaskV1Fragment {
                    id,
                    title,
                    body,
                    progress,
                } = (&task).into();
                let mut transaction = self.begin().await?;

                // updates the task fragment in the tasks table or creates a new task fragment if
                // it doesn't already exist
                query!("DELETE FROM pogo_tasks WHERE id=$1", id)
                    .execute(&mut transaction)
                    .await?;

                query!(
                    "INSERT INTO pogo_tasks VALUES ($1,$2,$3,$4)",
                    id,
                    title,
                    body,
                    progress
                )
                .execute(&mut transaction)
                .await?;

                // check for all media attached to the task object to see what is and isn't already
                // located within the resources table
                let media = task.media;
                let media = {
                    let mut new_media = Vec::new();
                    for med in media {
                        if query!("SELECT FROM pogo_resources WHERE location=$1", med)
                            .fetch_optional(&mut transaction)
                            .await?
                            .is_none()
                        {
                            new_media.push(med);
                        }
                    }
                    new_media
                };

                // insert all media that aren't already within the database and get a list of all
                // their ids
                let mut ids: Vec<Uuid> = Vec::new();
                for med in media {
                    ids.push(
                        query_as!(
                            Id,
                            "INSERT INTO pogo_resources (location) VALUES ($1) RETURNING id",
                            med
                        )
                        .fetch_one(&mut transaction)
                        .await?
                        .into(),
                    );
                }

                // specify that all the new resource ids are attached to the task
                for resource_id in ids {
                    query!(
                        "INSERT INTO pogo_resource_mapping VALUES ($1,$2)",
                        task.id,
                        resource_id
                    )
                    .execute(&mut transaction)
                    .await?;
                }
                transaction.commit().await?;
                Ok(id)
            }
        }
    }
    async fn decode_task(
        &mut self,
        id: Uuid,
        login: &str,
    ) -> Result<Option<TaskVersioning>, sqlx::Error> {
        let partial_task = pg_fetch_task_frag_v1(self as &Self, id, login).await?;
        let media = pg_fetch_media_v1(self as &Self, id).await?;

        let children = pg_fetch_children_v1(self as &Self, id).await?;

        let is_root = pg_fetch_rooted_v1(self as &Self, id).await?;

        Ok(partial_task.map(
            move |TaskV1Fragment {
                      id,
                      title,
                      body,
                      progress,
                  }| {
                TaskVersioning::V1(TaskV1 {
                    id,
                    title,
                    body,
                    progress,
                    media,
                    children,
                    is_root,
                })
            },
        ))
    }
    async fn provide_identifiers(&mut self, login: &str) -> Result<Vec<Uuid>, sqlx::Error> {
        query_as!(Id, "SELECT id FROM pogo_tasks WHERE login=$1", login)
            .fetch_all(self as &Self)
            .await
            .map(|v| v.into_iter().map(Uuid::from).collect())
    }
    async fn relate_identifiers(
        &mut self,
        parent: Self::Identifier,
        child: Self::Identifier,
    ) -> Result<(), sqlx::Error> {
        query!("INSERT INTO pogo_relations VALUES ($1,$2)", parent, child)
            .execute(self as &Self)
            .await?;
        Ok(())
    }
}

/// fetches the media related to a task from the postgres db via the provided executor
pub async fn pg_fetch_media_v1<'a, E: PgExecutor<'a>>(
    exec: E,
    id: Uuid,
) -> Result<Vec<String>, sqlx::Error> {
    Ok(query!(
        r#"SELECT location 
            FROM pogo_resources 
            WHERE id = ANY(
                SELECT resource_id FROM pogo_resource_mapping 
                WHERE task_id=$1 
                AND resource_id IS NOT NULL)
            AND location IS NOT NULL"#,
        id
    )
    .fetch_all(exec)
    .await?
    .into_iter()
    .map(|row| unsafe { row.location.unwrap_unchecked() })
    .collect())
}
/// fetches the children of a task
pub async fn pg_fetch_children_v1<'a, E: PgExecutor<'a> + Copy>(
    exec: E,
    id: Uuid,
) -> Result<Vec<Uuid>, sqlx::Error> {
    let children: Vec<_> = query!(
        "SELECT child FROM pogo_relations WHERE parent=$1 AND child IS NOT NULL",
        id
    )
    .fetch_all(exec)
    .await?
    .into_iter()
    .map(|rec| unsafe { rec.child.unwrap_unchecked() })
    .collect();
    Ok(children)
}
async fn pg_fetch_task_frag_v1<'a, E: PgExecutor<'a>>(
    exec: E,
    id: Uuid,
    login: &str,
) -> Result<Option<TaskV1Fragment>, sqlx::Error> {
    Ok(query!(
        r#"SELECT * FROM pogo_tasks
                                WHERE id=$1
                                AND login=$2
                                AND title IS NOT NULL 
                                AND body IS NOT NULL 
                                AND progress IS NOT NULL"#,
        id,
        login
    )
    .fetch_optional(exec)
    .await?
    .map(|s| {
        // unsafe is for unwrap_unchecked which is fine because the SQL query already does
        // the checking for us
        unsafe {
            TaskV1Fragment {
                id: s.id.unwrap_unchecked(),
                title: s.title.unwrap_unchecked(),
                body: s.body.unwrap_unchecked(),
                progress: s.progress.unwrap_unchecked(),
            }
        }
    }))
}
pub async fn pg_fetch_rooted_v1<'a, E: PgExecutor<'a>>(
    exec: E,
    id: Uuid,
) -> Result<bool, sqlx::Error> {
    Ok(query!("SELECT FROM pogo_relations WHERE child=$1", id)
        .execute(exec)
        .await?
        .rows_affected()
        > 0)
}
