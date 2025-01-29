use super::models::{
    Agent, AgentFull, AgentHardware, AgentNetworkInfos, CreateHardware, CreateNetwork,
    UpdateHardware, UpdateNetwork,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use http::StatusCode;
use uuid::Uuid;

use crate::state::GlobalState;

async fn get_all_agents(State(state): State<GlobalState>) -> impl IntoResponse {
    let agents = match state.get_agents().await {
        Ok(agents) => agents,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::OK, Json(agents)).into_response()
}
async fn get_single_agent(
    State(state): State<GlobalState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let agent = match state.get_agent(id).await {
        Ok(agent) => match agent {
            Some(agent) => agent,
            None => return (StatusCode::NOT_FOUND).into_response(),
        },
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };
    let hardware = match state.get_hardware(id).await {
        Ok(hardware) => hardware,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };
    let network = match state.get_network_info(id).await {
        Ok(network) => network,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let agent_full = AgentFull::from((agent, network, hardware));
    (StatusCode::OK, Json(agent_full)).into_response()
}

async fn create_agent(State(state): State<GlobalState>) -> impl IntoResponse {
    let agent = Agent::new();
    match state.create_agent(&agent).await {
        Ok(_) => (),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };
    (StatusCode::CREATED, Json(agent)).into_response()
}

async fn delete_agent(State(state): State<GlobalState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match state.delete_agent(id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => match e {
            diesel::result::Error::NotFound => (StatusCode::NOT_FOUND).into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
    }
}
async fn create_hardware(
    State(state): State<GlobalState>,
    Path(id): Path<Uuid>,
    Json(create_hardware): Json<CreateHardware>,
) -> impl IntoResponse {
    match state.get_agent(id).await {
        Ok(agent) => match agent {
            Some(agent) => agent,
            None => return (StatusCode::NOT_FOUND).into_response(),
        },
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let create_hardware = AgentHardware::from((id, create_hardware));
    match state.create_hardware(&create_hardware).await {
        Ok(_) => (),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::CREATED, Json(create_hardware)).into_response()
}

async fn update_hardware(
    State(state): State<GlobalState>,
    Path(id): Path<Uuid>,
    Json(update_hardware): Json<UpdateHardware>,
) -> impl IntoResponse {
    match state.get_agent(id).await {
        Ok(agent) => match agent {
            Some(agent) => agent,
            None => return (StatusCode::NOT_FOUND).into_response(),
        },
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    match state.update_hardware(id, update_hardware).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => match e {
            diesel::result::Error::NotFound => (StatusCode::NOT_FOUND).into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
    }
}

async fn create_network(
    State(state): State<GlobalState>,
    Path(id): Path<Uuid>,
    Json(create_network): Json<CreateNetwork>,
) -> impl IntoResponse {
    match state.get_agent(id).await {
        Ok(agent) => match agent {
            Some(agent) => agent,
            None => return (StatusCode::NOT_FOUND).into_response(),
        },
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let create_network = AgentNetworkInfos::from((id, create_network));
    match state.create_network(&create_network).await {
        Ok(_) => (),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    (StatusCode::CREATED, Json(create_network)).into_response()
}

async fn update_network(
    State(state): State<GlobalState>,
    Path(id): Path<Uuid>,
    Json(update_network): Json<UpdateNetwork>,
) -> impl IntoResponse {
    let _ = match state.get_agent(id).await {
        Ok(agent) => match agent {
            Some(agent) => agent,
            None => return (StatusCode::NOT_FOUND).into_response(),
        },
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    match state.update_network(id, update_network).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => match e {
            diesel::result::Error::NotFound => (StatusCode::NOT_FOUND).into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
    }
}

pub fn get_router(state: &GlobalState) -> Router {
    Router::new()
        .route("/", get(get_all_agents))
        .route("/{uuid}", get(get_single_agent))
        .route("/register", post(create_agent))
        .route("/{uuid}", delete(delete_agent))
        // Agent Hardware
        .route("/{uuid}/hardware", post(create_hardware))
        .route("/{uuid}/hardware", put(update_hardware))
        // Agent Network
        .route("/{uuid}/network", post(create_network))
        .route("/{uuid}/network", put(update_network))
        .with_state(state.clone())
}
