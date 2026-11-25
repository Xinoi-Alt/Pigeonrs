mod error;
pub use error::{Result, TransportError};

use std::net::{TcpListener, TcpStream, ToSocketAddrs, SocketAddr};
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{io, thread};

pub trait Handler: Fn(u64, TcpStream, &Sender<Event>) -> io::Result<()> + Send + Sync + 'static {}
impl<F> Handler for F where
    F: Fn(u64, TcpStream, &Sender<Event>) -> io::Result<()> + Send + Sync + 'static {}

// Events for Channel
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Connected { id: u64, peer: SocketAddr },
    Received { id: u64, bytes: usize },
    // None bei absichtlichem Ende
    Disconnected { id: u64, error: Option<String> },
    AcceptFailed {error: String}
}
// ID for Threads
static NEXT_ID: AtomicU64 = AtomicU64::new(0);
fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

pub fn serve<F: Handler>(addr: impl ToSocketAddrs + std::fmt::Debug, handler: F, channel: Sender<Event>) -> Result<()> {
    let address = format!("{addr:?}");
    let listener =
        TcpListener::bind(addr).map_err(|e| TransportError::Bind { address, source: e })?;

    serve_listener(listener, handler, channel)
}

pub fn serve_listener<F: Handler>(listener: TcpListener, handler: F, channel: Sender<Event>) -> Result<()> {
    let handler = Arc::new(handler);

    // listen for streams and spawn a worker thread for each
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => spawn_worker(next_id(), stream, Arc::clone(&handler), channel.clone()),
            Err(e) => {
                let _ = channel.send(Event::AcceptFailed {error: e.to_string()});
            },
        }
    }

    Ok(())
}

// spawn thread for every Connection
fn spawn_worker<F: Handler>(id: u64, stream: TcpStream, handler: Arc<F>, channel: Sender<Event>) {
    thread::spawn(move || {
        if let Ok(peer) = stream.peer_addr() {
            let _ = channel.send(Event::Connected{id, peer});
        }
        let result = handler(id, stream, &channel);
        let _ = channel.send(Event::Disconnected{
            id,
            error: result.err().map(|e| e.to_string())
        });
    });
}

#[cfg(test)]
mod test;
