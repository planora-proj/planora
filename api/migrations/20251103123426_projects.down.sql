-- Add down migration script here
DROP POLICY IF EXISTS projects_policy ON projects;
DROP TABLE IF EXISTS projects;
