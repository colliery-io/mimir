-- Drop items table and indexes
DROP INDEX IF EXISTS idx_items_base;
DROP INDEX IF EXISTS idx_items_magic;
DROP INDEX IF EXISTS idx_items_rarity;
DROP INDEX IF EXISTS idx_items_type;
DROP INDEX IF EXISTS idx_items_rule_system;
DROP INDEX IF EXISTS idx_items_source;
DROP TABLE IF EXISTS items;