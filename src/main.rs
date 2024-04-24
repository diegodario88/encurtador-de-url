mod routes;

use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use routes::health_check;
use std::error::Error;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "encurtador-de-url=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app = Router::new()
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Não foi possível inicializar TcpListener");

    tracing::debug!(
        "ouvindo em {}",
        listener
            .local_addr()
            .expect("Não foi possível converter listener em endereço local")
    );

    axum::serve(listener, app)
        .await
        .expect("Não foi possível criar o servidor");

    return Ok(());
}
