use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

mod schema;
mod models;

use self::models::User;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use self::schema::users::dsl::*;

    let mut connection = &mut establish_connection();

    
    
    println!("Displaying {} users", results.len());
    for user in results {
        println!("Name: {}, Email: {}", user.amount, user.id);
    }
}
