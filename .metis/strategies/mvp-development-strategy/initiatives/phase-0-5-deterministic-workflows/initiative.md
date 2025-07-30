---
id: phase-0-5-deterministic-workflows
level: initiative
title: "Phase 0.5: Deterministic Workflows - Core Rules Only"
created_at: 2025-07-30T00:52:28.680079+00:00
updated_at: 2025-07-30T00:52:28.680079+00:00
parent: mvp-development-strategy
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: M
---

# Phase 0.5: Deterministic Workflows - Core Rules Only Initiative

## Context

With our data foundation in place from Phase 0, we can now build the core domain logic that makes Mimir useful for DMs. This phase focuses on implementing deterministic workflows based strictly on Rules as Written (RAW) from the Dungeon Master's Guide and Player's Handbook.

We're building comprehensive tools for character creation, campaign management, NPC generation, and session planning - all without requiring AI. This validates our data model, provides immediate value to DMs, and establishes the workflows that Phase 1's intelligence will enhance.

## Goals & Non-Goals

**Goals:**
- Implement complete character creation following PHB rules exactly
- Build campaign initialization wizard with DMG best practices
- Create comprehensive NPC generator using DMG tables
- Develop session planning tools based on DMG guidance
- Implement encounter builder with XP calculations
- Add treasure generation following DMG tables
- Create downtime activity tracking (XGE rules)
- Ensure all workflows validate against RAW

**Non-Goals:**
- AI-generated content (Phase 1)
- Homebrew rule support
- Combat automation
- Real-time dice rolling
- Character sheet PDF export
- Online/multiplayer features

## Detailed Design

### Core Domain Models

```rust
// Character Creation
pub struct CharacterBuilder {
    race: Race,
    class: Class,
    background: Background,
    abilities: AbilityScores,
    equipment: Vec<Equipment>,
    features: Vec<Feature>,
}

// Campaign Management
pub struct Campaign {
    settings: WorldSettings,
    calendar: Calendar,
    locations: HashMap<Uuid, Location>,
    factions: HashMap<Uuid, Faction>,
    party: Party,
    timeline: Vec<CampaignEvent>,
}

// NPC Framework (DMG Ch. 4)
pub struct NPC {
    appearance: Appearance,
    abilities: Abilities,
    talent: Talent,
    mannerism: Mannerism,
    interaction: InteractionTrait,
    ideal: Ideal,
    bond: Bond,
    flaw: FlawOrSecret,
    stat_block: Option<CreatureStats>,
}
```

### Key Workflows

**1. Character Creation Wizard**
- Step-by-step process following PHB order
- Automatic calculation of modifiers
- Class feature application
- Starting equipment selection
- Background feature integration
- Validation at each step

**2. Campaign Initialization**
- World type selection (homebrew/published)
- Calendar system setup
- Starting location definition
- Initial faction relationships
- Party composition
- Economic baseline

**3. NPC Generator**
- Quick NPC: Random DMG tables
- Detailed NPC: Full personality framework
- Stat block assignment
- Relationship web tracking
- Goal/scheme generation
- Loyalty mechanics

**4. Session Planning Assistant**
```
Session Prep Checklist:
□ Review last session summary
□ Update NPC locations/goals  
□ Prepare encounters (social/combat)
□ Ready treasure parcels
□ Plan decision points
□ Prepare contingencies
```

**5. Encounter Builder**
- Party size/level input
- Difficulty target (Easy/Med/Hard/Deadly)
- Environment considerations
- XP budget calculation
- Creature suggestions
- Terrain/complication options

### Data Integration
All workflows directly query the Phase 0 database:
- Character options from races/classes tables
- NPC traits from DMG lookup tables
- Monsters from creatures table (CR-filtered)
- Spells/items for rewards
- Equipment for starting gear

## Alternatives Considered

**Generic RPG System**
- Pro: Support multiple game systems
- Con: Loses D&D-specific optimizations
- Rejected: Focus on doing one thing excellently

**Web-Based Forms**
- Pro: Easier UI development, rich interactions
- Con: Violates TUI/local-first principles
- Rejected: Terminal interface is core requirement

**AI-First Approach**
- Pro: More flexible NPC generation
- Con: Unreliable without deterministic base
- Rejected: Need solid foundation before adding AI

**Minimal Rules Implementation**
- Pro: Faster to build
- Con: Not useful enough for real DMs
- Rejected: Comprehensive tools needed for adoption

**Import Character Builders**
- Pro: Reuse D&D Beyond, etc.
- Con: Legal issues, external dependencies
- Rejected: Must be fully self-contained

## Implementation Plan

### Week 1-2: Domain Modeling
- Define all core structs and enums
- Map DMG/PHB rules to code
- Create validation framework
- Design state machines for workflows
- Build calculation engine

### Week 3-4: Character Creation
- Implement race/class/background selection
- Build ability score generation
- Add equipment selection logic
- Implement feature application
- Create character sheet output

### Week 5-6: Campaign Tools
- Campaign initialization wizard
- Calendar and time tracking
- Location/faction management
- Party tracking system
- Economic calculations

### Week 7-8: NPC System
- DMG table implementations
- NPC generator with all traits
- Relationship tracking
- Goal/scheme generation
- Stat block integration

### Week 9-10: DM Workflows
- Session planning checklist
- Encounter builder with XP
- Treasure generation
- Random encounter tables
- Downtime activity tracking

### Week 11-12: Integration & Polish
- TUI interface for all features
- Workflow persistence
- Import/export capabilities
- Performance optimization
- Comprehensive testing

## Testing Strategy

### Rules Validation
- Every calculation matches PHB/DMG examples
- Character creation produces legal characters
- Encounter XP matches published adventures
- Treasure generation follows DMG tables exactly
- All optional rules properly implemented

### Workflow Testing
- Complete character creation paths
- Campaign initialization variations
- NPC generation coverage (all traits)
- Session planning completeness
- Encounter builder edge cases

### Integration Tests
- Database queries return correct data
- State persistence across sessions
- Import/export round trips
- Performance under load
- Memory usage reasonable

### User Acceptance Criteria
- Experienced DMs can create campaign in < 10 minutes
- Character creation faster than paper
- NPC generation produces usable results
- Session prep reduces time by 50%
- All outputs match RAW expectations

### Regression Suite
- Automated tests for all rules
- Workflow state machine validation
- Data integrity checks
- Performance benchmarks
- TUI interaction tests
