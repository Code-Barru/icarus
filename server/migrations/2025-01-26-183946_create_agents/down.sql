-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS "agent_hardwares";
DROP TABLE IF EXISTS "agent_network_infos";
DROP TABLE IF EXISTS "agents";
DROP FUNCTION IF EXISTS update_agents_timestamp();
DROP TRIGGER IF EXISTS trigger_update_agents_from_hardwares ON agent_hardwares;
DROP TRIGGER IF EXISTS trigger_update_agents_from_network_infos ON agent_network_infos;
