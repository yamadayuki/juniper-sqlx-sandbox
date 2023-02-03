-- Add up migration script here

CREATE TYPE actor_role AS ENUM ('admin', 'editor', 'viewer');

CREATE TABLE actors (
  id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(255) NOT NULL,
  role actor_role NOT NULL DEFAULT 'viewer',

  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
