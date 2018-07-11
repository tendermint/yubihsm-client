//! Export an wrapped object
//!
//! <https://developers.yubico.com/YubiHSM2/Commands/Export_Wrapped.html>

/*
    /// Get an object in encrypted form (wrapped)
    pub fn export_wrapped(
        &mut self,
        wrapkey_id: ObjectId,
        object_type: ObjectType,
        object_id: ObjectId,
    ) -> Result<ExportWrappedResponse, SessionError>
    {
        self.send_encrypted_command(ExportWrappedCommand {
            wrapkey_id,
            object_type,
            object_id,
        })
    }



/// Response from `CommandType::ExportWrapped`
#[derive(Serialize, Deserialize, Debug)]
pub struct ExportWrappedResponse {
    /// Nonce with wrapped data
    pub data: Vec<u8>,
}

impl Response for ExportWrappedResponse {
    const COMMAND_TYPE: CommandType = CommandType::ExportWrapped;
}


/// Request parameters for `CommandType::ExportWrapped`
/// 
/// <https://developers.yubico.com/YubiHSM2/Commands/Export_Wrapped.html>
#[derive(Serialize, Deserialize, Debug)]
pub struct ExportWrappedCommand {
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

*/