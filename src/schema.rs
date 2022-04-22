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
    table! {
        request_books{
            user_id -> Integer,
            book_id -> Integer,
            book_title -> Text,
            book_status -> Bool,
        }
    }
    table! {
        books_borrowed{
            user_id -> Integer,
            book_id -> Integer,
            book_title -> Text,
            renwal_date -> String,
            renewal_status -> Bool,
            return_status -> Bool,
            late_fee -> Integer,
        }
    }
}

use self::schema::users;
use self::schema::book;
use self::schema::request_books;
use self::schema::books_borrowed;

#[derive(Queryable, Insertable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, Insertable, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub quantity: i32,
}

#[derive(Queryable, Insertable, Debug)]
pub struct request_books{
    pub user_id: i32,
    pub book_id: i32,
    pub book_title: String,
    pub book_status: Bool,
}

#[derive(Queryable, Debug)]
pub struct books_borrowed{
    pub user_id: i32,
    pub book_id: i32,
    pub book_title: String,
    pub renwal_date: String,
    pub renewal_status: Bool,
    pub return_status: Bool,
    pub late_fee: i32,
}