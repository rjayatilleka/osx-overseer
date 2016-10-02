//! Various data structures for Overseer
//!
//! Author: Ramith Jayatilleka

use std::path::PathBuf;

/// Holds concrete values for XDG_CONFIG_HOME and XDG_DATA_HOME
#[derive(Debug)]
pub struct Homes {
    pub config_home: PathBuf,
    pub data_home: PathBuf,
}

/// Holds a response from the daemon
#[derive(Debug)]
pub struct Response {
    pub data: String,
}
