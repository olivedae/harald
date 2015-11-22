use std::fmt::{Debug, Formatter, Result};

pub enum UUID {
    Custom(u16),
}

impl UUID {
    pub fn as_hex(hex_string: &'static str) -> UUID {
        return UUID::Custom(0xff)
    }

    pub fn id(&self) -> u16 {
        match *self { UUID::Custom(ref id) => *id, }
    }
}

impl PartialEq for UUID {
    fn eq(&self, other: &UUID) -> bool {
        self.id() == other.id()
    }
}

impl Debug for UUID {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "UUID:{}", self.id())
    }
}
