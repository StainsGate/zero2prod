use clap::Parser;
use sqlx::PgPool;
use std::net::TcpListener;
use std::path::PathBuf;
use zero2prod::app::run;
use zero2prod::settings::Settings;
use zero2prod::trace::{get_subscriber, init_subscriber, stdout, TraceSettings};

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// Configuration file path
    #[arg(short = 'f')]
    configuration: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let args = Args::parse();
    let configuration = args.configuration.as_ref().map(|c| c.to_str().unwrap());
    let settings = Settings::load(configuration).expect("failed to load configuration");

    let subscriber = get_subscriber(TraceSettings {
        level: settings.log.level,
        writer: stdout(),
        endpoint: settings.log.endpoint.as_deref(),
        namespace: settings.log.namespace.as_deref(),
    });
    init_subscriber(subscriber);

    let address = format!("{}:{}", settings.address, settings.port);
    let listener = TcpListener::bind(&address).expect("failed to bind address");
    let db_pool = PgPool::connect_lazy_with(settings.database.connect_options());
    if settings.database.migrate {
        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("failed to migrate the database")
    }
    tracing::info!("serving on {}", address);
    run(listener, db_pool)?.await
}
