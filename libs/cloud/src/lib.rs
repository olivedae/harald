#[macro_use]
extern crate rustless;

extern crate iron;
extern crate rustc_serialize as serialize;
extern crate valico;
extern crate hyper;
extern crate curl;

#[cfg(test)]
use curl::easy::{Easy, List};

use std::thread;

use hyper::status::StatusCode;
use iron::Iron;
use rustless::{
    Application, Api, Nesting, Versioning
};
use valico::json_dsl;

pub struct Cloud {
    app: Application
}

impl Cloud {
    pub fn new() -> Self {
        let api = Api::build(|api| {

            api.prefix("api");
            api.version("v1", Versioning::AcceptHeader("chat"));

            api.after(|client, _params| {
                client.set_status(StatusCode::NotFound);
                Ok(())
            });

            api.namespace("chats/:id", |chats_ns| {

                chats_ns.params(|params| {
                    params.req_typed("id", json_dsl::u64());
                });

                chats_ns.post("users/:user_id", |endpoint| {

                    endpoint.summary("");
                    endpoint.desc("");

                    endpoint.params(|params| {
                        params.req_typed("user_id", json_dsl::u64());
                        params.req_typed("name", json_dsl::string());
                    });

                    endpoint.handle(|client, params| {
                        client.json(params)
                    })
                });
            });

            api.get("emoji", |endpoint| {

                endpoint.summary("");
                endpoint.desc("");

                endpoint.handle(|client, _params| {
                    client.text("🙀".to_string())
                })
            });
        });

        let app = Application::new(api);

        Cloud {
            app: app
        }
    }

    pub fn start(self) -> thread::JoinHandle<()> {
        println!("Running on localhost:3030");

        thread::spawn(move || {
            Iron::new(self.app)
                .http("localhost:3030")
                .unwrap();
        })
    }
}

#[cfg(test)]
pub struct Client {
    curl: Easy
}

#[cfg(test)]
impl Client {
    fn new(req: &str) -> Self {
        let mut curl = Easy::new();

        let path = &
            format!("http://localhost:3030/api/{}", req)[..];

        curl.url(path).unwrap();

        let mut list = List::new();
        list.append("Accept:application/vnd.chat.v1+json").unwrap();
        curl.http_headers(list).unwrap();

        Client {
            curl: curl
        }
    }

    fn get(&mut self, dest: &mut Vec<u8>) {
        let mut transfer = self.curl.transfer();

        transfer.write_function(|data| {
            let bytes = data.len();
            dest.extend_from_slice(data);
            Ok(bytes)
        }).unwrap();

        transfer.perform().unwrap();
    }

    fn post(&mut self) {

    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std;

    #[test]
    fn test_simple_request() {
        Cloud::new().start();

        let mut dest = Vec::new();

        Client::new("emoji").get(&mut dest);

        let res = std::str::from_utf8(&dest).unwrap();

        assert_eq!("🙀", res);
    }
}
