-- Your SQL goes here

CREATE TYPE role AS ENUM ('admin', 'user');

CREATE TABLE users
(
    id          int8 PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY,
    name        TEXT NOT NULL,
    password    TEXT NOT NULL,
    user_role   role NOT NULL
)
