//! Overseer Daemon
//!
//! Author: Ramith Jayatilleka

use types;
use types::Homes;

use std::io;
use std::io::Write;
use std::error::Error;
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
    Fatal(DaemonError),
    Done,
}


/// Start -> InitHomes
fn handle_start() -> DaemonState {
    DaemonState::InitHomes
}


/// InitHomes -> OpenSocket or Exit
fn handle_init_homes() -> DaemonState {
    let homes = types::get_homes();
    println!("{:?}", homes);
    DaemonState::Done
}

fn handle_fatal(e: DaemonError) -> DaemonState {
    let _ = writeln!(&mut io::stderr(), "FATAL: {}", e.description());
    DaemonState::Done
}

pub fn execute_daemon_step(state: DaemonState) -> Option<DaemonState> {
    match state {
        DaemonState::Start => Some(handle_start()),
        DaemonState::InitHomes => Some(handle_init_homes()),
        DaemonState::Fatal(e) => Some(handle_fatal(e)),
        DaemonState::Done => None,
        _ => Some(DaemonState::Done),
    }
}
