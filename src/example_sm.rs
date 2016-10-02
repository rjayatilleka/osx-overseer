//! Stuff. Should use for tests later
//!
//! Author: Ramith Jayatilleka

#[derive(Debug)]
enum TestState {
    Alpha(i32),
    Beta(i32),
    Gamma(i32),
}

fn test_exec(g: &mut i32, s: &TestState) -> Option<TestState> {
    match *s {
        TestState::Alpha(a) => Some(TestState::Beta(a * *g)),
        TestState::Beta(a) => {
            *g -= 1;
            Some(TestState::Gamma(a + *g))
        },
        TestState::Gamma(_) => None
    }
}
