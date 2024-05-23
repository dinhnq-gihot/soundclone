use crate::{models::User, schema::users};

use super::database::Database;
use anyhow::Result;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;

pub struct Users {
    db: Arc<Database>,
}

impl Users {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    // pub async fn add(&self, email: String, password: String, username: String) -> Result<User> {
    //     let mut conn = self.db.get_connection().await;
    //     if let Ok(_user) = self.get(username).await {

    //     }

    //     Ok(())
    // }

    pub async fn get(&self, username: String) -> Result<User> {
        let mut conn = self.db.get_connection().await;
        let user = users::table
            .filter(users::username.eq(username))
            .select(User::as_select())
            .first(&mut conn)
            .await?;

        Ok(user)
    }
}
