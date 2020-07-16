extern crate ballot;

use ballot::Ballot;
use flexbuffers;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::sign::ed25519::{PublicKey, Signature};
use uuid::Uuid;

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

#[derive(Deserialize, Serialize)]
pub struct Receipt {
    accepted: bool,
    ballot_id: Uuid,
    sub_info: SubmissionData,
}

#[derive(Deserialize, Serialize)]
pub struct SubmissionData {
    bbox: Uuid,
    box_sig: Signature,
}

#[derive(Deserialize, Serialize)]
pub struct Registration {
    agent_id: Uuid,
    agent_key: PublicKey,
}

#[derive(Deserialize, Serialize)]
pub struct Challenge {
    questions: Vec<Question>,
}

#[derive(Deserialize, Serialize)]
pub struct Question {
    query: String,
    answer: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegistrationResponse {
    accepted: bool,
    ballot: Ballot,
}

#[derive(Deserialize, Serialize)]
pub struct Submission {
    agent_id: Uuid,
    key: PublicKey,
    signature: Signature,
    ballot: Ballot,
}

#[derive(Deserialize, Serialize)]
pub struct Sync {
    agent_id: Uuid,
    event: SyncEvent,
}

#[derive(Deserialize, Serialize)]
pub enum SyncEvent {
    Registered,
    Submitted,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorMessage {
    kind: ErrorType,
}

#[derive(Deserialize, Serialize)]
pub enum ErrorType {
    UnexpectedMessage,
    InvalidMessage,
    BadAnswer,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
