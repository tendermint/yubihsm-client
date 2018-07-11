/*

    /// Generate wrapkey
    pub fn generate_wrap_key(
        &mut self,
        key_id: ObjectId,
        label: ObjectLabel,
        domains: Domains,
        capabilities: Capabilities,
        algorithm: Algorithm,
        dc: Capabilities,
    ) -> Result<GenerateWrapKeyResponse, SessionError>
    {
        self.send_encrypted_command(GenerateWrapKeyCommand {
            key_id,
            label,
            domains,
            capabilities,
            algorithm,
            dc,
        })
    }

/// Response from `CommandType::GenerateWrapKey`
#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateWrapKeyResponse {
    /// ID of the key to be generated
    pub key_id: ObjectId,
}

impl Response for GenerateWrapKeyResponse {
    const COMMAND_TYPE: CommandType = CommandType::GenerateWrapKey;
}

/// Request parameters for `CommandType::GenerateWrapKey`
/// 
/// <https://developers.yubico.com/YubiHSM2/Commands/Generate_Wrap_Key.html>
#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateWrapKeyCommand {
    pub key_id: ObjectId,
    pub label: ObjectLabel,
    pub domains: Domains,
    pub capabilities: Capabilities,
    pub algorithm: Algorithm,
    pub dc: Capabilities,
}

impl Command for GenerateWrapKeyCommand {
    type ResponseType = GenerateWrapKeyResponse;
}
*/