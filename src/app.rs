use axum_macros::FromRef;
use futures::future::BoxFuture;
use std::{
    fmt::{self, Display},
    future::Future,
    net::{SocketAddr, TcpListener},
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

pub type Server = BoxFuture<'static, hyper::Result<()>>;

pub struct Application {
    port: u16,
    server: Server,
}

#[derive(Clone, Debug)]
pub struct BaseUrl(String);

impl Display for BaseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, FromRef)]
struct AppState {
    // db_pool: PgPool,
    // email_client: EmailClient,
    base_url: BaseUrl,
}
