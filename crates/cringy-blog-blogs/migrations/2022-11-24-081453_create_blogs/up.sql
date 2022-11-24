-- Your SQL goes here
CREATE TABLE blogs (
    blog_id uuid PRIMARY KEY,
    user_id uuid NOT NULL,
    name varchar NOT NULL
);
