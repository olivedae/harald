use controller::data::HasData;

pub enum Command {
    Reserved(u16),
    CommandReject(u16),
    ConnectionRequest,
    ConfigureRequest,
    ConfigureResponse,
    ConnectionResponse,
    DisconnectionRequest,
    DisconnectionResponse,
    EchoRequest,
    EchoResponse,
    InformationRequest,
    InformationResponse,
    CreateChannelRequest,
    CreateChannelResponse,
    MoveChannelRequest,
    MoveChannelResponse,
    MoveChannelConfirmation,
    MoveChannelConfirmationReponse,
    ConnectionParameterUpdateRequest,
    ConnectionParameterUpdateResponse,
    Unknown(u16),
}

pub enum CommandRejectReason {
    NotUnderstood,
    MTUExceeded,
    InvalidCID,
    Reserved,
}

impl HasData for Command {
    fn encode(&self) -> Vec<u8> {
        vec![15, 7]
    }
}

impl Command {
    pub fn from_u16(code: u16) -> Command {
        match code {
            0x00 | 0x17 ... 0xff => Command::Reserved(code),
            0x01 => Command::CommandReject(0x42),
            0x02 => Command::ConnectionRequest,
            0x03 => Command::ConnectionResponse,
            0x04 => Command::ConfigureRequest,
            0x05 => Command::ConfigureResponse,
            0x06 => Command::DisconnectionRequest,
            0x07 => Command::DisconnectionResponse,
            0x08 => Command::EchoRequest,
            0x09 => Command::EchoResponse,
            0x0a => Command::InformationRequest,
            0x0b => Command::InformationResponse,
            0x0c => Command::CreateChannelRequest,
            0x0d => Command::CreateChannelResponse,
            0x0e => Command::MoveChannelRequest,
            0x0f => Command::MoveChannelResponse,
            0x10 => Command::MoveChannelConfirmation,
            0x11 => Command::MoveChannelConfirmationReponse,
            0x12 => Command::ConnectionParameterUpdateRequest,
            0x13 => Command::ConnectionParameterUpdateResponse,
            _ => Command::Unknown(code),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            Command::Reserved(ref id) => *id,
            Command::CommandReject(ref id) => *id,
            Command::ConnectionRequest => 0x02,
            Command::ConfigureRequest => 0x03,
            Command::ConfigureResponse => 0x04,
            Command::ConnectionResponse => 0x05,
            Command::DisconnectionRequest => 0x06,
            Command::DisconnectionResponse => 0x07,
            Command::EchoRequest => 0x08,
            Command::EchoResponse => 0x09,
            Command::InformationRequest => 0x0a,
            Command::InformationResponse => 0x0b,
            Command::CreateChannelRequest => 0x0c,
            Command::CreateChannelResponse => 0x0d,
            Command::MoveChannelRequest => 0x0e,
            Command::MoveChannelResponse => 0x0f,
            Command::MoveChannelConfirmation => 0x10,
            Command::MoveChannelConfirmationReponse => 0x11,
            Command::ConnectionParameterUpdateRequest => 0x12,
            Command::ConnectionParameterUpdateResponse => 0x13,
            Command::Unknown(ref id) => *id,
        }
    }
}
