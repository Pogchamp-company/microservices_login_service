-- Add up migration script here
create table "user" (
    email varchar(100) primary key,
    password varchar not null
);