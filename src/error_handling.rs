use warp::reject;

#[derive(Debug)]
pub enum Error {
    Sql(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Sql(e)
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
