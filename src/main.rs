use std::net::SocketAddr;

use axum::{handler::Handler, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use axum_local_library::{handle_404, routes, shutdown_signal};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "axum_local_library=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build a stack of tower middleware
    let middleware = ServiceBuilder::new().layer(TraceLayer::new_for_http());

    // build our app with a route
    let users_router = routes::users::users_router();
    let app = Router::new()
        .route("/", get(routes::index::root))
        // add tracing (aka logging)
        // .layer(TraceLayer::new_for_http());
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
