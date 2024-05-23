-- Your SQL goes here
CREATE TABLE albums (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  artist_id INTEGER REFERENCES users(id),
  release_date DATE,
  cover_art VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);