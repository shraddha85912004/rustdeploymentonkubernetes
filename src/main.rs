use axum::{
    routing::get,
    Router,
};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rustdeployment=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let builder = PrometheusBuilder::new();
    let handle = builder
        .install_recorder()
        .expect("failed to install Prometheus recorder");

    // We can define a custom route for metrics, or just expose what prometheus-exporter provides
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(move || async move { handle.render() }));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
