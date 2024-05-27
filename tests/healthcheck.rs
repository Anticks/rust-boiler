use once_cell::sync::Lazy;
use sqlx::{Connection, PgConnection, PgPool};
use zero2prod::configuration::get_config;
use zero2prod::telemetry::{init_subscriber, subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "debug".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[sqlx::test]
async fn healthcheck_works(pool: PgPool) {
    // Arrange
    let app = spawn_app(pool).await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/healthcheck", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[sqlx::test]
async fn subscribe_returns_422_when_data_is_missing(pool: PgPool) {
    // Arrange
    let app = spawn_app(pool).await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            422,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[sqlx::test]
async fn subscribe_returns_a_200_for_valid_form_data(pool: PgPool) {
    // Arrange
    let app = spawn_app(pool).await;
    let configuration = get_config().expect("Failed to get config");
    let config_string = configuration.database.connection_string();
    let _connection = PgConnection::connect(&config_string)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

async fn spawn_app(pool: PgPool) -> TestApp {
    Lazy::force(&TRACING);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();
    let connection_pool_clone = pool.clone();

    tokio::spawn(async move {
        zero2prod::startup::run(listener, connection_pool_clone)
            .unwrap()
            .await
            .unwrap();
    });

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: pool,
    }
}
