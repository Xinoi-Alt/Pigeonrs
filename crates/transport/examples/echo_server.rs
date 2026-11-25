use pigeonrs_transport::serve;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Sender};
use std::thread;

const PORT: u16 = 52530;

fn echo(mut stream: TcpStream, channel: &Sender<String>) -> io::Result<()> {
    let mut buffer = [0u8; 1024];

    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            return Ok(());
        }
        let message = String::from_utf8_lossy(&buffer[..n]).trim_end().to_string();
        let _ = channel.send(format!("Nachricht '{}' erhalten!", message));
        stream.write_all(&buffer[..n])?;
    }
}

fn main() -> pigeonrs_transport::Result<()> {
    // Channel to communicate with threads
    let (tx, rx) = mpsc::channel();

    // extra thread für Channel
    thread::spawn(move || {
        for event in rx {
            println!("DEBUG: {:?}", event);
        }
    });

    serve(("127.0.0.1", PORT), echo, tx)
}
