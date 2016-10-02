//! Overseer Daemon
//!
//! Author: Ramith Jayatilleka

use types;
use types::Homes;
use sm;
use sm::Verdict::{Continue, End};

use std::io;
use std::io::Write;
use unix_socket::UnixDatagram;

/// Represents errors in client state machine
#[derive(Debug)]
pub enum DaemonError {
    TodoErr,
}

type Request = [u8; 10];
type Verdict = sm::Verdict<DaemonState>;

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
fn handle_start() -> Verdict {
    Continue(DaemonState::InitHomes)
}

/// InitHomes -> OpenSocket
fn handle_init_homes() -> Verdict {
    let homes = types::get_homes();
    Continue(DaemonState::OpenSocket(homes))
}

/// OpenSocket -> ReceiveRequest or Fatal
fn handle_open_socket(homes: Homes) -> Verdict {
    Continue(DaemonState::Success)
}

/// Fatal
fn handle_fatal(e: DaemonError) -> Verdict {
    let _ = writeln!(&mut io::stderr(), "overseerd FATAL: {}", e);
    End(1)
}

/// Success
fn handle_success() -> Verdict {
    println!("overseerd success");
    End(0)
}

/// Execute one step in DaemonState state machine.
pub fn execute_daemon_step(state: DaemonState) -> Verdict {
    match state {
        DaemonState::Start => handle_start(),
        DaemonState::InitHomes => handle_init_homes(),
        DaemonState::OpenSocket(homes) => handle_open_socket(homes),
        DaemonState::Fatal(e) => handle_fatal(e),
        DaemonState::Success => handle_success(),
        _ => Continue(DaemonState::Fatal(DaemonError::TodoErr)),
    }
}
