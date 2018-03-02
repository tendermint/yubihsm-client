//! Request data for `CommandType::DeleteObject`

use {ObjectId, ObjectType};
use byteorder::{BigEndian, WriteBytesExt};
#[cfg(feature = "mockhsm")]
use byteorder::ByteOrder;
use responses::DeleteObjectResponse;
use super::{Command, CommandType};
#[cfg(feature = "mockhsm")]
use super::{CommandMessage, Error};

/// Request data for `CommandType::DeleteObject`
///
/// <https://developers.yubico.com/YubiHSM2/Commands/Delete_Object.html>
#[derive(Debug)]
pub struct DeleteObjectCommand {
    /// Object ID to delete
    pub object_id: ObjectId,

    /// Type of object to delete
    pub object_type: ObjectType,
}

impl Command for DeleteObjectCommand {
    const COMMAND_TYPE: CommandType = CommandType::DeleteObject;
    type ResponseType = DeleteObjectResponse;

    /// Serialize data
    // TODO: procedurally generate this
    fn into_vec(self) -> Vec<u8> {
        let mut data = Vec::with_capacity(3);
        data.write_u16::<BigEndian>(self.object_id).unwrap();
        data.push(self.object_type.to_u8());
        data
    }

    /// Deserialize data
    #[cfg(feature = "mockhsm")]
    fn parse(command_msg: CommandMessage) -> Result<Self, Error> {
        if command_msg.data.len() != 3 {
            bail!(
                "invalid command data length {} (expected {})",
                command_msg.len(),
                3
            );
        }

        Ok(Self {
            object_id: BigEndian::read_u16(&command_msg.data[..2]),
            object_type: ObjectType::from_u8(command_msg.data[2])?,
        })
    }
}
