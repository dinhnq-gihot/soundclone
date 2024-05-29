#[cfg(test)]
mod tests {
    use {
        crate::repositories::{database::Database, user_repository::Users as UsersHandler},
        anyhow::Result,
        std::sync::Arc,
    };

    fn setup_db() -> Result<Arc<Database>> {
        Ok(Arc::new(Database::default()))
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
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

        let ret1 = users.get_by_email("dinh@email.com").await.unwrap();
        let ret2 = users.get_by_email("minh@email.com").await.unwrap();

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
            .add("abc@email.com".into(), "abc".into(), "abc".into())
            .await
            .unwrap();

        let ret1 = users
            .update_info("abc@email.com".into(), None, Some("abc.jpg".into()))
            .await
            .unwrap();

        println!("{ret1:?}");
        assert_eq!(ret1.profile_picture, Some("abc.jpg".into()));

        users.delete("abc@email.com".into()).await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_update_password_user() {
        let users = UsersHandler::new(setup_db().unwrap());
        users
            .add("minh123@email.com".into(), "456".into(), "minh123".into())
            .await
            .unwrap();
        let new_password = "new password".to_string();

        let ret1 = users
            .update_password("minh123@email.com".into(), Some(new_password.clone()))
            .await
            .unwrap();

        println!("{ret1:?}");
        assert_eq!(ret1.password, new_password);

        users.delete("minh123@email.com".into()).await.unwrap();
    }
}
