use lazy_static::lazy_static;
use sqlx::postgres::PgPool;
use crate::runtime;
use crate::error_handling::Error;
lazy_static!{
    static ref DB_CONNECTION:PgPool = connect_db().expect("Database connection failed");
}

fn connect_db()->Result<PgPool,Error>{
    runtime::get_handle().block_on(async {
        let db_opts = sqlx::postgres::PgConnectOptions::new()
            .host("localhost")
            .username("sqlx")
            //once I'm actually getting to work I should probably do this with an env var or a key
            //store of some kind so I'm not just yeeting the password into the binary
            .password(include_str!("postgres_passwd"))
            .port(5432)
            .database("sqlx")
            .application_name("pogo");
        Ok(sqlx::pool::PoolOptions::new()
            .connect_with(db_opts).await?)
    })

}
pub fn get_handle()->PgPool{
    DB_CONNECTION.clone()
}
