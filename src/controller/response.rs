use controller::command::Command;
use controller::data::HasData;

pub struct ChannelResponse {
    id: u16
}

impl HasData for ChannelResponse {
    fn encode(&self) -> u64 {
        42u64
    }

    fn size(&self) -> u16 {
        42u16
    }
}

impl ChannelResponse {
    pub fn new(id: Command) -> ChannelResponse {
        ChannelResponse { id: id.to_u16() }
    }
}
