use std::sync::Arc;

use super::database::Database;
use crate::{
    models::{NewPlayist, Playlist, User},
    schema::{playlists, users},
};
use anyhow::{anyhow, Result};
use diesel::{dsl::exists, insert_into, prelude::*, select};
use diesel_async::RunQueryDsl;

pub struct Playlists {
    db: Arc<Database>,
}

impl Playlists {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, playlist_id: i32) -> Result<Playlist> {
        let mut conn = self.db.get_connection().await;
        playlists::table
            .filter(playlists::id.eq(playlist_id))
            .select(Playlist::as_select())
            .first(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn get_by_user(&self, user: &User) -> Result<Vec<Playlist>> {
        let mut conn = self.db.get_connection().await;
        Playlist::belonging_to(user)
            .select(Playlist::as_select())
            .load(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn add(
        &self,
        name: String,
        user_id: Option<i32>,
        description: Option<String>,
        is_public: Option<bool>,
    ) -> Result<Playlist> {
        let mut conn = self.db.get_connection().await;

        let user_id = match user_id {
            Some(id) => {
                if select(exists(users::table.filter(users::id.eq(&id))))
                    .get_result::<bool>(&mut conn)
                    .await?
                {
                    Some(id)
                } else {
                    None
                }
            }
            None => None,
        };

        let new_playlist = NewPlayist {
            name: &name,
            user_id: user_id.as_ref(),
            description: description.as_ref().map(|x| x.as_str()),
            is_public: is_public.as_ref(),
        };

        insert_into(playlists::table)
            .values(new_playlist)
            .returning(Playlist::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn update(
        &self,
        playlist_id: i32,
        user_id: Option<i32>,
        description: Option<String>,
        is_public: Option<bool>,
    ) -> Result<Playlist> {
        let mut conn = self.db.get_connection().await;
        let existed_playlist = playlists::table
            .filter(playlists::id.eq(playlist_id))
            .select(Playlist::as_select())
            .get_result(&mut conn)
            .await?;
        

        Ok(())
    }

    pub async fn delete(&self) {}
}
