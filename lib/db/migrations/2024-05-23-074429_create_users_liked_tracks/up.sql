-- Your SQL goes here
CREATE TABLE users_liked_tracks (
  user_id INTEGER REFERENCES users(id),
  track_id INTEGER REFERENCES tracks(id),
  PRIMARY KEY (user_id, track_id)
);