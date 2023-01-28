// These allows are to suppress a flood of warnings from the end_points module that from what I can
// tell aren't fixable in this code base
#![allow(type_alias_bounds)]
#![allow(opaque_hidden_inferred_bound)]

extern crate async_trait;
extern crate futures_util;
extern crate lazy_static;
extern crate serde;
extern crate sqlx;
extern crate tokio;
extern crate uuid;
extern crate warp;
mod end_points;
mod error_handling;
mod postgres_connection;
mod runtime;
mod sql_impl;
mod task;
use sqlx::prelude::*;

fn main() {
    // initialize the DB before we use it
    let _ = postgres_connection::get_handle();
    let _: Result<(), error_handling::Error> = runtime::get_handle().block_on(async { Ok(()) });
    //println!("Hello, world!");
}
