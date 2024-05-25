use std::sync::Arc;

use super::database::Database;
use crate::{
    models::{Album, NewAlbum, User},
    schema::{albums, users},
};
use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDate};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;

pub struct Albums {
    db: Arc<Database>,
}

impl Albums {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Vec<Album>> {
        let mut conn = self.db.get_connection().await;
        let album_list = albums::table
            .filter(albums::id.eq(id))
            .select(Album::as_select())
            .load(&mut conn)
            .await?;

        Ok(album_list)
    }

    pub async fn get_by_title(&self, title: String) -> Result<Vec<Album>> {
        let mut conn = self.db.get_connection().await;
        let album_list = albums::table
            .filter(albums::title.like(title))
            .select(Album::as_select())
            .load(&mut conn)
            .await?;

        Ok(album_list)
    }

    pub async fn get_by_artist(&self, artist: &User) -> Result<Vec<Album>> {
        let mut conn = self.db.get_connection().await;
        let album_list = Album::belonging_to(artist)
            .select(Album::as_select())
            .load(&mut conn)
            .await?;

        Ok(album_list)
    }

    pub async fn get_all(&self) -> Result<Vec<Album>> {
        let mut conn = self.db.get_connection().await;
        albums::table
            .load(&mut conn)
            .await
            .map_err(|e|    anyhow!(e.to_string()))
    }

    pub async fn add(
        &self,
        artist_id: i32,
        title: String,
        release_date: Option<NaiveDate>,
        cover_art: Option<String>,
    ) -> Result<Album> {
        let mut conn = self.db.get_connection().await;
        let new_album = NewAlbum {
            title: &title,
            release_date: release_date.as_ref(),
            artist_id: &artist_id,
            cover_art: cover_art.as_ref().map(|x| x.as_str()),
            created_at: &Local::now(),
        };

        let added_album = insert_into(albums::table)
            .values(new_album)
            .returning(Album::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(added_album)
    }

    async fn update(
        &self,
        title: String,
        artist_id: Option<i32>,
        new_title: Option<String>,
        release_date: Option<NaiveDate>,
        cover_art: Option<String>,
    ) -> Result<Album> {
        let mut conn = self.db.get_connection().await;
        let album_list = self.get_by_title(title.clone()).await?;
        let mut existed_album = if !album_list.is_empty() {
            album_list[0].to_owned()
        } else {
            return Err(anyhow!("Album not existed"));
        };
        if artist_id.is_some() {
            existed_album.artist_id = artist_id.unwrap();
        }
        if new_title.is_some() {
            existed_album.title = new_title.unwrap();
        }
        if release_date.is_some() {
            existed_album.release_date = release_date;
        }
        if cover_art.is_some() {
            existed_album.cover_art = cover_art;
        }

        let ret = update(albums::table.filter(albums::title.eq(title)))
            .set(existed_album)
            .returning(Album::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn update_artist(&self, title: String, new_artist_id: Option<i32>) -> Result<Album> {
        self.update(title, new_artist_id, None, None, None).await
    }

    pub async fn update_info(
        &self,
        title: String,
        new_title: Option<String>,
        release_date: Option<NaiveDate>,
        cover_art: Option<String>,
    ) -> Result<Album> {
        self.update(title, None, new_title, release_date, cover_art)
            .await
    }

    pub async fn delete(&self, title: String, artist_id: i32) -> Result<()> {
        let mut conn = self.db.get_connection().await;
        delete(albums::table)
            .filter(albums::title.eq(title))
            .filter(albums::artist_id.eq(artist_id))
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}
