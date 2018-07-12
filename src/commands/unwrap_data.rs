//! Decrypt (unwrap) data using a Wrapkey.
//!
//! <https://developers.yubico.com/YubiHSM2/Commands/Unwrap_Data.html>

use super::{Command, Response};
use {
    CommandType, Connector, ObjectId, Session,
    SessionError,
};

/// Decrypt (unwrap) data using a Wrapkey.
pub fn unwrap_data<C: Connector, T: Into<Vec<u8>>>(
    session: &mut Session<C>,
    key_id: ObjectId,
    data: T,
) -> Result<UnwrapDataResponse, SessionError> {
    let data = data.into();

    session.send_encrypted_command(UnwrapDataCommand {
        key_id,
        data,
    })
}

/// Request parameters for `commands::unwrap_data`
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UnwrapDataCommand {
    /// Key that use for unwrap data
    pub key_id : ObjectId,
    /// Data to be unwrapped
    pub data : Vec<u8>,
}

impl Command for UnwrapDataCommand {
    type ResponseType = UnwrapDataResponse;
}

/// Response from `commands::unwrap_data`
/// Raw Data
#[derive(Serialize, Deserialize, Debug)]
pub struct UnwrapDataResponse {
    /// Unwrapped Data
    pub data: Vec<u8>,
}

impl Response for UnwrapDataResponse {
    const COMMAND_TYPE: CommandType = CommandType::UnwrapData;
}

// Further trait and impl for UnwrapData

