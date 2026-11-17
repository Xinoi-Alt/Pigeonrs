use std::net::{TcpListener, TcpStream};

const PORT: u16 = 52530;

fn main() {
    let listener = TcpListener::bind(("127.0.0.1", PORT)).unwrap(); 
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream)=> println!("Connection successful!"),
            Err(e) => println!("{e}")
        }
    }
}