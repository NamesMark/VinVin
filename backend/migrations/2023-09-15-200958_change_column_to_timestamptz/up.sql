-- Your SQL goes here
ALTER TABLE posts ADD COLUMN created_at TIMESTAMP;
ALTER TABLE posts
ALTER COLUMN created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC';