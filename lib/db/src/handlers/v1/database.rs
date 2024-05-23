use diesel_async::{
    pooled_connection::{
        bb8::{Pool, PooledConnection},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use diesel_async_migrations::{embed_migrations, EmbeddedMigrations};
use futures::executor::block_on;
use once_cell::sync::Lazy;

pub static MIGRATIONS: Lazy<EmbeddedMigrations> = Lazy::new(|| embed_migrations!("./migrations"));

pub struct Database {
    pool: Pool<AsyncPgConnection>,
    url: String,
    _in_use: bool,
}

impl Default for Database {
    fn default() -> Self {
        block_on(Database::new(
            "postgresql://soundclone:123@localhost:15432/soundclone".into(),
        ))
    }
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database")
            .field("url", &self.url)
            .field("in_use", &self._in_use)
            .finish()
    }
}

impl Database {
    pub async fn new(url: String) -> Self {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.clone());
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .await
            .expect("Could not build connection pool");
        let mut _conn = pool.get_owned().await.unwrap();
        MIGRATIONS.run_pending_migrations(&mut _conn).await.unwrap();

        Database {
            pool,
            url,
            _in_use: true,
        }
    }

    pub async fn get_connection(&self) -> PooledConnection<AsyncPgConnection> {
        self.pool.get().await.unwrap()
    }
}

