CREATE TABLE todos
(
    id       SERIAL PRIMARY KEY NOT NULL,
    user_id  INT                NOT NULL,
    title    VARCHAR            NOT NULL,
    finished BOOLEAN            NOT NULL DEFAULT 'f'
);

CREATE TABLE users
(
    id       SERIAL PRIMARY KEY,
    name     VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT 'f'
);

INSERT INTO users
VALUES (0, 'Alice', '123', 't');
INSERT INTO users
VALUES (1, 'Bob', '123', 'f');