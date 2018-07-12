//! Encrypt (wrap) data using a Wrapkey.
//!
//! <https://developers.yubico.com/YubiHSM2/Commands/Wrap_Data.html>

use super::{Command, Response};
use {
    CommandType, Connector, ObjectId, Session,
    SessionError,
};

/// Encrypt (wrap) data using a Wrapkey.
pub fn wrap_data<C: Connector, T: Into<Vec<u8>>>(
    session: &mut Session<C>,
    key_id: ObjectId,
    data: T,
) -> Result<WrapDataResponse, SessionError> {
    let data = data.into();

    session.send_encrypted_command(WrapDataCommand {
        key_id,
        data,
    })
}

/// Request parameters for `commands::wrap_data`
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct WrapDataCommand {
    /// Key that use for wrap data
    pub key_id : ObjectId,
    /// Data to be wrapped
    pub data : Vec<u8>,
}

impl Command for WrapDataCommand {
    type ResponseType = WrapDataResponse;
}

/// Response from `commands::wrap_data`
/// Wrapped data
#[derive(Serialize, Deserialize, Debug)]
pub struct WrapDataResponse{
    /// Wrapped Data by wrapkey
    pub wrapped_data: Vec<u8>,
}

impl Response for WrapDataResponse {
    const COMMAND_TYPE: CommandType = CommandType::WrapData;
}

// Further trait and impl for WrapData