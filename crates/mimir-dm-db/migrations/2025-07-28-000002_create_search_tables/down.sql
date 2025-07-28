-- Drop triggers first
DROP TRIGGER IF EXISTS sessions_fts_update;
DROP TRIGGER IF EXISTS sessions_fts_delete;
DROP TRIGGER IF EXISTS sessions_fts_insert;
DROP TRIGGER IF EXISTS plots_fts_update;
DROP TRIGGER IF EXISTS plots_fts_delete;
DROP TRIGGER IF EXISTS plots_fts_insert;
DROP TRIGGER IF EXISTS npcs_fts_update;
DROP TRIGGER IF EXISTS npcs_fts_delete;
DROP TRIGGER IF EXISTS npcs_fts_insert;
DROP TRIGGER IF EXISTS rules_fts_update;
DROP TRIGGER IF EXISTS rules_fts_delete;
DROP TRIGGER IF EXISTS rules_fts_insert;

-- Drop FTS5 virtual tables
DROP TABLE IF EXISTS sessions_fts;
DROP TABLE IF EXISTS plots_fts;
DROP TABLE IF EXISTS npcs_fts;
DROP TABLE IF EXISTS rules_fts;

-- Drop metadata table first
DROP TABLE IF EXISTS embedding_metadata;

-- Drop vector virtual table
DROP TABLE IF EXISTS embeddings;