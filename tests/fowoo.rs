use sqlx::migrate::Migrator;

static _MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() {}
