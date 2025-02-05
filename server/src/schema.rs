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
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_seen_at -> Timestamp,
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        agent_uuid -> Uuid,
        task_type -> Varchar,
        status -> Varchar,
        parameters -> Nullable<Varchar>,
        result -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    agent_hardwares,
    agent_network_infos,
    agents,
    tasks,
);
