use diesel::prelude::*;

use rocket::{http::Status, State};
use rocket::{Route, routes};
use rocket_contrib::json::Json;
// use rocket::request::Query;
use rocket::data::{FromData, Outcome};
use rocket::Data;
use rocket::request::Form;
use rocket::request::Options;
use rocket::http::RawStr;

use crate::models::{Post, User};
use crate::db_connection::DbConn;
use crate::schema::{users, posts};

use async_trait::async_trait;

#[derive(Debug, FromForm)]
pub struct Pagination {
    page: Option<i64>,
    page_size: Option<i64>,
}

// #[async_trait]
// impl<'r> FromData<'r> for Pagination {
//     type Error = std::io::Error;

//     async fn from_data(_req: &mut rocket::Request<'_>, data: Data) -> rocket::data::Outcome<Self, Self::Error> {
//         let query = data.open().stream_to_string().await.expect("Failed to read query");
//         rocket::request::Form::parse(query, rocket::request::Options::default()).await
//     }
// }

#[get("/posts?<pagination..>")]
pub fn all_posts(conn: DbConn, pagination: Pagination) -> Result<Json<Vec<Post>>, Status> {
    let page = pagination.page.unwrap_or(0);
    let page_size = pagination.page_size.unwrap_or(20); // defaulting to 20 posts per page
    
    let results: Result<Vec<Post>, diesel::result::Error> = posts::table
        .order(posts::created_at.desc()) // Order by created_at in descending order
        .limit(page_size)
        .offset(page * page_size)
        .load(conn);
    
    match results {
        Ok(posts) => Ok(Json(posts)),
        Err(_) => Err(Status::InternalServerError)
    }
}

// Fetch a single post by ID
#[get("/post/<id>")]
pub fn get_post(conn: DbConn, id: i32) -> Result<Json<Post>, Status> {
    use crate::schema::posts::dsl::*;

    let post = posts.find(id).first::<Post>(&*conn)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(post))
}

// Fetch a single user by ID
#[get("/users/<id>")]
pub fn get_user(conn: DbConn, id: i32) -> Result<Json<User>, Status> {
    use crate::schema::users::dsl::*;

    let user = users.find(id).first::<User>(&*conn)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(user))
}

// Add a new post
#[post("/posts", format = "json", data = "<new_post>")]
pub fn create_post(conn: DbConn, new_post: Json<Post>) -> Result<Status, Status> {
    use crate::schema::posts::dsl::*;

    diesel::insert_into(posts).values(&new_post.into_inner())
        .execute(&*conn)
        .map(|_| Status::Created)
        .map_err(|_| Status::InternalServerError)
}

// Add a new user
#[post("/users", format = "json", data = "<new_user>")]
pub fn create_user(conn: DbConn, new_user: Json<User>) -> Result<Status, Status> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users).values(&new_user.into_inner())
        .execute(&*conn)
        .map(|_| Status::Created)
        .map_err(|_| Status::InternalServerError)
}