use controller::response::*;
use controller::stream::*;
use controller::packet::*;
use controller::command::*;
use controller::identifiers::*;
use uuid::*;
use peripheral::manager::*;
use state::*;

use mio::tcp::*;
use mio::util::Slab;
use bytes::{Buf, Take};
use std::io::Cursor;
use std::net::SocketAddr;
use std::mem;
use mio::{TryRead, TryWrite};


/*
 *
 * Represents a stub to be used for develpoment of
 * layers located above the L2CAP. In addition, to fine tune
 * its interface (which refers to to the trait BluetoothStream)
 * which is implemented on known type of OSes (E.G. OS X)
 *
*/

pub struct StubStream {
    ads:     Option<Vec<PeripheralManager>>,
    ads_pdu: Option<Vec<ChannelPDU>>,
    signals: Option<Vec<ChannelPDU>>,
}

impl StubStream {
    pub fn new() -> StubStream {
        StubStream {
            ads: None,
            ads_pdu: None,
            signals: None,
        }
    }

    pub fn open_ads() {

    }
}

impl L2CAPStream for StubStream {
    fn send(&self, address: UUID, message: ChannelPDU) -> ChannelPDU {
        ChannelPDU::new(
            ChannelID::from_u16(message.id()),
            ChannelResponse::new(Command::ConfigureResponse)
        )
    }

    /*
     * We assume that Stub is Bluetooth is capable and
     * currently up and runnning untill further development.
    */
    fn le_status(&self) -> State {
        State::PoweredOn
    }
}

pub struct StubCloud;

impl StubCloud {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_something() {

    }
}
