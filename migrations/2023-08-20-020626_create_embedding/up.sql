-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE embedding (
    id SERIAL PRIMARY KEY,
    image VECTOR(100) NOT NULL
);

CREATE TABLE post_embedding (
    post_id INT REFERENCES post ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    embedding_id INT REFERENCES embedding ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    PRIMARY KEY(post_id, embedding_id)
);