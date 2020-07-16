use ballot::{Ballot, Event};

use std::io;

// List voting history for this Agent
pub fn list() -> Option<Vec<Event>> {
    // Similarly to Key storage, just return None for now
    None
}

// View the details of an already cast Ballot
pub fn view(ballot: &str) -> Result<Ballot, io::Error> {

}