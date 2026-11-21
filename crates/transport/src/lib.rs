use std::io;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub fn serve<F>(addr: impl ToSocketAddrs, handler: F) -> io::Result<()>
where
    F: Fn(TcpStream) -> io::Result<()>,
{
    // ...
}
