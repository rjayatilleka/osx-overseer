//! Primary executable for Overseerd (daemon)
//!
//! Author: Ramith Jayatilleka

extern crate osx_overseer;
extern crate unix_socket;

use osx_overseer::{daemon, sm};
use osx_overseer::daemon::DaemonState;

fn main() {
    let mut globals = ();
    let initial_state = DaemonState::Start;
    let final_state = sm::execute_sm(&mut globals, initial_state, daemon::execute_daemon_step);
    println!("{:?}", final_state);
}
