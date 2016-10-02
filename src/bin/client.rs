//! Primary executable for Overseer
//!
//! Author: Ramith Jayatilleka

extern crate unix_socket;

use std::{error, fmt};
use std::path::PathBuf;
use unix_socket::UnixDatagram;

/// Represents errors in client state machine
#[derive(Debug)]
enum ClientSMError {
    TodoErr,
}

impl error::Error for ClientSMError {
    fn description(&self) -> &str {
        match *self {
            ClientSMError::TodoErr => "Todo",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ClientSMError::TodoErr => None,
        }
    }
}

impl fmt::Display for ClientSMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientSMError::TodoErr => write!(f, "TodoErr"),
        }
    }
}

/// Holds concrete values for XDG_CONFIG_HOME and XDG_DATA_HOME
struct Homes {
    config_home: PathBuf,
    data_home: PathBuf,
}

/// Holds a response from the daemon
struct Response {
    data: String,
}

/// States for client state machine
enum ClientState {
    Start,
    InitHomes,
    CheckSocketExists(Homes),
    OpenSocket(Homes),
    SendRequest(UnixDatagram),
    CloseSocket(UnixDatagram, Box<ClientState>),
    DeleteSocketFile(Homes),
    LaunchDaemon(Homes),
    ReadResponse(UnixDatagram),
    PrintResponse(Response),
    Exit(Result<(), ClientSMError>),
}

fn execute_sm<G, S, E, P>(
    globals: &mut G, start_state: S, execute_step: E, is_finished: P) -> S
    where E: Fn(&mut G, S) -> S,
          P: Fn(S) -> bool {
    start_state
}

fn main() {
    println!("Hello from client");
    let forty = 40;
    let dex = forty + 4;
    println!("{} is dex", dex);
}
