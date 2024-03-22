-- Your SQL goes here

CREATE TABLE user_mounts_points
(
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER NOT NULL REFERENCES users(id),
    path        TEXT NOT NULL
);
