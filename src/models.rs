use serde::{Deserialize, Serialize};
use super::schema::posts;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct Chinese {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct QueryNewPost {
    pub title: String,
    pub body: String,
}