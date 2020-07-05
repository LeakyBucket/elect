use ballot::Ballot;
use sodiumoxide::crypto::{sign, sign::ed25519::PublicKey, sign::ed25519::PrivateKey};

use std::error::Error;

mod history;
mod state;
mod vote;

struct KeyPair {
    public: PublicKey,
    secret: SecretKey,
}

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

fn keys() -> KeyPair {
    match existing_keys() {
        Some(kp) => kp,
        None => {
            let (public, secret) = sign::ed25519::gen_keypair();
            KeyPair { public, secret }
        }
    }
}

fn existing_keys() -> Option<KeyPair> {
    // Return None until we determine how to manage saved keys
    None
}

fn register(host: &str) -> Result<Ballot, Box<dyn Error>> {

}

fn vote() {

}

fn cast(ballot: Ballot, bbox: &str) -> Result<(), Box<dyn Error>> {

}
