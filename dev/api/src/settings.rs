pub struct Settings {
    pub db_url: String,
    pub port: String,
}

impl Settings {
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        dotenv::dotenv().expect(".env not found");

        Self {
            db_url: std::env::var("DATABASE_URL").expect(".env DATABASE_URL not found"),
            port: std::env::var("PORT").unwrap_or("8080".to_string()),
        }
    }
}
