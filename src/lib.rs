#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn db_connection() -> PgConnection {

    dotenv().ok();

    let database_url = env::var("DATABSE_URL")
    .expect("DATABSE_URL must be set");

    PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))

}
