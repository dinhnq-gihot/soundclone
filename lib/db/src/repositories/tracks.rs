use std::sync::Arc;

use super::database::Database;
use crate::{
    models::{Album, NewTrack, Track, User},
    schema::tracks,
};
use anyhow::{anyhow, Result};
use chrono::Local;
use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;

pub struct Tracks {
    db: Arc<Database>,
}

impl Tracks {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Vec<Track>> {
        tracks::table
            .filter(tracks::id.eq(id))
            .select(Track::as_select())
            .load(&mut self.db.get_connection().await)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn get_all(&self) -> Result<Vec<Track>> {
        tracks::table
            .load(&mut self.db.get_connection().await)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn get_by_title(&self, title: String) -> Result<Vec<Track>> {
        let mut conn = self.db.get_connection().await;
        tracks::table
            .filter(tracks::title.like(title))
            .select(Track::as_select())
            .load(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn get_by_album(&self, album: &Album) -> Result<Vec<Track>> {
        let mut conn = self.db.get_connection().await;
        Track::belonging_to(album)
            .select(Track::as_select())
            .load(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn get_by_artist(&self, artist: &User) -> Result<Vec<Track>> {
        let mut conn = self.db.get_connection().await;
        Track::belonging_to(artist)
            .select(Track::as_select())
            .load(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn add(
        &self,
        title: String,
        artist_id: Option<i32>,
        album_id: Option<i32>,
        duration: Option<i32>,
        file_path: Option<String>,
    ) -> Result<Track> {
        let mut conn = self.db.get_connection().await;
        let now = Local::now();

        let new_track = NewTrack {
            title: &title,
            album_id: album_id.as_ref(),
            artist_id: artist_id.as_ref(),
            duration: duration.as_ref(),
            file_path: file_path.as_ref().map(|x| x.as_str()),
            created_at: Some(&now),
        };

        insert_into(tracks::table)
            .values(new_track)
            .returning(Track::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn update(&self) {}

    pub async fn delete(&self) {}
}
