-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    wine_name VARCHAR NOT NULL,
    wine_year INTEGER,
    region VARCHAR,
    look TEXT,
    nose TEXT,
    palate TEXT,
    tail TEXT
);