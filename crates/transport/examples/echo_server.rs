use pigeonrs_transport::{Event, serve};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Sender};
use std::thread;

const PORT: u16 = 52530;

fn echo(id: u64, mut stream: TcpStream, channel: &Sender<Event>) -> io::Result<()> {
    let mut buffer = [0u8; 1024];

    loop {
        let bytes = stream.read(&mut buffer)?;
        if bytes == 0 {
            return Ok(());
        }
        let _ = channel.send(Event::Received{id, bytes});
        stream.write_all(&buffer[..bytes])?;
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
