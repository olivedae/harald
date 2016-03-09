extern crate objc;

use objc::*;
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

pub struct Scan
{
    busy: bool,
    found_devices: Vec<Device>,
}

pub struct Device
{
    name: &'static str,
    address: i32,
}

impl Scan
{
    fn scan(interval: i32, cb: Box<Fn(Device)>)
    {

    }
}

impl Device
{
    fn name(&self) -> &'static str
    {
        self.0
    }

    fn address(&self) -> i32
    {
        self.1
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn bindings()
    {

    }

    #[test]
    fn scan()
    {
        let mut scan = Scan::new();
        let cb: Box<Fn(Device)> = Box::new(
            |d: Device| {
                let name = d.name();
                println!(name);
            }
        );
        scan.search(5, cb);
        assert_eq!(scan.found_count(), 0);
    }
}
