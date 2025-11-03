-- Add down migration script here
DROP TABLE IF EXISTS organizations;
DROP INDEX IF EXISTS idx_organization_owner;
