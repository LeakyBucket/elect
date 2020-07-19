# WTF

Here we find an attempt to organize and layout how this voting tool is intended to work.  Each of the three major components are described, at least in function, below.

## Components

### Agent

The Agent reflects the voter, it is responsible for negotiating with the Roll during registration, filling out the Ballot and casting the ballot (submitting to the Ballot Box).

### Ballot Box

The Ballot Box collects ballots from registered Agents.

### Roll

The roll reflects a traditional voter roll.  It is responsible for any form of voter identification/validation as well as assigning ballots for the vote.  The Roll also synchronizes Agent data with the Ballot Box, such as Agent ID and Public Key.

#### Registration

The Roll listens for and accepts registrations via Web Socket.  Upon receiving a registration request the roll determines if a challenge of any kind is required for the current vote.  If so it responds to the Agent with an appropriate challenge.  The Agent then returns it's response to the challenge.  The challenge can either be verified automatically (programatically) or manually.

Upon successful registration the Agent is provided with a ballot, the address for the Ballot Box as well as they Ballot Box's key so that it can verify the identity.

##### Automatic Verification

If the challenge response is valid the Roll automatically registers the Agent, syncs the Agent data with the Ballot Box, and sends a ballot then closes the connection.

##### Manual Verification

In the case of a manual verification the challenge response is queued and verified by a user.  If the response is valid then the operator indicates success in the UI and the Agent is registered, synced with the Ballot Box, and given a ballot.  If the response is invalid an error is returned to the agent.

In both cases the connection is closed.  In the even of a failed challenge the Agent would need to connect again and retry.

#### Ballot Box Association

On launch the Roll attempts to associate with the specified Ballot Box.  During this process they Roll receives the Public Key for the Ballot Box.  This key is provided to Agents when they register so that they may verify the signature on the receipt from the Ballot Box.

#### Ballot Box Sync

The Roll also syncs registration information with the Ballot Box.  The ballot box will not accept a ballot unless it has been told by the Roll that the Agent ID has registered successfully and the signature on the Ballot validates with the key provided by the Roll.

##### Sync

When an Agent registers successfully the Roll notifies the associated Ballot Box.  In this message it provides the ID of the Agent as well as the Public Key provided by that agent during registration.

## Identity

This application uses libsodium to generate and manage public/private keys as well as signatures both on the ballot and receipt from the Ballot Box.

Identifying an Agent potentially happens in two different contexts.  In the first context the Roll may need to verify specific details about the person using the Agent for regulatory or compliance reasons.  Then there is the context of the voting process/system.

### User Identity

Depending on the vote being conducted there may be external regulatory requirements that necessitate some form of "real world" validation or validation in a context that exists outside the domain of the Roll.

In this case the Roll will issue one or more challenge "questions" to the Agent upon registration.  The person using the agent will then provide answers which will be validated by the operator of the Roll.

### Voting Identity

Identity is also important in the context of the voting system.  It is important for the system to know which Agents are registered and be able to verify that a ballot was submitted by a registered Agent.  It is also important that an Agent be able to verify that the Ballot Box it is giving its ballot to is the correct Ballot Box for the vote.

To this end the following is true:

  * The Roll collects and distributes public keys for both the Agent and Ballot Box
  * The Agent signs it's completed ballot using it's private key
  * The Ballot Box validates the signature using the Agent's public key (as provided by the Roll).
  * The Agent validates the Ballot Box identity through the signature provided on Ballot submission.
    * The public key for the Ballot Box is provided to the Agent by the Roll on registration.

## Protocols

The system uses WebSockets for communication between all components.  This has the benefit of making firewall setup simpler and potentially allowing for other implementations of the various components.
