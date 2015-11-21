struct Service {
    peripheral: Peripheral,
    is_primary: bool,
    characteristics: Vec<Characteristic>,
    included_services: Vec<Service>,
}
