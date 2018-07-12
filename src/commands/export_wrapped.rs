//! Export an wrapped object
//!
//! <https://developers.yubico.com/YubiHSM2/Commands/Export_Wrapped.html>

use super::{Command, Response};
use {
    CommandType, Connector, ObjectId, ObjectType,
    Session, SessionError,
};

/// Get an object in encrypted form (wrapped)
pub fn export_wrapped<C: Connector>(
    session: &mut Session<C>,
    wrapkey_id: ObjectId,
    object_type: ObjectType,
    object_id: ObjectId,
) -> Result<ExportWrappedResponse, SessionError> {
    session.send_encrypted_command(ExportWrappedCommand {
        wrapkey_id,
        object_type,
        object_id,
    })
}

/// Request parameters for `commands::export_wrapped`
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ExportWrappedCommand {
    /// ID of wrapkey
    pub wrapkey_id: ObjectId,
    /// Type of object to wrap
    pub object_type: ObjectType,
    /// ID of object to wrap
    pub object_id: ObjectId,
}

impl Command for ExportWrappedCommand {
    type ResponseType = ExportWrappedResponse;
}

/// Response from `commands::export_wrapped`
#[derive(Serialize, Deserialize, Debug)]
pub struct ExportWrappedResponse {
    /// Nonce with wrapped data
    pub data: Vec<u8>,
}

impl Response for ExportWrappedResponse {
    const COMMAND_TYPE: CommandType = CommandType::ExportWrapped;
}

// Further traits for ExportWrappedResponse