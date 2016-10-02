//! Overseer Daemon
//!
//! Author: Ramith Jayatilleka

use types;
use types::Homes;
use sm::Verdict;

use std::io;
use std::io::Write;
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
    Success,
}


/// Start -> InitHomes
fn handle_start() -> Verdict<DaemonState> {
    Verdict::Continue(DaemonState::InitHomes)
}

/// InitHomes -> OpenSocket or Fatal
fn handle_init_homes() -> Verdict<DaemonState> {
    let homes = types::get_homes();
    println!("{:?}", homes);
    Verdict::Continue(DaemonState::Success)
}

/// Fatal
fn handle_fatal(e: DaemonError) -> Verdict<DaemonState> {
    let _ = writeln!(&mut io::stderr(), "overseerd FATAL: {}", e);
    Verdict::End(1)
}

/// Success
fn handle_success() -> Verdict<DaemonState> {
    println!("overseerd success");
    Verdict::End(0)
}

pub fn execute_daemon_step(state: DaemonState) -> Verdict<DaemonState> {
    match state {
        DaemonState::Start => handle_start(),
        DaemonState::InitHomes => handle_init_homes(),
        DaemonState::Fatal(e) => handle_fatal(e),
        DaemonState::Success => handle_success(),
        _ => Verdict::Continue(DaemonState::Fatal(DaemonError::TodoErr)),
    }
}
