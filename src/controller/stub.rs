extern crate mio;
extern crate bytes;

use controller::response::*;
use controller::stream::*;
use controller::packet::*;
use controller::command::*;
use controller::identifiers::*;
use uuid::*;
use peripheral::manager::*;
use state::State as LeState;

use mio::tcp::*;
use mio::util::Slab;
use bytes::{Buf, Take};
use std::io::Cursor;
use std::net::SocketAddr;
use std::mem;
use mio::{TryRead, TryWrite};
use std::thread;


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
    fn le_status(&self) -> LeState {
        LeState::PoweredOn
    }
}

const SERVER: mio::Token = mio::Token(0);
const MAX_LINE: usize = 128;

#[derive(Debug)]
pub struct Pong {
    server: TcpListener,
    connections: Slab<Connection>,
}

impl Pong {
    fn new(server: TcpListener) -> Pong {
        let slab = Slab::new_starting_at(mio::Token(1), 1024);
        Pong {
            server: server,
            connections: slab,
        }
    }
}

impl mio::Handler for Pong {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut mio::EventLoop<Pong>, token: mio::Token, events: mio::EventSet) {
        match token {
            SERVER => {
                assert!(events.is_readable());

                match self.server.accept() {
                    Ok(Some(sock)) => {
                        let token = self.connections.
                            insert_with(|token| Connection::new(sock, token))
                            .unwrap();

                        event_loop.register_opt(
                            &self.connections[token].socket,
                            token,
                            mio::EventSet::readable(),
                            mio::PollOpt::edge() | mio::PollOpt::oneshot()
                        ).unwrap();
                    },
                    Ok(None) => {
                        println!("The server's socket was not ready!");
                    },
                    Err(e) => {
                        println!("Encountered an error while accpeting a connection: {}", e);
                        event_loop.shutdown();
                    },
                }
            },
            _ => {
                self.connections[token].ready(event_loop, events);

                if self.connections[token].is_closed() {
                    let _ = self.connections.remove(token);
                }
            }

        }
    }
}

#[derive(Debug)]
struct Connection {
    socket: TcpStream,
    token: mio::Token,
    state: State,
}

impl Connection {
    fn new(socket: TcpStream, token: mio::Token) -> Connection {
        Connection {
            socket: socket,
            token: token,
            state: State::Reading(Vec::with_capacity(MAX_LINE)),
        }
    }

    fn ready(&mut self, event_loop: &mut mio::EventLoop<Pong>, events: mio::EventSet) {
        match self.state {
            State::Reading(..) => {
                assert!(events.is_readable());
                self.read(event_loop);
            },
            State::Writing(..) => {
                assert!(events.is_writable());
                self.write(event_loop);
            },
            _ => unimplemented!(),
        }
    }

    fn read(&mut self, event_loop: &mut mio::EventLoop<Pong>) {
        match self.socket.try_read_buf(self.state.mut_read_buf()) {
            Ok(Some(0)) => {
                self.state = State::Closed;
            },
            Ok(Some(n)) => {
                self.state.try_transition_to_writing();
                self.reregister(event_loop);
            },
            Ok(None) => {
                self.reregister(event_loop);
            },
            Err(e) => {
                panic!("Got an errorrrz: {:?}", e);
            }
        }
    }

    fn write(&mut self, event_loop: &mut mio::EventLoop<Pong>) {
        match self.socket.try_write_buf(self.state.mut_write_buf()) {
            Ok(Some(_)) => {
                self.state.try_transition_to_reading();
                self.reregister(event_loop);
            },
            Ok(None) => {
                self.reregister(event_loop);
            },
            Err(e) => {
                panic!("GOT ANOTHER ERRZ: {:?}", e);
            },
        }
    }

    fn reregister(&self, event_loop: &mut mio::EventLoop<Pong>) {
        event_loop.reregister(
            &self.socket,
            self.token,
            self.state.event_set(),
            mio::PollOpt::oneshot()
        ).unwrap();
    }

