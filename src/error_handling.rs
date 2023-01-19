

#[derive(Debug)]
pub enum Error{Sql(sqlx::Error)}

impl From<sqlx::Error> for Error{
    fn from(e:sqlx::Error)->Self{
        Self::Sql(e)
    }
}
