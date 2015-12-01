use std::fmt::{Debug, Formatter, Result};
use uuid::*;
use service::*;

pub struct Peripheral {
    services: Option<Vec<Service>>,
    uuid: UUID,
}

impl Peripheral {
    pub fn new(uuid: UUID) -> Peripheral {
        Peripheral {
            uuid: uuid,
            services: None,
        }
    }
}

impl PartialEq for Peripheral {
    fn eq(&self, other: &Peripheral) -> bool {
        self.uuid.eq(&other.uuid)
    }
}

impl Debug for Peripheral {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Peripheral {:?}", self.uuid)
    }
}
