extern crate serde;
extern crate tokio;
extern crate warp;
extern crate sqlx;
extern crate lazy_static;
extern crate uuid;
extern crate async_trait;
mod task;
mod sql_impl;
mod runtime;
mod error_handling;

use sqlx::prelude::*;


fn main() {
    let _:Result<(),error_handling::Error> = runtime::get_handle().block_on(async {
        let db_opts = sqlx::postgres::PgConnectOptions::new()
            .host("localhost")
            .username("sqlx")
            //once I'm actually getting to work I should probably do this with an env var or a key
            //store of some kind so I'm not just yeeting the password into the binary
            .password(include_str!("postgres_passwd"))
            .port(5432)
            .database("sqlx")
            .application_name("pogo");
        let mut conn:sqlx::postgres::PgPool = sqlx::pool::PoolOptions::new()
            .connect_with(db_opts).await?;
        Ok(())
    });
    //println!("Hello, world!");
}
