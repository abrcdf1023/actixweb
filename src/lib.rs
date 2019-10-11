#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use models::{Post, NewPost, UpdatePost};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().max_size(8).build(manager).expect("Failed to create pool.")
}

pub fn create_post<'a>(conn: &PgConnection, data: NewPost) -> Post {
    use schema::posts;

    diesel::insert_into(posts::table)
        .values(&data)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn read_posts(conn: &PgConnection) -> Vec<Post> {
    use schema::posts::dsl::{posts};

    posts.load::<Post>(conn)
        .expect("Error loading posts")
}

pub fn update_post<'a>(conn: &PgConnection, id: &'a str, data: UpdatePost) -> Post {
    use schema::posts::dsl::{posts, published, body, title};
    let id = id.parse::<i32>().expect("Invalid ID");

    diesel::update(posts.find(id))
        .set(
            (published.eq(data.published), title.eq(data.title), body.eq(data.body))
        )
        .get_result::<Post>(conn)
        .expect(&format!("Unable to find post {}", id))
}

pub fn delete_post<'a>(conn: &PgConnection, id: &'a str) -> String {
    use schema::posts::dsl::{posts};
    let id = id.parse::<i32>().expect("Invalid ID");

    diesel::delete(posts.find(id))
        .execute(conn)
        .expect("Error deleting posts");

    format!("Post id {} deleted success.", id)
}