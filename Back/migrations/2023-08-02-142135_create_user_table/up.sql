-- Your SQL goes here

CREATE TYPE role AS ENUM ('admin', 'user');

CREATE TABLE users
(
    id          SERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    password    TEXT NOT NULL,
    user_role   role NOT NULL
);
