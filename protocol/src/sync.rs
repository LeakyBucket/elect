use flexbuffers;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::sign::ed25519::PublicKey;
use uuid::Uuid;

// The Sync protocol consists of the following messages:
//
// * Cast
// * Discovery
// * Registration
//
// The Sync flow is as follows:
//
// * The Roll connects to the specified Ballot Box and asks for it's public key
// * Whenever an Agent registers successfully its key and id are sent to the Ballot Box
// * Whenever a ballot is submitted the Box informs the Roll

#[derive(Deserialize, Serialize)]
pub enum Discovery {
    Request { roll_id: Uuid },
    Response { bbox_id: Uuid, bbox_key: PublicKey },
}

#[derive(Deserialize, Serialize)]
pub struct Registration {
    agent_id: Uuid,
    agent_key: PublicKey,
}

#[derive(Deserialize, Serialize)]
pub struct Cast {
    agent_id: Uuid,
}