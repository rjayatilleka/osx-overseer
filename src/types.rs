
//! Various data structures for Overseer
//!
//! Author: Ramith Jayatilleka

/// Holds concrete values for XDG_CONFIG_HOME and XDG_DATA_HOME
struct Homes {
    config_home: PathBuf,
    data_home: PathBuf,
}

/// Holds a response from the daemon
struct Response {
    data: String,
}
