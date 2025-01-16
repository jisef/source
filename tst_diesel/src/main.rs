pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::posts;
use self::models::{NewPost, Post};

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
fn main() {
    use self::schema::posts::dsl::*;
    let sigma = &mut establish_connection();
    let new_post = NewPost { title: "Hello, world!", body: "This is my first blog post!" };
    diesel::insert_into(posts::table()).values(&new_post).returning(Post::as_returning()).get_result(sigma).expect("Error saving new post");
    
    
    
    let result = posts
        .limit(5)
        .select(models::Post::as_select())
        .load(sigma)
        .expect("Error loading posts");
    for simga in result {
        println!("{} \n publiched:{}", simga.title, simga.published);
    }
}


