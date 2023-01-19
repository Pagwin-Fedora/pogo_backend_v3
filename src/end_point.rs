use warp::{Reply,Rejection};
use uuid::Uuid;
use serde::{Serialize,Deserialize};

type EndPointResult = Result<impl Reply, Rejection>;

async fn create_task(id:Uuid, login:String) -> EndPointResult{
    Ok("Hello there")
}
pub fn create_task_filter()->impl warp::Filter<Extract = (), Error = Rejection>{
    warp::post()
        .and_then(create_task)
}

async fn delete_task(id:Uuid, login:String) -> EndPointResult{
    Ok("delete")
}

#[derive(Serialize,Deserialize)]
struct TaskUpdate{
    title: Option<String>,
    body: Option<String>,
    progress: Option<f32>
}
async fn update_task(id:Uuid, login:String,update: TaskUpdate) -> EndPointResult{
    Ok("update")
}

async fn add_relation(parent:Uuid, child:Uuid, login:String) -> EndPointResult {
    Ok("added")
}

async fn del_relation(parent:Uuid, child:Uuid, login:String)-> EndpointResult {
    Ok("del")
}

async fn add_media(media:String, login:String)->EndPointResult{
    Ok("add med")
}
async fn del_media(media:String, login:String)->EndPointResult{
    Ok("del med")
}

