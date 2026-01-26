mod data_access; // bring the top-level module into scope

use data_access::read_data::{find_all_users};
use data_access::delete_data::delete_data;
use data_access::update_data::Add_user;
use serde::Deserialize;



use std::fmt::Debug;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use serde_json::json;

use crate::data_access::update_data::Users;

//use crate::data_access::read_data::CLIENT;

//use crate::data_access::read_data::CLIENT;

fn handle_connection(mut stream: TcpStream) {
    // let mut buffer = [0; 512];
    // stream.read(&mut buffer).unwrap();

    // let request = String::from_utf8_lossy(&buffer);// cow stands for clone on write
   let mut buffer = [0; 1024];
let bytes_read = stream.read(&mut buffer).unwrap();

let request = String::from_utf8_lossy(&buffer[..bytes_read]);
     let request_line = request.lines().next().unwrap();

    let path = request_line.split_whitespace().nth(1).unwrap().to_string();
    let file_path=String::new();
    match path.as_str(){
    "/"=>{
        let data = json!({
        "status": "ok",
        "message": "Hello, vikas, I am live"
    });
        let contents=data.to_string();
     let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    },
    "/find_all_users"=>{
       // let mut client =  CLIENT.lock().unwrap();
        
        let all_users=find_all_users();

     match all_users{
        Ok(arr)=>{
            let data=json!({
               "status":"ok",
            "message":arr,
            });
              let contents=data.to_string();
     let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    ); 
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
        },
        Err(e)=>{
        stream.write_all(e.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
     }
    
   
    }
    "/delete_user"=>{
        let parts :Vec<&str>= request.split("\r\n\r\n").collect();
        let headers = parts.get(0).unwrap();
    let body = parts.get(1).unwrap();
     // println!("Body:\n{:?}", body);
    let json_body :Delete_req=serde_json::from_str(*body).unwrap();
    println!("Body:\n{:?}", json_body);
    let val=delete_data(json_body.phone_number).unwrap_or(0);
    let data=json!({
        "status":"ok",
        "message":val
    });
    let contents=data.to_string();
     let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    ); 
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
 
     "/create_user"=>{
        let parts :Vec<&str>= request.split("\r\n\r\n").collect();
        let headers = parts.get(0).unwrap();
    let body = parts.get(1).unwrap();
     // println!("Body:\n{:?}", body);
   let user1 :Users=serde_json::from_str(*body).unwrap();
    
    println!("Body:\n{:?}", user1);
    // Add_user(user1);
let val=Add_user(user1).unwrap_or(0);
    let data=json!({
        "status":"ok",
        "message":val
    });
    let contents=data.to_string();
     let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    ); 
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
     }
    _=>{
    let response="something went wrong at the backend ".to_string();
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
    
    }
    
 

   
  
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
       
        handle_connection(stream);
    
        //fn all_users() {
   // let users = find_all_users();
    //println!("{:?}", users);
}
    }
#[derive(Debug,Deserialize)]
struct Delete_req{
    name:String,
    phone_number:String
}