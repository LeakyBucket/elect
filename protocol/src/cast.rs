extern crate ballot;

use ballot::Ballot;
use flexbuffers;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::sign::ed25519::{PublicKey, Signature};
use uuid::Uuid;

// The Cast protocol consists of the following messages:
//
// * Cast
// * Response
//   * Receipt
//   * Error
//
// The cast protocol flow is as follows:
//
// * Agent casts a ballot
// * Ballot Box responds with a Receipt or an Error

#[derive(Deserialize, Serialize)]
pub struct Cast {
    agent_id: Uuid,
    ballot: Ballot,
    signature: Signature,
}

#[derive(Deserialize, Serialize)]
pub enum Response {
    Receipt { ballot_id: Uuid, signature: Signature },
    Error { message: String},
}