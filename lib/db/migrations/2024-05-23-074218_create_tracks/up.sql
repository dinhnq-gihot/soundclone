-- Your SQL goes here
CREATE TABLE tracks (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  album_id INTEGER REFERENCES albums(id),  -- Can be null for singles
  artist_id INTEGER REFERENCES users(id),
  duration INTEGER,
  file_path VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);