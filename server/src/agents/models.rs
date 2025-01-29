use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::agents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub tasks: Vec<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub last_seen_at: chrono::NaiveDateTime,
}

impl Agent {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();

        Agent {
            id: uuid,
            name: uuid.to_string(),
            tasks: Vec::new(),
            created_at: chrono::Local::now().naive_utc(),
            updated_at: chrono::Local::now().naive_utc(),
            last_seen_at: chrono::Local::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::agent_network_infos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AgentNetworkInfos {
    pub agent_id: Uuid,
    pub hostname: String,
    pub ipv4: String,
    pub ipv6: String,
    pub mac: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<(Uuid, CreateNetwork)> for AgentNetworkInfos {
    fn from(tuple: (Uuid, CreateNetwork)) -> Self {
        let (agent_id, network) = tuple;
        AgentNetworkInfos {
            agent_id,
            hostname: network.hostname,
            ipv4: network.ipv4,
            ipv6: network.ipv6,
            mac: network.mac,
            created_at: chrono::Local::now().naive_utc(),
            updated_at: chrono::Local::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::agent_hardwares)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AgentHardware {
    pub agent_id: Uuid,
    pub cpu: String,
    pub memory: String,
    pub disk: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl AgentHardware {
    pub fn new(cpu: String, memory: String, disk: String) -> Self {
        AgentHardware {
            agent_id: Uuid::new_v4(),
            cpu,
            memory,
            disk,
            created_at: chrono::Local::now().naive_utc(),
            updated_at: chrono::Local::now().naive_utc(),
        }
    }
}

impl From<(Uuid, CreateHardware)> for AgentHardware {
    fn from(tuple: (Uuid, CreateHardware)) -> Self {
        let (agent_id, hardware) = tuple;
        AgentHardware {
            agent_id,
            cpu: hardware.cpu,
            memory: hardware.memory,
            disk: hardware.disk,
            created_at: chrono::Local::now().naive_utc(),
            updated_at: chrono::Local::now().naive_utc(),
        }
    }
}

// data validation
#[derive(Deserialize)]
pub struct CreateHardware {
    pub cpu: String,
    pub memory: String,
    pub disk: String,
}

#[derive(Deserialize)]
pub struct UpdateHardware {
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub disk: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateNetwork {
    pub hostname: String,
    pub ipv4: String,
    pub ipv6: String,
    pub mac: String,
}

#[derive(Deserialize)]
pub struct UpdateNetwork {
    pub hostname: Option<String>,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub mac: Option<String>,
}

#[derive(Serialize)]
pub struct AgentFull {
    pub agent: Agent,
    pub network_info: Option<AgentNetworkInfos>,
    pub hardware: Option<AgentHardware>,
}

impl From<(Agent, Option<AgentNetworkInfos>, Option<AgentHardware>)> for AgentFull {
    fn from(
        (agent, network_info, hardware): (Agent, Option<AgentNetworkInfos>, Option<AgentHardware>),
    ) -> Self {
        AgentFull {
            agent,
            network_info: network_info,
            hardware: hardware,
        }
    }
}
