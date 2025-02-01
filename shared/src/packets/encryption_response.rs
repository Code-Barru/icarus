use super::EncryptionResponse;

impl super::Packet for EncryptionResponse {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x03);
        data.extend_from_slice(&self.shared_secret);
        data.extend_from_slice(&self.verify_token);
        data
    }

    fn deserialize(data: &[u8]) -> Result<EncryptionResponse, super::Error> {
        if data.len() != 512 {
            return Err(super::Error::ParseError);
        }
        let mut shared_secret = [0; 256];
        shared_secret.copy_from_slice(&data[0..256]);
        let mut verify_token = [0; 256];
        verify_token.copy_from_slice(&data[256..512]);
        Ok(EncryptionResponse {
            shared_secret,
            verify_token,
        })
    }
}

impl EncryptionResponse {
    pub fn new(shared_secret: [u8; 256], verify_token: [u8; 256]) -> Self {
        EncryptionResponse {
            shared_secret,
            verify_token,
        }
    }
}
