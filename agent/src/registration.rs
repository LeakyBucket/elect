use protocol::messages::Registration;

use std::net::TcpStream;
use std::io;

// Connect to a Roll in order to regsiter for an ongoing vote
pub fn connect(roll: &str) -> io::Result<TcpStream> {

}

// Provide identity credentials to the Roll
pub fn identify(creds: Registration, conn: &mut TcpStream) -> io::Result<Ballot> {

}