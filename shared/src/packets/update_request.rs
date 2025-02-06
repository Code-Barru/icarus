use super::UpdateRequest;

impl super::Packet for UpdateRequest {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x04);
        data.append(&mut self.agent_hash.bytes().collect::<Vec<u8>>());
        data
    }

    fn deserialize(data: &[u8]) -> Result<Self, super::Error> {
        if data.len() != 64 {
            return Err(super::Error::InvalidData);
        }

        let agent_hash: String = data[0..64].iter().map(|&x| x as char).collect();

        Ok(UpdateRequest { agent_hash })
    }
}

impl UpdateRequest {
    pub fn new(agent_hash: String) -> Self {
        UpdateRequest { agent_hash }
    }
}
