mod cluster;
mod cron;

use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::env;
use std::sync::Arc;

use crate::config::Config;
use axum::{routing, Extension, Router};
use log::info;

pub async fn start_server(config: &'static Config) {
  let pool = Pool::<Postgres>::connect(&env::var("DATABASE_URL").unwrap())
    .await
    .unwrap();
  let shared_pool = Arc::new(pool);
  info!("Successfully connected to Postgres");
  cluster::start_node(config);
  cron::start_job_manager(config, Arc::clone(&shared_pool));
  let app = Router::new()
    .route("/cron-jobs", routing::post(cron::api_create_job))
    .route("/cron-jobs", routing::get(cron::api_get_cron_jobs))
    .layer(Extension(Arc::clone(&shared_pool)));
  let address = config.address.to_string();
  info!("Dosei running on http://{} (Press CTRL+C to quit", address);
  axum::Server::bind(&address.parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
