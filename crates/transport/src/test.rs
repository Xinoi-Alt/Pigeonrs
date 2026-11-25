use super::*;
use std::collections::HashSet;
use std::io::{Read, Write};
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(2);

fn echo(id: u64, mut stream: TcpStream, channel: &Sender<Event>) -> io::Result<()> {
    let mut buffer = [0u8; 1024];

    loop {
        let bytes = stream.read(&mut buffer)?;
        if bytes == 0 {
            return Ok(());
        }
        let _ = channel.send(Event::Received { id, bytes });
        stream.write_all(&buffer[..bytes])?;
    }
}

fn start_server<F: Handler>(handler: F) -> (SocketAddr, Receiver<Event>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || serve_listener(listener, handler, tx));

    (addr, rx)
}

fn wait_for(events: &Receiver<Event>, matches: impl Fn(&Event) -> bool) -> Event {
    loop {
        let event = events
            .recv_timeout(TIMEOUT)
            .expect("expected event did not arrive in time");
        if matches(&event) {
            return event;
        }
    }
}

#[test]
fn echo_roundtrip() {
    let (addr, events) = start_server(echo);

    let mut client = TcpStream::connect(addr).unwrap();
    client.set_read_timeout(Some(TIMEOUT)).unwrap();
    client.write_all(b"hello").unwrap();
    let mut buffer = [0u8; 5];
    client.read_exact(&mut buffer).unwrap();

    assert_eq!(&buffer, b"hello");

    let event = wait_for(&events, |e| matches!(e, Event::Received { .. }));
    assert!(matches!(event, Event::Received { bytes: 5, .. }));
}

#[test]
fn bind_fails_when_port_is_taken() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, _rx) = mpsc::channel();

    let result = serve(addr, |_, _, _| Ok(()), tx);

    assert!(matches!(result, Err(TransportError::Bind { .. })));
}

#[test]
fn idle_client_does_not_block_others() {
    let (addr, _events) = start_server(echo);

    let _idle = TcpStream::connect(addr).unwrap();

    let mut client = TcpStream::connect(addr).unwrap();
    client.set_read_timeout(Some(TIMEOUT)).unwrap();
    client.write_all(b"hello").unwrap();
    let mut buffer = [0u8; 5];
    client.read_exact(&mut buffer).unwrap();

    assert_eq!(&buffer, b"hello");
}

#[test]
fn each_connection_reports_connected_and_disconnected() {
    let (addr, events) = start_server(echo);

    let client_a = TcpStream::connect(addr).unwrap();
    let client_b = TcpStream::connect(addr).unwrap();

    let mut connected = HashSet::new();
    while connected.len() < 2 {
        if let Event::Connected { id, .. } =
            wait_for(&events, |e| matches!(e, Event::Connected { .. }))
        {
            connected.insert(id);
        }
    }

    drop(client_a);
    drop(client_b);

    let mut disconnected = HashSet::new();
    while disconnected.len() < 2 {
        if let Event::Disconnected { id, error } =
            wait_for(&events, |e| matches!(e, Event::Disconnected { .. }))
        {
            assert_eq!(error, None);
            disconnected.insert(id);
        }
    }

    assert_eq!(connected.len(), 2);
    assert_eq!(connected, disconnected);
}

#[test]
fn handler_error_is_reported_in_disconnected_event() {
    let failing =
        |_id: u64, _stream: TcpStream, _channel: &Sender<Event>| Err(io::Error::other("fail"));
    let (addr, events) = start_server(failing);

    let _client = TcpStream::connect(addr).unwrap();

    let event = wait_for(&events, |e| matches!(e, Event::Disconnected { .. }));
    match event {
        Event::Disconnected {
            error: Some(message),
            ..
        } => assert!(message.contains("fail")),
        other => panic!("expected Disconnected with an error, got {other:?}"),
    }
}
