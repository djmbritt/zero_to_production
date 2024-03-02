use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero_to_prod::configuration::get_configuration;
use zero_to_prod::startup;
use zero_to_prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero_to_prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");

    let _ = startup::run(listener, connection_pool)?.await;

    Ok(())
}
