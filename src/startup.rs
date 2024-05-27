use crate::routes::healthcheck;
use crate::routes::subscribe;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use uuid::Uuid;

use axum::{
    extract::MatchedPath,
    http::Request,
    routing::{get, post},
    serve::Serve,
    Extension, Router,
};

#[derive(Clone)]
struct State {}

pub fn run(
    listener: TcpListener,
    connection_pool: PgPool,
) -> Result<Serve<Router, Router>, std::io::Error> {
    let server = axum::serve(
        listener,
        Router::new()
            .route("/healthcheck", get(healthcheck))
            .route("/subscriptions", post(subscribe))
            .layer(
                ServiceBuilder::new()
                    .layer(
                        TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                            let matched_path = request
                                .extensions()
                                .get::<MatchedPath>()
                                .map(MatchedPath::as_str);

                            info_span!(
                                "request",
                                method = ?request.method(),
                                matched_path,
                                "request_id" = Uuid::new_v4().to_string(),
                            )
                        }),
                    )
                    .layer(Extension(State {})),
            )
            .with_state(connection_pool),
    );

    Ok(server)
}
