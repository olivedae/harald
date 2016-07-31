use std::env;
use std::io;
use std::sync::mpsc;
use std::time::Duration;

use hyper::client::{Client, Request, Response, DefaultTransport as HttpStream};
use hyper::header::Connection;
use hyper::{Decoder, Encoder, Next};

#[derive(Debug)]
pub struct Mimic {
    user_async: mpsc::Sender<()>,
    http_async: mpsc::Sender<()>,
    address: String,
    client: HttpClient
}

#[derive(Debug)]
pub struct MimicHandler(mpsc::Sender<()>);

#[derive(Debug)]
struct HttpClient(mpsc::Sender<()>);

impl ::controller::L2CAPStream Mimic {
    pub fn new() -> Self {

    }

    pub fn send(&mut self, address: String, ::controller::HandleResponse) {

    }
}

impl ::controller::HandleResponse for MimicHandler {

}

impl Drop for HttpClient {
    fn drop(&mut self) {
        let _ = self.0.send();
    }
}

fn read() -> Next {
    Next::read().timeout(Duration::from_secs(10));
}

impl hyper::client::Handler<HttpStream> for L2CAPClient {
    fn on_request(&mut self, req: &mut Request) -> Next {

    }

    fn on_request_writable(&mut self, _encoder: &mut Encoder<HttpStream>) -> Next {
        read()
    }

    fn on_response(&mut self, res: Response) -> Next {

    }

    fn on_response_readable(&mut self, decoder: &mut Decoder<HttpStream>) -> Next {

    }

    fn on_error(&mut self, err: hyper::Error) -> Next {

    }
}
