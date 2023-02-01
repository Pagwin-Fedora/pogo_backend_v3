use warp::reject;
use std::fmt::{Display,Formatter};

#[derive(Debug)]
pub enum Error {
    Sql(sqlx::Error),
    NonExistentTask(uuid::Uuid),
    WarpError(reject::Rejection),
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Sql(e)
    }
}
impl Display for Error{
    fn fmt(&self, fmt:&mut Formatter)->Result<(),std::fmt::Error>{
        match self{
            Self::Sql(_)=>fmt.write_str("Sql error"),
            Self::NonExistentTask(u)=> fmt.write_str(format!("{} doesn't exist",u).as_str()),
            Self::WarpError(_)=> fmt.write_str("Error in Warp")
        }
    }
}
impl std::error::Error for Error{
    fn description(&self)->&str{
        format!("{}",self).as_str()
    }
    fn cause(&self)->Option<&dyn std::error::Error>{
        match self{
            Self::Sql(e)=>Some(e),
            Self::NonExistentTask(_)=>None
        }
    }
    fn source(&self)->Option<&(dyn std::error::Error + 'static)>{
        match self{
            Self::Sql(e)=>e.source(),
            Self::NonExistentTask(_)=>None
        }
    }
}

// this may be bad
impl reject::Reject for Error {}

//impl From<Error> for warp::Rejection{
//    fn from(e:Error)->Self{
//        match e{
//            Error::Sql(e)=>{
//                reject::custom(e)
//            }
//        }
//    }
//}
