-- Add up migration script here

CREATE TABLE posts (
  id SERIAL PRIMARY KEY NOT NULL,
  title VARCHAR(255) NOT NULL,
  actor_id INTEGER NOT NULL,
  FOREIGN KEY (actor_id) REFERENCES actors(id) ON DELETE CASCADE,

  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
