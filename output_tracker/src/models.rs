use diesel::{Selectable,Queryable};
use crate::schema::users;

#[derive(Queryable, Selectable)]
pub struct User {
    pub id: i32,
    pub amount: f32,
    pub reason: String,
}
