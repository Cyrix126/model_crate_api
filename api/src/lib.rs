use axum::Router;
use config::Config;
use db::migration::run_migrations;
use deadpool_diesel::postgres::Pool;
use get_pass::get_password;
use reqwest::{Client, ClientBuilder};

pub mod config;
pub mod db;
pub mod error;
pub mod handler;

#[derive(Clone)]
pub struct AppState {
    // Configuration that the program will run with.
    // roadmap could include allowing to use command line args and environments variable.
    pub config: Config,
    // Database pool connections
    pub pool: Pool,
    // reqwest client to interact with Mondial Relay API
    pub client: Client,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let mut db_uri = config.db_uri.clone();
        db_uri
            .set_password(Some(
                &get_password(&config.db_pass_path).expect("Invalid utf-8"),
            ))
            .unwrap();
        let pool = Pool::builder(deadpool_diesel::Manager::new(
            db_uri.as_str(),
            deadpool_diesel::Runtime::Tokio1,
        ))
        .build()?;
        run_migrations(&pool).await?;
        let client = ClientBuilder::new()
            .build()
            .expect("value given to builder should be valid");
        Ok(AppState {
            config,
            pool,
            client,
        })
    }
}
pub fn router(state: AppState) -> Router {
    Router::new()
        // .route("/endpoint", axum::routing::post(shipment))
        .with_state(state)
}
