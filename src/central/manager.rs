use state::*;
use controller::hci::*;
use central::peer::*;
use std::thread;
use controller::stream::*;

pub struct CentralManager<'stream> {
    state: State,
    hci: HCI,
    stream: &'stream (L2CAPStream + 'stream),
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

impl<'stream> CentralManager<'stream> {
    pub fn new(hci: HCI, stream: &'stream L2CAPStream) -> CentralManager<'stream> {
        CentralManager {
            state: State::Unknown,
            hci: hci,
            stream: stream,
            periphs: vec![None],
            scan: false,
        }
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn start(&mut self) {
        match true {
            true => self.state = State::PoweredOn,
            false => println!("Couldn't start Bluetooth Baseband {:?}", self.state),
        }
    }

    pub fn scan(&mut self) {
        match self.state {
            State::PoweredOn => self.scan = true,
            _ => panic!("Bluetooth status is {:?}", self.state)
        }

        // let (sx, rx) = channel();

        thread::spawn(move || {
            // while self.scan {
            //     self.stream.open(|p| sx.send(p).unwrap() );
            // }
        });

        // rx.recv();
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
    use super::CentralManager;
    use state::*;
    use central::peer::*;
    use uuid::*;
    use controller::hci::*;
    use controller::stub::*;

    #[test]
    fn test_new() {
        let stub_stream = Stub::new();
        let mut manager = CentralManager::new(HCI::Stub, &stub_stream);
        assert_eq!(manager.state(), State::Unknown);
    }

    #[test]
    fn test_empty_scan() {
        let stub_stream = Stub::new();
        let mut manager = CentralManager::new(HCI::Stub, &stub_stream);
        manager.start();
        manager.scan();
        assert_eq!(manager.found_peripherals(), None);
        manager.stop_scan();
    }

    #[test]
    fn test_starting_manager() {
        let stub_stream = Stub::new();
        let mut manager = CentralManager::new(HCI::Stub, &stub_stream);
        manager.start();
        manager.scan();
        assert_eq!(manager.state, State::PoweredOn);
    }

    #[test]
    fn test_scan_with_peripherals() {
        let mut stub_stream = Stub::new();
        let mut manager = CentralManager::new(HCI::Stub, &stub_stream);
        let a_uuid = UUID::as_hex("3a4f");
        let a_peripheral = Peripheral::new(a_uuid);

        /*
         * TODO:
         *
         * Example of the Stub struct can included
         * peripherals on the other end (in addition to how
         * CentralManger and other strucutes interact with it)
        */
    }
}
