-- Create items table (includes mundane and magical items)
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