-- Your SQL goes here
CREATE TABLE playlists_tracks (
  playlist_id INTEGER REFERENCES playlists(id),
  track_id INTEGER REFERENCES tracks(id),
  PRIMARY KEY (playlist_id, track_id)
);