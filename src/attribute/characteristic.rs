struct Characteristic {
    service: Service,
    value: Data,
    descriptors: Vec<Descriptors>,
    properties: CharacteristicProperties,
    is_notifying: bool,
}
