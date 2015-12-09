use uuid::*;
use bytes::*;
use std::{slice,mem};

const MTU: usize = 23;

pub struct Attribute {
    handle: u16,
    the_type: UUID,
    value: Vec<u8>,
}

impl Attribute {
    pub fn new(handle: u16, a_type: UUID, data: Vec<u8>) -> Attribute {
        match handle {
            0x0000 => panic!("Use of a reserved attribute handle. Attribute handles should be non-zero values between 0x0001 and 0xffff."),
            _ => {
                Attribute {
                    handle: handle,
                    the_type: a_type,
                    value: data,
                }
            }
        }
    }

    pub fn expand(incoming: Vec<u8>) -> Attribute {
        let mut bytes = incoming.clone();
        bytes.reverse();
        let mut pdu: Vec<String> = Vec::with_capacity(4);
        for _ in 0..4 {
            pdu.push(
                match bytes.pop() {
                    Some(byte) => format!("{:x}", byte).to_string(),
                    _ => panic!("Error!"),
                }
            );
        }
        let h: String = pdu[0..1].join("");
        let t: String = pdu[2..3].join("");
        bytes.reverse();
        Attribute {
            handle: h.parse::<u16>().unwrap(),
            the_type: UUID::Custom(
                t.parse::<u16>().unwrap()
            ),
            value: bytes,
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let h: *const u16 = &self.handle;
        let bh: *const u8 = h as *const _;
        let t: *const u16 = &self.the_type.to_hex();
        let bt: *const u8 = t as *const _;
        let mut raw_handle = Attribute::to_vec(
            unsafe {
                slice::from_raw_parts(
                    bh,
                    mem::size_of::<u16>()
                )
            }
        );
        let mut a_raw_type = Attribute::to_vec(
            unsafe {
                slice::from_raw_parts(
                    bt,
                    mem::size_of::<u16>(),
                )
            }
        );
        let mut raw_value = self.value.clone();

        let mut buf = Vec::with_capacity(MTU);

        buf.append(&mut raw_handle);
        buf.append(&mut a_raw_type);
        buf.append(&mut raw_value);
        buf
    }

    fn to_vec(arr: &[u8]) -> Vec<u8> {
        arr.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::*;

    #[test]
    fn test_new_attribute() {
        let pdu = Attribute::new(
            0x0001, UUID::Custom(0x0002), String::from("Example Data").into_bytes()
        );
        assert_eq!(pdu.handle, 0x0001);
        assert_eq!(pdu.the_type, UUID::Custom(0x0002));
        assert_eq!(pdu.value, String::from("Example Data").into_bytes());
    }

    #[test]
    #[should_panic]
    fn test_reserved_handle() {
        let pdu = Attribute::new(
            0x0000, UUID::Custom(0x0001), String::from("Example Data").into_bytes()
        );
    }

    #[test]
    fn test_to_bytes() {
        let pdu = Attribute::new(
            0x0002, UUID::Custom(0x1234), String::from("Test").into_bytes()
        );
        let b = pdu.to_bytes();
        assert_eq!(b, [2, 0, 52, 18, 84, 101, 115, 116])
    }

    #[test]
    fn test_expanding() {
        let mut pdu = Attribute::new(
            0x0001, UUID::Custom(0x0002), String::from("Example Data").into_bytes()
        ).to_bytes();
        let recieve = Attribute::expand(pdu);
        assert_eq!(recieve.handle, 0x0001);
        assert_eq!(recieve.the_type, UUID::Custom(0x0002));
        assert_eq!(recieve.value, String::from("Example Data").into_bytes());
    }
}
