extern crate haraldcloud;

use self::haraldcloud::Cloud;

fn main() {
    let _ = Cloud::new().start().join();
}
