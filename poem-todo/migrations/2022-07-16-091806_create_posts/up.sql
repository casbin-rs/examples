CREATE TABLE todos
(
    id       SERIAL PRIMARY KEY NOT NULL,
    user_id  INT                NOT NULL,
    title    VARCHAR            NOT NULL,
    finished BOOLEAN            NOT NULL DEFAULT 'f'
);

CREATE TABLE users
(
    id       SERIAL PRIMARY KEY NOT NULL,
    name     VARCHAR            NOT NULL,
    password VARCHAR            NOT NULL,
    is_admin BOOLEAN            NOT NULL DEFAULT 'f'
);

INSERT INTO users
VALUES (0, 'alice', '123', 't');
INSERT INTO users
VALUES (1, 'bob', '123', 'f');