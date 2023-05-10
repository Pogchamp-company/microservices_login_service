use std::env;
use std::sync::atomic::{AtomicBool, Ordering};

use sqlx::{PgPool};
use sqlx::postgres::PgPoolOptions;

static MIGRATIONS_RAN: AtomicBool = AtomicBool::new(false);

pub async fn create_connection_pool() -> PgPool {
    let database_uri = env::var("DATABASE_URL")
        .expect("DATABASE_URI not provided in .env");

    let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_uri).await
        .expect("Database connection pool was not created");

    if !MIGRATIONS_RAN.load(Ordering::SeqCst) {
        println!("Migrations ran");

        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await.expect("Migrations failed");

        MIGRATIONS_RAN.store(true, Ordering::SeqCst);
    }

    return connection_pool;
}