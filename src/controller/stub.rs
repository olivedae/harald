extern crate hyper;

use controller::response::*;
use controller::stream::*;
use controller::packet::*;
use controller::command::*;
use controller::identifiers::*;
use uuid::*;
use peripheral::manager::*;

/*
 *
 * Represents a stub to be used for develpoment of
 * layers located above the L2CAP. In addition, to fine tune
 * its interface (which refers to to the trait BluetoothStream)
 * which is implemented on known type of OSes (E.G. OS X)
 *
*/

pub struct Stub {
    ads:     Option<Vec<PeripheralManager>>,
    ads_pdu: Option<Vec<ChannelPDU>>,
    signals: Option<Vec<ChannelPDU>>,
}

impl Stub {
    pub fn new() -> Stub {
        Stub {
            ads: None,
            ads_pdu: None,
            signals: None,
        }
    }

    pub fn open_ads() {

    }
}

impl L2CAPStream for Stub {
    fn send(&self, address: UUID, message: ChannelPDU) -> ChannelPDU {
        ChannelPDU::new(
            ChannelID::from_u16(message.id()),
            ChannelResponse::new(Command::ConfigureResponse)
        )
    }

    fn le_capable(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
