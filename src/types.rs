//! Various data structures for Overseer
//!
//! Author: Ramith Jayatilleka

use std::env;

/// Holds concrete values for XDG_CONFIG_HOME and XDG_DATA_HOME
#[derive(Debug)]
pub struct Homes {
    pub config_home: String,
    pub data_home: String,
}

/// Holds a response from the daemon
#[derive(Debug)]
pub struct Response {
    pub data: String,
}

pub fn get_homes() -> Homes {
    let config_home_prefix =
        get_safe_env_var("XDG_CONFIG_HOME", env::var("HOME").unwrap() + "/.config");
    let data_home_prefix =
        get_safe_env_var("XDG_DATA_HOME", env::var("HOME").unwrap() + "/.local/share");

    Homes {
        config_home: config_home_prefix + "/overseer",
        data_home: data_home_prefix + "/overseer",
    }
}

fn get_safe_env_var(key: &str, default: String) -> String {
    env::var(key)
        .map(|value| value.trim().to_owned())
        .and_then(|value|
            if value.is_empty() {
                Err(env::VarError::NotPresent)
            } else {
                Ok(value)
            })
        .unwrap_or(default)
}
