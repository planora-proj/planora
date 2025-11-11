-- Add down migration script here

-- functions / triggers
DROP TRIGGER IF EXISTS trg_update_timestamp ON tasks;
DROP FUNCTION IF EXISTS update_task_timestamp;
DROP FUNCTION IF EXISTS recalc_project_progress;


-- indexes
DROP INDEX IF EXISTS idx_project_tasks;
DROP INDEX IF EXISTS idx_task_status;
DROP INDEX IF EXISTS idx_task_assignee;
DROP INDEX IF EXISTS idx_task_deadline;
DROP INDEX IF EXISTS idx_tasks_tags;


/* === tables === */
DROP TABLE IF EXISTS task_dependencies;
DROP TABLE IF EXISTS task_comments;
DROP TABLE IF EXISTS task_activity_log;
DROP TABLE IF EXISTS tasks;


/* === types === */
DROP TYPE IF EXISTS task_status;
DROP TYPE IF EXISTS task_priority;
