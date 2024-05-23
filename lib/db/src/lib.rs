mod handlers;
mod models;
mod schema;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        anyhow::Result,
        handlers::v1::{database::Database, users::Users as UsersHandler},
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
    async fn test_get_user() {
        let users = UsersHandler::new(setup_db().unwrap());
        let ret = users.get("john_doe".into()).await.unwrap();

        println!("{ret:?}");
    }
}
