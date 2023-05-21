use sqlx::postgres::PgPoolOptions;
use std::env;

mod helpers;
mod http;

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("Could not connect to the database");

    http::serve(db).await
}
