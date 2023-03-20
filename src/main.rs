use axum::{routing, Router};
use zero2prod::routes::health_check;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    hyper::Server::bind(&"127.0.0.1:8000".parse().expect("invalid bind adress"))
        .serve(
            Router::new()
                .route("/health_check", routing::get(health_check))
                .into_make_service(),
        )
        .await
}