    fn is_closed(&self) -> bool {
        match self.state {
            State::Closed => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum State {
    Reading(Vec<u8>),
    Writing(Take<Cursor<Vec<u8>>>),
    Closed,
}

impl State {
    fn mut_read_buf(&mut self) -> &mut Vec<u8> {
        match *self {
            State::Reading(ref mut buf) => buf,
            _ => panic!("Connection not in reading state!"),
        }
    }

    fn read_buf(&self) -> &[u8] {
        match *self {
            State::Reading(ref buf) => buf,
            _ => panic!("Connection not in reading state!"),
        }
    }

    fn write_buf(&self) -> &Take<Cursor<Vec<u8>>> {
        match *self {
            State::Writing(ref buf) => buf,
            _ => panic!("Connection not in Writing state!"),
        }
    }

    fn mut_write_buf(&mut self) -> &mut Take<Cursor<Vec<u8>>> {
        match *self {
            State::Writing(ref mut buf) => buf,
            _ => panic!("Connection to in Writing state!"),
        }
    }

    fn try_transition_to_writing(&mut self) {
        if let Some(pos) = self.read_buf().iter().position(|b| *b == b'\n') {
            let buf = mem::replace(self, State::Closed)
                .unwrap_read_buf();
            let buf = Cursor::new(buf);
            *self = State::Writing(Take::new(buf, pos + 1));
        }
    }

    fn try_transition_to_reading(&mut self) {
        if !self.write_buf().has_remaining() {
            let cursor = mem::replace(self, State::Closed)
                .unwrap_write_buf()
                .into_inner();
            let pos = cursor.position();
            let mut buf = cursor.into_inner();

            drain_to(&mut buf, pos as usize); /* drops all data that has been writen to the client */

            *self = State::Reading(buf);

            self.try_transition_to_writing(); /* checks for any new lines that have already been read */
        }
    }

    fn event_set(&self) -> mio::EventSet {
        match *self {
            State::Reading(..) => mio::EventSet::readable(),
            State::Writing(..) => mio::EventSet::writable(),
            _ => mio::EventSet::none(),
        }
    }

    fn unwrap_read_buf(self) -> Vec<u8> {
        match self {
            State::Reading(buf) => buf,
            _ => panic!("Connection not in reading state!"),
        }
    }

    fn unwrap_write_buf(self) -> Take<Cursor<Vec<u8>>> {
        match self {
            State::Writing(buf) => buf,
            _ => panic!("Connection not in writing state!"),
        }
    }
}

fn drain_to(vec: &mut Vec<u8>, count: usize) {
    for _ in 0..count {
        vec.remove(0);
    }
}

#[derive(Debug)]
pub struct LeCloud {
    server: Pong,
    addr: SocketAddr,
    state: LeState,
    // handle: Option<thread::JoinHandle<mio::EventLoop<Pong>>>,
    bcasts: Vec<Option<Vec<u8>>>,
}

impl LeCloud {
    fn new(address: SocketAddr) -> LeCloud {
        let mut pong = Pong::new(
            TcpListener::bind(
                &address
            ).unwrap()
        );
        LeCloud {
            server: pong,
            addr: address,
            state: LeState::PoweredOff,
            // handle: None,
            bcasts: vec![],
        }
    }

    fn listen(&self) {
        match self.state {
            LeState::PoweredOn => { /* handles listening here */ },
            _ => {
                panic!("THEE POWERR MUST BE ON");
            }
        }
    }

    fn power_on(&mut self) {
        // match self.state {
        //     LeState::PoweredOn => { /* continues as usual for now */ },
        //     _ => {
        //         self.state = LeState::PoweredOn;
        //         let mut event_loop = mio::EventLoop::new().unwrap();
        //
        //         event_loop.register(
        //             &self.server.server, SERVER
        //         ).unwrap();
        //
        //         let mut pong = self.server;
        //
        //         self.handle = Some(
        //             thread::spawn(move || {
        //                 event_loop.run(&mut pong).unwrap()
        //             })
        //         );
        //     }
        // }
        self.state = LeState::PoweredOn;
    }

    fn power_off(&mut self) {
        self.state = LeState::PoweredOff;
    }

    fn get_address(&self) -> SocketAddr {
        self.addr.clone()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::net::SocketAddr;
    use state::*;
    use std::net::TcpStream;
    use std::io::prelude::*;

    #[test]
    fn test_cloud_address() {
        let mut cloud = LeCloud::new(
            "0.0.0.0:5050"
                .parse()
                .unwrap()
        );
        let addr_details = "0.0.0.0:5050";
        let addr: SocketAddr =
            addr_details.parse().unwrap();
        assert_eq!(
            cloud.get_address(),
            addr
        );
    }

    #[test]
    fn test_toggle_power() {
        let mut cloud = LeCloud::new(
            "0.0.0.0:5051"
                .parse()
                .unwrap()
        );
        cloud.power_on();
        assert_eq!(cloud.state, State::PoweredOn);
        cloud.power_off();
        assert_eq!(cloud.state, State::PoweredOff);
    }

    #[test]
    fn test_empty_listen() {
        let mut cloud = LeCloud::new(
            "0.0.0.0:5052"
                .parse()
                .unwrap()
        );
        cloud.power_on();
        cloud.listen();
        assert_eq!(cloud.bcasts.len(), 0);
    }

    #[test]
    fn test_listen_with_message() {
        let mut cloud = LeCloud::new(
            "0.0.0.0:5053"
                .parse()
                .unwrap()
        );
        cloud.power_on();
        cloud.listen();

        let mut stream;
        match TcpStream::connect("0.0.0.0:5053") {
            Ok(st) => { stream = st },
            Err(e) => panic!("ERROR SOMWHEREE: {:?}", e),
        }

        let _ = stream.write(&[24,53,53]);
    }
}
