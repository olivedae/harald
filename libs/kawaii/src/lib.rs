#![feature(trace_macros)]

extern crate rustless;
extern crate iron;
extern crate curl;

pub use iron::Iron;
pub use rustless::{
    Application, Api, Nesting, Versioning
};

macro_rules! 可愛い {
    ($a:ident) => {
        let api = Api::build(|api| {
            api.prefix("api");
            api.version("v1", Versioning::AcceptHeader("chat"));
        });

        let $a = Application::new(api);
    };
}

macro_rules! client_get {
    ($h:expr, url => $v:expr) => {{
        $h.url($v).unwrap();
    }};

    ($h:expr, header => $v:expr) => {{
        use curl::easy::List;

        let mut list = List::new();

        list.append($v).unwrap();

        $h.http_headers(list).unwrap();
    }};

    (
        buf: $buf:expr, $( $t:ident [ $v:expr ] )*
    ) => {{
        use std::str;
        use curl::easy::Easy;

        let mut handle = Easy::new();

        $(
            client_get!(handle, $t => $v);
        )*

        {
            let mut transfer = handle.transfer();

            transfer.write_function(|data| {
                $buf.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();

            transfer.perform().unwrap();
        }

        str::from_utf8(&$buf).unwrap()
    }};
}

macro_rules! client_post {
    ($h:expr, url => $v:expr) => {{
        $h.url($v).unwrap();
    }};

    ($h:expr, header => $v:expr) => {{
        use curl::easy::List;

        let mut list = List::new();

        list.append($v).unwrap();

        $h.http_headers(list).unwrap();
    }};

    ($h:expr, body => $v:expr) => {{
        let payload = $v.as_bytes();

        $h.post(true).unwrap();
        $h.post_field_size(payload.len() as u64).unwrap();
    }};

    (
        buf: $buf:expr,
        url[ $u:expr ]
        header[ $h:expr ]
        body[ $b:expr ]
    ) => {{
        use std::str;
        use curl::easy::Easy;
        use std::io::Read;

        let mut handle   = Easy::new();

        client_post!(handle, url => $u);
        client_post!(handle, header => $h);
        client_post!(handle, body => $b);

        let mut payload = $b.as_bytes();
        {
            let mut transfer = handle.transfer();

            transfer.read_function(|buf| {
                Ok(payload.read(buf).unwrap_or(0))
            }).unwrap();

            transfer.write_function(|data| {
                $buf.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();

            transfer.perform().unwrap();
        }

        str::from_utf8(&$buf).unwrap()
    }};
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;
    use std::net::SocketAddr;

    struct Mock {
        application: Application,
        url: SocketAddr
    }

    struct Controller;

    impl Mock {
        fn new(app: Application, url: SocketAddr) -> Self {
            Mock {
                application: app,
                url: url
            }
        }

        fn start(self) -> thread::JoinHandle<()> {
            thread::spawn(move || {
                Iron::new(self.application)
                    .http(self.url)
                    .unwrap();
            })
        }
    }

    impl Controller {
        fn emoji(client: Client) -> Response {
            client.text("🙈")
        }

        fn echo(client: Client, msg: String) -> Response {
            client.text(msg)
        }
    }

    fn socket(url: &str) -> SocketAddr {
        url.parse()
            .expect("Unable to parse socket address")
    }

    #[test]
    fn simple_get() {
        可愛い!(application, get(emoji) -> Controller.emoji);
        let url = socket("127.0.0.1:8000");
        let _ = Mock::new(application, url).start();

        let mut buffer = Vec::new();

        let emoji =
            client_get!(
                buf: buffer,
                url["http://127.0.0.1:8000/api/emoji"]
                header["Accept:application/vnd.chat.v1+json"]
            );

        assert_eq!(emoji, "🙈");
    }

    #[test]
    fn simple_post() {
        可愛い!(application, post(echo/:msg) -> Controller.echo);
        let url = socket("127.0.0.1:8001");
        let _ = Mock::new(application, url).start();

        let mut buffer = Vec::new();

        let emoji =
            client_post!(
                buf: buffer,
                url["https://127.0.0.1:8001/api/echo"]
                header["Accept:application/vnd.chat.v1+json"]
                body["msg=👋"]
            );

        assert_eq!(emoji, "👋");
    }
}
