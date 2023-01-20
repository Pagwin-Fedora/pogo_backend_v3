
// These allows are to suppress a flood of warnings from the end_points module that from what I can
// tell aren't fixable in this code base
#![allow(type_alias_bounds)]
#![allow(opaque_hidden_inferred_bound)]


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
mod end_points;
mod postgres_connection;
use sqlx::prelude::*;


fn main() {
    // initialize the DB before we
    let _ = postgres_connection::get_handle();
    let _:Result<(),error_handling::Error> = runtime::get_handle().block_on(async {
        Ok(())
    });
    //println!("Hello, world!");
}
