use crate::error_handling as error;
use crate::postgres_connection as pg_conn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use futures_util::stream;
use warp::Filter;
use warp::sse;
use warp::{Rejection, Reply};
use crate::http_ret::*;


async fn create_task(login: String) -> Result<TaskCreation, Rejection>{
    let db = pg_conn::get_handle();
    let id = sqlx::query!("INSERT INTO pogo_tasks DEFAULT VALUES RETURNING id")
        .fetch_one(&db)
        .await
        .map_err(error::Error::from)?;
    Ok(id.id.ok_or(error::Error::Sql(sqlx::Error::RowNotFound))?.into())
}
pub fn create_task_filter() -> impl warp::Filter<Extract = (TaskCreation,), Error = Rejection> {
    warp::put()
        .and(warp::path!("task"))
        .and(warp::cookie("login"))
        .and_then(create_task)
        //.map_async(create_task)
}

async fn delete_task(id: Uuid, login: String) -> Result<TaskDeletion,Rejection> {
    let db = pg_conn::get_handle();
    //delete
    let id_present:bool = sqlx::query!(
        "DELETE FROM pogo_tasks WHERE login=$1 AND id=$2",
        login,
        id
    )
    .execute(&db)
    .await
    .map_err(error::Error::from)?
    .rows_affected() > 0;
    if !id_present {
        Err(error::Error::NonExistentTask(id).into())
    }
    else{
        Ok(TaskDeletion)
    }
}
pub fn delete_task_filter() -> impl warp::Filter<Extract = (TaskDeletion,), Error = Rejection> {
    warp::delete()
        .and(warp::path!("task" / Uuid))
        .and(warp::cookie("login"))
        .and_then(delete_task)
}

#[derive(Serialize, Deserialize)]
struct TaskSerial {
    title: Option<String>,
    body: Option<String>,
    progress: Option<f32>,
    children: Option<Vec<Uuid>>,
    parents: Option<Vec<Uuid>>,
    media: Option<Vec<Uuid>>,
}

async fn update_task(id: Uuid, login: String, update: TaskSerial) -> Result<TaskUpdate,Rejection> {
    let conn = pg_conn::get_handle();
    let gotten_id = sqlx::query!("SELECT id FROM pogo_tasks WHERE id=$1 AND login=$2",id,login)
        .fetch_optional(&conn).await
        .map_err(error::Error::from)?;
    match gotten_id {
        Some(_)=>{
            //no type safety sadly
            let fields:Vec<TaskFields> = (&update).into()
            sqlx::query("UPDATE pogo_tasks SET ({})");
            Ok(TaskUpdate)
        },
        None=>Err(error::Error::NonExistentTask(id).into())
    }
}
pub fn update_task_filter() -> impl warp::Filter<Extract = (TaskUpdate,), Error = Rejection> {
    warp::patch()
        .and(warp::path!("task" / Uuid))
        .and(warp::cookie("login"))
        .and(warp::body::json())
        .and_then(update_task)
}

#[derive(Serialize, Deserialize)]
enum TaskFields {
    Id,
    Title,
    Body,
    Media,
    Parents,
    Children,
}
impl From<&TaskSerial> for Vec<TaskFields>{
    fn from(s:&TaskSerial)->Self{
        let mut container = Vec::with_capacity(5);
        container
    }
}
//async fn get_task(id: Uuid, login: String, fields: Vec<TaskFields>) -> EndPointResult<impl Reply> {
//    todo!();
//    Ok("get task")
//}
//pub fn get_task_filter() -> impl warp::Filter<Extract = impl Reply, Error = Rejection> {
//    warp::get()
//        .and(warp::path!("task" / Uuid))
//        .and(warp::cookie("login"))
//        .and(warp::body::json())
//        .and_then(get_task)
//}
//
//async fn get_relations(id: Uuid) -> EndPointResult<impl Reply> {
//    todo!();
//    Ok("get relation")
//}
//
//async fn add_media(media: String, login: String) -> EndPointResult<impl Reply> {
//    todo!();
//    Ok("add media")
//}
//pub fn add_media_filter() -> impl warp::Filter<Extract = impl Reply, Error = Rejection> {
//    warp::post()
//        .and(warp::path("media"))
//        .and(warp::body::json())
//        .and(warp::cookie("login"))
//        .and_then(add_media)
//}
//
//async fn delete_media(media: String, login: String) -> EndPointResult<impl Reply> {
//    todo!();
//    Ok("delete media")
//}
//pub fn delete_media_filter() -> impl warp::Filter<Extract = impl Reply, Error = Rejection> {
//    warp::delete()
//        .and(warp::path("media"))
//        .and(warp::body::json())
//        .and(warp::cookie("login"))
//        .and_then(delete_media)
//}
//fn subscribe(login: String) -> impl futures_util::Stream<Item = Result<sse::Event, Error>>{
//    stream::iter(vec![])
//}
//pub fn subscribe_filter() -> impl warp::Filter<Extract = impl Reply, Error = Rejection> {
//    warp::get()
//        .and(warp::path("subscribe"))
//        .and(warp::cookie("login"))
//        .map(|login| sse::reply(sse::keep_alive().stream(subscribe(login))))
//}
