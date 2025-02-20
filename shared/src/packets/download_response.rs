use super::DownloadResponse;

impl super::Packet for DownloadResponse {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x09);
        data.push(self.response as u8);
        data
    }
    fn deserialize(data: &[u8]) -> Result<DownloadResponse, super::Error> {
        if data.len() != 1 {
            return Err(super::Error::ParseError);
        }
        let response = data[0] != 0;
        Ok(DownloadResponse { response })
    }
}

impl DownloadResponse {
    pub fn new(response: bool) -> Self {
        DownloadResponse { response }
    }
}
