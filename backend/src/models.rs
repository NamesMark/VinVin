#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub profile_pic: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub user_id: Option<i32>,
    pub wine_name: String,
    pub wine_year: Option<i32>,
    pub region: Option<String>,
    pub look: Option<String>,
    pub nose: Option<String>,
    pub palate: Option<String>,
    pub tail: Option<String>,
}