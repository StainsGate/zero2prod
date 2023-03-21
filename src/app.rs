use axum::{routing, Router};
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

use futures::future::BoxFuture;

pub type Server = BoxFuture<'static, hyper::Result<()>>;

pub fn run(listener: TcpListener) -> hyper::Result<Server> {
    let server = hyper::Server::from_tcp(listener)?.serve(
        Router::new()
            .route("/health_check", routing::get(health_check))
            .route("/subscriptions", routing::post(subscribe))
            .into_make_service(),
    );
    Ok(Box::pin(server))
}
