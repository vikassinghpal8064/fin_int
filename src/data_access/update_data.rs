use dotenvy::dotenv;
use postgres::Client;
use openssl::{error::Error, ssl::{SslConnector, SslMethod}};
use postgres_openssl::MakeTlsConnector;
use std::{env, f64::consts::E, fmt::format};
use serde::{Deserialize, Serialize};


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

// pub fn Add_user(user:Users){
//       dotenv();
//     let conn_string = env::var("DATA_BASE").unwrap();
    
//     let builder = SslConnector::builder(SslMethod::tls()).unwrap();
//     let connector = MakeTlsConnector::new(builder.build());

//     let mut client = Client::connect(&conn_string, connector).unwrap();
//     //println!("Connection established");
//    let val= client.execute("INSERT INTO users (name, age, phone_number, email_id, occupation,trust)
//  VALUES ($1, $2, $3, $4, $5,$6)",
//         &[&user.name, &user.age, &user.phone_number, &user.email_id, &user.occupation,&user.trust]);
//     //println!("{}",val.unwrap());
// match val{
//     Ok(value)=>{ println!("{}",value)}
//     Err(e)=>{ println!("{}",e)}
// }
// }
pub fn Add_user(user: Users)->Result<u64,String> {
    dotenv();
    let conn_string = env::var("DATA_BASE").unwrap();

    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let connector = MakeTlsConnector::new(builder.build());

    let mut client = Client::connect(&conn_string, connector).unwrap();

    let val = client.execute(
        "INSERT INTO users (name, age, phone_number, email_id, occupation, trust)
         VALUES ($1, $2, $3, $4, $5, $6)",
        &[&user.name, &user.age, &user.phone_number, &user.email_id, &user.occupation, &user.trust],
    );

    match val {
        Ok(value) => {return Ok(value);},
        Err(e) => {return Err(format!("Error while adding user {:?}",e).to_string())},
    }
}
#[derive(Debug,Clone,Serialize,Deserialize )]
pub struct Users{
pub id:i32,
pub name:String,
pub age:i32,
pub email_id:String,
pub phone_number:String,
pub occupation:String,
pub trust:String
}