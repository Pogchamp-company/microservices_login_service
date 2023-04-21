-- Add up migration script here
ALTER TABLE user_to_role ADD CONSTRAINT user_to_role_unique UNIQUE (user_email, role);
