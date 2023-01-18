use crate::task::*;
use sqlx::{query,query_as};
use sqlx::{Row,Column};
use uuid::Uuid;
use async_trait::async_trait;

struct TaskV1Schema{
    id: Uuid,
    title:String,
    body:String,
    progress: f32,
}

#[derive(sqlx::Encode, sqlx::Decode)]
struct ParentChild{
    parent: Uuid,
    child: Uuid
}

impl From<&TaskV1> for TaskV1Schema{
    fn from(t:&TaskV1)->Self{
        Self{
            id:t.id,
            title:t.title,
            body:t.body,
            progress: t.progress
        }
    }
}

struct Id{
    id:Option<Uuid>
}
impl From<Id> for Uuid {
    fn from(i:Id)->Self{
        i.id.unwrap_or(Uuid::from_u128(0))
    }
}

#[async_trait]
impl TaskEncoder for sqlx::Pool<sqlx::postgres::Postgres> {
    type Identifier = Uuid;
    type DecodingError = sqlx::Error;
    type EncodingError = sqlx::Error;
    type IdentityFetchError = sqlx::Error;
    async fn encode_task(&mut self, task:TaskVersioning)->Result<Self::Identifier,Self::EncodingError>{
        match task {
            TaskVersioning::V1(task)=>{
                let TaskV1Schema {id,title,body,progress} = (&task).into();
                let mut transaction = self.begin().await?;
                query!("DELETE FROM pogo_tasks WHERE id=$1",task.id).execute(&mut transaction).await?;
                
                query!("INSERT INTO pogo_tasks VALUES ($1,$2,$3,$4)", id,title,body,progress).execute(&mut transaction).await?;
                transaction.commit().await?;
                Ok(task.id)
            }
        }
    }
    async fn decode_task(&mut self, id:Uuid)->Result<Option<TaskVersioning>,sqlx::Error>{
        let potential_task = query!("SELECT * FROM pogo_tasks WHERE id=$1 AND title IS NOT NULL AND body IS NOT NULL AND progress IS NOT NULL", id).fetch_optional(self as &Self).await?.map(|s|{
                // unsafe is for unwrap_unchecked which is fine because the SQL query already does
                // the checking for us
                unsafe {
                    TaskVersioning::V1(TaskV1{
                        id:s.id.unwrap_unchecked(),
                        title:s.title.unwrap_unchecked(),
                        body:s.body.unwrap_unchecked(),
                        progress:s.progress.unwrap_unchecked(),
                        media:Vec::new(),
                        parents: Vec::new(),
                        children: Vec::new()
                    })
                }
            }
        );
        match potential_task {
            Some(TaskVersioning::V1(mut task))=>{
                let media_ids = query!("SELECT resource_id FROM pogo_resource_mapping WHERE task_id=$1 AND resource_id IS NOT NULL",task.id).fetch_all(self as &Self).await?.into_iter().map(|r|unsafe{r.resource_id.unwrap_unchecked()}).collect::<Vec<Uuid>>();
                let media:Vec<String> = query!("SELECT location FROM pogo_resources WHERE id = ANY($1) AND location IS NOT NULL",media_ids.as_slice())
                    .fetch_all(self as &Self).await?.into_iter()
                    .map(|row|row.location.unwrap_unchecked())
                    .collect();
                task.media = media;
                Ok(Some(TaskVersioning::V1(task)))
            },
            None=>Ok(None)
        }
    }
    async fn provide_identifiers(&mut self)->Result<Vec<Uuid>,sqlx::Error>{
        query_as!(Id,"SELECT id FROM pogo_tasks")
            .fetch_all(self as &Self).await
            .map(|v|v.into_iter().map(Uuid::from).collect())
    }
}
