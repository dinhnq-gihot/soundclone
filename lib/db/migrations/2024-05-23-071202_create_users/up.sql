-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR UNIQUE NOT NULL,
  email VARCHAR UNIQUE NOT NULL,
  password VARCHAR NOT NULL, -- Hash the password before storing
  profile_picture VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);