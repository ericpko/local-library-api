use axum::{http::StatusCode, response::IntoResponse};
use tokio::signal;

pub mod controllers;
pub mod models;
pub mod routes;

pub async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("\nsignal received, starting graceful shutdown");
}
