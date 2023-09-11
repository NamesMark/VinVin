#[macro_use]
extern crate diesel;
extern crate dotenvy;
#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}