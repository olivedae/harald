pub enum ChannelID {
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
    OutOfRangeIDError(u16)
}

impl ChannelID {
    pub fn from_u16(id: u16) -> ChannelID {
        match id {
            0  => ChannelID::PrivateReserved,
            1  => ChannelID::Classic,
            2  => ChannelID::Connectionless,
            3  => ChannelID::AMPManagerProtocol,
            4  => ChannelID::AttributeProtocol,
            5  => ChannelID::Signaling,
            6  => ChannelID::SMP,
            7 ... 62 => ChannelID::PublicReserved(id),
            63 => ChannelID::AMPTest,
            64 ... 65535 => ChannelID::ConnectionOriented(id),
            _ => ChannelID::OutOfRangeIDError(id),
        }
    }
    pub fn to_u16(&self) -> u16 {
        match *self {
            ChannelID::PrivateReserved => 0,
            ChannelID::Classic => 1,
            ChannelID::Connectionless => 2,
            ChannelID::AMPManagerProtocol => 3,
            ChannelID::AttributeProtocol => 4,
            ChannelID::Signaling => 5,
            ChannelID::SMP => 6,
            ChannelID::PublicReserved(ref id) => *id,
            ChannelID::AMPTest => 63,
            ChannelID::ConnectionOriented(ref id) => *id,
            ChannelID::OutOfRangeIDError(ref id) => *id,
        }
    }
}
