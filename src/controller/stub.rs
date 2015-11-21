use controller::response::ChannelResponse;
use controller::stream::BluetoothChannel;
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

pub struct StubChannel;

impl BluetoothChannel for StubChannel {
    fn send(address: UUID, message: ChannelPDU) -> ChannelPDU {
        ChannelPDU::new(
            ChannelID::from_u16(message.id()),
            ChannelResponse::new(Command::ConfigureResponse)
        )
    }
}
