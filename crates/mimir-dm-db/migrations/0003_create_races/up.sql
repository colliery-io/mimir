-- Create races table (includes both races and subraces)
CREATE TABLE races (
    id TEXT PRIMARY KEY,        -- 'elf', 'high-elf', 'wood-elf'
    name TEXT NOT NULL,         -- 'Elf', 'High Elf', 'Wood Elf'
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    source_id TEXT NOT NULL REFERENCES sources(id),
    page INTEGER,
    race_type TEXT NOT NULL,    -- 'race' or 'subrace'
    parent_race_id TEXT REFERENCES races(id),  -- NULL for base races
    
    -- All fields populated for both races and subraces (no inheritance)
    size TEXT,                  -- 'S', 'M', 'L', etc.
    speed TEXT CHECK(json_valid(speed)),            -- {"walk": 30, "fly": 50}
    ability_scores TEXT CHECK(json_valid(ability_scores)),   -- Full scores, not deltas
    age TEXT CHECK(json_valid(age)),              -- {"mature": 20, "max": 180}
    alignment_tendency TEXT,
    language_proficiencies TEXT CHECK(json_valid(language_proficiencies)),
    trait_tags TEXT CHECK(json_valid(trait_tags)),       -- ["Natural Weapon", "Darkvision"]
    entries TEXT NOT NULL CHECK(json_valid(entries)), -- Full entry content
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_races_source ON races(source_id);
CREATE INDEX idx_races_rule_system ON races(rule_system_id);
CREATE INDEX idx_races_name ON races(name);
CREATE INDEX idx_races_type ON races(race_type);
CREATE INDEX idx_races_parent ON races(parent_race_id) WHERE parent_race_id IS NOT NULL;