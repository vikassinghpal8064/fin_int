use dotenvy::dotenv;
use postgres::Client;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load environment variables from .env file
    dotenv()?;
    let conn_string = env::var("DATA_BASE")?;

    let builder = SslConnector::builder(SslMethod::tls())?;
    let connector = MakeTlsConnector::new(builder.build());

    let mut client = Client::connect(&conn_string, connector)?;
    println!("Connection established");

    client.batch_execute("DROP TABLE IF EXISTS users;")?;
    println!("Finished dropping table (if it existed).");

    // client.batch_execute(
    //     "CREATE TABLE users (
    //         id SERIAL PRIMARY KEY,
           
    //         name VARCHAR(255) NOT NULL,
    //         age INT,
    //         phone_number VARCHAR(20),
    //         email_id VARCHAR(255),
    //         occupation VARCHAR(255),
    //         photo BYTEA,
    //         trust VARCHAR(10) NOT NULL CHECK (trust IN ('low','high','medium')),
            
    //     );"
    // )?;
    client.batch_execute(
    "CREATE TABLE users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        age INT,
        phone_number VARCHAR(20),
        email_id VARCHAR(255),
        occupation VARCHAR(255),
        photo BYTEA,
        trust VARCHAR(10) NOT NULL CHECK (trust IN ('low','high','medium'))
    );"
)?;


    client.batch_execute("ALTER SEQUENCE users_id_seq RESTART WITH 7097")?;
    println!("Finished creating table.");

    // Insert a single book record
   // Data to be inserted (Name, Age, Phone, Email, Occupation, Photo Bytes, Trust)
let users_to_insert = [
    ("Aarav Sharma", 28, "+91-9876543210", "aarav.s@example.com", "Software Engineer", b"\xDE\xAD\xBE\xEF\x01".to_vec(), "high"),
    ("Priya Patel", 34, "+91-8765432109", "priya.patel@testmail.org", "Data Scientist", b"\xCA\xFE\xBA\xBE\x02".to_vec(), "medium"),
    ("Rohan Gupta", 22, "+91-7654321098", "rohan.g@webmail.com", "Student", b"\x01\x23\x45\x67\x89".to_vec(), "low"),
    ("Sanya Iyer", 41, "+91-6543210987", "sanya.iyer@corp.in", "Project Manager", b"\xAB\xCD\xEF\x12\x34".to_vec(), "high"),
    ("Vikram Singh", 29, "+91-5432109876", "v.singh@provider.net", "Graphic Designer", b"\x55\xAA\x55\xAA\x05".to_vec(), "medium"),
    ("Ananya Das", 31, "+91-4321098765", "ananya.das@startup.io", "Product Designer", b"\x88\x99\xAA\xBB\xCC".to_vec(), "high"),
    ("Kabir Malhotra", 45, "+91-3210987654", "k.malhotra@firm.com", "Consultant", b"\xFF\xEE\xDD\xCC\xBB".to_vec(), "low"),
    ("Meera Reddy", 27, "+91-2109876543", "meera.r@freelance.com", "Content Writer", b"\x11\x22\x33\x44\x55".to_vec(), "medium"),
    ("Ishaan Verma", 38, "+91-1098765432", "ishaan.v@tech.com", "DevOps Engineer", b"\x66\x77\x88\x99\x00".to_vec(), "high"),
    ("Zoya Khan", 25, "+91-0987654321", "zoya.k@edu.org", "Researcher", b"\xAA\x11\xBB\x22\xCC".to_vec(), "medium"),
];

// Start a transaction
let mut transaction = client.transaction()?;
println!("Starting transaction to insert multiple users...");

for user in &users_to_insert {
    transaction.execute(
        "INSERT INTO users (name, age, phone_number, email_id, occupation, photo, trust) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&user.0, &user.1, &user.2, &user.3, &user.4, &user.5, &user.6],
    )?;
}

// Commit the transaction
transaction.commit()?;
println!("Successfully inserted 10 users.");


 

   Ok(())
}