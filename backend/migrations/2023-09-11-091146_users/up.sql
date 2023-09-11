-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    profile_pic VARCHAR,
    location VARCHAR,
    bio TEXT
);