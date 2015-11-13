trait HasCentralManager {
    fn scan(serviceUUIDs: Vec<UUIDS>, options: <AnOption>) -> Vec<Peripheral>;
    fn stop_scan();
    fn connect_peripheral(periferal: Peripheral, options: Vec<AnOption>);
    fn cancel_connection_peripheral(periferal: Peripheral, options: Vec<AnOption>);
    fn get_peripherals_with_services();
    fn get_peripherals_with_identifiers();
}

struct CentralManager {
    state: CentralManagerState,
}

impl HasCentralManager for CentralManager {

}

pub fn hello_world() -> String {
    return "Hello, world".to_string()
}
