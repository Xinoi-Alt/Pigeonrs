use pigeonrs_discovery::{Announce, DISCOVERY_PORT, announce_loop, listen_loop};
use std::env;
use std::io;

const TCP_PORT: u16 = 52530;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "listen" {
        listen_loop(DISCOVERY_PORT)
    } else if args.len() == 3 && args[1] == "announce" {
        let announce = Announce {
            device_name: args[2].clone(),
            tcp_port: TCP_PORT,
        };
        println!("Rufe alle 2 Sekunden als \"{}\"", announce.device_name);
        announce_loop(&announce, DISCOVERY_PORT)
    } else {
        println!("Benutzung:");
        println!("  cargo run -p pigeonrs-discovery --example broadcast -- listen");
        println!("  cargo run -p pigeonrs-discovery --example broadcast -- announce <gerätename>");
        Ok(())
    }
}
