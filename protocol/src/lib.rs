use flexbuffers;

pub mod cast;
pub mod registration;
pub mod sync;

// Protocols need to exist for three main processes:
//
//  * Registering with a Roll
//  * Submitting a ballot
//  * Syncing the Roll and Ballot Box
//
// Registering with a Roll is likely the most complex of the two.
//  It needs to handle a challenge, key <-> ballot exchange.  Exchange
//  of the ballot box information (location etc).
//
// Submitting a ballot is simpler.  It involves connecting to the
//  ballot box and exchanging ballot <-> receipt.
//
// Syncing the Roll and Ballot Box really just means ensuring both
//  have the id and public key for an Agent (the roll sends this on
//  successful registration) as well as ensuring both sides have a
//  a record of a ballot being cast.

// Identity
//
// A verification/challenge process should exist in the protocol.  After both
// parties are satisfied the Agent should exchange it's public key for a copy
// of the ballot.
//
// When submitting the ballot to the ballot box it must be signed.  There should
// be a key exchange with the ballot box as well so that both the roll and ballot
// box have a copy of the public key used by that party and can verify the
// signature on the completed ballot.

// HTTP API
//
// Registration
//
//  * Agent makes request
//  * Roll responds with challenge or ballot
//  * If challenge Agent replies with answers
//    * checks may be automated or manual
//    * automated may be quick enough to complete in same request
//    * manual would need async behavior as they won't be fast and may queue
//
//  
//
// Submission
//
//  * 

pub enum Component {
    Agent,
    Box,
    Roll,
}

pub struct Cast {
    component: Component,
    previous: Option<cast::Messages>,
}

impl Cast {
    fn new(component: Component) -> Self {
        Self { component, previous: None }
    }

    fn deserialize(&mut self, data: &[u8]) -> cast::Messages {
        if self.is_agent() {
            self.agent_receive(data)
        } else {
            self.box_receive(data)
        }
    }

    fn agent_receive(&mut self, data: &[u8]) -> cast::Messages {
        match self.previous {
            None => {

            },
            cast::Message::Cast => {

            },
            cast::Message::Response => {

            }
        }
    }

    fn box_receive(&mut self, data: &[u8]) -> cast::Messages {
        match self.previous {
            None => {
                cast::Cast.deserialize(data).unwrap()
            },
            _ => {
            
            }
        }
    }

    fn is_agent(&mut self) -> bool {
        self.component == Component::Agent
    }

    fn is_box(&mut self) -> bool {
        self.component == Component::Box
    }
}

pub struct Registration {
    component: Component,
    previous: Option<cast::Messages>,
}

impl Registration {
    fn new(component: Component) -> Self {
        Self { component, previous: None }
    }

    fn deserialize(&mut self, data: &[u8]) -> registration::Messages {

    }

    fn is_roll(&mut self) -> bool {
        self.component == Component::Roll
    }

    fn is_agent(&mut self) -> bool {
        self.component == Component::Agent
    }
}

pub struct Sync {
    component: Component,
    previous: Option<cast::Messages>,
}

impl Sync {
    fn new(component: Component) -> Self {
        Self { component, previous: None }
    }

    fn deserialize(&mut self, date: &[u8]) -> sync::Messages {

    }

    fn is_box(&mut self) -> bool {
        self.component == Component::Box
    }

    fn is_roll(&mut self) -> bool {
        self.component == Component::Roll
    }
}

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
