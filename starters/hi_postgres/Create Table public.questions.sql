-- DDL generated by Postico 1.5.20
-- Not all database features are supported. Do not use for backup.

-- Table Definition ----------------------------------------------

CREATE TABLE questions (
    id serial GENERATED ALWAYS AS IDENTITY PRIMARY KEY PRIMARY KEY,
    title varcha(255) NOT NULL,
    content text NOT NULL,
    tags text,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

