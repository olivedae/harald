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
            0x00  => ChannelID::PrivateReserved,
            0x01  => ChannelID::Classic,
            0x02  => ChannelID::Connectionless,
            0x03  => ChannelID::AMPManagerProtocol,
            0x04  => ChannelID::AttributeProtocol,
            0x05  => ChannelID::Signaling,
            0x06  => ChannelID::SMP,
            0x07 ... 0x3e => ChannelID::PublicReserved(id),
            0x3f => ChannelID::AMPTest,
            0x40 ... 0xfff => ChannelID::ConnectionOriented(id),
            _ => ChannelID::OutOfRangeIDError(id),
        }
    }
    pub fn to_u16(&self) -> u16 {
        match *self {
            ChannelID::PrivateReserved => 0x00,
            ChannelID::Classic => 0x01,
            ChannelID::Connectionless => 0x02,
            ChannelID::AMPManagerProtocol => 0x03,
            ChannelID::AttributeProtocol => 0x04,
            ChannelID::Signaling => 0x05,
            ChannelID::SMP => 0x06,
            ChannelID::PublicReserved(ref id) => *id,
            ChannelID::AMPTest => 0x3e,
            ChannelID::ConnectionOriented(ref id) => *id,
            ChannelID::OutOfRangeIDError(ref id) => *id,
        }
    }
}
