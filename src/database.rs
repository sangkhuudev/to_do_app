use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in .env file");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Failed to establish connection to database: {}", database_url))
}