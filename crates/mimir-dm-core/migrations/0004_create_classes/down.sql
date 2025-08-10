-- Drop classes table and indexes
DROP INDEX IF EXISTS idx_classes_parent;
DROP INDEX IF EXISTS idx_classes_type;
DROP INDEX IF EXISTS idx_classes_name;
DROP INDEX IF EXISTS idx_classes_rule_system;
DROP INDEX IF EXISTS idx_classes_source;
DROP TABLE IF EXISTS classes;