use sqlx::PgPool;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config().expect("failed to read config");
    let address = format!("127.0.0.1:{}", config.application_port);
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    run(listener, connection_pool)?.await
}
