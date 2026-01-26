mod data_access; // bring the top-level module into scope

use data_access::read_data::{find_all_users};



use std::fmt::Debug;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use serde_json::json;

//use crate::data_access::read_data::CLIENT;

//use crate::data_access::read_data::CLIENT;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);// cow stands for clone on write
    let new_request=request.to_string();
    println!("Request lrngth: {}", new_request.len());
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

