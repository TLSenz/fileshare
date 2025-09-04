-- Add up migration script here
CREATE TABLE users (
                       id INTEGER PRIMARY KEY AUTOINCREMENT,
                       name TEXT NOT NULL,
                       email TEXT UNIQUE NOT NULL,
                       password TEXT NOT NULL
);


