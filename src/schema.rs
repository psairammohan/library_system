use diesel::{self, prelude::*};

mod schema {
    table! {
        users {
            id -> Integer,
            username -> Text,
        }
    }
    table! {
        books {
            id -> Integer,
            title -> Text,
            author -> Text,
            quantity -> Integer,
        }
    }
    table! {
        request_books (user_id, book_id) {
            user_id -> Integer,
            book_id -> Integer,
            book_title -> Text,
            book_status -> Bool,
        }
    }
    table! {
        books_borrowed (user_id, book_id) {
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
use self::schema::books;
use self::schema::request_books;
use self::schema::books_borrowed;

#[table_name = "users"]
#[derive(Queryable, Insertable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[table_name = "books"]
#[derive(Queryable, Insertable, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub quantity: i32,
}

#[table_name = "request_books"]
#[derive(Queryable, Insertable, Debug)]
pub struct request_book {
    pub user_id: i32,
    pub book_id: i32,
    pub book_title: String,
    pub book_status: Bool,
}

#[table_name = "books_borrowed"]
#[derive(Queryable, Debug)]
pub struct book_borrowed {
    pub user_id: i32,
    pub book_id: i32,
    pub book_title: String,
    pub renwal_date: String,
    pub renewal_status: Bool,
    pub return_status: Bool,
    pub late_fee: i32,
}

use self::schema::request_books::dsl::request_books as all_request_books;



