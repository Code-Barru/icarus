use super::EncryptionRequest;

impl super::Packet for EncryptionRequest {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x02);
        data.extend_from_slice(&self.key_length.to_be_bytes());
        data.extend_from_slice(&self.public_key);
        data.extend_from_slice(&self.verify_token.to_be_bytes());
        data
    }

    fn deserialize(data: &[u8]) -> Result<EncryptionRequest, super::Error> {
        let key_length = u16::from_be_bytes([data[0], data[1]]);

        if data.len() != (2 + key_length as usize + 4) {
            return Err(super::Error::ParseError);
        }

        let public_key = data[2..(key_length as usize) + 2].to_vec();
        let offset = (key_length as usize) + 2;
        let verify_token = u32::from_be_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        Ok(EncryptionRequest {
            key_length,
            public_key,
            verify_token,
        })
    }
}

impl EncryptionRequest {
    pub fn new(public_key: &[u8], verify_token: u32) -> Self {
        EncryptionRequest {
            key_length: public_key.len() as u16,
            public_key: public_key.to_vec(),
            verify_token,
        }
    }
}
