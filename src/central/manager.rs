use state::*;
use central::peer::*;

pub struct CentralManager {
    state: State,
    /*
     * Currently, found_peripherals consists of a
     * naive option type vector of Peripheral.
     *
     * Ideally it will use a Binary Search Tree (E.G. BSTMap)
     * which is currently under development. Allowing for
     * faster updates and tracking.
    */
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

    pub fn scan(&mut self) {

    }

    pub fn stop_scan(&mut self) {
        self.scan = false;
    }

    pub fn found_peripherals(&self) -> Option<Vec<Peripheral>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use state::*;
    use central::peer::*;
    use uuid::*;

    #[test]
    fn test_new() {
        let mut manager = CentralManager::new();
        assert_eq!(manager.state(), State::PoweredOn);
    }

    #[test]
    fn test_empty_scan() {
        let mut manager = CentralManager::new();
        manager.scan();
        assert_eq!(manager.found_peripherals(), None);
    }

    #[test]
    fn test_starting_manager() {
        let mut manager = CentralManager::new();
        manager.scan();
        assert_eq!(manager.state, State::PoweredOn);
    }

    #[test]
    fn test_scan_with_peripherals() {
        let mut manager = CentralManager::new();

        /*
         * TODO:
         *
         * Example of the Stub struct can included
         * peripherals on the other end (in addition to how
         * CentralManger and other strucutes interact with it)
        */
    }
}
