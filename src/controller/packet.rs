use controller::identifiers::ChannelID;
use controller::data::HasData;
use controller::command::Command;

/*
 *
 * ChannelPDU represents a data packet used in
 * Basic L2CAP Mode for connection-oriented and
 * connectionless (by specifying the CID of Connectionless).
 *
 * BluetoothStream will contain support primarily for
 * basic L2CAP mode while development continues at levels
 * above.
 *
*/

pub struct ChannelPDU {
    pub length: u16,
    pub id: u16,
    pub information: u64,
}

pub struct ChannelCommand;

impl ChannelPDU {
    pub fn new<D: HasData>(id: ChannelID, payload: D) -> ChannelPDU {
        ChannelPDU {
            id: id.to_u16(),
            length: payload.size(),
            information: payload.encode(),
        }
    }

    pub fn transmit(&self) {

    }

    pub fn id(&self) -> u16 {
        self.id
    }
}
