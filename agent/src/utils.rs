use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use uuid::Uuid;

pub struct State {
    pub uuid: Uuid,
    pub addr: String,
    pub rt_port: u16,
    pub api_port: u16,
}

impl From<String> for State {
    fn from(file_path: String) -> Self {
        let path = Path::new(&file_path);
        let file = File::open(&path).expect("Unable to open file");
        let reader = io::BufReader::new(file);

        let mut lines = reader.lines();
        let uuid_str = lines
            .next()
            .expect("Missing uuid")
            .expect("Unable to read uuid");
        let uuid = Uuid::parse_str(&uuid_str).expect("Invalid uuid");
        let addr = lines
            .next()
            .expect("Missing addr")
            .expect("Unable to read addr");
        let rt_port = lines
            .next()
            .expect("Missing rt_port")
            .expect("Unable to read rt_port")
            .parse()
            .expect("Invalid rt_port");
        let api_port = lines
            .next()
            .expect("Missing api_port")
            .expect("Unable to read api_port")
            .parse()
            .expect("Invalid api_port");

        State {
            uuid,
            addr,
            rt_port,
            api_port,
        }
    }
}
