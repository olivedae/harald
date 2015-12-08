use std::fmt::{Debug, Formatter, Result};
use uuid::*;
use service::*;

#[derive(Clone)]
pub struct Peripheral {
    uuid: UUID,
    services: Option<Vec<Service>>,
}

impl Peripheral {
    pub fn new(uuid: UUID) -> Peripheral {
        Peripheral {
            uuid: uuid,
            services: None,
        }
    }

    pub fn name(&self) -> String {
        "Example Name".to_string()
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
