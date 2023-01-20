use warp::{Reply,Rejection};
use uuid::Uuid;
use serde::{Serialize,Deserialize};
use crate::task::TaskEncoder;
use warp::reject::Reject;
use warp::Filter;
use crate::error_handling::Error;
use crate::postgres_connection as pg_conn;

type EndPointResult<R1:Reply,R2:Reply> = Result<R1, R2>;

async fn create_task(login:String) -> EndPointResult<impl Reply, Rejection>{
    let db = pg_conn::get_handle();
    let id = sqlx::query!("INSERT INTO pogo_tasks DEFAULT VALUES RETURNING id").fetch_one(&db).await.map_err(Error::from)?;
    Ok(id.id.unwrap().to_string())
}
pub fn create_task_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::post()
        .and(warp::path!("task"))
        .and(warp::body::json())
        .and_then(create_task)
}

async fn delete_task(id:Uuid, login:String) -> EndPointResult<impl Reply, Rejection>{
    Ok("delete task")
}
pub fn delete_task_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::delete()
        .and(warp::path!("task" / Uuid))
        .and(warp::cookie("login"))
        .and_then(delete_task)
}

#[derive(Serialize,Deserialize)]
struct TaskUpdate{
    title: Option<String>,
    body: Option<String>,
    progress: Option<f32>
}
async fn update_task(id:Uuid, login:String,update: TaskUpdate) -> EndPointResult<impl Reply, Rejection>{
    Ok("update task")
}
pub fn update_task_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::patch()
        .and(warp::path!("task"/Uuid))
        .and(warp::cookie("login"))
        .and(warp::body::json())
        .and_then(update_task)
}

async fn add_relation(parent:Uuid, child:Uuid, login:String) -> EndPointResult<impl Reply, Rejection>{
    Ok("add relation")
}
pub fn add_relation_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::post()
        .and(warp::path!("relation"/Uuid/Uuid))
        .and(warp::cookie("login"))
        .and_then(add_relation)
}

async fn delete_relation(parent:Uuid, child:Uuid, login:String)-> EndPointResult<impl Reply, Rejection>{
    Ok("delete relation")
}
pub fn delete_relation_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::delete()
        .and(warp::path!("relation"/Uuid/Uuid))
        .and(warp::cookie("login"))
        .and_then(delete_relation)
}

async fn add_media(media:String, login:String)->EndPointResult<impl Reply, Rejection>{
    Ok("add media")
}
pub fn add_media_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::post()
        .and(warp::path("media"))
        .and(warp::body::json())
        .and(warp::cookie("login"))
        .and_then(add_media)
}

async fn delete_media(media:String, login:String)->EndPointResult<impl Reply, Rejection>{
    Ok("delete media")
}
pub fn delete_media_filter()->impl warp::Filter<Extract = impl Reply, Error = Rejection>{
    warp::delete()
        .and(warp::path("media"))
        .and(warp::body::json())
        .and(warp::cookie("login"))
        .and_then(delete_media)
}
