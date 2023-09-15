-- This file should undo anything in `up.sql`
ALTER TABLE posts
ALTER COLUMN created_at TYPE timestamp;
ALTER TABLE posts DROP COLUMN created_at;