#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate diesel;
extern crate dotenvy;
#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;

mod db_connection;
mod routes;
mod models;
mod schema;

use crate::db_connection::DbConn;

// use rocket::fairing::AdHoc;
// use db_connection::{establish_connection, PgPool};


fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .mount("/api", routes![
            routes::all_posts, 
            routes::get_post, 
            routes::get_user, 
            routes::create_post, 
            routes::create_user
        ]);
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}