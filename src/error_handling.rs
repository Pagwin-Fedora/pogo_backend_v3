use warp::reject;
use std::fmt::{Display,Formatter};

#[derive(Debug)]
pub enum Error {
    Sql(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Sql(e)
    }
}
impl Display for Error{
    fn fmt(&self, fmt:&mut Formatter)->Result<(),std::fmt::Error>{
        match self{
            Self::Sql(e)=>e.fmt(fmt)
        }
    }
}
impl std::error::Error for Error{
    fn description(&self)->&str{
        match self{
            Self::Sql(e)=>e.description()
        }
    }
    fn cause(&self)->Option<&dyn std::error::Error>{
        match self{
            Self::Sql(e)=>Some(e),
            _=>None
        }
    }
    fn source(&self)->Option<&(dyn std::error::Error + 'static)>{
        match self{
            Self::Sql(e)=>e.source()
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
