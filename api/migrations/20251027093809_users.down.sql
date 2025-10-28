-- Add down migration script here
DROP INDEX IF EXISTS idx_user_email;
DROP INDEX IF EXISTS idx_user_tag;
DROP TABLE IF EXISTS users;
