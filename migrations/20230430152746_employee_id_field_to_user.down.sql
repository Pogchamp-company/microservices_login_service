-- Add down migration script here
ALTER TABLE "user"
    DROP COLUMN employee_id;
