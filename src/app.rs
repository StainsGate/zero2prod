use axum::{routing, Router};
use axum_macros::FromRef;
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use futures::future::BoxFuture;
use sqlx::PgPool;

pub type Server = BoxFuture<'static, hyper::Result<()>>;

pub fn run(listener: TcpListener, db_pool: PgPool) -> hyper::Result<Server> {
    let state = AppState { db_pool };

    let server = hyper::Server::from_tcp(listener)?.serve(
        Router::new()
            .route("/health_check", routing::get(health_check))
            .route("/subscriptions", routing::post(subscribe))
            .with_state(state)
            .into_make_service(),
    );
    Ok(Box::pin(server))
}

#[derive(Clone, FromRef)]
struct AppState {
    db_pool: PgPool,
}
