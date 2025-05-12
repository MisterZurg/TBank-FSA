mod model;
mod config;

use axum::Router;
use tokio_postgres::{NoTls, Error};

use crate::model::Note;
use crate::config::AppConfig;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    let cfg = AppConfig::new();
    println!("{:?}", cfg);
    // Connect to the database.
    let (client, connection) = tokio_postgres::Config::new()
        .host(cfg.postgres_host)
        .port(cfg.postgres_port)
        .user(cfg.postgres_user)
        .password(cfg.postgres_password)
        .dbname(cfg.postgres_db)
        .connect(NoTls)
        .await?;
    //
    // // The connection object performs the actual communication with the database,
    // // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let row = client.query_one("SELECT version();", &[]).await?;
    let db_version: &str = row.get(0);
    //
    println!("Database version: {}", db_version);

    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
