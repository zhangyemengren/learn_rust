use std::net::TcpListener;
use sqlx::{PgPool};
use zero_to_prod_actix_web::configuration::get_configuration;
use zero_to_prod_actix_web::startup::run;
use zero_to_prod_actix_web::telemetry::{get_subscriber, init_subscriber};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    ).await.expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}