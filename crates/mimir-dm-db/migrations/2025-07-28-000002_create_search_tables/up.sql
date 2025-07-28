-- Vector storage for embeddings using sqlite-vec
CREATE VIRTUAL TABLE embeddings USING vec0(
    embedding float[384] -- nomic-embed-text produces 384-dimensional vectors
);

-- Additional metadata table for embeddings (since vec0 only supports the vector itself)
CREATE TABLE embedding_metadata (
    rowid INTEGER PRIMARY KEY,
    content_type TEXT NOT NULL,
    content_id TEXT NOT NULL,
    content_text TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- FTS5 virtual tables for full-text search (standalone, no external content)
CREATE VIRTUAL TABLE rules_fts USING fts5(
    rule_id UNINDEXED,
    title,
    content,
    category,
    tokenize='porter unicode61 remove_diacritics 2'
);

CREATE VIRTUAL TABLE npcs_fts USING fts5(
    npc_id UNINDEXED,
    name,
    description,
    personality,
    tokenize='porter unicode61 remove_diacritics 2'
);

CREATE VIRTUAL TABLE plots_fts USING fts5(
    plot_id UNINDEXED,
    title,
    summary,
    tokenize='porter unicode61 remove_diacritics 2'
);

CREATE VIRTUAL TABLE sessions_fts USING fts5(
    session_id UNINDEXED,
    summary,
    notes,
    tokenize='porter unicode61 remove_diacritics 2'
);

-- Triggers to keep FTS5 tables in sync
CREATE TRIGGER rules_fts_insert AFTER INSERT ON rules BEGIN
    INSERT INTO rules_fts(rule_id, title, content, category) 
    VALUES (new.id, new.title, new.content, new.category);
END;

CREATE TRIGGER rules_fts_delete AFTER DELETE ON rules BEGIN
    DELETE FROM rules_fts WHERE rule_id = old.id;
END;

CREATE TRIGGER rules_fts_update AFTER UPDATE ON rules BEGIN
    DELETE FROM rules_fts WHERE rule_id = old.id;
    INSERT INTO rules_fts(rule_id, title, content, category) 
    VALUES (new.id, new.title, new.content, new.category);
END;

CREATE TRIGGER npcs_fts_insert AFTER INSERT ON npcs BEGIN
    INSERT INTO npcs_fts(npc_id, name, description, personality) 
    VALUES (new.id, new.name, new.description, new.personality);
END;

CREATE TRIGGER npcs_fts_delete AFTER DELETE ON npcs BEGIN
    DELETE FROM npcs_fts WHERE npc_id = old.id;
END;

CREATE TRIGGER npcs_fts_update AFTER UPDATE ON npcs BEGIN
    DELETE FROM npcs_fts WHERE npc_id = old.id;
    INSERT INTO npcs_fts(npc_id, name, description, personality) 
    VALUES (new.id, new.name, new.description, new.personality);
END;

CREATE TRIGGER plots_fts_insert AFTER INSERT ON plots BEGIN
    INSERT INTO plots_fts(plot_id, title, summary) 
    VALUES (new.id, new.title, new.summary);
END;

CREATE TRIGGER plots_fts_delete AFTER DELETE ON plots BEGIN
    DELETE FROM plots_fts WHERE plot_id = old.id;
END;

CREATE TRIGGER plots_fts_update AFTER UPDATE ON plots BEGIN
    DELETE FROM plots_fts WHERE plot_id = old.id;
    INSERT INTO plots_fts(plot_id, title, summary) 
    VALUES (new.id, new.title, new.summary);
END;

CREATE TRIGGER sessions_fts_insert AFTER INSERT ON sessions BEGIN
    INSERT INTO sessions_fts(session_id, summary, notes) 
    VALUES (new.id, new.summary, new.notes);
END;

CREATE TRIGGER sessions_fts_delete AFTER DELETE ON sessions BEGIN
    DELETE FROM sessions_fts WHERE session_id = old.id;
END;

CREATE TRIGGER sessions_fts_update AFTER UPDATE ON sessions BEGIN
    DELETE FROM sessions_fts WHERE session_id = old.id;
    INSERT INTO sessions_fts(session_id, summary, notes) 
    VALUES (new.id, new.summary, new.notes);
END;

-- Note: sqlite-vec virtual tables don't use traditional indexes
-- Performance is optimized through the vector index built into vec0