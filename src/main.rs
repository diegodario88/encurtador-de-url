mod routes;
mod utils;

use axum::routing::post;
use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "encurtador-de-url=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL é uma variável de ambiente obrigatória.");

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Não foi possível executar corretamente as migrations");

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app = Router::new()
        .route("/create", post(routes::create_link))
        .route("/:id", get(routes::redirect))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .route("/health", get(routes::health_check))
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Não foi possível inicializar TcpListener");

    tracing::debug!(
        "aplicação ouvindo em {}",
        listener
            .local_addr()
            .expect("Não foi possível converter listener em endereço local")
    );

    axum::serve(listener, app)
        .await
        .expect("Não foi possível criar o servidor");

    return Ok(());
}
