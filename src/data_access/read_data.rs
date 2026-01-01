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

    // Fetch all rows from the books table
    let users = client.query("SELECT * FROM users ORDER BY id;", &[])?;

   // println!("{:?}",users);
    for item in users {
        let id: i32 = item.get("id");
        let name: &str = item.get("name");
        let email_id: &str = item.get("email_id");
        let age: i32 = item.get("age");
        let phone_number: &str = item.get("phone_number");
        let occupation:&str=item.get("occupation");
        println!("ID: {}, Title: {}, Author: {}, Year: {}, In Stock: {} ,occupation:{}", id, name, email_id, age, phone_number,occupation);
    }
    println!("--------------------\n");
    let user1=client.query("SELECT * FROM users WHERE name = $1",&[&"Vikas Singh Pal"]);
    println!("{:#?}",user1.ok());
    Ok(())
}