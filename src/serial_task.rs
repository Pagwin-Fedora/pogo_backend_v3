use serde::{Serialize,Deserialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize)]
pub struct TaskSerial {
    title: Option<String>,
    body: Option<String>,
    progress: Option<f32>,
    children: Option<Vec<Uuid>>,
    parents: Option<Vec<Uuid>>,
    media: Option<Vec<Uuid>>,
}
