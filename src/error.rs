//! Implementations for std::error::Error for Overseer error types
//!
//! Author: Ramith Jayatilleka

use client::ClientError;
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
