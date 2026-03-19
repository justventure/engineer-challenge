use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;

pub type DbPool = Pool<AsyncPgConnection>;

pub async fn create_pool(database_url: &str) -> Result<DbPool, String> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .build(config)
        .await
        .map_err(|e| format!("DB pool error: {e}"))
}
