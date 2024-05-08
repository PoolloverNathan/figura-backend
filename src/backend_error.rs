use std::convert::TryFrom;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendError {
    NormalClosure = 1000,
    GoingAway,
    ProtocolError,
    UnsupportedData,
    NoStatusReceived,
    AbnormalClosure,
    InvalidFramePayloadData,
    PolicyViolation,
    MessageTooBig,
    MandatoryExt,
    InternalError,
    ServiceRestart,
    TryAgainLater,
    BadGateway,
    TlsHandshake,
    Unauthorized = 3000,
    ReAuth = 4000,
    Banned,
    TooManyConnections,
}
impl TryFrom<u16> for BackendError {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, ()> {
        match value {
            1000 => Ok(BackendError::NormalClosure),
            1001 => Ok(BackendError::GoingAway),
            1002 => Ok(BackendError::ProtocolError),
            1003 => Ok(BackendError::UnsupportedData),
            1004 => Ok(BackendError::NoStatusReceived),
            1005 => Ok(BackendError::AbnormalClosure),
            1006 => Ok(BackendError::InvalidFramePayloadData),
            1007 => Ok(BackendError::PolicyViolation),
            1008 => Ok(BackendError::MessageTooBig),
            1009 => Ok(BackendError::MandatoryExt),
            1010 => Ok(BackendError::InternalError),
            1011 => Ok(BackendError::ServiceRestart),
            1012 => Ok(BackendError::TryAgainLater),
            1013 => Ok(BackendError::BadGateway),
            1014 => Ok(BackendError::TlsHandshake),
            3000 => Ok(BackendError::Unauthorized),
            4000 => Ok(BackendError::ReAuth),
            4001 => Ok(BackendError::Banned),
            4002 => Ok(BackendError::TooManyConnections),
            _ => Err(()),
        }
    }
}
#[cfg(test)]
#[test]
fn backend_error_round_trip() {
    let values = [
        BackendError::NormalClosure,
        BackendError::GoingAway,
        BackendError::ProtocolError,
        BackendError::UnsupportedData,
        BackendError::NoStatusReceived,
        BackendError::AbnormalClosure,
        BackendError::InvalidFramePayloadData,
        BackendError::PolicyViolation,
        BackendError::MessageTooBig,
        BackendError::MandatoryExt,
        BackendError::InternalError,
        BackendError::ServiceRestart,
        BackendError::TryAgainLater,
        BackendError::BadGateway,
        BackendError::TlsHandshake,
        BackendError::Unauthorized,
        BackendError::ReAuth,
        BackendError::Banned,
        BackendError::TooManyConnections,
    ];
    for v in values {
        assert_eq!(BackendError::try_from(v as u16), Ok(v))
    }
}
