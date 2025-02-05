-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS "tasks";
DROP FUNCTION IF EXISTS update_tasks_timestamp();
DROP TRIGGER IF EXISTS trigger_update_tasks_timestamp ON tasks;