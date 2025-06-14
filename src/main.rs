use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// DATABASE URL
const DB_URL : &str = !env("DATABASE_URL");

// constants
const OK_RESPONSE : &str = "HTTP 1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND_RESPONSE : &str = "HTTP 1.1 404 Not Found\r\nContent-Type: application/json\r\n\r\n";
const INTERNAL_SERVER_ERROR_RESPONSE : &str = "HTTP 1.1 500 Internal Server Error\r\nContent-Type: application/json\r\n\r\n";

fn main() {
    if let Err(e) = set_database() {
        println!("Error: {}", e);
        return;
    }
    let listener = TcpListener::bind(format!(0.0.0.0:8080)).unwrap();
    println!("Server started at port 8080");
}

fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;

    client.execute("CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL
    )", &[])?;
}