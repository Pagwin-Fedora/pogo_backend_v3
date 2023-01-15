use std::rc;

//Not sure if using rc is the best choice atm or if I want to have the Rcs hold onto refcells but
//whatever
pub struct TaskV1{
    title:String,
    body:String,
    /// connected files/media/etc should be URLs
    connected:Vec<String>,
    //note our parents are the only things holding references to us so if they all die we die
    /// Parent nodes
    parents:Vec<rc::Weak<TaskV1>>,
    /// Child nodes
    children:Vec<rc::Rc<TaskV1>>
}
pub enum TaskVersioning{V1(TaskV1)}

/// Trait that any method of encoding and decoding tasks needs to implement
pub trait TaskEncoder{
    /// The type that can be gotten from a call to either provide_identifiers or
    /// encode_task and if a value of it is gotten that way then should be usable with decode_task
    /// to retrieve the original task it must be serializable with serde due to it being the value
    /// passed around when working with tasks potentially onto disk or over network
    type Identifier:serde::Serialize + serde::Deserialize;
    type EncodingError;
    type DecodingError;
    type IdentityFetchError;
    fn encode_task(&mut self, task:TaskVersioning)->Result<Self::Identifier,Self::EncodingError>;
    fn decode_task(&mut self,id:Self::Identifier)->Result<TaskVersioning,Self::DecodingError>;
    fn provide_identifiers(&mut self)->Result<Vec<Self::Identifier>,Self::IdentityFetchError>;
}
