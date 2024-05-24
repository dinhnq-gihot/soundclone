use std::sync::Arc;

use super::database::Database;

pub struct Playlists {
    db: Arc<Database>,
}

impl Playlists {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get(&self) {}

    pub async fn add(&self) {}

    pub async fn update(&self) {}

    pub async fn delete(&self) {}
}
