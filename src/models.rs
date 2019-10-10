use serde::{Deserialize, Serialize};
use super::schema::posts;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="posts"]
pub struct UpdatePost {
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Chinese {
    pub text: String,
}