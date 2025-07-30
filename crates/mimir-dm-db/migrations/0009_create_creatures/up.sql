-- Create creatures table
CREATE TABLE creatures (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    source_id TEXT NOT NULL REFERENCES sources(id),
    page INTEGER,
    size TEXT,                  -- 'T', 'S', 'M', 'L', 'H', 'G'
    type TEXT,                  -- 'beast', 'humanoid', 'dragon', etc.
    type_tags TEXT CHECK(json_valid(type_tags)),         -- ["goblinoid", "shapechanger"]
    alignment TEXT CHECK(json_valid(alignment)),         -- ["L", "E"] for Lawful Evil
    armor_class TEXT CHECK(json_valid(armor_class)),     -- Can be complex: [{"ac": 15, "from": ["natural armor"]}]
    hit_points TEXT CHECK(json_valid(hit_points)),       -- {"average": 52, "formula": "8d8 + 16"}
    speed TEXT CHECK(json_valid(speed)),                 -- {"walk": 30, "fly": 60}
    ability_scores TEXT CHECK(json_valid(ability_scores)), -- {"str": 15, "dex": 14, ...}
    saving_throws TEXT CHECK(json_valid(saving_throws)), -- {"dex": "+5", "con": "+4"}
    skills TEXT CHECK(json_valid(skills)),               -- {"perception": "+3", "stealth": "+5"}
    damage_resistances TEXT CHECK(json_valid(damage_resistances)),
    damage_immunities TEXT CHECK(json_valid(damage_immunities)),
    condition_immunities TEXT CHECK(json_valid(condition_immunities)),
    senses TEXT CHECK(json_valid(senses)),               -- ["darkvision 60 ft.", "passive Perception 13"]
    languages TEXT CHECK(json_valid(languages)),         -- ["Common", "Goblin"]
    challenge_rating TEXT,      -- "1/4", "1", "20", etc.
    proficiency_bonus INTEGER,
    
    -- Denormalized actions (no separate table needed)
    traits TEXT CHECK(json_valid(traits)),               -- Array of trait objects
    actions TEXT CHECK(json_valid(actions)),             -- Array of action objects
    reactions TEXT CHECK(json_valid(reactions)),         -- Array of reaction objects
    legendary_actions TEXT CHECK(json_valid(legendary_actions)), -- Legendary action details
    lair_actions TEXT CHECK(json_valid(lair_actions)),   -- Lair action details
    regional_effects TEXT CHECK(json_valid(regional_effects)), -- Regional effect details
    
    entries TEXT NOT NULL CHECK(json_valid(entries)),  -- Full entry content
    environment TEXT CHECK(json_valid(environment)),       -- ["forest", "urban"]
    is_npc BOOLEAN DEFAULT FALSE,
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_creatures_source ON creatures(source_id);
CREATE INDEX idx_creatures_rule_system ON creatures(rule_system_id);
CREATE INDEX idx_creatures_type ON creatures(type);
CREATE INDEX idx_creatures_size ON creatures(size);
CREATE INDEX idx_creatures_challenge_rating ON creatures(challenge_rating);
CREATE INDEX idx_creatures_name ON creatures(name);