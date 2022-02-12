use sodiumoxide::crypto::sign::ed25519::{PublicKey, SecretKey};
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

pub struct AgentState<S> {
    pkey: PublicKey,
    skey: SecretKey,
    state: S
}

struct ViewingHistory;
struct ViewingBallot;
struct Registration;
struct Verification;
struct Voting;

impl AgentState<ViewingHistory> {
    fn new(pkey: PublicKey, skey: SecretKey) -> Self {
        AgentState {
            pkey,
            skey,
            state: ViewingHistory {},
        }
    }

    fn next(message)
}

impl From<AgentState<ViewingHistory>> for AgentState<ViewingBallot> {
    fn from(val: AgentState<ViewingHistory>) -> AgentState<ViewingBallot> {
        AgentState {
            pkey: val.pkey,
            skey: val.skey,
            state: ViewingBallot {},
        }
    }
}

impl From<AgentState<ViewingHistory>> for AgentState<Registration> {
    fn from(val: AgentState<ViewingHistory>) -> AgentState<Registration> {
        AgentState {
            pkey: val.pkey,
            skey: val.skey,
            state: Registration {},
        }
    }
}

impl From<AgentState<ViewingBallot>> for AgentState<ViewingHistory> {
    fn from(val: AgentState<ViewingBallot>) -> AgentState<ViewingHistory> {
        AgentState {
            pkey: val.pkey,
            skey: val.skey,
            state: ViewingHistory {},
        }
    }
}

impl From<AgentState<Registration>> for AgentState<Verification> {
    fn from(val: AgentState<Registration>) -> AgentState<Verification> {
        AgentState {
            pkey: val.pkey,
            skey: val.skey,
            state: Verification {},
        }
    }
}

impl From<AgentState<Registration>> for AgentState<Voting> {
    fn from(val: AgentState<Regsitration>) -> AgentState<Voting> {
        AgentState {
            pkey: val.pkey,
            skey: val.skey,
            state: Voting {},
        }
    }
}

impl From<AgentState<Voting>> for AgentState<ViewingReceipt> {
    fn from(val: AgentState<Voting>) -> AgentState<ViewingReceipt> {
        AgentState {
            pkey: val.pkey,
            skey: val.skey,
            state: ViewingReceipt {},
        }
    }
}

impl From<AgentState<ViewingReceipt>> for AgentState<ViewingHistory> {
    fn from(val: AgentState<ViewingReceipt>) -> AgentState<ViewingHistory> {
        AgentState {
            pkey: val.pkey,
            skey: val.skey,
            state: ViewingHistory {},
        }
    }
}
