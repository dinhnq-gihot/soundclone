#[cfg(test)]
mod album_tests {
    use {
        crate::repositories::{
            album_repository::Albums as AlbumsHandler, database::Database,
            user_repository::Users as UsersHandler,
        },
        anyhow::Result,
        std::sync::Arc,
    };

    fn setup_db() -> Result<Arc<Database>> {
        Ok(Arc::new(Database::default()))
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_insert_and_get_album() {
        let db = setup_db().unwrap();
        let albums = AlbumsHandler::new(Arc::clone(&db));
        let users = UsersHandler::new(Arc::clone(&db));

        let user = users.get_by_email("john.doe@example.com").await.unwrap();

        let album_by_title = albums.get_by_title("%ill%".into()).await.unwrap();
        let album_by_artist = albums.get_by_artist(&user).await.unwrap();

        println!("{album_by_title:?}");
        println!("{album_by_artist:?}");
    }
}
