-- Add up migration script here
CREATE TYPE user_role AS ENUM ('TaskManager', 'HumanResources', 'director');

CREATE TABLE user_to_role (
    user_email varchar(100) not null,
    role user_role,

    CONSTRAINT fk_user
        FOREIGN KEY (user_email)
            REFERENCES "user"(email)
            ON DELETE CASCADE
);