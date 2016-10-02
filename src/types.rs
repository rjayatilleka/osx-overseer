//! Various data structures for Overseer
//!
//! Author: Ramith Jayatilleka

use std::path::PathBuf;

/// Holds concrete values for XDG_CONFIG_HOME and XDG_DATA_HOME
pub struct Homes {
    config_home: PathBuf,
    data_home: PathBuf,
}

/// Holds a response from the daemon
pub struct Response {
    data: String,
}
