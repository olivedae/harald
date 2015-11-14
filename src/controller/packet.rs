use controller::identifiers::StreamID;
use controller::data::StreamData;

pub struct StreamPacket {
    pub length: u32,
    pub id: u16,
    pub information: u64,
}

impl StreamPacket {
    fn new(id: StreamID, payload: StreamData) {

    }

    fn transmit(&self) {

    }
}
