-- Your SQL goes here
CREATE TABLE "tasks"(
	"id" UUID NOT NULL PRIMARY KEY,
	"agent_uuid" UUID NOT NULL,
	"task_type" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"parameters" VARCHAR,
	"result" VARCHAR,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);


CREATE OR REPLACE FUNCTION update_tasks_timestamp()
RETURNS TRIGGER AS $$
BEGIN
	IF NEW.updated_at IS DISTINCT FROM OLD.updated_at THEN
		NEW.updated_at = NOW();
	END IF;
	RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_tasks_timestamp
AFTER INSERT OR UPDATE ON tasks
FOR EACH ROW
EXECUTE FUNCTION update_tasks_timestamp();
