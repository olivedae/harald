pub enum StreamID {
    PrivateReserved,
    Classic,
    Connectionless,
    AMPManagerProtocol,
    AttributeProtocol,
    Signaling,
    SMP,
    PublicReserved(u16),
    AMPTest,
    ConnectionOriented(u16),
    UnmatchedIDError(u16)
}

impl StreamID {
    pub fn from_u16(id: u16) -> StreamID {
        match id {
            0  => StreamID::PrivateReserved,
            1  => StreamID::Classic,
            2  => StreamID::Connectionless,
            3  => StreamID::AMPManagerProtocol,
            4  => StreamID::AttributeProtocol,
            5  => StreamID::Signaling,
            6  => StreamID::SMP,
            7 ... 62 => StreamID::PublicReserved(id),
            63 => StreamID::AMPTest,
            64 ... 65535 => StreamID::ConnectionOriented(id),
            _ => StreamID::UnmatchedIDError(id),
        }
    }
    pub fn to_u16(&self) -> u16 {
        match *self {
            StreamID::PrivateReserved => 0,
            StreamID::Classic => 1,
            StreamID::Connectionless => 2,
            StreamID::AMPManagerProtocol => 3,
            StreamID::AttributeProtocol => 4,
            StreamID::Signaling => 5,
            StreamID::SMP => 6,
            StreamID::PublicReserved(ref id) => *id,
            StreamID::AMPTest => 63,
            StreamID::ConnectionOriented(ref id) => *id,
            StreamID::UnmatchedIDError(ref id) => *id,
        }
    }
}
