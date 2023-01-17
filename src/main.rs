extern crate serde;
extern crate tokio;
extern crate rocket;
extern crate sqlx;
extern crate lazy_static;
mod schema;

use sqlx::prelude::*;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static!{
    static ref RUNTIME:Runtime = Runtime::new().expect("Async runtime creation failed");
}

fn main() {
    RUNTIME.block_on(async {
        let mut conn = sqlx::postgres::PgConnectOptions::new()
            .host("localhost")
            .username("sqlx")
            //once I'm actually getting to work I should probably do this with an env var or a key
            //store of some kind so I'm not just yeeting the password into the binary
            .password(include_str!("postgres_passwd"))
            .port(5432)
            .database("sqlx")
            .application_name("sqlx_test")
            .connect().await
            .expect("Connection failed");
        sqlx::query!("SELECT asdf as id").execute(&mut conn).await.unwrap();
    });
    //println!("Hello, world!");
}
