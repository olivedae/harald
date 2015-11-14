use controller::result::ChannelResult;
use controller::stream::BluetoothChannel;

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
    fn send_request() -> ChannelResult {
        ChannelResult::new()
    }

    fn send_response() -> ChannelResult {
        ChannelResult::new()
    }
}
