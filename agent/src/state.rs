use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tracing::error;
use uuid::Uuid;

#[derive(Debug)]
pub struct State {
    pub uuid: Uuid,
    pub addr: String,
    pub rt_port: u16,
    pub api_port: u16,
}

impl State {
    pub fn new(file_path: &str) -> Result<State, std::io::Error> {
        let mut state = State {
            uuid: Uuid::new_v4(),
            addr: "".to_string(),
            rt_port: 0,
            api_port: 0,
        };
        let path = Path::new(file_path);
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open file: {:?}", e);
                return Err(e);
            }
        };
        let lines = io::BufReader::new(file).lines();

        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(e) => {
                    error!("Failed to read line: {:?}", e);
                    return Err(e);
                }
            };
            let parts: Vec<&str> = line.split("=").collect();
            match parts[0] {
                "uuid" => state.uuid = Uuid::parse_str(parts[1]).unwrap(),
                "addr" => state.addr = parts[1].to_string(),
                "rt_port" => state.rt_port = parts[1].parse().unwrap(),
                "api_port" => state.api_port = parts[1].parse().unwrap(),
                _ => (),
            }
        }
        Ok(state)
    }
}
