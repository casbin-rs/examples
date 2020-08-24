CREATE TABLE users
(
    id         SERIAL       NOT NULL PRIMARY KEY,
    username   VARCHAR(32)  NOT NULL,
    email      VARCHAR(100) NOT NULL,
    password   VARCHAR(200) NOT NULL,
    role       INT          NOT NULL,
    is_deleted BOOLEAN      NOT NULL DEFAULT 'f',
    created_at TIMESTAMP    NOT NULL,
    deleted_at TIMESTAMP
);