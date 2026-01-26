use dotenvy::dotenv;
use postgres::Client;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::env;
use serde::{Deserialize, Serialize};
#[derive(Debug,Clone,Serialize ,Deserialize)]
pub struct Users{
pub id:i32,
pub name:String,
pub age:i32,
pub email_id:String,
pub phone_number:String,
pub occupation:String,
pub trust:String
}
fn main()  {
    // dotenv()?;
    // let conn_string = env::var("DATA_BASE")?;

    // let builder = SslConnector::builder(SslMethod::tls())?;
    // let connector = MakeTlsConnector::new(builder.build());

    // let mut client = Client::connect(&conn_string, connector)?;
    // println!("Connection established");

    // // Delete a data row from the table
    // let deleted_rows = client.execute(
    //     "DELETE FROM users WHERE name = $1",
    //     &[&"Meera Reddy"],
    // )?;

    // if deleted_rows > 0 {
    //     println!("Deleted ");
    // } else {
    //     println!("not found in the table.");
    // }

    // Ok(())
}

pub fn delete_data(phone_number:String)->Result<u64,String>{
 dotenv();
    let conn_string = env::var("DATA_BASE").unwrap();

    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let connector = MakeTlsConnector::new(builder.build());

    let mut client = Client::connect(&conn_string, connector).unwrap();
    println!("Connection established");

    // Delete a data row from the table
    let deleted_rows = client.execute(
        "DELETE FROM users WHERE phone_number = $1",
        &[&phone_number],
    ).unwrap();

    if deleted_rows > 0 {
       // println!("Deleted user");
        return Ok(deleted_rows)
    } else {
       // println!("not found in the table.");
        return Err("something went wrong while deleting the user".to_string());
    }

    
}