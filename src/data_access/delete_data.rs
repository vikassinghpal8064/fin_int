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

    // Delete a data row from the table
    let deleted_rows = client.execute(
        "DELETE FROM users WHERE name = $1",
        &[&"Meera Reddy"],
    )?;

    if deleted_rows > 0 {
        println!("Deleted ");
    } else {
        println!("not found in the table.");
    }

    Ok(())
}