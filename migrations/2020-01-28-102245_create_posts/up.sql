CREATE TABLE posts (
    id SERIAL PRIMARY KEY ,
    title VARCHAR NOT NULL ,
    body VARCHAR NOT NULL ,
    published BOOLEAN NOT NULL DEFAULT 'f',
    user_id INTEGER NOT NULL REFERENCES users (id)
)