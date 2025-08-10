-- Create spells table (denormalized with classes)
CREATE TABLE spells (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    source_id TEXT NOT NULL REFERENCES sources(id),
    page INTEGER,
    level INTEGER,              -- 0-9
    school TEXT,                -- 'A' (abjuration), 'C' (conjuration), etc.
    casting_time TEXT CHECK(json_valid(casting_time)),      -- {"number": 1, "unit": "action"}
    range TEXT CHECK(json_valid(range)),             -- {"type": "point", "distance": {"type": "feet", "amount": 120}}
    components TEXT CHECK(json_valid(components)),        -- {"v": true, "s": true, "m": "a pinch of salt"}
    duration TEXT CHECK(json_valid(duration)),          -- {"type": "timed", "duration": {"type": "minute", "amount": 10}}
    is_ritual BOOLEAN DEFAULT FALSE,
    is_concentration BOOLEAN DEFAULT FALSE,
    saving_throw TEXT CHECK(json_valid(saving_throw)),      -- ["dexterity", "wisdom"]
    damage_type TEXT CHECK(json_valid(damage_type)),       -- ["fire", "radiant"]
    entries TEXT NOT NULL CHECK(json_valid(entries)),
    upcast_info TEXT CHECK(json_valid(upcast_info)),       -- Higher level casting effects
    
    -- Denormalized class list (no junction table needed)
    classes TEXT CHECK(json_valid(classes)),           -- ["wizard", "sorcerer", "warlock"]
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_spells_source ON spells(source_id);
CREATE INDEX idx_spells_rule_system ON spells(rule_system_id);
CREATE INDEX idx_spells_level ON spells(level);
CREATE INDEX idx_spells_school ON spells(school);
CREATE INDEX idx_spells_name ON spells(name);
-- Index for JSON array contains using json_each
CREATE INDEX idx_spells_classes_json ON spells(classes);