extern crate seabed_security;
use seabed_security::GameState;

fn main() {
    let mut state = GameState::init();

    loop {
        let commands = state.turn();
        commands.iter().for_each(|c| println!("{c}"));
    }
}
