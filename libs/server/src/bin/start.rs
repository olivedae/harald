extern crate haraldserver;

use self::haraldserver::Cloud;

fn main() {
    let _ = Cloud::new().start().join();
}
