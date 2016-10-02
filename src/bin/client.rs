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

#[derive(Debug)]
enum TestState {
    Alpha(i32),
    Beta(i32),
    Gamma(i32),
}

fn test_exec(g: &mut i32, s: &TestState) -> Option<TestState> {
    match *s {
        TestState::Alpha(a) => Some(TestState::Beta(a * *g)),
        TestState::Beta(a) => {
            *g -= 1;
            Some(TestState::Gamma(a + *g))
        },
        TestState::Gamma(_) => None
    }
}

fn execute_sm<G, S, E>(globals: &mut G, start_state: S, execute_step: E) -> S
        where E: Fn(&mut G, &S) -> Option<S> {
    let mut state = start_state;

    loop {
        match execute_step(globals, &state) {
            Some(next) => state = next,
            None => return state,
        }
    }
}

fn main() {
    let s = TestState::Alpha(1);
    let mut g = 3;
    println!("{:?}", execute_sm(&mut g, s, test_exec));
}
