use crate::{
    models::{NewUser, User},
    schema::users,
};

use super::database::Database;
use anyhow::{anyhow, Result};
use chrono::Local;
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub struct Users {
    db: Arc<Database>,
}

impl Users {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn add(&self, email: String, password: String, username: String) -> Result<User> {
        let mut conn = self.db.get_connection().await;
        if let Some(_) = self.get(&email).await.ok() {
            return Err(anyhow!("User existed!"));
        }
        let new_user = NewUser {
            username: &username,
            email: &email,
            password: &password,
            created_at: &Local::now(),
        };

        insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn get(&self, email: &str) -> Result<User> {
        let mut conn = self.db.get_connection().await;
        users::table
            .filter(users::email.eq(email))
            .select(User::as_select())
            .first(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    async fn update(
        &self,
        email: String,
        username: Option<String>,
        password: Option<String>,
        profile_picture: Option<String>,
    ) -> Result<User> {
        let mut conn = self.db.get_connection().await;
        let mut existed_user = self.get(&email).await?;
        if username.is_some() {
            existed_user.username = username.unwrap();
        }
        if password.is_some() {
            existed_user.password = password.unwrap();
        }
        if profile_picture.is_some() {
            existed_user.profile_picture = profile_picture;
        }
        update(users::table.filter(users::email.eq(&email)))
            .set(existed_user)
            .returning(User::as_select())
            .get_result(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn update_info(
        &self,
        email: String,
        username: Option<String>,
        profile_picture: Option<String>,
    ) -> Result<User> {
        self.update(email, username, None, profile_picture).await
    }

    pub async fn update_password(&self, email: String, password: Option<String>) -> Result<User> {
        self.update(email, None, password, None).await
    }

    pub async fn delete(&self, email: String) -> Result<usize> {
        delete(users::table.filter(users::email.eq(&email)))
            .execute(&mut self.db.get_connection().await)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }
}
