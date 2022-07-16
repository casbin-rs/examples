-- Your SQL goes here
CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(32)  NOT NULL,
    email      VARCHAR(100) NOT NULL,
    password   VARCHAR(200) NOT NULL,
    role       VARCHAR(32)  NOT NULL
);

INSERT INTO users
VALUES (0, 'John', 'john@john.com', 'imjohn', 'doctor');
INSERT INTO users
VALUES (1, 'Sam', 'sam@sam.com', 'imsam', 'patient'); 