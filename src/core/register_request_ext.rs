use crate::{SipMessage, SipMessageError, SipMessageExt};

pub trait RegisterRequestExt {
    /// Returns one of:
    /// * "expires" of "Contact"
    /// * "Expires"
    ///
    /// [RFC3261, Page 65](https://tools.ietf.org/html/rfc3261#page-65) defines this behavior
    fn expires(&self) -> Result<u32, SipMessageError>;
}

impl RegisterRequestExt for SipMessage {
    fn expires(&self) -> Result<u32, SipMessageError> {
        if let Ok(expires) = self.contact_header_expires() {
            Ok(expires)
        } else if let Ok(expires) = self.expires_header() {
            Ok(expires)
        } else {
            Err(SipMessageError::MissingRegisterExpires)
        }
    }
}
