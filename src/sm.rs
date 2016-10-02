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

/// Verdict of running one step in SM 
/// 1. Continue with next state
/// 2. End with given exit code
pub enum Verdict<S> {
    Continue(S),
    End(i32),
}

impl <S> From<i32> for Verdict<S> {
    fn from(i: i32) -> Verdict<S> {
        Verdict::End(i)
    }
}

impl <S, E> From<Result<S, E>> for Verdict<S> where S: From<E> {
    fn from(r: Result<S, E>) -> Verdict<S> {
        let next: S = r.unwrap_or_else(|e: E| From::from(e));
        Verdict::Continue(next)
    }
}

/// Run a state machine with the given executor and initial state.
/// Return the final exit code.
pub fn execute_sm<S, F>(initial_state: S, execute_step: F) -> i32
        where F: Fn(S) -> Verdict<S> {
    let mut state = initial_state;
    loop {
        match execute_step(state) {
            Verdict::Continue(next) => state = next,
            Verdict::End(code) => return code,
        }
    }
}
