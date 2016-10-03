//! Primary executable for Overseerd (daemon)
//!
//! Author: Ramith Jayatilleka

extern crate osx_overseer;
extern crate unix_socket;

use std::process;
use osx_overseer::{daemon, sm};

macro_rules! state_machine {
    ( $name:ident { $( $state:ident ),* }) => {
        fn $name() {
            $( println!("{}", stringify!($state)); )*
        }
    };
}

/// States for daemon state machine
state_machine! {
    TestState {
        Alpha,
        Beta,
        Gamma
    }
}

fn main() {
    TestState();
    let exit_code = sm::execute_sm(
        daemon::DaemonState::Start, daemon::execute_daemon_step);
    process::exit(exit_code);
}
