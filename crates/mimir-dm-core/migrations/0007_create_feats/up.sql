-- Create feats table
CREATE TABLE feats (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    source_id TEXT NOT NULL REFERENCES sources(id),
    page INTEGER,
    prerequisites TEXT CHECK(json_valid(prerequisites)),    -- Complex prereq structure
    ability_increases TEXT CHECK(json_valid(ability_increases)),
    feat_type TEXT,             -- '2024': 'general', 'origin', 'fighting-style', 'epic'
    entries TEXT NOT NULL CHECK(json_valid(entries)),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_feats_source ON feats(source_id);
CREATE INDEX idx_feats_rule_system ON feats(rule_system_id);
CREATE INDEX idx_feats_name ON feats(name);
CREATE INDEX idx_feats_type ON feats(feat_type) WHERE feat_type IS NOT NULL;