-- Core campaign management tables
CREATE TABLE campaigns (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    settings TEXT, -- JSON serialized settings
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE npcs (
    id TEXT PRIMARY KEY NOT NULL,
    campaign_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    personality TEXT,
    relationships TEXT, -- JSON serialized relationships
    stats TEXT, -- JSON serialized stats
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns (id) ON DELETE CASCADE
);

CREATE TABLE plots (
    id TEXT PRIMARY KEY NOT NULL,
    campaign_id TEXT NOT NULL,
    title TEXT NOT NULL,
    summary TEXT,
    status TEXT NOT NULL DEFAULT 'active', -- active, paused, completed
    connections TEXT, -- JSON serialized connections to NPCs/other plots
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns (id) ON DELETE CASCADE
);

CREATE TABLE sessions (
    id TEXT PRIMARY KEY NOT NULL,
    campaign_id TEXT NOT NULL,
    session_number INTEGER NOT NULL,
    date DATE NOT NULL,
    summary TEXT,
    notes TEXT,
    participants TEXT, -- JSON serialized list of participants
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns (id) ON DELETE CASCADE,
    UNIQUE(campaign_id, session_number)
);

CREATE TABLE rules (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    category TEXT NOT NULL, -- spell, class, race, item, etc.
    source TEXT, -- PHB, DMG, etc.
    page_reference TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_campaigns_name ON campaigns(name);
CREATE INDEX idx_npcs_campaign_id ON npcs(campaign_id);
CREATE INDEX idx_npcs_name ON npcs(name);
CREATE INDEX idx_plots_campaign_id ON plots(campaign_id);
CREATE INDEX idx_plots_status ON plots(status);
CREATE INDEX idx_sessions_campaign_id ON sessions(campaign_id);
CREATE INDEX idx_sessions_date ON sessions(date);
CREATE INDEX idx_rules_category ON rules(category);
CREATE INDEX idx_rules_title ON rules(title);