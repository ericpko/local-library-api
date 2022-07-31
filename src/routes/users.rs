use axum::{response::IntoResponse, routing::get, Router};

pub fn users_router() -> Router {
    let router = Router::new()
        .route("/users", get(handle_users))
        .route("/users/cool", get(handle_cool));
    router
}

async fn handle_users() -> impl IntoResponse {
    String::from("Users!")
}

async fn handle_cool() -> impl IntoResponse {
    String::from("you're so cool!")
}
