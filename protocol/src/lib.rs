// Protocols need to exist for two main processes:
//
//  * Registering with a Roll
//  * Submitting a ballot
//
// Registering with a Roll is likely the most complex of the two.
//  It needs to handle a challenge, key <-> ballot exchange.  Exchange
//  of the ballot box information (location etc).
//
// Submitting a ballot is simpler.  It involves connecting to the
//  ballot box and exchanging ballot <-> receipt.

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

// The message definitions come from Cap'n Proto. The types are defined in
//  `messages.capnp`
mod messages;

struct Verification {

}

struct Cast {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
