use dotenvy::dotenv;
use postgres::{Client, Row};
use openssl::{error::Error, ssl::{SslConnector, SslMethod}};
use postgres_openssl::MakeTlsConnector;
use serde::Serialize;
use std::sync::{LazyLock,Mutex};
//use core::error;
use std::env;

//Result<(), Box<dyn std::error::Error>>
#[derive(Debug,Clone,Serialize )]
pub struct Users{
pub id:i32,
pub name:String,
pub age:i32,
pub email_id:String,
pub phone_number:String,
pub occupation:String,
}
pub fn find_all_users()->Result<Vec<Users>,String>{
    dotenv();
     let conn_string = env::var("DATA_BASE").expect("DATA_BASE env var not set");

    let builder = SslConnector::builder(SslMethod::tls())
        .expect("Failed to build SSL connector");
    let connector = MakeTlsConnector::new(builder.build());

    let mut  client = Client::connect(&conn_string, connector)
        .expect("Failed to connect to database");

    let users = client.query("SELECT * FROM users ORDER BY id;", &[]).unwrap();
    let mut users_arr = Vec::new();

    for item in users {
        let id: i32 = item.get("id");
        let name: String = item.get("name");
        let email_id: String = item.get("email_id");
        let age: i32 = item.get("age");
        let phone_number: String = item.get("phone_number");
        let occupation: String = item.get("occupation");

        users_arr.push(Users {
            id,
            name,
            email_id,
            age,
            phone_number,
            occupation,
        });
    }
if users_arr.len()!=0{
return Ok(users_arr);
}
else{
   return Err("something went in users_collection".to_string());
}
 
  
 //Ok(())
}
fn main(){

}

// pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//    // dotenv()?;

//     dotenv();
//      let conn_string = env::var("DATA_BASE").expect("DATA_BASE env var not set");

//     let builder = SslConnector::builder(SslMethod::tls())
//         .expect("Failed to build SSL connector");
//     let connector = MakeTlsConnector::new(builder.build());

//     let mut  client = Client::connect(&conn_string, connector)
//         .expect("Failed to connect to database");

//     let users = client.query("SELECT * FROM users ORDER BY id;", &[])?;
//     let mut users_arr = Vec::new();

//     for item in users {
//         let id: i32 = item.get("id");
//         let name: String = item.get("name");
//         let email_id: String = item.get("email_id");
//         let age: i32 = item.get("age");
//         let phone_number: String = item.get("phone_number");
//         let occupation: String = item.get("occupation");

//         users_arr.push(Users {
//             id,
//             name,
//             email_id,
//             age,
//             phone_number,
//             occupation,
//         });
//     }

//    // Ok(users_arr)
  
//  Ok(())
// }

//use postgres::{Client, Error, Row};

