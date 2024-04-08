use crate::routes::healthcheck;
use crate::routes::subscribe;
use sqlx::PgPool;
use tokio::net::TcpListener;

use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};

pub fn run(
    listener: TcpListener,
    connection_pool: PgPool,
) -> Result<Serve<Router, Router>, std::io::Error> {
    let server = axum::serve(
        listener,
        Router::new()
            .route("/healthcheck", get(healthcheck))
            .route("/subscriptions", post(subscribe))
            .with_state(connection_pool),
    );

    Ok(server)
}
