use dotenvy::dotenv;
use std::env;
use tokio_postgres::{Client, NoTls, Error};

pub struct Database {
    pub client: Client,
}

impl Database {
    pub async fn connect() -> Result<Self, Error> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

        // Spawn the connection handler
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Database { client })
    }
}
