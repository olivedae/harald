#![feature(trace_macros, non_ascii_idents)]

extern crate rustless;
extern crate iron;
extern crate curl;

pub use rustless::
{
    Application,
    Api,
    Nesting,
    Versioning
};
pub use rustless::framework::client::ClientResult;
pub use rustless::Client as RustlessClient;
pub use rustless::json::JsonValue;

pub type Response<'a>   = ClientResult<'a>;
pub type Parameters<'a> = &'a JsonValue;
pub type Client<'a>     = RustlessClient<'a>;

#[macro_export]
macro_rules! 可愛い {
    ( $action:ident ( $n:ident / $($rest:tt)* ) -> $func:expr; $($next:tt)* )
        => {{
            let api = Api::build(|api| {
                api.prefix("api");
                api.version("v1", Versioning::AcceptHeader("chat"));

                可愛い!(next api, $action ( $n / $($rest)* ) -> $func; $($next)* );
            });

            Application::new(api)
        }};

    ( $action:ident ( $n:ident : $m:ident / $($rest:tt)* ) -> $func:expr; $($next:tt)* )
        => {{
            let api = Api::build(|api| {
                api.prefix("api");
                api.version("v1", Versioning::AcceptHeader("chat"));

                可愛い!(next api, $action ( $n : $m / $($rest)* ) -> $func; $($next)* );
            });

            Application::new(api)
        }};

    ( $action:ident ( $n:ident ) -> $func:expr; $($next:tt)* )
        => {{
            let api = Api::build(|api| {
                api.prefix("api");
                api.version("v1", Versioning::AcceptHeader("chat"));

                可愛い!(next api, $action ( $n ) -> $func; $($next)* );
            });

            Application::new(api)
        }};

    ( $action:ident ( $n:ident : $m:ident ) -> $func:expr; $($next:tt)* )
        => {{
            let api = Api::build(|api| {
                api.prefix("api");
                api.version("v1", Versioning::AcceptHeader("chat"));

                可愛い!(next api, $action ( $n : $m ) -> $func; $($next)* );
            });

            Application::new(api)
        }};

    ( next $api:ident, $action:ident ( $n:ident / $($rest:tt)* ) -> $func:expr; $($next:tt)* )
        => {{
            use std::fmt::Write;

            let mut route = String::new();
            可愛い!(route route, $n / $($rest)*);
            可愛い!(scaffold $api, $action &route[..], $func);
            可愛い!(next $api, $($next)*);
        }};

    ( next $api:ident, $action:ident ( $n:ident : $m:ident / $($rest:tt)* ) -> $func:expr; $($next:tt)* )
        => {{
            use std::fmt::Write;

            let mut route = String::new();
            可愛い!(route route, $n : $m / $($rest)*);
            可愛い!(scaffold $api, $action &route[..], $func);
            可愛い!(next $api, $($next)*);
        }};

    ( next $api:ident, $action:ident ( $n:ident ) -> $func:expr; $($next:tt)* )
        => {{
            可愛い!(scaffold $api, $action stringify!($n), $func);
            可愛い!(next $api, $($next)*);
        }};

    ( next api:ident, $action:ident ( $n:ident : $m:ident ) -> $func:expr; $($next:tt)* )
        => {{
            可愛い!(scaffold $api, $action stringify!($n), $func);
            可愛い!(next $api, $($next)*);
        }};

    ( next $api:expr, )
        => ();

    ( route $buf:expr, $n:ident / $($rest:tt)* )
        => (let _ = write!($buf, "{}/", stringify!($n)); 可愛い!(route $buf, $($rest)*) );

    ( route $buf:expr, $n:ident : $m:ident / $($rest:tt)* )
        => (let _ = write!($buf, ":{}/", stringify!($n)); 可愛い!(route $buf, $($rest)*) );

    ( route $buf:expr, $n:ident )
        => (let _ = write!($buf, "{}", stringify!($n)); );

    ( route $buf:expr, $n:ident : $m:ident )
        => (let _ = write!($buf, ":{}", stringify!($n)); );

    // ( scaffold $ns:expr; $action:ident $route:expr ( $($n:ident : $t:ident)* ) -> $func:expr )
    ( scaffold $ns:expr, $action:ident $route:expr, $func:expr )
        => ($ns.$action($route, |ep| ep.handle(|c, p| $func(c, p))));
        // => (
        //     $ns.$action($route, |ep| {
        //         ep.params(|params| $( params.req_typed($n, json_dsl::$t()); )* );
        //         ep.handle(|c, p| $func(c, p))
        //     })
        // );

    () => ();
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

        let mut handle = Easy::new();

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
    use iron::Iron;

    struct Mock {
        application: Application,
        url: SocketAddr
    }

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

    macro_rules! start {
        ($port:expr => $app:expr) => {
            let _ = Mock::new($app, socket($port))
                .start();
        };
    }

    fn socket(url: &str) -> SocketAddr {
        url.parse()
            .expect("Unable to parse socket address")
    }

    fn emoji<'a>(client: Client<'a>, _params: Parameters) -> Response<'a> {
        client.text("🙈".to_string())
    }

    fn echo<'a>(client: Client<'a>, params: Parameters) -> Response<'a> {
        println!("working!");
        let msg = match params.find("msg") {
            Some(m) => m.as_str().unwrap(),
            None    => "😅"
        };

        client.text(msg.to_string())
    }

    #[test]
    fn simple_get() {
        let application =
            可愛い!(
                get (emoji) -> emoji;
            );

        start!("127.0.0.1:8000" => application);

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
        trace_macros!(true);
        let application =
            可愛い!(
                post (echo/msg:string) -> echo;
            );
        trace_macros!(false);
        start!("127.0.0.1:8001" => application);

        let mut buffer = Vec::new();

        let emoji =
            client_post!(
                buf: buffer,
                url["http://127.0.0.1:8001/api/echo/👋"]
                header["Accept:application/vnd.chat.v1+json"]
                body["msg=👋"]
            );

        assert_eq!(emoji, "👋");
    }
}
