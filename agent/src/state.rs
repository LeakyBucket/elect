// Agent states:
//
//  * Unregistered
//  * Registered
//  * Voting (Processing Ballot)
//  * Voted (Ballot Cast)

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
