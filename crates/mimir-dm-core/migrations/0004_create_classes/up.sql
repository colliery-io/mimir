-- Create classes table (includes both classes and subclasses)
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