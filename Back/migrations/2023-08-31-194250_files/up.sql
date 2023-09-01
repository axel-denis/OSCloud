-- Your SQL goes here

CREATE TABLE files
(
    path        TEXT PRIMARY KEY NOT NULL,
    tags        int8[]
)
