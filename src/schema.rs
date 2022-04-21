use diesel::{self, prelude::*};

mod schema {
    table! {
        users {
            id -> Integer,
            username -> Text,
        }
    }
    table! {
        book {
            id -> Integer,
            title -> Text,
            author -> Text,
            quantity -> Integer,
        }
    }
}

use self::schema::users;
use self::schema::book;

#[derive(Queryable, Insertable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub quantity: i32,
}

