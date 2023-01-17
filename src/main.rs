extern crate serde;
extern crate tokio;
extern crate rocket;
extern crate sqlx;
extern crate lazy_static;
extern crate uuid;
mod schema;
mod sql_impl;

use sqlx::prelude::*;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static!{
    static ref RUNTIME:Runtime = Runtime::new().expect("Async runtime creation failed");
}

struct tmp{
    id:Option<uuid::Uuid>
}
impl From<tmp> for Option<uuid::Uuid>{
    fn from(t:tmp)->Self{
        t.id
    }
}
impl From<Option<uuid::Uuid>> for tmp {
    fn from(id:Option<uuid::Uuid>)->Self{
        tmp{id}
    }
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

        let obj = sqlx::query_as!(tmp,"SELECT id FROM pogo_tasks WHERE id='67d17a45-7b99-46be-ac85-338e2c8f0d4d';")
            .fetch_optional(&mut conn).await
            .unwrap_or(None)
            .unwrap_or(Some(uuid::Uuid::from_u128(0)).into());
        sql::query!("UPDATE")
        //sqlx::query!("SELECT asdf as id").execute(&mut conn).await.unwrap();
    });
    //println!("Hello, world!");
}
