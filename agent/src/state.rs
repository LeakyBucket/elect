// The Agent can really be in 1 of two main states:
//
//  * Not voting
//  * Voting

// Each of the two main states can have several smaller states:
//
// ## Not Voting
//
//  * Viewing history
//  * Connecting to a Roll to start voting
//
// ## Voting
//
//  * Filling out a ballot
//  * Submitting a ballot to a ballot box

// States
pub struct History;
pub struct Registration;
pub struct Voting;

pub struct State<S> {
    _inner: S,
}

impl State<History> {
    pub fn next(self) -> State<Registration> {
        State { _inner: Registration{} }
    }
}

impl State<Registration> {
    pub fn next(self) -> State<Voting> {
        State { _inner: Voting{} }
    }
}

impl State<Voting> {
    pub fn next(self) -> State<History> {
        State { _inner: History{} }
    }
}

pub fn new() -> State<History> {
    State { _inner: History{} }
}