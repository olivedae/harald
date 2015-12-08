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

    pub fn expand(incoming_pdu: Vec<u8>) -> Attribute {
        Attribute::new(
            0x0001, UUID::Custom(0x00ff), String::from("Example Data").into_bytes()
        )
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let handle = self.handle as *const u8;
        let a_type = self.the_type.to_hex() as *const u8;
        let raw_handle: &[u8] = unsafe {
            slice::from_raw_parts(
                handle,
                mem::size_of::<u8>(),
            )
        };
        let a_raw_type: &[u8] = unsafe {
            slice::from_raw_parts(
                a_type,
                mem::size_of::<u8>(),
            )
        };
        let mut raw_pdu: Vec<u8> = Vec::with_capacity(MTU);
        // for b in self.to_vec(raw_handle) {
        //     raw_pdu.push(b);
        //     println!("{:?}", b);
        // }
        // for b in self.to_vec(a_raw_type) {
        //     // raw_pdu.push(b);
        //     println!("{:?}", b);
        // }
        for b in self.value {
            raw_pdu.push(b);
        }
        raw_pdu
    }

    fn to_vec(&self, arr: &[u8]) -> Vec<u8> {
        arr.iter().cloned().collect()
    }
}

#[cfg(test)]
mod test_attribute {
    use super::*;
    use uuid::*;
    use bytes::*;

    #[test]
    fn test_new_attribute() {
        let pdu = Attribute::new(
            0x0001, UUID::Custom(0x00ff), String::from("Example Data").into_bytes()
        );
        assert_eq!(pdu.handle, 0x0001);
        assert_eq!(pdu.the_type, UUID::Custom(0x00ff));
        assert_eq!(pdu.value, String::from("Example Data").into_bytes());
    }

    #[test]
    #[should_panic]
    fn test_reserved_handle() {
        let pdu = Attribute::new(
            0x0000, UUID::Custom(0x00ff), String::from("Example Data").into_bytes()
        );
    }

    #[test]
    fn test_expanding() {
        let mut pdu = Attribute::new(
            0x0001, UUID::Custom(0x00ff), String::from("Example Data").into_bytes()
        ).to_bytes();
        let recieve = Attribute::expand(pdu);
        assert_eq!(recieve.handle, 0x0001);
        assert_eq!(recieve.the_type, UUID::Custom(0x00ff));
        assert_eq!(recieve.value, String::from("Example Data").into_bytes());
    }
}
