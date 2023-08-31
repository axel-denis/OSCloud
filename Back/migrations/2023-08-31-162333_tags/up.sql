-- Your SQL goes here

CREATE TABLE tags
(
    id          int8 PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY,
    name        TEXT NOT NULL
)
