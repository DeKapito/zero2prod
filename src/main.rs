use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::SqlitePool;
use std::net::TcpListener;
use std::str::FromStr;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_options =
        SqliteConnectOptions::from_str(&configuration.database.connection_string())
            .unwrap()
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true);
    let connection_pool = SqlitePool::connect_with(connection_options)
        .await
        .expect("Failed to connect to Database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
