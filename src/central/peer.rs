use uuid::*;
use service::*;

pub struct Peripheral {
    services: Vec<Service>,
    uuid: UUID,
}

impl Peripheral {
    fn new(uuid: UUID) -> Peripheral {
        Peripheral { uuid: uuid }
    }
}
