use super::Error::ParseError;
use uuid::Uuid;

use super::LoginRequest;

impl super::Packet for LoginRequest {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x01);
        data.extend_from_slice(self.uuid.as_bytes());
        data
    }

    fn deserialize(data: &[u8]) -> Result<LoginRequest, super::Error> {
        let uuid = match Uuid::from_slice(data) {
            Ok(uuid) => uuid,
            Err(_) => {
                return Err(ParseError);
            }
        };
        Ok(LoginRequest { uuid })
    }
}

impl LoginRequest {
    pub fn new(uuid: Uuid) -> Self {
        LoginRequest { uuid }
    }
}
