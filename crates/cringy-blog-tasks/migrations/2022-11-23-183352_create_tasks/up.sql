-- Your SQL goes here
CREATE TYPE task_completion_state AS ENUM ('completed', 'not_completed');
CREATE TABLE tasks (
    task_id uuid PRIMARY KEY,
    blog_id uuid NOT NULL,
    name varchar NOT NULL,
    deadline timestamptz NULL,
    completion task_completion_state NOT NULL
);
