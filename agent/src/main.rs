use ballot::Ballot;
use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::{Either, Label, ViewSwitcher};
use sodiumoxide::crypto::{sign, sign::ed25519::PublicKey, sign::ed25519::PrivateKey};

use std::error::Error;

mod history;
mod state;
mod vote;

// UI Layout
//
// Either (root container)
// | --> Either (Not Voting)
//       | --> Registration | History
// | --> Flex (Voting)
//       | --> Ballot

// Registration
//
// ViewSwitcher (container)
// | --> Flex (instructions)
// | --> TextBox (roll address)
// | --> Button (connect button)
//
// How to display an error?
// How to transition from success to Voting?

// History
//
// Either (history container)
// | --> PastBallots | PastBallot

// PastBallots
//
// Flex (container)
// | --> List (ballots)

// PastBallot
//
// # This is a Raw Ballot but with non interactive choices

// RawBallot
//
// Flex (Event Description)
// | --> Flex (Intro)
// | --> List (Items)
//       | --> Flex (Description)
//       | --> Choices::Ranked | Choices::Single | Choices::Tally

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
