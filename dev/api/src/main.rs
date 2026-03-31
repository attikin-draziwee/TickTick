use sqlx::{MySqlPool, Pool};

mod handler;
mod repository;
mod router;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    dotenv::dotenv()?;

    let db_connect: Pool<sqlx::MySql> = MySqlPool::connect(
        &std::env::var("DATABASE_URL").expect(".env var DATABASE_URL not found"),
    )
    .await?;

    sqlx::migrate!()
        .run(&db_connect)
        .await
        .expect("cannot migrate tables");

    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Cannot use this port");

    axum::serve(listener, router::main_router()).await?;

    Ok(())
}
