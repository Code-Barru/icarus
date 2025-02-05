use super::UpdateResponse;

impl super::Packet for UpdateResponse {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x05);
        data.push(self.need_update as u8);
        data
    }

    fn deserialize(data: &[u8]) -> Result<Self, super::Error> {
        if data.len() != 1 {
            return Err(super::Error::InvalidData);
        }

        let need_update: bool = data[0] != 0;

        Ok(UpdateResponse { need_update })
    }
}

impl UpdateResponse {
    pub fn new(need_update: bool) -> Self {
        UpdateResponse { need_update }
    }
}
