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

use sqlx::prelude::*;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;


#[derive(sqlx::Encode,sqlx::Decode)]
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
enum MyErrors{Sql(sqlx::Error)}
impl From<sqlx::Error> for MyErrors{
    fn from(e:sqlx::Error)->Self{
        Self::Sql(e)
    }
}
fn main() {
    let _:Result<(),MyErrors> = runtime::get_handle().block_on(async {
        let db_opts = sqlx::postgres::PgConnectOptions::new()
            .host("localhost")
            .username("sqlx")
            //once I'm actually getting to work I should probably do this with an env var or a key
            //store of some kind so I'm not just yeeting the password into the binary
            .password(include_str!("postgres_passwd"))
            .port(5432)
            .database("sqlx")
            .application_name("pogo");
        let conn = sqlx::pool::PoolOptions::new()
            .connect_with(db_opts).await?;

        let obj = sqlx::query_as!(tmp,"SELECT id FROM pogo_tasks WHERE id='67d17a45-7b99-46be-ac85-338e2c8f0d4d';")
            .fetch_optional(&conn).await?
            .unwrap_or(Some(uuid::Uuid::from_u128(0)).into());

        sqlx::query!("UPDATE pogo_tasks SET id=$1,title='hello', body='sqlx' WHERE id=$1",obj.id).execute(&conn).await?;
        //sqlx::query!("SELECT asdf as id").execute(&mut conn).await.unwrap();
        Ok(())
    });
    //println!("Hello, world!");
}
