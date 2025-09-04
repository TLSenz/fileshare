-- Add up migration script here
CREATE TABLE file_to_link(
                             id SERIAL PRIMARY KEY ,
                             link TEXT,
                             filename TEXT
);