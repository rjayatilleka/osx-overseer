//! Generic state machine code
//!
//! Author: Ramith Jayatilleka

// pub fn execute_sm_with_globals<G, S, F>(globals: &mut G, initial_state: S, execute_step: F) -> S
//         where F: Fn(&mut G, &S) -> Option<S> {
//     let mut state = initial_state;
//     loop {
//         match execute_step(globals, &state) {
//             Some(next) => state = next,
//             None => return state,
//         }
//     }
// }

pub fn execute_sm<S, F>(initial_state: S, execute_step: F) where F: Fn(S) -> Option<S> {
    let mut state = initial_state;
    loop {
        match execute_step(state) {
            Some(next) => state = next,
            None => return,
        }
    }
}
