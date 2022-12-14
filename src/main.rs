use axum_local_library::database::init_db;
use std::{net::SocketAddr, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    handler::Handler,
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Redirect},
    routing::get,
    BoxError, Extension, Router,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use axum_local_library::{controllers, handle_404, routes, shutdown_signal};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv::dotenv().ok();

    // add tracing (aka logging)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "axum_local_library=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // connect to mongodb database
    let db = init_db().await?;

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
        .layer(Extension(db))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        )
        .into_inner();

    // compose our routes
    let api_routes = Router::new()
        .route("/", get(controllers::api_index_handler))
        .nest("/authors", routes::author_routes())
        .nest("/books", routes::book_routes());
    let app = Router::new()
        .route("/", get(root))
        .nest("/api", api_routes)
        // add middleware to all our routes
        .layer(middleware);

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handle_404.into_service());

    // run our app
    let port = dotenv::var("PORT").unwrap().parse::<u16>().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

// index page / home page / localhost:3000/
pub async fn root() -> impl IntoResponse {
    Redirect::to("/api")
}
