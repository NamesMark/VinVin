#[get("/posts")]
fn all_posts(conn: DbConn) -> Json<Vec<Post>> {
}