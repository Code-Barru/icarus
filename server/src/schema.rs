// @generated automatically by Diesel CLI.

diesel::table! {
    agent_hardwares (agent_id) {
        agent_id -> Uuid,
        cpu -> Varchar,
        memory -> Varchar,
        disk -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    agent_network_infos (agent_id) {
        agent_id -> Uuid,
        hostname -> Varchar,
        ipv4 -> Varchar,
        ipv6 -> Varchar,
        mac -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    agents (id) {
        id -> Uuid,
        name -> Varchar,
        tasks -> Array<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_seen_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(agent_hardwares, agent_network_infos, agents,);
