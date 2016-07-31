#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]
#![allow(dead_code)]

#[macro_use]
extern crate rustless;

#[macro_use]
extern crate diesel;

extern crate dotenv;
extern crate iron;
extern crate valico;
extern crate hyper;
extern crate curl;
extern crate rustc_serialize;

#[cfg(test)]
use curl::easy::{Easy, List};

use std::thread;

use hyper::status::StatusCode;
use iron::Iron;
use rustless::{
    Application, Api, Nesting, Versioning
};
use valico::json_dsl;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use rustc_serialize::json::{ToJson, Json};

mod schema;
mod models;
mod error;

use self::error::ServerError;
use self::models::Post;

pub fn establish_connection() -> Result<PgConnection, ServerError> {
    dotenv().ok();

    let database_url =
        match env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => {
                return Err(
                    ServerError::UnspecifiedDatabaseUrl
                );
            }
        };

    match PgConnection::establish(&database_url) {
        Ok(conn) => Ok(conn),
        Err(_) => Err(
            ServerError::UnableToConnectWithDatabase(database_url)
        )
    }
}

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

            api.namespace("posts", |posts_ns| {
                posts_ns.get("", |endpoint| {
                    endpoint.summary("");
                    endpoint.desc("");

                    endpoint.handle(|client, _params| {
                        let connection = match establish_connection() {
                            Ok(url) => url,
                            Err(error) => {
                                return client.error(error);
                            }
                        };

                        let posts = match Post::get_all(&connection) {
                            Ok(posts) => posts,
                            Err(error) => {
                                return client.error(error);
                            }
                        }.to_json();

                        client.json(&posts)
                    })
                });

                posts_ns.post("new", |endpoint| {

                    endpoint.summary("");
                    endpoint.desc("");

                    endpoint.params(|params| {
                        params.req_typed("title", json_dsl::string());
                        params.req_typed("body", json_dsl::string());
                    });

                    endpoint.handle(|client, params| {
                        let connection = match establish_connection() {
                            Ok(url) => url,
                            Err(error) => {
                                return client.error(error);
                            }
                        };

                        let title = match params.find("title") {
                            Some(title) => title.as_string(),
                            None => panic!()
                        }.unwrap();

                        let body = match params.find("body") {
                            Some(body) => body.as_string(),
                            None => panic!()
                        }.unwrap();

                        let info = (
                            title.to_string(), body.to_string()
                        );

                        let post: Json = match Post::create(&connection, info) {
                            Ok(post) => post,
                            Err(error) => {
                                return client.error(error);
                            }
                        }.to_json();

                        client.json(&post)
                    })
                });

                posts_ns.namespace(":id", |post_ns| {
                    post_ns.params(|params| {
                        params.req_typed("id", json_dsl::i64());
                    });

                    post_ns.get("", |endpoint| {
                        endpoint.summary("");
                        endpoint.desc("");

                        endpoint.handle(|client, params| {
                            let connection = match establish_connection() {
                                Ok(url) => url,
                                Err(error) => {
                                    return client.error(error);
                                }
                            };

                            let id = match params.find("id") {
                                Some(id) => id.as_i64(),
                                None => panic!()
                            }.unwrap() as i32;

                            let post = match Post::get(&connection, id) {
                                Ok(post) => post,
                                Err(error) => {
                                    return client.error(error);
                                }
                            }.to_json();

                            client.json(&post)
                        })
                    });

                    post_ns.get("delete", |endpoint| {
                        endpoint.summary("");
                        endpoint.desc("");

                        endpoint.handle(|client, params| {
                            let connection = match establish_connection() {
                                Ok(url) => url,
                                Err(error) => {
                                    return client.error(error);
                                }
                            };

                            let id = match params.find("id") {
                                Some(id) => id.as_i64(),
                                None => panic!()
                            }.unwrap() as i32;

                            let count = match Post::delete(&connection, id) {
                                Ok(c) => c,
                                Err(error) => {
                                    return client.error(error);
                                }
                            }.to_json();

                            client.json(&count)
                        })
                    });

                    post_ns.post("edit", |endpoint| {
                        endpoint.summary("");
                        endpoint.desc("");

                        endpoint.handle(|client, params| {
                            let connection = match establish_connection() {
                                Ok(url) => url,
                                Err(error) => {
                                    return client.error(error);
                                }
                            };

                            let id = match params.find("id") {
                                Some(id) => id.as_i64(),
                                None => panic!()
                            }.unwrap() as i32;

                            let mut post: Post = Post {
                                id: -1,
                                title: "".to_string(),
                                body: "".to_string(),
                                published: false
                            };

                            match params.find("publish") {
                                Some(_publish) => {
                                    let publish = true;
                                    match Post::publish(&connection, id, publish) {
                                        Ok(p) => {
                                            post = p;
                                        }
                                        Err(error) => {
                                            return client.error(error);
                                        }
                                    }
                                }
                                None => ()
                            }

                            client.json(&post.to_json())
                        })
                    })
                })
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
    handle: Easy
}

#[cfg(test)]
impl Client {
    fn new(req: &str) -> Self {
        let mut handle = Easy::new();

        let path = &
            format!("http://localhost:3030/api/{}", req)[..];

        handle.url(path).unwrap();

        let mut list = List::new();
        list.append("Accept:application/vnd.chat.v1+json").unwrap();
        handle.http_headers(list).unwrap();

        Client {
            handle: handle
        }
    }

    fn get(&mut self, dest: &mut Vec<u8>) {
        let mut transfer = self.handle.transfer();

        transfer.write_function(|res| {
            let bytes = res.len();
            dest.extend_from_slice(res);
            Ok(bytes)
        }).unwrap();

        transfer.perform().unwrap();
    }

    fn post(&mut self, payload: &String, dest: &mut Vec<u8>) {
        use std::io::Read;

        let mut payload = payload.as_bytes();

        self.handle.post(true).unwrap();
        self.handle.post_field_size(payload.len() as u64).unwrap();

        let mut transfer = self.handle.transfer();

        transfer.read_function(|buf| {
            Ok(payload.read(buf).unwrap_or(0))
        }).unwrap();

        transfer.write_function(|res| {
            let bytes = res.len();
            dest.extend_from_slice(res);
            Ok(bytes)
        }).unwrap();

        transfer.perform().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std;
    use rustc_serialize::json;

    #[test]
    fn test_simple_request() {
        let mut res = Vec::new();

        Client::new("emoji").get(&mut res);

        let emoji = std::str::from_utf8(&res).unwrap();

        assert_eq!("🙀", emoji);
    }

    #[test]
    fn test_create_post() {
        let request = "posts/new";
        let payload = "title=Testing&body=testing testing testing".to_string();

        let mut res = Vec::new();

        Client::new(request).post(&payload, &mut res);

        let res = std::str::from_utf8(&res).unwrap();

        let post: super::models::Post = json::decode(&res).unwrap();

        assert_eq!(post.title, "Testing");
        assert_eq!(post.body, "testing testing testing");
        assert_eq!(post.published, false);
    }

    #[test]
    fn test_publish_post() {
        let request = "posts/new";
        let payload = "title=Testing&body=testing testing testnig".to_string();

        let mut res = Vec::new();

        Client::new(request).post(&payload, &mut res);

        let res = std::str::from_utf8(&res).unwrap();

        let post: super::models::Post = json::decode(&res).unwrap();


        let request = format!("posts/{}/edit", post.id);
        let payload = "publish=true".to_string();

        let mut res = Vec::new();

        Client::new(&request[..]).post(&payload, &mut res);

        let res = std::str::from_utf8(&res).unwrap();

        let post: super::models::Post = json::decode(&res).unwrap();

        assert_eq!(post.published, true);
    }

    #[test]
    fn test_view_posts() {
        let request = "posts/new";
        let payload = "title=Testing&body=testing testing testing".to_string();

        let mut res = Vec::new();

        Client::new(request).post(&payload, &mut res);

        //
        // Count the amount of posts created
        //

        let request = "posts";

        let mut res = Vec::new();

        Client::new(request).get(&mut res);

        let res = std::str::from_utf8(&res).unwrap();

        let posts: Vec<super::models::Post> = json::decode(&res).unwrap();

        let posts_exists = posts.len() >= 1;

        assert_eq!(true, posts_exists);
    }

    #[test]
    fn test_get() {
        let request = "posts/new";
        let payload = "title=Testing&body=testing testing testnig".to_string();

        let mut res = Vec::new();

        Client::new(request).post(&payload, &mut res);

        let res = std::str::from_utf8(&res).unwrap();

        let post: super::models::Post = json::decode(&res).unwrap();

        //
        // Get
        //

        let request = format!("posts/{}", post.id);

        let mut res = Vec::new();

        Client::new(&request[..]).get(&mut res);

        let res = std::str::from_utf8(&res).unwrap();

        let get_post: super::models::Post = json::decode(&res).unwrap();

        assert_eq!(post.id, get_post.id);
        assert_eq!(post.title, get_post.title);
        assert_eq!(post.body, get_post.body);
        assert_eq!(post.published, get_post.published);
    }

    #[test]
    fn test_delete_posts() {
        let request = "posts/new";
        let payload = "title=Testing&body=testing testing testnig".to_string();

        let mut res = Vec::new();

        Client::new(request).post(&payload, &mut res);

        let res = std::str::from_utf8(&res).unwrap();

        let post: super::models::Post = json::decode(&res).unwrap();

        //
        // Delete the recently created post
        //

        let request = format!("posts/{}/delete", post.id);

        let mut res = Vec::new();

        Client::new(&request[..]).get(&mut res);

        let res = std::str::from_utf8(&res).unwrap();

        let count = json::decode(&res).unwrap();

        assert_eq!(1, count);

        //
        // Assert that the post is no long accessible
        //

        let request = format!("posts/{}", post.id);

        let mut res = Vec::new();

        Client::new(&request[..]).get(&mut res);

        let res = std::str::from_utf8(&res).unwrap();

        assert_eq!(res, "");
    }
}
