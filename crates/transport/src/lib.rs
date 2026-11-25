mod error;
pub use error::{Result, TransportError};

use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::{io, thread};

pub fn serve<F>(addr: impl ToSocketAddrs + std::fmt::Debug, handler: F) -> Result<()>
where
    F: Fn(TcpStream) -> io::Result<()> + Send + Sync + 'static,
{
    let address = format!("{addr:?}");
    let listener =
        TcpListener::bind(addr).map_err(|e| TransportError::Bind { address, source: e })?;

    serve_listener(listener, handler)
}

pub fn serve_listener<F>(listener: TcpListener, handler: F) -> Result<()>
where
    F: Fn(TcpStream) -> io::Result<()> + Send + Sync + 'static,
{
    let handler = Arc::new(handler);

    // listen for streams and spawn a worker thread for each
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => spawn_worker(stream, Arc::clone(&handler)),
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }

    Ok(())
}

fn spawn_worker<F>(stream: TcpStream, handler: Arc<F>)
where
    F: Fn(TcpStream) -> io::Result<()> + Sync + Send + 'static,
{
    thread::spawn(move || {
        let result = handler(stream);
        println!("{:?}", result);
    });
}

#[cfg(test)]
mod test;
