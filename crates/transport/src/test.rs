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

#[test]
fn bind_fails_when_port_is_taken() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let result = serve(addr, |_| Ok(()));

    assert!(matches!(result, Err(TransportError::Bind { .. })));
}
