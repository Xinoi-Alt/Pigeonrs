use std::io;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub fn serve<F>(addr: impl ToSocketAddrs, handler: F) -> io::Result<()>
where
    F: Fn(TcpStream) -> io::Result<()>,
{
    let listener = TcpListener::bind(addr)?;

    serve_listener(listener, handler)
}

pub fn serve_listener<F>(listener: TcpListener, handler:F) -> io::Result<()> 
where
    F: Fn(TcpStream) -> io::Result<()>,{
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handler(stream) {
                    eprintln!("Handler error: {e}");
                }
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }

    Ok(())
} 
