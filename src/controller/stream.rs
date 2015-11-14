use controller::result::StreamResult;

/*
*
* Represents an abstracted interface for the
* L2CAP Protocol in addition to the internals of the controller
* for a given OS.
*/
pub trait BluetoothStream {
    fn send_request() -> StreamResult;
    fn send_response() -> StreamResult;
}
