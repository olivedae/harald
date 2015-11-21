use uuid::UUID;
use state::*;
use controller::stream::*;
use central::peer::*;

pub struct CentralManager<'s> {
    state: State,
    stream: Box<L2CAPStream + 's>,
}

impl<'s> CentralManager<'s> {
    pub fn new(stream: Box<L2CAPStream + 's>) -> CentralManager<'s> {
        CentralManager {
            state: State::Unknown,
            stream: stream,
        }
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn scan(&self) {

    }

    pub fn stop_scan(&self) {

    }

    pub fn found_peripherals(&self) -> Option<Vec<Peripheral>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::CentralManager;
    use controller::stub::*;
    use state::*;
    use central::peer::*;
    use uuid::*;

    #[test]
    fn test_new() {
        let manager = CentralManager::new(Stub::default());
        assert_eq!(manager.state(), State::Unknown);
    }

    #[test]
    fn test_empty_scan() {
        let manager = CentralManager::new(Stub::default());
        manager.scan();
        assert_eq!(manager.state(), State::PoweredOn);
        assert_eq!(manager.found_peripherals(), None);
        manager.stop_scan();
        assert_eq!(manager.state(), State::PoweredOff);
    }

    #[test]
    fn test_scan_with_peripherals() {
        let manager = CentralManager::new(Stub::default())
        let a_uuid = UUID::as_hex("3a4f");
        let a_peripheral = Peripheral::new(a_uuid);

        /*
         * TODO:
         *
         * Example of the Stub struct can included
         * peripherals on the other end (in addition to how
         * CentralManger and other strucutes interact with it)
        */

        manager.scan();
        manager.stop_scan();
        assert_eq!(manager.found_peripherals().unwrap(), vec![a_peripheral]);
    }
}
