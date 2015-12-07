use controller::data::*;

struct AttributePacket {
    type: UUID,
    handle: u16,
}

impl HasData for AttributePacket {
    fn encode(&self) -> Vec<u8> {
        vec![15, 15, 15]
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_new_attribute() {

    }
}
