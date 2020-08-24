CREATE TABLE posts
(
    id         SERIAL       NOT NULL PRIMARY KEY,
    title      VARCHAR(200) NOT NULL,
    body       TEXT         NOT NULL,
    is_deleted BOOLEAN      NOT NULL DEFAULT 'f',
    created_at TIMESTAMP    NOT NULL,
    deleted_at TIMESTAMP
)