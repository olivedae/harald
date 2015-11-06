use uuid::*;

pub enum Descriptor {
    ExtendedProperties,
    UserDescription,
    ClientConfiguration,
    ServerConfiguration,
    PresentationFormat,
    AggregateFormat,
    ValidRange,
    ExternalReportReference,
    ReportReference,
    NumberOfDigitals,
    TriggerSetting,
    TestComplexBitfield,
    Unknown(UUID),
}

impl Descriptor {
    fn to_uuid(&self) -> UUID {
        UUID::Custom(match *self {
            Descriptor::ExtendedProperties => 0x2900,
            Descriptor::UserDescription => 0x2901,
            Descriptor::ClientConfiguration => 0x2902,
            Descriptor::ServerConfiguration => 0x2903,
            Descriptor::PresentationFormat => 0x2904,
            Descriptor::AggregateFormat => 0x2905,
            Descriptor::ValidRange => 0x2906,
            Descriptor::ExternalReportReference => 0x2907,
            Descriptor::ReportReference => 0x2908,
            Descriptor::NumberOfDigitals => 0x2909,
            Descriptor::TriggerSetting => 0x290a,
            Descriptor::TestComplexBitfield => 0x0000,
            Descriptor::Unknown(ref uuid) => *uuid.to_hex(),
        })
    }

    fn from_uuid(uuid: UUID) -> Descriptor {
        match uuid.to_hex() {
            0x2900 => Descriptor::ExtendedProperties,
            0x2901 => Descriptor::UserDescription,
            0x2902 => Descriptor::ClientConfiguration,
            0x2903 => Descriptor::ServerConfiguration,
            0x2904 => Descriptor::PresentationFormat,
            0x2905 => Descriptor::AggregateFormat,
            0x2906 => Descriptor::ValidRange,
            0x2907 => Descriptor::ExternalReportReference,
            0x2908 => Descriptor::ReportReference,
            0x2909 => Descriptor::NumberOfDigitals,
            0x290a => Descriptor::TriggerSetting,
            0x0000 => Descriptor::TestComplexBitfield,
            _ => Descriptor::Unknown(uuid),
        }
    }

    fn to_str(&self) -> &'static str {
        match *self {
            Descriptor::ExtendedProperties => "Characteristic Extended Properties",
            Descriptor::UserDescription => "Characteristic User Description",
            Descriptor::ClientConfiguration => "Client Characteristic Configuration",
            Descriptor::ServerConfiguration => "Server Characteristic Configuration",
            Descriptor::PresentationFormat => "Characteristic Presentation Format",
            Descriptor::AggregateFormat => "Characteristic Aggregate Format",
            Descriptor::ValidRange => "Valid Range",
            Descriptor::ExternalReportReference => "External Report Reference",
            Descriptor::ReportReference => "Report Reference",
            Descriptor::NumberOfDigitals => "Number of Digitals",
            Descriptor::TriggerSetting => "Trigger Setting",
            Descriptor::TestComplexBitfield => "Text Complex BitField",
            Descriptor::Unknown(ref uuid) => "Unknown",
        }
    }
}
