use controller::packet::ChannelPDU;
use uuid::UUID;
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
    fn send(&self, address: UUID, packet: ChannelPDU) -> ChannelPDU;
    fn le_status(&self) -> State;
}
