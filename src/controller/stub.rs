use controller::result::StreamResult;
use controller::stream::BluetoothStream;

/*
*
* Represents a stub to be used for develpoment of
* layers located above the L2CAP. In addition, to fine tune
* its interface (which refers to to the trait BluetoothStream)
* which is implemented on known type of OSes (E.G. OS X)
*
*/

pub struct StubStream;

impl BluetoothStream for StubStream {
    fn open_fixed_stream() -> StreamResult {
        StreamResult::new()
    }
}
