use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

const PORT: u16 = 52530;

fn echo(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0u8; 1024];

    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            return Ok(());
        }
        stream.write_all(&buffer[..n])?;
    }
}
