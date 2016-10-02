//! Generic state machine code
//!
//! Author: Ramith Jayatilleka

pub fn execute_sm<G, S, E>(globals: &mut G, initial_state: S, execute_step: E) -> S
        where E: Fn(&mut G, &S) -> Option<S> {
    let mut state = initial_state;

    loop {
        match execute_step(globals, &state) {
            Some(next) => state = next,
            None => return state,
        }
    }
}
