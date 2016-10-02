//! Overseer Daemon
//!
//! Author: Ramith Jayatilleka

use types;
use types::Homes;

use unix_socket::UnixDatagram;

/// Represents errors in client state machine
#[derive(Debug)]
pub enum DaemonError {
    TodoErr,
}

type Request = [u8; 10];

/// States for client state machine
#[derive(Debug)]
pub enum DaemonState {
    Start,
    InitHomes,
    OpenSocket(Homes),
    ReceiveRequest(UnixDatagram),
    ProcessRequest(UnixDatagram, Request),
    SendResponse(UnixDatagram),
    Exit(Result<(), DaemonError>),
    Done,
}


/// Start -> InitHomes
fn handle_start() -> DaemonState {
    println!("Start");
    DaemonState::InitHomes
}


/// InitHomes -> OpenSocket or Exit
fn handle_init_homes() -> DaemonState {
    println!("InitHomes");

    let homes = types::get_homes();
    println!("{:?}", homes);
    DaemonState::Exit(Ok(()))
}

fn handle_exit(result: Result<(), DaemonError>) -> DaemonState {
    println!("Exit");

    match result {
        Ok(s) => println!("overseerd success"),
        Err(e) => println!("{:?}", e),
    }

    DaemonState::Done
}

pub fn execute_daemon_step(state: DaemonState) -> Option<DaemonState> {
    match state {
        DaemonState::Start => Some(handle_start()),
        DaemonState::InitHomes => Some(handle_init_homes()),
        DaemonState::Exit(r) => Some(handle_exit(r)),
        DaemonState::Done => None,
        _ => Some(DaemonState::Done),
    }
}
