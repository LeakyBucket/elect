mod stats;

struct Roll {
    registered: Option<Vec<Voter>>,
    voters: Vec<Voter>,
}

// Ballots
//
// The roll should use the environment to determine where ballots are stored.
//  The UI should present the operator with the ability to select the desired
//  ballot for the event from that location.
fn main() {
    println!("Hello, world!");
}
