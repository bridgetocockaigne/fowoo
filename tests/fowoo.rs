use std::env;

use sqlx::{migrate::Migrator, postgres::PgPoolOptions};

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").unwrap();

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .unwrap();

    MIGRATOR.run(&db).await.unwrap();
}
