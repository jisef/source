#[macro_use]
extern crate diesel;

mod schema;
mod models;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use log::error;
use crate::models::{Ausgabe, NewAusgabe};
use crate::schema::{ausgabe};
use crate::schema::ausgabe::dsl::*;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let mut conn = establish_connection();

    let new_user = NewAusgabe { amount: &3.44, category: "NEIN" };
    diesel::insert_into(ausgabe::table)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error creating new user");

    let result = ausgabe.load<Ausgabe>(&mut conn);


    /*let results = ausgabe
        .load::<Ausgabe>(&mut conn)
        .expect("Error loading users");

    println!("Users:");
    for ausgabeOf in results {
        println!("{}: {} ({})", ausgabeOf.id, ausgabeOf.amount, ausgabeOf.created_at);
    }*/
}