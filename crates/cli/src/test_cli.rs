use super::*;
use clap::CommandFactory;
use clap::error::ErrorKind;

#[test]
fn cli_definition_is_valid() {
    CLI::command().debug_assert();
}

#[test]
fn send_args_with_peer() {
    let cli = CLI::try_parse_from(["pigeonrs", "send", "testfile1.tf", "testpeer1"]).unwrap();
    let Commands::Send(args) = cli.command else {
        panic!("Befehl send erwartet!");
    };
    assert_eq!(
        args,
        SendArgs {
            file: PathBuf::from("testfile1.tf"),
            target: Target {
                peer: Some("testpeer1".to_string()),
                ip: None,
            },
            port: 52530,
        }
    );
}

#[test]
fn send_args_with_ip() {
    let cli = CLI::try_parse_from(["pigeonrs", "send", "testfile1.tf", "--ip", "192.168.178.35"])
        .unwrap();
    let Commands::Send(args) = cli.command else {
        panic!("Befehl send erwartet!");
    };
    assert_eq!(
        args,
        SendArgs {
            file: PathBuf::from("testfile1.tf"),
            target: Target {
                peer: None,
                ip: Some("192.168.178.35".parse().unwrap()),
            },
            port: 52530,
        }
    );
}

#[test]
fn send_peer_and_ip_conflict() {
    let err = CLI::try_parse_from(["pigeonrs", "send", "foto.jpg", "anna", "--ip", "10.0.0.2"])
        .err()
        .expect("sollte fehlschlagen");

    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}
