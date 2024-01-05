extern crate seabed_security;
use seabed_security::GameState;

fn main() {
    let mut state = GameState::init();

    loop {
        state.turn();
    }
}
