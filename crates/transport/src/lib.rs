mod error;
pub use error::{Result, TransportError};

use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::{io, thread};

pub fn serve<F>(
    addr: impl ToSocketAddrs + std::fmt::Debug,
    handler: F,
    channel: Sender<String>,
) -> Result<()>
where
    F: Fn(TcpStream, &Sender<String>) -> io::Result<()> + Send + Sync + 'static,
{
    let address = format!("{addr:?}");
    let listener =
        TcpListener::bind(addr).map_err(|e| TransportError::Bind { address, source: e })?;

    serve_listener(listener, handler, channel)
}

pub fn serve_listener<F>(listener: TcpListener, handler: F, channel: Sender<String>) -> Result<()>
where
    F: Fn(TcpStream, &Sender<String>) -> io::Result<()> + Send + Sync + 'static,
{
    let handler = Arc::new(handler);

    // listen for streams and spawn a worker thread for each
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => spawn_worker(stream, Arc::clone(&handler), channel.clone()),
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }

    Ok(())
}

// spawn thread for every Connection
fn spawn_worker<F>(stream: TcpStream, handler: Arc<F>, channel: Sender<String>)
where
    F: Fn(TcpStream, &Sender<String>) -> io::Result<()> + Sync + Send + 'static,
{
    thread::spawn(move || {
        let peer = stream.peer_addr().unwrap(); // only for Debug! Change later
                                                //
        let _ = channel.send(format!("Connected with Peer: {}", peer));
        let result = handler(stream, &channel);
        let _ = channel.send(format!("Peer {} has disconnected", peer));
    });
}

#[cfg(test)]
mod test;
