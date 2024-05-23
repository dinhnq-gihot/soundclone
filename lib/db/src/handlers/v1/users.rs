use std::sync::Arc;

use super::database::Database;

pub struct Users {
    db: Arc<Database>,
}

impl Users {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    // pub async fn create(&self, email: String, password: String, username: String) {
    //     let user =
    // }
}
