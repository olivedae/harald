use controller::packet::ChannelPDU;
use central::peer::*;
use state::*;

/*
 *
 * Represents an abstracted interface for the
 * L2CAP Protocol in addition to the internals of the controller
 * for a given OS.
 *
*/

pub trait L2CAPStream {
    fn send(&mut self, information: Vec<u8>) -> ChannelPDU;
    fn le_status(&self) -> State;
}
