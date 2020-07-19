extern crate ballot;

use ballot::Ballot;
use flexbuffers;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::sign::ed25519::{PublicKey, Signature};
use uuid::Uuid;

// The Registration protocol consists of the following messages:
//
//  * Request
//  * Verification
//    * Challenges (Vec<Challenge>)
//  * Response
//
//  The protocol flow is as follows:
//
//  * Agent requests to register
//  * Roll may respond with a Challenge consisting of one or more questions
//  * Roll determines eligibility
//  * Roll Responds with one of two messages
//    * Success (ballot, bbox address, bbox public key)
//    * Error/Failure

#[derive(Deserialize, Serialize)]
pub struct Request {
    agent_id: Uuid,
    agent_key: PublicKey,
}

#[derive(Deserialize, Serialize)]
pub struct Verification {
    challenges: Vec<Challenge>,
}

#[derive(Deserialize, Serialize)]
pub struct Challenge {
    question: String,
    answer: String,
}

#[derive(Deserialize, Serialize)]
pub enum Response {
    Success { box_address: String, box_key: PublicKey, ballot: Ballot },
    Failure { reason: String },
}
