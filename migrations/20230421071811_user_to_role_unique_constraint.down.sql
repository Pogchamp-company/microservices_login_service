-- Add down migration script here
ALTER TABLE user_to_role DROP CONSTRAINT user_to_role_unique;
