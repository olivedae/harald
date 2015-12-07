use controller::data::HasData;

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
    pub information: Vec<u8>,
}

pub struct ChannelCommand;

impl ChannelPDU {
    pub fn new<D: HasData>(id: u16, payload: D) -> ChannelPDU {
        let information = payload.encode();
        ChannelPDU {
            length: information.size(),
            id: id,
            information: information,
        }
    }

    pub fn transmit(&self) {

    }

    pub fn id(&self) -> u16 {
        self.id
    }
}
