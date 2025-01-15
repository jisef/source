use diesel::prelude::*;
use diesel::Insertable;
use crate::schema::ausgabe;


#[derive(Queryable)]
pub struct Ausgabe {
    pub id: i32,
    pub amount: f64,
    pub category: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = ausgabe)]
pub struct NewAusgabe<'a> {
    pub category: &'a str,
    pub amount: &'a f64,
}