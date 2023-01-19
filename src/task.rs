use std::sync;
use uuid::Uuid;
use async_trait::async_trait;
//Not sure if using rc is the best choice atm or if I want to have the Rcs hold onto refcells but
//whatever
#[derive(Clone)]
pub struct TaskV1{
    pub id: Uuid,
    pub title:String,
    pub body:String,
    pub progress: f32,
    /// connected files/media/etc should be URLs
    pub media:Vec<String>,
    /// Child nodes
    pub children:Vec<Uuid>,
    pub is_root:bool,
}

pub enum TaskVersioning{V1(TaskV1)}

/// Trait that any method of encoding and decoding tasks needs to implement
#[async_trait]
pub trait TaskEncoder{
    /// The type that can be gotten from a call to either provide_identifiers or
    /// encode_task and if a value of it is gotten that way then should be usable with decode_task
    /// to retrieve the original task it must be serializable with serde due to it being the value
    /// passed around when working with tasks potentially onto disk or over network
    //Specifying DeserializeOwned may be a problem in the future if I need to deal with types with
    //lifetimes but until then this is good
    type Identifier:serde::Serialize + serde::de::DeserializeOwned;
    type EncodingError;
    type DecodingError;
    type IdentityFetchError;
    async fn encode_task(&mut self, task:TaskVersioning, login:&str)->Result<Self::Identifier,Self::EncodingError>;
    async fn decode_task(&mut self,id:Self::Identifier, login:&str)->Result<Option<TaskVersioning>,Self::DecodingError>;
    async fn provide_identifiers(&mut self, login:&str)->Result<Vec<Self::Identifier>,Self::IdentityFetchError>;
    async fn relate_identifiers(&mut self, parent:Self::Identifier, child:Self::Identifier)->Result<(),sqlx::Error>;
}
