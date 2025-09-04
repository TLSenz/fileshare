-- Add up migration script here
CREATE TABLE file_to_link(
                             id INTEGER PRIMARY KEY AUTOINCREMENT,
                             link TEXT,
                             filename TEXT
);