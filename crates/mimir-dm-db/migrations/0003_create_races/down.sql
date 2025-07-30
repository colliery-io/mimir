-- Drop races table and indexes
DROP INDEX IF EXISTS idx_races_parent;
DROP INDEX IF EXISTS idx_races_type;
DROP INDEX IF EXISTS idx_races_name;
DROP INDEX IF EXISTS idx_races_rule_system;
DROP INDEX IF EXISTS idx_races_source;
DROP TABLE IF EXISTS races;