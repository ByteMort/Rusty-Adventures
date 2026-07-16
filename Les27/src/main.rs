use les27::run;
use les27::background_jobs::start_expired_code_cleanup;

use sqlx::{ PgPool, postgres::PgPoolOptions };
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // tracing_subscriber::fmt::init();
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(fmt::layer())
        .init();

    tracing::info!("Starting the DB Connection...");

    let pool:PgPool = PgPoolOptions::new()
        .max_connections(4)
        .connect("postgres://rusty:rusty@localhost:5555/rust_db")
        .await
        .unwrap_or_else(|err|{
            tracing::error!("Connection Error: {}", err);
            panic!("Connection Stopped.");
        });
    
    start_expired_code_cleanup(pool.clone()).await;
    run(&pool).await;

    tracing::info!("Connection Successfull!");
}
