mod error;
pub use error::{TransportError, Result};

use std::io;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};


pub fn serve<F>(addr: impl ToSocketAddrs + std::fmt::Debug, handler: F) -> Result<()>
where
    F: Fn(TcpStream) -> io::Result<()>,
{
    let address = format!("{addr:?}");
    let listener= TcpListener::bind(addr).map_err(|e| TransportError::Bind {address, source: e})?;

    serve_listener(listener, handler)
}

pub fn serve_listener<F>(listener: TcpListener, handler: F) -> Result<()>
where
    F: Fn(TcpStream) -> io::Result<()>,
{
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handler(stream) {
                    // TODO: replace eprint with proper logging
                    eprintln!("Handler error: {e}");
                }
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

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
    #[test]
    fn echo_roundtrip() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        std::thread::spawn(move || serve_listener(listener, echo));

        let mut client = TcpStream::connect(addr).unwrap();
        client.write_all(b"hello").unwrap();
        let mut buffer = [0u8; 5];
        client.read_exact(&mut buffer).unwrap();

        assert_eq!(&buffer, b"hello");
    }
}
