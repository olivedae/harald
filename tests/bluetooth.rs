extern crate bluetooth;

use bluetooth::central::manager as BCCentralManager;

#[test]
fn stuff_is_connected() {
    assert_eq!("Hello, world".to_string(), BCCentralManager::hello_world());
}
