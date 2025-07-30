-- Mimir SQLite Database Schema - Core Entities Only
-- Denormalized for read-heavy performance
-- Search tables and embeddings will be designed in Phase 1

-- ============================================
-- Core Tables (Source Management)
-- ============================================

-- Rule systems table (D&D 5e 2014, D&D 5e 2024, Pathfinder 2e, etc.)
CREATE TABLE rule_systems (
    id TEXT PRIMARY KEY,        -- 'dnd5e-2014', 'dnd5e-2024', 'pf2e'
    name TEXT NOT NULL,         -- 'D&D 5th Edition (2014)', 'D&D 5th Edition (2024)'
    short_name TEXT,            -- 'D&D 5e 2014', 'D&D 5e 2024'
    publisher TEXT,             -- 'Wizards of the Coast', 'Paizo'
    version TEXT,               -- '2014', '2024', '2.0'
    is_active BOOLEAN DEFAULT TRUE,
    metadata TEXT CHECK(json_valid(metadata))
);

CREATE INDEX idx_rule_systems_active ON rule_systems(is_active);

-- Sources table (books/supplements)
CREATE TABLE sources (
    id TEXT PRIMARY KEY,        -- e.g., 'PHB', 'MM', 'XGE'
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    full_name TEXT NOT NULL,    -- e.g., 'Player's Handbook'
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

-- ============================================
-- Entity Tables (Denormalized for Read Performance)
-- ============================================

-- Races table (includes both races and subraces)
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

-- Items table (includes base items and all variants)
CREATE TABLE items (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    source_id TEXT NOT NULL REFERENCES sources(id),
    page INTEGER,
    base_item_id TEXT REFERENCES items(id),  -- For filtering variants
    
    -- All fields populated (copied from base during import)
    type TEXT,                  -- 'weapon', 'armor', 'wondrous', etc.
    weight_lb REAL,
    value_cp INTEGER,           -- Value in copper pieces
    armor_class INTEGER,
    damage TEXT CHECK(json_valid(damage)),           -- {"dice": "1d8", "type": "slashing"}
    properties TEXT CHECK(json_valid(properties)),       -- ["finesse", "light", "thrown"]
    
    -- Magic item specific fields
    rarity TEXT,                -- NULL for mundane items
    requires_attunement BOOLEAN DEFAULT FALSE,
    attunement_prereq TEXT CHECK(json_valid(attunement_prereq)),
    magic_bonus INTEGER,        -- +1, +2, etc.
    additional_properties TEXT CHECK(json_valid(additional_properties)),  -- Special abilities
    
    entries TEXT NOT NULL CHECK(json_valid(entries)),
    is_magic BOOLEAN GENERATED ALWAYS AS (rarity IS NOT NULL) STORED,
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_items_source ON items(source_id);
CREATE INDEX idx_items_rule_system ON items(rule_system_id);
CREATE INDEX idx_items_type ON items(type);
CREATE INDEX idx_items_rarity ON items(rarity) WHERE rarity IS NOT NULL;
CREATE INDEX idx_items_magic ON items(is_magic);
CREATE INDEX idx_items_base ON items(base_item_id) WHERE base_item_id IS NOT NULL;

-- Backgrounds table
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

-- Feats table
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

-- Classes table (includes both classes and subclasses)
CREATE TABLE classes (
    id TEXT PRIMARY KEY,        -- 'fighter', 'fighter-champion', 'wizard-evoker'
    name TEXT NOT NULL,         -- 'Fighter', 'Champion'
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    source_id TEXT NOT NULL REFERENCES sources(id),
    page INTEGER,
    class_type TEXT NOT NULL,   -- 'class' or 'subclass'
    parent_class_id TEXT REFERENCES classes(id),  -- NULL for base classes
    
    -- Base class fields (populated for all entries)
    hit_die INTEGER,
    primary_abilities TEXT CHECK(json_valid(primary_abilities)),        -- ["str", "con"]
    saving_throws TEXT CHECK(json_valid(saving_throws)),            -- ["str", "con"]
    skill_proficiency_count INTEGER,
    skill_proficiency_choices TEXT CHECK(json_valid(skill_proficiency_choices)),
    starting_proficiencies TEXT CHECK(json_valid(starting_proficiencies)),
    starting_equipment TEXT CHECK(json_valid(starting_equipment)),
    spell_ability TEXT,         -- 'int', 'wis', 'cha'
    caster_progression TEXT,    -- 'full', 'half', 'third', 'pact'
    
    -- Subclass specific fields
    subclass_title TEXT,        -- "Martial Archetype", "Primal Path"
    subclass_level INTEGER,     -- Level when subclass is chosen
    
    -- Shared fields
    features TEXT CHECK(json_valid(features)),                 -- All features including base class features for subclasses
    spell_slots TEXT CHECK(json_valid(spell_slots)),              -- Spell progression if any
    entries TEXT NOT NULL CHECK(json_valid(entries)),
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_classes_source ON classes(source_id);
CREATE INDEX idx_classes_rule_system ON classes(rule_system_id);
CREATE INDEX idx_classes_name ON classes(name);
CREATE INDEX idx_classes_type ON classes(class_type);
CREATE INDEX idx_classes_parent ON classes(parent_class_id) WHERE parent_class_id IS NOT NULL;

-- Spells table (denormalized with classes)
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

-- Creatures table (denormalized with actions)
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
    armor_class TEXT CHECK(json_valid(armor_class)),       -- Can be complex: [{"ac": 15, "from": ["natural armor"]}]
    hit_points TEXT CHECK(json_valid(hit_points)),        -- {"average": 52, "formula": "8d8 + 16"}
    speed TEXT CHECK(json_valid(speed)),             -- {"walk": 30, "fly": 60}
    ability_scores TEXT CHECK(json_valid(ability_scores)),    -- {"str": 15, "dex": 14, ...}
    saving_throws TEXT CHECK(json_valid(saving_throws)),     -- {"dex": "+5", "con": "+4"}
    skills TEXT CHECK(json_valid(skills)),            -- {"perception": "+3", "stealth": "+5"}
    damage_resistances TEXT CHECK(json_valid(damage_resistances)),
    damage_immunities TEXT CHECK(json_valid(damage_immunities)),
    condition_immunities TEXT CHECK(json_valid(condition_immunities)),
    senses TEXT CHECK(json_valid(senses)),            -- ["darkvision 60 ft.", "passive Perception 13"]
    languages TEXT CHECK(json_valid(languages)),         -- ["Common", "Goblin"]
    challenge_rating TEXT,      -- "1/4", "1", "20", etc.
    proficiency_bonus INTEGER,
    
    -- Denormalized actions (no separate table needed)
    traits TEXT CHECK(json_valid(traits)),            -- Array of trait objects
    actions TEXT CHECK(json_valid(actions)),           -- Array of action objects
    reactions TEXT CHECK(json_valid(reactions)),         -- Array of reaction objects
    legendary_actions TEXT CHECK(json_valid(legendary_actions)), -- Legendary action details
    lair_actions TEXT CHECK(json_valid(lair_actions)),      -- Lair action details
    regional_effects TEXT CHECK(json_valid(regional_effects)),  -- Regional effect details
    
    entries TEXT NOT NULL CHECK(json_valid(entries)),  -- Full entry content
    environment TEXT CHECK(json_valid(environment)),       -- ["forest", "urban"]
    is_npc BOOLEAN DEFAULT FALSE,
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_creatures_source ON creatures(source_id);
CREATE INDEX idx_creatures_rule_system ON creatures(rule_system_id);
CREATE INDEX idx_creatures_name ON creatures(name);
CREATE INDEX idx_creatures_type ON creatures(type);
CREATE INDEX idx_creatures_cr ON creatures(challenge_rating);
CREATE INDEX idx_creatures_size ON creatures(size);

-- ============================================
-- Cross-Reference Tracking
-- ============================================

-- Still need this for tracking mentions/references between entities
CREATE TABLE content_references (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_entity_id TEXT NOT NULL,
    source_entity_type TEXT NOT NULL,
    target_entity_id TEXT NOT NULL,
    target_entity_type TEXT NOT NULL,
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    reference_type TEXT,     -- 'mentions', 'requires', 'grants', etc.
    context TEXT,
    UNIQUE(source_entity_id, source_entity_type, target_entity_id, target_entity_type)
);

CREATE INDEX idx_references_source ON content_references(source_entity_id, source_entity_type);
CREATE INDEX idx_references_target ON content_references(target_entity_id, target_entity_type);
CREATE INDEX idx_references_rule_system ON content_references(rule_system_id);

-- ============================================
-- Metadata and Configuration
-- ============================================

CREATE TABLE import_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path TEXT NOT NULL,
    file_type TEXT NOT NULL,
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    source_id TEXT REFERENCES sources(id),
    import_status TEXT NOT NULL, -- 'success', 'partial', 'failed'
    records_processed INTEGER,
    records_imported INTEGER,
    error_message TEXT,
    started_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    metadata TEXT CHECK(json_valid(metadata))
);

CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    description TEXT
);


-- ============================================
-- Example Queries Using JSON Functions
-- ============================================

-- Find all wizard spells in D&D 5e 2014
-- SELECT * FROM spells 
-- WHERE rule_system_id = 'dnd5e-2014'
-- AND EXISTS (
--     SELECT 1 FROM json_each(classes) 
--     WHERE json_each.value = 'wizard'
-- );

-- Find all items with the 'finesse' property in D&D 5e 2024
-- SELECT * FROM items 
-- WHERE rule_system_id = 'dnd5e-2024'
-- AND EXISTS (
--     SELECT 1 FROM json_each(properties) 
--     WHERE json_each.value = 'finesse'
-- );

-- Get all subclasses for Fighter
-- SELECT * FROM classes 
-- WHERE parent_class_id IN (
--     SELECT id FROM classes 
--     WHERE name = 'Fighter' AND class_type = 'class'
-- );

-- Find all longsword variants
-- SELECT * FROM items 
-- WHERE base_item_id IN (
--     SELECT id FROM items 
--     WHERE name = 'Longsword' AND base_item_id IS NULL
-- );