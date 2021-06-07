-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,

    UNIQUE(email),
    UNIQUE(username)
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    price TEXT NOT NULL
);