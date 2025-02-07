use crate::agents::models::{
    Agent, AgentHardware, AgentNetworkInfos, UpdateAgent, UpdateHardware, UpdateNetwork,
};
use crate::schema::agent_hardwares::dsl as agent_hardware_dsl;
use crate::schema::agent_network_infos::dsl as agent_network_info_dsl;
use crate::schema::agents::dsl as agent_dsl;
use crate::tasks::models::Task;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use shared::models::TaskStatus;
use uuid::Uuid;

use super::GlobalState;
use crate::schema::tasks::dsl as task_dsl;

impl GlobalState {
    pub async fn get_agents(&self) -> Result<Vec<Agent>, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;

        agent_dsl::agents.load::<Agent>(&mut *conn)
    }

    pub async fn get_agent(&self, id: Uuid) -> Result<Option<Agent>, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        agent_dsl::agents
            .filter(agent_dsl::id.eq(id))
            .first::<Agent>(&mut *conn)
            .optional()
    }

    pub async fn get_undone_tasks(&self, id: Uuid) -> Result<Vec<Task>, diesel::result::Error> {
        let agent = match self.get_agent(id).await {
            Ok(agent) => match agent {
                Some(agent) => agent,
                None => return Err(diesel::result::Error::NotFound),
            },
            Err(e) => return Err(e),
        };
        let mut conn = self.pg_connection.lock().await;

        task_dsl::tasks
            .filter(task_dsl::agent_uuid.eq(agent.id))
            .filter(task_dsl::status.eq(TaskStatus::Queued))
            .load::<Task>(&mut *conn)
    }

    pub async fn create_agent(&self, agent: &Agent) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        diesel::insert_into(agent_dsl::agents)
            .values(agent)
            .execute(&mut *conn)
    }

    pub async fn connect(&self, agent_id: Uuid) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        diesel::update(agent_dsl::agents.filter(agent_dsl::id.eq(agent_id)))
            .set((agent_dsl::connected.eq(true),))
            .execute(&mut *conn)
    }

    pub async fn disconnect(&self, agent_id: Uuid) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        diesel::update(agent_dsl::agents.filter(agent_dsl::id.eq(agent_id)))
            .set(agent_dsl::connected.eq(false))
            .execute(&mut *conn)
    }

    pub async fn disconnect_all(&self) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        diesel::update(agent_dsl::agents)
            .set(agent_dsl::connected.eq(false))
            .execute(&mut *conn)
    }

    pub async fn update_agent(
        &self,
        id: Uuid,
        update_agent: UpdateAgent,
    ) -> Result<usize, diesel::result::Error> {
        let mut con = self.pg_connection.lock().await;
        diesel::update(agent_dsl::agents.filter(agent_dsl::id.eq(id)))
            .set((
                agent_dsl::name.eq(update_agent.name),
                agent_dsl::updated_at.eq(chrono::Local::now().naive_utc()),
            ))
            .execute(&mut *con)
    }

    pub async fn delete_agent(&self, id: Uuid) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        diesel::delete(agent_dsl::agents.filter(agent_dsl::id.eq(id))).execute(&mut *conn)
    }
}

impl GlobalState {
    pub async fn create_hardware(
        &self,
        hardware: &AgentHardware,
    ) -> Result<(), diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        diesel::insert_into(agent_hardware_dsl::agent_hardwares)
            .values(hardware)
            .execute(&mut *conn)
            .map(|_| ())
    }

    pub async fn get_hardware(
        &self,
        id: Uuid,
    ) -> Result<Option<AgentHardware>, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        agent_hardware_dsl::agent_hardwares
            .filter(agent_hardware_dsl::agent_id.eq(id))
            .first::<AgentHardware>(&mut *conn)
            .optional()
    }

    pub async fn update_hardware(
        &self,
        id: Uuid,
        update_data: UpdateHardware,
    ) -> Result<(), diesel::result::Error> {
        let _ = match self.get_hardware(id).await {
            Ok(hardware) => match hardware {
                Some(_) => (),
                None => return Err(diesel::result::Error::NotFound),
            },
            Err(e) => return Err(e),
        };

        let query = diesel::update(
            agent_hardware_dsl::agent_hardwares.filter(agent_hardware_dsl::agent_id.eq(id)),
        )
        .set((
            update_data.cpu.map(|cpu| agent_hardware_dsl::cpu.eq(cpu)),
            update_data
                .memory
                .map(|memory| agent_hardware_dsl::memory.eq(memory)),
            update_data
                .disk
                .map(|disk| agent_hardware_dsl::disk.eq(disk)),
        ));

        match query.execute(&mut *self.pg_connection.lock().await) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                Err(e)
            }
        }
    }
}

impl GlobalState {
    pub async fn create_network(
        &self,
        network: &AgentNetworkInfos,
    ) -> Result<(), diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        diesel::insert_into(agent_network_info_dsl::agent_network_infos)
            .values(network)
            .execute(&mut *conn)
            .map(|_| ())
    }

    pub async fn get_network_info(
        &self,
        id: Uuid,
    ) -> Result<Option<AgentNetworkInfos>, diesel::result::Error> {
        let mut conn = self.pg_connection.lock().await;
        agent_network_info_dsl::agent_network_infos
            .filter(agent_network_info_dsl::agent_id.eq(id))
            .first::<AgentNetworkInfos>(&mut *conn)
            .optional()
    }

    pub async fn update_network(
        &self,
        id: Uuid,
        update_data: UpdateNetwork,
    ) -> Result<(), diesel::result::Error> {
        let _ = match self.get_network_info(id).await {
            Ok(network) => match network {
                Some(_) => (),
                None => return Err(diesel::result::Error::NotFound),
            },
            Err(e) => return Err(e),
        };

        let mut conn = self.pg_connection.lock().await;

        let query = diesel::update(
            agent_network_info_dsl::agent_network_infos
                .filter(agent_network_info_dsl::agent_id.eq(id)),
        )
        .set((
            update_data
                .hostname
                .map(|interface| agent_network_info_dsl::hostname.eq(interface)),
            update_data
                .ipv4
                .map(|ipv4| agent_network_info_dsl::ipv4.eq(ipv4)),
            update_data
                .ipv6
                .map(|ipv6| agent_network_info_dsl::ipv6.eq(ipv6)),
            update_data
                .mac
                .map(|mac| agent_network_info_dsl::mac.eq(mac)),
        ));

        match query.execute(&mut *conn) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
