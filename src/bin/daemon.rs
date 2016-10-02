//! Primary executable for Overseerd (daemon)
//!
//! Author: Ramith Jayatilleka

extern crate osx_overseer;
extern crate unix_socket;

use std::process;
use osx_overseer::{daemon, sm};

fn main() {
    let exit_code = sm::execute_sm(
        daemon::DaemonState::Start, daemon::execute_daemon_step);
    process::exit(exit_code);
}
