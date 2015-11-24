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
use std::sync::Arc;

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
    // socket: Option<net::UdpSocket>,
    ip: String,
}

impl StubCloud {
    fn listen(&self) -> thread::JoinHandle<Vec<u8>> {
        let sock = self.set_socket();
        let handle = thread::spawn(move || {
            StubCloud::read_mssg(sock)
        });
        /*
         * Prevents possible race conditions while SubCloud sets up
         * a UDP socket to listen on
        */
        thread::sleep_ms(3000);
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

    #[test]
    fn test_address() {
        let port: u16 = 3131;
        let ip = "localhost".to_string();
        let cloud = StubCloud {
            port: port,
            ip: ip,
        };
        assert_eq!(cloud.port, port);
    }

    #[test]
    fn test_ip() {
        let port: u16 = 3131;
        let ip = "localhost".to_string();
        let cloud = StubCloud {
            port: port,
            ip: ip.clone(),
        };
        assert_eq!(cloud.ip, ip);
    }

    #[test]
    fn test_socket() {
        let port: u16 = 3131;
        let ip = "localhost".to_string();
        let cloud = StubCloud {
            port: port,
            ip: ip.clone(),
        };
        let sock = cloud.set_socket();
        let addr = sock.local_addr();
        assert_eq!(
            sock.local_addr().unwrap(),
            addr.unwrap()
        );
    }

    #[test]
    fn test_udp() {
        let port: u16 = 3131;
        let ip = "localhost".to_string();
        let cloud = StubCloud {
            port: port,
            ip: ip.clone(),
        };
        let handle = cloud.listen();

        let mut socket;
        match UdpSocket::bind("localhost:3000") {
            Ok(sock) => socket = sock,
            Err(e) => panic!("ERROR SOMWHERE: {}", e),
        };
        let greets: &[u8] = b"Hello, world!";
        socket.send_to(greets, "localhost:3131");

        let recieved = handle.join().unwrap();
        assert_eq!(13, recieved.len());
    }
}
