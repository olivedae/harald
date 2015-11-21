use controller::response::ChannelResponse;
use controller::stream::L2CAPStream;
use controller::packet::ChannelPDU;
use controller::command::Command;
use uuid::UUID;
use controller::identifiers::ChannelID;

/*
 *
 * Represents a stub to be used for develpoment of
 * layers located above the L2CAP. In addition, to fine tune
 * its interface (which refers to to the trait BluetoothStream)
 * which is implemented on known type of OSes (E.G. OS X)
 *
*/

pub struct Stub;

impl Stub {
    pub fn default() -> Box<Stub> {
        Box::new(Stub)
    }
}

impl L2CAPStream for Stub {
    fn send(&self, address: UUID, message: ChannelPDU) -> ChannelPDU {
        ChannelPDU::new(
            ChannelID::from_u16(message.id()),
            ChannelResponse::new(Command::ConfigureResponse)
        )
    }
}
