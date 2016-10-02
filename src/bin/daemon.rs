//! Primary executable for Overseerd (daemon)
//!
//! Author: Ramith Jayatilleka

extern crate osx_overseer;
extern crate unix_socket;

use osx_overseer::{daemon, sm};

fn main() {
    sm::execute_sm(daemon::DaemonState::Start, daemon::execute_daemon_step);
}
