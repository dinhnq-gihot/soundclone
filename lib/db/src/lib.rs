mod models;
mod repositories;
mod schema;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        anyhow::Result,
        repositories::{database::Database, users::Users as UsersHandler},
        std::sync::Arc,
    };

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn setup_db() -> Result<Arc<Database>> {
        Ok(Arc::new(Database::default()))
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_insert_and_get_user() {
        let users = UsersHandler::new(setup_db().unwrap());
        users
            .add("dinh@email.com".into(), "123".into(), "dinhqt".into())
            .await
            .unwrap();
        users
            .add("minh@email.com".into(), "456".into(), "minhqt".into())
            .await
            .unwrap();

        let ret1 = users.get("dinh@email.com").await.unwrap();
        let ret2 = users.get("minh@email.com").await.unwrap();

        println!("{ret1:?}\n{ret2:?}");
        assert_eq!(ret1.username, "dinhqt".to_string());
        assert_eq!(ret2.username, "minhqt".to_string());

        users.delete("dinh@email.com".into()).await.unwrap();
        users.delete("minh@email.com".into()).await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_update_info_user() {
        let users = UsersHandler::new(setup_db().unwrap());
        users
            .add("minh@email.com".into(), "456".into(), "minhqt".into())
            .await
            .unwrap();

        let ret1 = users
            .update_info("minh@email.com".into(), None, Some("abc".into()))
            .await
            .unwrap();

        println!("{ret1:?}");
        assert_eq!(ret1.profile_picture, Some("abc".into()));

        users.delete("minh@email.com".into()).await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_update_password_user() {
        let users = UsersHandler::new(setup_db().unwrap());
        users
            .add("minh@email.com".into(), "456".into(), "minhqt".into())
            .await
            .unwrap();
        let new_password = "new password".to_string();

        let ret1 = users
            .update_password("minh@email.com".into(), Some(new_password.clone()))
            .await
            .unwrap();

        println!("{ret1:?}");
        assert_eq!(ret1.password, new_password);

        users.delete("minh@email.com".into()).await.unwrap();
    }
}

#[cfg(test)]
mod album_tests {
    use {
        super::*,
        anyhow::Result,
        repositories::{albums::Albums as AlbumsHandler, database::Database},
        std::sync::Arc,
    };

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn setup_db() -> Result<Arc<Database>> {
        Ok(Arc::new(Database::default()))
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_insert_and_get_album() {
        let albums = AlbumsHandler::new(setup_db().unwrap());
        let album_by_title = albums.get_by_title("%ill%".into()).await.unwrap();
        let album_by_artist = albums.get_by_artist("jane_smith".into()).await.unwrap();

        println!("{album_by_title:?}");
        println!("{album_by_artist:?}");
    }
}
