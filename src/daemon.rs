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
    SocketOpenErr(io::Error),
    TodoErr,
}

impl From<DaemonError> for DaemonState {
    fn from(e: DaemonError) -> DaemonState {
        DaemonState::Fatal(e)
    }
}

type StepResult = Result<DaemonState, DaemonError>;

/// States for client state machine
#[derive(Debug)]
pub enum DaemonState {
    Start,
    InitHomes,
    OpenSocket(Homes),
    // ReceiveRequest(UnixDatagram),
    // ProcessRequest(UnixDatagram, Request),
    // SendResponse(UnixDatagram),
    Fatal(DaemonError),
    Success,
}

/// Start -> InitHomes
fn handle_start() -> StepResult {
    Ok(DaemonState::InitHomes)
}

/// InitHomes -> OpenSocket
fn handle_init_homes() -> StepResult {
    let homes = types::get_homes();
    Ok(DaemonState::OpenSocket(homes))
}

/// OpenSocket -> ReceiveRequest or Fatal
fn handle_open_socket(homes: Homes) -> StepResult {
    let socket_path = homes.data_home + "/socket";

        .map_err(DaemonError::SocketOpenErr)
        .map(|_| DaemonState::Success)
        // .map(DaemonState::ReceiveRequest)
}

/// Fatal
fn handle_fatal(e: DaemonError) -> i32 {
    let _ = writeln!(&mut io::stderr(), "overseerd FATAL: {}", e);
    1
}

/// Success
fn handle_success() -> i32 {
    println!("overseerd success");
    0
}

// /// Execute one step in DaemonState state machine.
// pub fn execute_daemon_step(state: DaemonState) -> Verdict<DaemonState> {
//     sm_generate!(
//         state,
//         DaemonState;
//         Start => handle_start,
//         InitHomes => handle_init_homes,
//         OpenSocket => handle_open_socket,
//         Fatal => handle_fatal,
//         Success => handle_success
//     )
// }

/// Execute one step in DaemonState state machine.
pub fn execute_daemon_step(state: DaemonState) -> Verdict<DaemonState> {
    match state {
        DaemonState::Start => From::from(handle_start()),
        DaemonState::InitHomes => From::from(handle_init_homes()),
        DaemonState::OpenSocket(homes) => From::from(handle_open_socket(homes)),
        DaemonState::Fatal(e) => From::from(handle_fatal(e)),
        DaemonState::Success => From::from(handle_success()),
    }
}
