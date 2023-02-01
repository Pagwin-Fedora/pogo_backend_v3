use crate::error_handling as error;
use warp::Reply;
use warp::hyper::{Response,Body};
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct TaskCreation{
    id:uuid::Uuid
}
impl Reply for TaskCreation{
    fn into_response(self)->Response<Body>{
        Response::builder()
            .status(200)
            .body(Body::from("")).unwrap()
    }
}
impl From<uuid::Uuid> for TaskCreation{
    fn from(u:uuid::Uuid)->Self{
        Self{id:u}
    }
}
#[derive(Serialize,Deserialize)]
pub struct TaskDeletion;
impl Reply for TaskDeletion{
    fn into_response(self)->Response<Body>{
        Response::builder()
            .status(200)
            .body(Body::from(serde_json::to_string(&self).unwrap())).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskUpdate;
impl Reply for TaskUpdate{
    fn into_response(self)->Response<Body>{
        Response::builder()
            .status(200)
            .body(Body::from(serde_json::to_string(&self).unwrap())).unwrap()
    }
}

impl Reply for error::Error{
    fn into_response(self)->Response<Body>{
        let builder = Response::builder();
        match self{
            Self::Sql(_) | Self::WarpError(_)=>{
                builder.status(500)
            },
            Self::NonExistentTask(_)=>{
                builder.status(404)
            },
        }.body(Body::from(format!("{}",self))).unwrap()
    }
}
