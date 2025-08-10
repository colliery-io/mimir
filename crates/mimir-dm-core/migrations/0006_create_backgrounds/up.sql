-- Create backgrounds table
CREATE TABLE backgrounds (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    source_id TEXT NOT NULL REFERENCES sources(id),
    page INTEGER,
    skill_proficiencies TEXT CHECK(json_valid(skill_proficiencies)),
    language_proficiencies TEXT CHECK(json_valid(language_proficiencies)),
    tool_proficiencies TEXT CHECK(json_valid(tool_proficiencies)),
    starting_equipment TEXT CHECK(json_valid(starting_equipment)),
    feature_name TEXT,
    feature_text TEXT,          -- Extracted for easier searching
    entries TEXT NOT NULL CHECK(json_valid(entries)),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_backgrounds_source ON backgrounds(source_id);
CREATE INDEX idx_backgrounds_rule_system ON backgrounds(rule_system_id);
CREATE INDEX idx_backgrounds_name ON backgrounds(name);