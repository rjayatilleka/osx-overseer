//! Overseer Client
//!
//! Author: Ramith Jayatilleka

use types::*;

use unix_socket::UnixDatagram;

/// Represents errors in client state machine
#[derive(Debug)]
pub enum ClientError {
    TodoErr,
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
    Exit(Result<(), ClientError>),
    Done,
}
