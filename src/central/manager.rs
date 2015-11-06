use state::*;
use central::peer::*;
use uuid::*;

#[derive(Clone)]
pub struct CentralManager {
    state: State,
    periphs: Vec<Option<Peripheral>>,
    scan: bool,
}

impl CentralManager {
    pub fn new() -> CentralManager {
        CentralManager {
            state: State::PoweredOn,
            periphs: vec![None],
            scan: false,
        }
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn scan(&mut self) -> Option<Vec<Peripheral>> {
        let count: usize;
        match self.periphs.len() {
            1 => return None,
            _ => count = self.periphs.len() - 1,
        }
        let mut periphs: Vec<Peripheral> = Vec::with_capacity(count);
        for p in self.periphs.clone() {
            match p {
                Some(peripheral) => periphs.push(peripheral),
                None => continue,
            }
        }
        self.periphs = vec![None];
        Some(periphs)
    }

    pub fn stop_scan(&mut self) {
        self.scan = false;
    }

    pub fn recieve(&mut self, pdu: &[u8]) {
        let a_uuid = UUID::Custom(0x1234);
        let a_peripheral = Peripheral::new(a_uuid);
        self.periphs.push(Some(a_peripheral));
    }
}

#[cfg(test)]
mod test_manager {
    use super::*;
    use state::*;
    use gap::broadcast::*;

    #[test]
    fn test_new() {
        let mut manager = CentralManager::new();
        assert_eq!(manager.state(), State::PoweredOn);
    }

    #[test]
    fn test_empty_scan() {
        let mut manager = CentralManager::new();
        let peripherals = manager.scan();
        assert_eq!(peripherals, None);
    }

    #[test]
    fn test_starting_manager() {
        let mut manager = CentralManager::new();
        let _ = manager.scan();
        assert_eq!(manager.state, State::PoweredOn);
    }

    /*
     * Not so much of a unit test but an integration test
     * since it involves various portions of the library to function.
     *
     * Allows for a starting point to creating the client interface and from
     * there model logic.
    */
    #[test]
    fn test_understanding_advertisements() {
        let mut central = CentralManager::new();
        let adv_pdu = Broadcast::advertisement_format();
        central.recieve(adv_pdu);
        let peripherals = central.scan();
        let peripheral = peripherals.unwrap()[0].clone();
        assert_eq!(peripheral.name(), "Example Name".to_string());
    }
}
