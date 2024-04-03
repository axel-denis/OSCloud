-- Your SQL goes here

CREATE TYPE share_type AS ENUM ('public', 'users');

CREATE TABLE files_shares
(
    id              SERIAL PRIMARY KEY,
    owner_user_id   INTEGER NOT NULL REFERENCES users(id),
    path            TEXT NOT NULL,
    share_type      share_type NOT NULL,
    link            TEXT NOT NULL
);

CREATE TABLE files_shares_users
(
    id              SERIAL PRIMARY KEY,
    file_share_id   INTEGER NOT NULL REFERENCES files_shares(id),
    shared_to       INTEGER NOT NULL REFERENCES users(id)
)
