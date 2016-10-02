//! Implementations for std::error::Error for Overseer error types
//!
//! Author: Ramith Jayatilleka

use client::ClientSMError;
use std::{error, fmt};

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
