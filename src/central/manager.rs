use uuid::UUID;
use peripheral::peer::PeripheralPeer as Peripheral;

trait HasCentralManager {
    fn scan(serviceUUIDs: Vec<UUID>) -> Vec<Peripheral>;
    fn stop_scan();
    fn connect_peripheral(periferal: Peripheral);
    fn cancel_connection_peripheral(periferal: Peripheral);
    fn get_peripherals_with_services();
    fn get_peripherals_with_identifiers();
}

struct CentralManager;

pub fn hello_world() -> String {
    return "Hello, world".to_string()
}
