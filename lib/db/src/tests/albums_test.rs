#[cfg(test)]
mod album_tests {
    use {
        crate::repositories::{
            album_repository::Albums, database::Database, user_repository::Users,
        },
        anyhow::Result,
        std::sync::Arc,
    };

    fn setup_db() -> Result<Arc<Database>> {
        Ok(Arc::new(Database::default()))
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_insert_and_get_all() {
        let db = setup_db().unwrap();
        let albums = Albums::new(Arc::clone(&db));

        let added_album = albums.add(3, "abc".into(), None, None).await.unwrap();
        let all_albums = albums.get_all().await.unwrap();

        println!("{added_album:?}");
        println!("{all_albums:?}");
        // albums.delete(added_album.id).await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_album_by_user() {
        let db = setup_db().unwrap();
        let albums = Albums::new(Arc::clone(&db));
        let users = Users::new(Arc::clone(&db));

        let user = users.get_by_email("musiclover@example.com").await.unwrap();

        let album_by_title = albums.get_by_title("%er%".into()).await.unwrap();
        let album_by_artist = albums.get_by_artist(&user).await.unwrap();

        println!("{album_by_title:?}");
        println!("{album_by_artist:?}");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_and_update() {
        let db = setup_db().unwrap();
        let albums = Albums::new(Arc::clone(&db));
        let users = Users::new(Arc::clone(&db));

        let aritst = users.get_by_email("musiclover@example.com").await.unwrap();
        let new_artist = users.get_by_email("djmixmaster@example.com").await.unwrap();

        let album_by_artist = albums.get_by_artist(&aritst).await.unwrap();
        println!("{album_by_artist:?}");

        if !album_by_artist.is_empty() {
            let updated_album = albums
                .update_artist(album_by_artist[0].id, Some(new_artist.id))
                .await
                .unwrap();
            println!("{updated_album:?}");

            assert_eq!(updated_album.artist_id, new_artist.id);
        }
    }
}
