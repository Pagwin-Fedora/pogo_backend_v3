use warp::{Reply,Rejection};
use uuid::Uuid;
use serde::{Serialize,Deserialize};
use crate::task::TaskEncoder;
use warp::Filter;
use warp::reply;
use crate::error_handling::Error;
use crate::postgres_connection as pg_conn;

type EndPointResult<R:Reply> = Result<R, Rejection>;

async fn create_task(login:String) -> EndPointResult<impl Reply>{
    let db = pg_conn::get_handle();
    let id = sqlx::query!("INSERT INTO pogo_tasks DEFAULT VALUES RETURNING id").fetch_one(&db).await.map_err(Error::from)?;
    todo!();
    Ok(id.id.unwrap().to_string())
}
pub fn create_task_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::put()
        .and(warp::path!("task"))
        .and(warp::cookie("login"))
        .and_then(create_task)
}

async fn delete_task(id:Uuid, login:String) -> EndPointResult<impl Reply>{
    todo!();
    Ok("delete task")
}
pub fn delete_task_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::delete()
        .and(warp::path!("task" / Uuid))
        .and(warp::cookie("login"))
        .and_then(delete_task)
}

#[derive(Serialize,Deserialize)]
struct TaskSerial{
    title: Option<String>,
    body: Option<String>,
    progress: Option<f32>,
    children: Option<Vec<Uuid>>,
    parents: Option<Vec<Uuid>>,
    media: Option<Vec<Uuid>>
}
async fn update_task(id:Uuid, login:String,update: TaskSerial) -> EndPointResult<impl Reply>{
    todo!();
    Ok("update task")
}
pub fn update_task_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::patch()
        .and(warp::path!("task"/Uuid))
        .and(warp::cookie("login"))
        .and(warp::body::json())
        .and_then(update_task)
}

#[derive(Serialize,Deserialize)]
enum TaskFields{Id,Title,Body,Media,Parents,Children}
async fn get_task(id:Uuid, login:String, fields:Vec<TaskFields>) -> EndPointResult<impl Reply>{
    todo!();
    Ok("get task")
}
pub fn get_task_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::get()
        .and(warp::path!("task"/Uuid))
        .and(warp::cookie("login"))
        .and(warp::body::json())
        .and_then(get_task)
}

async fn get_relations(id: Uuid)->EndPointResult<impl Reply> {
    todo!();
    Ok("get relation")
}

async fn add_media(media:String, login:String)->EndPointResult<impl Reply>{
    todo!();
    Ok("add media")
}
pub fn add_media_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::post()
        .and(warp::path("media"))
        .and(warp::body::json())
        .and(warp::cookie("login"))
        .and_then(add_media)
}

async fn delete_media(media:String, login:String)->EndPointResult<impl Reply>{
    todo!();
    Ok("delete media")
}
pub fn delete_media_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::delete()
        .and(warp::path("media"))
        .and(warp::body::json())
        .and(warp::cookie("login"))
        .and_then(delete_media)
}
async fn subscribe(content_type:String,login:String)-> impl Reply{
    reply
}
pub fn subscribe_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::get()
        .map()
        .and(warp::header("Content-Type"))
}
