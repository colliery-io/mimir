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