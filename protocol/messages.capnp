@0xf8664fb3080a8a9f;

struct Registration @0xe625dfeaa5c700a2 {
  # The Registration type is used to register an Agent with a Roll for an election

  agent_id @0 :Text; # The UUID of the Agent attempting to register
  key @1 :UInt32; # The Agent's Public Key
}

struct Submission @0xb4dea48a035bfdfa {
  # The Submission type is used when the Agent delivers a Ballot to the Box

  agent_id @0 :Text; # The UUID of the Agent submitting the ballot
  key @1 :UInt32; # The Agent's public key
  signature @2 :UInt64; # The Agent's signature for the ballot
  ballot @3 :Blob; # The completed Ballot
}

struct Sync @0xccf30309015ca01b {
  # The Sync type is used to coordinate a Roll and a Ballot Box

}