use dotenvy::dotenv;
use postgres::Client;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let conn_string = env::var("DATA_BASE")?;

    let builder = SslConnector::builder(SslMethod::tls())?;
    let connector = MakeTlsConnector::new(builder.build());

    let mut client = Client::connect(&conn_string, connector)?;
    println!("Connection established");

    // Update a data row in the table
    let updated_rows = client.execute(
        "UPDATE users SET name = $1 WHERE name = $2",
        &[&"Vikas Singh Pal", &"Aarav Sharma"],
    )?;

    if updated_rows > 0 {
        println!("Updated stock status for 'Dune'.");
    } else {
        println!("'Dune' not found or stock status already up to date.");
    }

    Ok(())
}