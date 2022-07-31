use std::{net::SocketAddr, time::Duration};

use axum::{
    error_handling::HandleErrorLayer, handler::Handler, http::StatusCode, routing::get, BoxError,
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use axum_local_library::{handle_404, routes, shutdown_signal};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // add tracing (aka logging)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "axum_local_library=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build a stack of tower middleware
    let middleware = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|error: BoxError| async move {
            if error.is::<tower::timeout::error::Elapsed>() {
                Ok(StatusCode::REQUEST_TIMEOUT)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            }
        }))
        .timeout(Duration::from_secs(10))
        // add tracing (aka logging)
        .layer(TraceLayer::new_for_http())
        // give our handlers access to the db
        // .layer(Extension(db))
        .into_inner();

    // compose our routes
    let users_router = routes::users::users_router();
    let app = Router::new()
        .route("/", get(routes::index::root))
        .merge(users_router)
        // add middleware to all our routes
        .layer(middleware);

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handle_404.into_service());

    // run our app
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
