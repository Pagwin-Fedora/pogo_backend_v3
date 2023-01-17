use std::rc;

//Not sure if using rc is the best choice atm or if I want to have the Rcs hold onto refcells but
//whatever
#[derive(Clone)]
pub struct TaskV1{
    pub id: u128,
    pub title:String,
    pub body:String,
    /// connected files/media/etc should be URLs
    pub connected:Vec<String>,
    //note our parents are the only things holding references to us so if they all die we die
    /// Parent nodes
    pub parents:Vec<rc::Weak<TaskV1>>,
    /// Child nodes
    pub children:Vec<rc::Rc<TaskV1>>
}

pub enum TaskVersioning{V1(TaskV1)}

/// Trait that any method of encoding and decoding tasks needs to implement
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
    fn encode_task<'a>(&mut self, task:TaskVersioning)->Result<Self::Identifier,Self::EncodingError>;
    fn decode_task<'a>(&mut self,id:Self::Identifier)->Result<TaskVersioning,Self::DecodingError>;
    fn provide_identifiers<'a>(&mut self)->Result<Vec<Self::Identifier>,Self::IdentityFetchError>;
}
