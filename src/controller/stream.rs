use controller::result::StreamResult;

/*
*
* Represents an abstracted interface for the
* L2CAP Protocol in addition to the internals of the controller
* for a given OS.
*/
pub trait BluetoothStream {
    fn open_fixed_stream() -> StreamResult;
}
