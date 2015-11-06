use state::*;
use uuid::*;
use controller::stream::*;
use controller::packet::*;

struct StubStream {
    current_id: u16
}

impl L2CAPStream for StubStream {
    fn send(&mut self, information: Vec<u8>) -> ChannelPDU {
        self.current_id += 1;
        ChannelPDU::new(
            self.current_id,
            information,
        )
    }

    fn le_status(&self) -> State {
        State::PoweredOn
    }
}

impl StubStream {
    fn new() -> StubStream {
        StubStream {
            current_id: 0x01,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_new_stub_stream() {
        let stub_stream = StubStream::new();
        assert_eq!(stub_stream.current_id, 1);
    }
}
