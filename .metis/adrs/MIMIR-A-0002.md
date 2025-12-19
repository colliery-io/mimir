---
id: 002-multi-ruleset-architecture
level: adr
title: "Multi-Ruleset Architecture"
number: 2
short_code: "MIMIR-A-0002"
created_at: 2025-10-24T12:01:06.025814+00:00
updated_at: 2025-10-24T12:01:46.051518+00:00
decision_date: 
decision_maker: Engineering Team
parent: 
archived: false

tags:
  - "#adr"
  - "#phase/decided"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# ADR-2: Multi-Ruleset Architecture

## Context

D&D has multiple editions and rulesets (5e 2014, 5e 2024, potentially Pathfinder or other systems in the future). The same spell, item, or creature may have different mechanics across these rulesets. DMs may want to:
- Run campaigns using specific rulesets
- Compare implementations across versions
- Mix content from different sources while maintaining clarity

Additionally, each ruleset has multiple official and third-party source books that need to be tracked.

## Decision

We will implement a two-level hierarchy for content organization:

1. **Rule Systems Table**: Top-level container for game systems (e.g., 'dnd5e-2014', 'dnd5e-2024', 'pf2e')
2. **Sources Table**: Books/supplements that belong to a rule system (e.g., 'PHB', 'MM', 'XGE')
3. **Entity Foreign Keys**: Every game entity (spell, item, creature, etc.) references both rule_system_id and source_id
4. **Compound IDs**: Primary keys include the rule system to allow the same content across different systems

This allows "Fireball" to exist as both 'fireball_phb_dnd5e2014' and 'fireball_phb_dnd5e2024' with different mechanics.

## Rationale

This architecture was chosen because:

1. **Clear Separation**: Content from different rulesets never conflicts
2. **Easy Filtering**: Queries can easily filter by rule system or source
3. **Future Proof**: New game systems can be added without schema changes
4. **Comparison Queries**: Can query across rulesets to compare implementations

Alternatives considered:
- **Single Ruleset**: Would require separate databases or complex versioning
- **Version Field**: Would make it hard to support non-D&D systems
- **Separate Tables**: Would duplicate schema and complicate queries

## Consequences

### Positive
- **Complete Flexibility**: Can support any tabletop RPG system
- **No Data Conflicts**: Same-named content can coexist across rulesets
- **Source Tracking**: Always know which book content came from
- **SRD/OGL Filtering**: Can easily filter by is_official or is_srd flags

### Negative
- **Larger IDs**: Compound IDs are longer and less readable
- **Required Filtering**: Queries must always specify rule system to avoid mixing
- **Duplicate Content**: Common items (like "Rope") exist multiple times

### Neutral
- **Import Process**: Must map source files to correct rule system
- **UI Consideration**: Applications must clearly show which ruleset is active