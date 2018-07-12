//! Generate Wrapkey for wrapping objects
//!
//! <https://developers.yubico.com/YubiHSM2/Commands/Generate_Wrap_Key.html>

use super::{Command, Response};
use {
    CommandType, Capability, Connector, Domain, ObjectId, ObjectLabel,
    Session, SessionError, WrapAlgorithm,
};

/// Generate a new wrap key within the `YubiHSM2`
pub fn generate_wrap_key<C: Connector>(
    session: &mut Session<C>,
    key_id: ObjectId,
    label: ObjectLabel,
    domains: Domain,
    capabilities: Capability,
    algorithm: WrapAlgorithm,
    delegated_capabilities: Capability,
) -> Result<GenerateWrapKeyResponse, SessionError> {
    session.send_encrypted_command(GenerateWrapKeyCommand {
        key_id,
        label,
        domains,
        capabilities,
        algorithm,
        delegated_capabilities,
    })
}


/// Request parameters for `commands::generate_wrap_key`
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GenerateWrapKeyCommand {
    /// ID of the key
    pub key_id: ObjectId,

    /// Label for the key (40-bytes)
    pub label: ObjectLabel,

    /// Domain in which the key will be accessible
    pub domains: Domain,

    /// Capability of the key
    pub capabilities: Capability,

    /// Key algorithm
    pub algorithm: WrapAlgorithm,

    /// Capability of object import/export by the key
    pub delegated_capabilities: Capability,
}

impl Command for GenerateWrapKeyCommand {
    type ResponseType = GenerateWrapKeyResponse;
}

/// Response from `commands::generate_wrap_key`
#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateWrapKeyResponse {
    /// ID of the key
    pub key_id: ObjectId,
}

impl Response for GenerateWrapKeyResponse {
    const COMMAND_TYPE: CommandType = CommandType::GenerateWrapKey;
}