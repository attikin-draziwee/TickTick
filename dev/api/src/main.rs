use std::time::Duration;

use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

use crate::{service::user::UserService, settings::Settings};

mod settings;

mod handler;
mod repository;
mod router;
mod service;

#[derive(Clone)]
pub struct AppState {
    db: MySqlPool,
    user_service: UserService,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: Settings = Settings::new();

    let db_connect = MySqlPoolOptions::new()
        .min_connections(5) // Минимум 5 подключений (прогрев)
        .max_connections(20) // Максимум 20 подключений
        .acquire_timeout(Duration::from_secs(10)) // Таймаут получения подключения
        .idle_timeout(Duration::from_secs(600)) // Время жизни idle подключения
        .max_lifetime(Duration::from_secs(1800)) // Максимальное время жизни
        .connect(&settings.db_url)
        .await?;

    sqlx::migrate!()
        .run(&db_connect)
        .await
        .expect("cannot migrate tables");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &settings.port))
        .await
        .expect("Cannot use this port");

    tracing::info!("Starting server on: {}", listener.local_addr()?);

    let app_state: AppState = AppState {
        db: db_connect,
        user_service: UserService::new(),
    };
    axum::serve(listener, router::main_router(app_state)).await?;

    Ok(())
}
