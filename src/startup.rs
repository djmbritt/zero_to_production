use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    #[tracing::instrument(name = "Building Server with configuration.")]
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address.");

        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, email_client)?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}

#[tracing::instrument(name = "Run HttpServer")]
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let db_pool_connection = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            // Register Logger with actix_web server
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the app state
            .app_data(db_pool_connection.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
