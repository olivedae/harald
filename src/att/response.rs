enum Response {
    ExchangeMTU,
    ErrorResponse,
    FindInformationResponse,
    ReadResponse,
    WriteResponse,
}

enum ErrorResponse {
    // The attribute handle given was not valid on this server.
    InvalidHandle,
    // The attribute cannot be read.
    ReadNotPermitted,
    // The attribute cannot be written.
    WriteNotPermitted,
    // The attribute PDU was invalid.
    InvalidPDU,
    // The attribute requires authentication before it
    // can be read or written.
    InsufficientAuthentication,
    // Attribute server does not support the request
    // received from the client.
    RequestNotSupported,
    // Offset specified was past the end of the attribute.
    InvalidOffset,
    // The attribute requires authorization before it can be
    // read or written.
    InsufficientAutherization,
    // Too many prepare writes have been queued.
    PrepareQueueFull,
    // No attribute found within the given
    // attribute handle range.
    AttributeNotFound,
    // The attribute cannot be read or written using the
    // Read Blob Request.
    AttributeNotLong,
    // The Encryption Key Size used for encrypting
    // this link is insufficient.
    InsufficientEncryptionKeySize,
    // The attribute value length is invalid for the operation.
    InvalidAttributeValueLength,
    // The attribute request that was requested has
    // encountered an error that was unlikely, and
    // therefore could not be completed as requested.
    UnlikelyError,
    // The attribute requires encryption before it can
    // be read or written.
    InsuffecientEncryption,
    // The attribute type is not a supported grouping
    // attribute as defined by a higher
    // layer specification.
    UnsupportedGroupType,
    // Insufficient Resources to complete the request.
    InsufficientResources,
    // Reserved for future use.
    Reserved,
    // Application error code defined by a higher
    // layer specification.
    ApplicationError,
}

enum ReadResponse {
    // Sent in reply to a received Read By Type Request
    // and contains the handles and
    // values of the attributes that have been read.
    ByType,
    // Sent in reply to a received Read Request and
    // contains the value of the attribute
    // that has been read.
    Standard,
    // Sent in reply to a received Read Blob Request
    // and contains part of the
    // value of the attribute that has been read.
    Blob,
    // Sent in reply to a received
    // Read Multiple Request and contains
    // the values of the attributes that
    // have been read.
    Multiple,
    // Sent in reply to a received
    // Read By Group Type Request and contains
    // the handles and values of the attributes that have been read.
    GroupType,
}

enum FindInformationResponse {
    // Sent in reply to a received
    // Find Information Request and contains
    // information about this server.
    Standard,
    // Sent in reply to a received
    // Find By Type Value Request and contains
    // information about this server.
    TypeValue,
}

enum WriteResponse {
    // Sent in reply to a valid Write Request
    // and acknowledges that the
    // attribute has been successfully written.
    Standard,
    // The Prepare Write Response is sent in
    // response to a received Prepare Write Request
    // and acknowledges that the value has been
    // successfully received and placed in the prepare write queue.
    Prepare,
    // The Execute Write Response is sent in
    // response to a received Execute Write Request.
    Execute,
}
