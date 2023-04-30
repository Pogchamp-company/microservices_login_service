-- Add up migration script here
ALTER TABLE "user"
    ADD COLUMN employee_id INTEGER;
DELETE FROM "user";
ALTER TABLE "user"
    ALTER COLUMN employee_id SET NOT NULL;
