//! Implementations for std::error::Error for Overseer error types
//!
//! Author: Ramith Jayatilleka

use client::ClientError;
use daemon::DaemonError;
use std::{error, fmt};

impl error::Error for ClientError {
    fn description(&self) -> &str {
        match *self {
            ClientError::TodoErr => "Todo",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ClientError::TodoErr => None,
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientError::TodoErr => write!(f, "TodoErr"),
        }
    }
}

impl error::Error for DaemonError {
    fn description(&self) -> &str {
        match *self {
            DaemonError::SocketOpenErr(_) => "Failed to open socket",
            DaemonError::TodoErr => "Todo",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DaemonError::SocketOpenErr(ref e) => Some(e),
            DaemonError::TodoErr => None,
        }
    }
}

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DaemonError::SocketOpenErr(ref c) => write!(f, "Failed to open socket: {}", c),
            DaemonError::TodoErr => write!(f, "TodoErr"),
        }
    }
}
