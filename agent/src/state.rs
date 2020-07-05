// Agent states:
//
//  * Unregistered
//  * Registered
//  * Voting (Processing Ballot)
//  * Voted (Ballot Cast)

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

pub struct Unregistered;
pub struct Registered;
pub struct Voting;
pub struct Voted;

pub struct State<S> {
    _inner: S
}

impl State<Unregistered> {
    pub fn new() -> State<Unregistered> {
        State { _inner: Unregistered{} }
    }
}

impl State<Voted> {
    pub fn new() -> State<Unregistered> {
        State { _inner: Unregistered{} }
    }

    fn next(self) -> State<Unregistered> {
        State { _inner: Unregistered{} }
    }
}

impl State<Unregistered> {
    fn next(self) -> State<Registered> {
        State { _inner: Registered{} }
    }
}

impl State<Registered> {
    fn next(self) -> State<Voting> {
        State { _inner: Voting{} }
    }
}

impl State<Voting> {
    fn next(self) -> State<Voted> {
        State { _inner: Voted{} }
    }
}

pub fn new() -> State<Unregistered> {
    State { _inner: Unregistered{} }
}
