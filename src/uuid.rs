use std::fmt::{Debug, Formatter, Result};

#[derive(Clone)]
pub enum UUID {
    Custom(u16),
}

impl UUID {
    pub fn as_hex(_hex_string: &'static str) -> UUID {
        /*
         * Users can enter a hexadeciaml
         * strings and will convert the given value
         * to its numerical representation.
         *
         * Returns a UUID::Custom with the
         * given hex string.
        */
        return UUID::Custom(0xff)
    }

    pub fn to_hex(&self) -> u16 {
        match *self { UUID::Custom(ref hex) => *hex, }
    }
}

impl PartialEq for UUID {
    fn eq(&self, other: &UUID) -> bool {
        self.to_hex() == other.to_hex()
    }
}

impl Debug for UUID {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "UUID:{}", self.to_hex())
    }
}
