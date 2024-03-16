use std::net::TcpListener;

use newsletter_service::{
    configuration::get_config,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter_service".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(config.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres");
    let addr = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(addr).expect("Failed to bind to random port");

    run(listener, connection_pool)?.await?;
    Ok(())
}
