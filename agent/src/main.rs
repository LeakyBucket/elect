use ballot::Ballot;

use std::error::Error;

mod state;

fn main() {
    let state = state::new();

    let mut ballot = match register(host) {
        Ok(ballot) => ballot,
        Err(e) => println!("Registration failed {:?}", e)
    };

    match state._inner {
        Unregistered{} => register(host),
        Registered{} => vote(),
        Voted{} => cast(ballot, bbox),
    }
}

fn register(host: &str) -> Result<Ballot, Box<dyn Error>> {

}

fn vote() {

}

fn cast(ballot: Ballot, bbox: &str) -> Result<(), Box<dyn Error>> {

}
