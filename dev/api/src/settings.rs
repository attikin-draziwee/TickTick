use clap::Parser;
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub struct Settings {
    pub db_url: String,
    pub port: String,
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

impl Settings {
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        {
            dotenv::dotenv().expect(".env not found");
        }
        let tracing_level: &str = "todo=debug";

        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_line_number(true)
                    .with_target(true),
            )
            .with(
                EnvFilter::from_default_env()
                    .add_directive(tracing_level.parse().unwrap())
                    .add_directive("sqlx=warn".parse().unwrap()),
            )
            .with(ErrorLayer::default())
            .init();

        Self {
            db_url: std::env::var("DATABASE_URL").expect(".env DATABASE_URL not found"),
            port: Args::parse().port.to_string(),
        }
    }
}
