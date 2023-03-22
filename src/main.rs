use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::app::run;
use zero2prod::settings::Settings;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let settings = Settings::load().expect("failed to load configuration");

    let address = format!("127.0.0.1:{}", settings.port);
    let listener = TcpListener::bind(address).expect("failed to bind address");
    let db_pool = PgPool::connect_with(settings.database.connect_options())
        .await
        .expect("failed to connect to database");
    run(listener, db_pool)?.await
}
