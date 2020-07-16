@0xf8664fb3080a8a9f;

struct Receipt @0xeafdeba7605c5845 {
  # The Receipt is acknowledgement from the Ballot Box that is has accepted the submitted Ballot

  ballot_id @0 :Text; # The UUID assigned to the Ballot
}

struct Registration @0xe625dfeaa5c700a2 {
  # The Registration type is used to register an Agent with a Roll for an election

  agent_id @0 :Text; # The UUID of the Agent attempting to register
  key @1 :UInt32; # The Agent's Public Key
}

struct Challenge @0x9299567688183393 {
  # The Challenge is an optional message allowing for the Roll and Agent to exchange shared identity information

  questions @0 :List(Question); # The challenge questions for registration
}

struct Question @0x97dd21244a13bfe9 {
  # A question and answer pair that are used in the challenge process

  query @0 :Text; # The query part of the challenge, this should always be present
  answer @1 :Text; # The answer part of the challenge, this is provided by the Agent
}

struct RegistrationResponse @0xc214bcd8224fc952 {
  # The RegistrationResponse is returned by the Roll indicating whether the Agent was accepted or not

  accepted @0 :Bool; # Whether the Roll has accepted the Registration or not
  ballot @1 :Blob; # The Ballot for the election.  This will only contain data if registration was accepted
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