//! Primary executable for Overseer
//!
//! Author: Ramith Jayatilleka

extern crate osx-overseer;
extern crate unix_socket;

use std::{error, fmt};
use std::path::PathBuf;
use unix_socket::UnixDatagram;

fn main() {
    let s = TestState::Alpha(1);
    let mut g = 3;
    println!("{:?}", execute_sm(&mut g, s, test_exec));
}
