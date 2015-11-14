use controller::result::ChannelResult;

/*
*
* Represents an abstracted interface for the
* L2CAP Protocol in addition to the internals of the controller
* for a given OS.
*/
pub trait BluetoothChannel {
    fn send_request() -> ChannelResult;
    fn send_response() -> ChannelResult;
}
