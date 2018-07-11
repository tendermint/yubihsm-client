//! Decrypt (unwrap) data using a Wrapkey.
//!
//! <https://developers.yubico.com/YubiHSM2/Commands/Unwrap_Data.html>

use super::{Command, Response};
use {
    CommandType, Connector, ObjectId, Session,
    SessionError,
};

/// Put an existing auth key into the `YubiHSM2`
pub fn unwrap_data<C: Connector, T: Into<Vec<u8>>>(
    session: &mut Session<C>,
    key_id: ObjectId,
    data: T,
) -> Result<WrapData, SessionError> {
    let data = data.into();
    
    session.send_encrypted_command(WrapDataCommand {
        key_id,
        data,
    }))
}

/// Request parameters for `commands::unwrap_data`
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UnwrapDataCommand {
    /// Key that use for wrap data
    pub key_id : ObjectId,
    /// Data to be wrapped
    pub data : Vec<u8>,
}

impl Command for UnwrapDataCommand {
    type ResponseType = WrapData;
}

/// Response from `commands::unwrap_data`
/// Wrapped data
#[derive(Serialize, Deserialize, Debug)]
pub struct UnwrapDataResponse(pub Vec<u8>);

impl Response for UnwrapDataResponse {
    const COMMAND_TYPE: CommandType = CommandType::UnwrapData;
}

// Further trait and impl for UnwrapData



/*

/// Request parameters for `CommandType::UnwrapData`
/// 
/// <https://developers.yubico.com/YubiHSM2/Commands/Unwrap_Data.html>
#[derive(Serialize, Deserialize, Debug)]
pub struct UnwrapDataCommand {
    pub key_id: ObjectId,
    pub data: Vec<u8>,
}

impl Command for UnwrapDataCommand {
    type ResponseType = UnwrapDataResponse;
}






/// Response from `CommandType::UnwrapData`
#[derive(Serialize, Deserialize, Debug)]
pub struct UnwrapDataResponse {
    /// Unwrapped data
    pub data: Vec<u8>,
}

impl Response for UnwrapDataResponse {
    const COMMAND_TYPE: CommandType = CommandType::UnwrapData;
}





    /// Decrypt (unwrap) data using a wrapkey.
    pub fn unwrap_data<T>(
        &mut self,
        key_id: ObjectId,
        data: T,
    ) -> Result<UnwrapDataResponse, SessionError>
    where T: Into<Vec<u8>>,
    {
        self.send_encrypted_command(UnwrapDataCommand {
            key_id,
            data: data.into(),
        })
    }

*/