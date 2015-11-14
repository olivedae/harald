use controller::identifiers::ChannelID;
use controller::data::ChannelData;

pub struct ChannelPacket {
    pub length: u32,
    pub id: u16,
    pub information: u64,
}

impl ChannelPacket {
    fn new(id: ChannelID, payload: ChannelData) {

    }

    fn transmit(&self) {

    }
}
