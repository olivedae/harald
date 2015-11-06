pub struct Broadcast;

impl Broadcast {
    pub fn advertisement_format() -> &'static [u8] {
        "hello".as_bytes()
    }
}

#[cfg(test)]
mod test_broadcast {
    use super::*;

    #[test]
    fn test_advertisement() {
        assert_eq!(Broadcast::advertisement_format(), [104, 101, 108, 108, 111]);
    }
}
