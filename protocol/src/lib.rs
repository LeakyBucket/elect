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
