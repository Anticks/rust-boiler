use sqlx::PgPool;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;
use zero2prod::telemetry::{init_subscriber, subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = subscriber("zero2prod".into(), "info".into(), || std::io::stdout());
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read config.");

    let address = format!("{}:{}", config.application.host, config.application.port);

    let connection_pool = PgPool::connect_lazy(&config.database.connection_string())
        .expect("Failed to connect to Postgres connection pool.");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to start listener");

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    run(listener, connection_pool)?.await
}
