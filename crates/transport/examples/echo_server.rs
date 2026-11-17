use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

const PORT: u16 = 52530;

fn handle_client(mut stream: TcpStream) {   
    let mut buffer = [0u8; 1024]; 
    let n = stream.read(&mut buffer).unwrap();
    println!("Read {n} bytes");
}
fn main() {
    let listener = TcpListener::bind(("127.0.0.1", PORT)).unwrap(); 
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream)=> handle_client(stream),
            Err(e) => println!("{e}")
        }
    }
}