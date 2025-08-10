-- Create sources table for content sources
CREATE TABLE sources (
    id TEXT PRIMARY KEY,
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    full_name TEXT NOT NULL,
    abbreviation TEXT,
    published_date DATE,
    version TEXT,
    is_official BOOLEAN DEFAULT TRUE,
    is_srd BOOLEAN DEFAULT FALSE,  -- Is this SRD/OGL content?
    metadata TEXT CHECK(json_valid(metadata))  -- Additional source metadata
);

CREATE INDEX idx_sources_official ON sources(is_official);
CREATE INDEX idx_sources_rule_system ON sources(rule_system_id);
CREATE INDEX idx_sources_srd ON sources(is_srd);