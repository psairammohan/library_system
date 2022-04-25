DROP TABLE users;
CREATE TABLE users (
    id INTEGER AUTO_INCREMENT NOT NULL,
    username TEXT NOT NULL UNIQUE
);

DROP TABLE books;
CREATE TABLE books (
    id INTEGER AUTO_INCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
);

DROP TABLE book_genre;
CREATE TABLE book_genre (
    book_title TEXT NOT NULL,
    genre TEXT NOT NULL   
);

DROP TABLE request_books;
CREATE TABLE request_books (
    user_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    book_title TEXT NOT NULL,
    book_status BOOL DEFAULT 'f', 

    CONSTRAINT no_dup_votes UNIQUE (user_id, book_id)
    FOREIGN KEY(user_id) REFERENCES users(id)
    FOREIGN KEY(book_id) REFERENCES books(id)
);

DROP TABLE books_borrowed;
CREATE TABLE books_borrowed (
    user_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    book_title TEXT NOT NULL,
    renewal_date varchar(10),
    renewal_status BOOL DEFAULT 'f',
    return_status BOOL DEFAULT 'f',
    late_fee INTEGER,

    CONSTRAINT no_dup_votes UNIQUE (user_id, book_id)
    FOREIGN KEY(user_id) REFERENCES users(id)
    FOREIGN KEY(book_id) REFERENCES books(id)
);

