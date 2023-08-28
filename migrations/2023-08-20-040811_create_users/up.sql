-- Your SQL goes here
CREATE TABlE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    unique_id VARCHAR NOT NULL,
    UNIQUE (email),
    UNIQUE (username)
);

INSERT INTO users (username, email, password, unique_id)
VALUES ('placeholder', 'placeholder email', 'placeholder password', 'placeholder unique id');

ALTER TABLE to_do
ADD COLUMN user_id INTEGER DEFAULT 1 REFERENCES users(id) NOT NULL;