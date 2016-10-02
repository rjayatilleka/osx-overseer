//! Overseer Client
//!
//! Author: Ramith Jayatilleka

/// Represents errors in client state machine
#[derive(Debug)]
enum ClientSMError {
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
    Exit(Result<(), ClientSMError>),
    End,
}
