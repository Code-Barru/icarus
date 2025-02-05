-- Your SQL goes here
CREATE TABLE "agent_hardwares"(
	"agent_id" UUID NOT NULL PRIMARY KEY,
	"cpu" VARCHAR NOT NULL,
	"memory" VARCHAR NOT NULL,
	"disk" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "agent_network_infos"(
	"agent_id" UUID NOT NULL PRIMARY KEY,
	"hostname" VARCHAR NOT NULL,
	"ipv4" VARCHAR NOT NULL,
	"ipv6" VARCHAR NOT NULL,
	"mac" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "agents"(
	"id" UUID NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	"last_seen_at" TIMESTAMP NOT NULL
);

CREATE OR REPLACE FUNCTION update_agents_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE agents
    SET updated_at = NOW()
    WHERE id = NEW.agent_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_agents_from_hardwares
AFTER INSERT OR UPDATE ON agent_hardwares
FOR EACH ROW
EXECUTE FUNCTION update_agents_timestamp();

CREATE TRIGGER trigger_update_agents_from_network_infos
AFTER INSERT OR UPDATE ON agent_network_infos
FOR EACH ROW
EXECUTE FUNCTION update_agents_timestamp();