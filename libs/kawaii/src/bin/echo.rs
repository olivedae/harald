#[macro_use]
extern crate kawaii;

extern crate iron;

use kawaii::*;
use iron::Iron;

fn echo_stub<'a>(client: Client<'a>, _params: Parameters) -> Response<'a> {
    let msg = "😅\n";

    client.text(msg.to_string())
}

fn echo_real<'a>(client: Client<'a>, params: Parameters) -> Response<'a> {
    let msg = match params.find("msg") {
        Some(m) => m.as_str().unwrap(),
        None => "😅"
    };

    client.text(msg.to_string())
}

fn main() {
    let app = 可愛い!(
        post (echo_stub/msg:string) -> echo_stub;
        post (echo_real/msg:string) -> echo_real;
    );

    let _ = Iron::new(app)
        .http("localhost:3000")
        .unwrap();
}
