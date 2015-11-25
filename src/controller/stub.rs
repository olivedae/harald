extern crate hyper;

use controller::response::*;
use controller::stream::*;
use controller::packet::*;
use controller::command::*;
use controller::identifiers::*;
use uuid::*;
use peripheral::manager::*;
use state::*;

use std::net;
use std::thread;
use std::sync::mpsc::channel;

/*
 *
 * Represents a stub to be used for develpoment of
 * layers located above the L2CAP. In addition, to fine tune
 * its interface (which refers to to the trait BluetoothStream)
 * which is implemented on known type of OSes (E.G. OS X)
 *
*/

pub struct StubStream {
    ads:     Option<Vec<PeripheralManager>>,
    ads_pdu: Option<Vec<ChannelPDU>>,
    signals: Option<Vec<ChannelPDU>>,
}

impl StubStream {
    pub fn new() -> StubStream {
        StubStream {
            ads: None,
            ads_pdu: None,
            signals: None,
        }
    }

    pub fn open_ads() {

    }
}

impl L2CAPStream for StubStream {
    fn send(&self, address: UUID, message: ChannelPDU) -> ChannelPDU {
        ChannelPDU::new(
            ChannelID::from_u16(message.id()),
            ChannelResponse::new(Command::ConfigureResponse)
        )
    }

    /*
     * We assume that Stub is Bluetooth is capable and
     * currently up and runnning untill further development.
    */
    fn le_status(&self) -> State {
        State::PoweredOn
    }
}

unsafe impl Sync for StubStream { }

pub struct StubCloud {
    port: u16,
    ip: String,
    bcast_mssgs: Option<Vec<Vec<u8>>>,
    state: State,
}

impl StubCloud {
    fn power_on(&mut self) {
        let handle = self.listen();
        self.state = State::PoweredOn;
        // while self.state == State::PoweredOn {
        //     thread::sleep_ms(1500);
        //     println!("STILL ONNNN");
        // }
        // handle.join();
    }

    fn power_off(&mut self) {
        self.state = State::PoweredOff;
    }

    fn listen(&mut self) -> thread::JoinHandle<Vec<u8>> {
        let sock = self.set_socket();
        let (tx, rx) = channel();

        let mut mssg: Vec<u8> = vec![];
        let handle = thread::spawn(move || {
            tx.send(b"Kik?^Live^").unwrap();

            mssg = StubCloud::read_mssg(sock);
            // let mut mssg2 = mssg.clone();
            // mssg2.push(94);
            // tx.send(mssg2).unwrap();
            mssg
        });
        /*
         * Prevents possible race conditions while SubCloud sets up
         * a UDP socket to listen on
        */
        thread::sleep_ms(5000);

        let mut all_mssgs: Vec<Vec<u8>> = vec![];
        let mut mssg: Vec<u8> = vec![];
        for b in rx.recv().unwrap() {
            let b = b.clone();
            println!("CAALLLED");
            println!("{:?}", mssg);
            match b {
                94 => {
                    all_mssgs.push(mssg);
                    mssg = vec![];
                },
                _ => mssg.push(b),
            }
        }

        self.bcast_mssgs = Some(
            all_mssgs
        );

        handle
    }

    fn set_socket(&self) -> net::UdpSocket {
        let attempt = net::UdpSocket::bind(
            (&self.ip[..], self.port)
        );
        let mut socket;
        match attempt {
            Ok(sock) => {
                socket = sock;
            },
            Err(err) => {
                panic!("THERE WAS AN ERR DEar lOORD: {}", err);
            },
        }
        socket
    }

    fn read_mssg(socket: net::UdpSocket) -> Vec<u8> {
        let mut buf: [u8; 100] = [0; 100];
        let result = socket.recv_from(&mut buf);
        drop(socket);
        let mut data;
        match result {
            Ok((amt, src)) => {
                data = Vec::from(&buf[0..amt]);
            },
            Err(e) => panic!("THERE WASS SOME READ ERR: {}", e),
        }
        data
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::{SocketAddr, UdpSocket};
    use state::*;

    #[test]
    fn test_port() {
        let port: u16 = 3131;
        let ip = "localhost".to_string();
        let cloud = StubCloud {
            port: port,
            ip: ip,
            bcast_mssgs: None,
            state: State::PoweredOff,
        };
        assert_eq!(cloud.port, port);
    }

    #[test]
    fn test_ip() {
        let port: u16 = 3132;
        let ip = "localhost".to_string();
        let cloud = StubCloud {
            port: port,
            ip: ip.clone(),
            bcast_mssgs: None,
            state: State::PoweredOff,
        };
        assert_eq!(cloud.ip, ip);
    }

    #[test]
    fn test_set_socket() {
        let port: u16 = 3133;
        let ip = "localhost".to_string();
        let cloud = StubCloud {
            port: port,
            ip: ip.clone(),
            bcast_mssgs: None,
            state: State::PoweredOff,
        };
        let sock = cloud.set_socket();
        let addr = sock.local_addr().unwrap();
        assert_eq!(
            sock.local_addr().unwrap(),
            addr
        );
    }

    #[test]
    fn test_listen() {
        let port: u16 = 3134;
        let ip = "localhost".to_string();
        let mut cloud = StubCloud {
            port: port,
            ip: ip.clone(),
            bcast_mssgs: None,
            state: State::PoweredOff,
        };
        let handle = cloud.listen();

        let mut socket;
        match UdpSocket::bind("localhost:3000") {
            Ok(sock) => socket = sock,
            Err(e) => panic!("ERROR SOMWHERE: {}", e),
        }
        let greets: &[u8] = b"Hello, world!";
        socket.send_to(greets, "localhost:3134");

        let recieved = handle.join().unwrap();
        assert_eq!(13, recieved.len());
    }

    #[test]
    fn test_toggle_power() {
        let port: u16 = 3135;
        let ip = "localhost".to_string();
        let mut cloud = StubCloud {
            port: port,
            ip: ip.clone(),
            bcast_mssgs: None,
            state: State::PoweredOff,
        };

        cloud.power_on();
        cloud.power_off();
        assert_eq!(State::PoweredOff, cloud.state);
    }

    #[test]
    fn test_broadcast_save() {
        let port: u16 = 3136;
        let ip = "localhost".to_string();
        let mut cloud = StubCloud {
            port: port,
            ip: ip.clone(),
            bcast_mssgs: None,
            state: State::PoweredOff,
        };

        cloud.power_on();

        let mut sock;
        match UdpSocket::bind("localhost:3000") {
            Ok(socket) => sock = socket,
            Err(e) => {
                panic!("ERRORR SOMEWHERE: {}", e);
            },
        }

        let disc_mssg1: &[u8] = b"My Device";
        let disc_mssg2: &[u8] = b"Another Device";

        sock.send_to(disc_mssg1, "localhost:3136");
        // sock.send_to(disc_mssg2, "localhost:3136");

        let mut mssgs: Vec<Vec<u8>>;
        match cloud.bcast_mssgs {
            Some(ref b) => { mssgs = b.clone(); },
            None => panic!("ITS NOT WORKING YETTi"),
        };

        cloud.power_off();

        assert_eq!(2, mssgs.len())
    }
}
