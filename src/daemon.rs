//! Overseer Daemon
//!
//! Author: Ramith Jayatilleka

use types::*;

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

fn handle_start(globals: &mut (), state: &DaemonState) -> DaemonState {
    DaemonState::Done
}

pub fn execute_daemon_step(globals: &mut (), state: &DaemonState) -> Option<DaemonState> {
    match *state {
        DaemonState::Done => None,
        _ => Some(DaemonState::Done),
    }
}
