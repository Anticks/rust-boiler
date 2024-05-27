use axum::http::StatusCode;

#[axum::debug_handler]
pub async fn healthcheck() -> StatusCode {
    StatusCode::OK
}
