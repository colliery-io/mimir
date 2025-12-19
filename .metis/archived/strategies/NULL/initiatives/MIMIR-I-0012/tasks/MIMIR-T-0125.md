---
id: seed-dev-build-with-test-campaign
level: task
title: "Seed dev build with test campaign, players, and characters"
short_code: "MIMIR-T-0125"
created_at: 2025-11-25T13:13:58.889724+00:00
updated_at: 2025-11-25T18:19:38.485342+00:00
parent: MIMIR-I-0012
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0012
---

# Seed dev build with test campaign, players, and characters

## Parent Initiative

[[MIMIR-I-0012]]

## Objective

Automatically populate development builds with realistic test data including a sample campaign, players, and characters. This provides developers and testers with a working environment immediately after launching the app, eliminating manual setup and enabling faster iteration on features that depend on existing data.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Dev builds automatically seed a "Test Campaign" on first launch
- [x] Campaign includes at least 2 modules with sessions
- [x] At least 3 test players are created with realistic names
- [x] At least 4 characters created spanning different classes and levels
- [x] Characters have equipment, spells (where applicable), and backstory
- [x] Seed data only runs in debug builds or when MIMIR_DEV env var is set
- [x] Seeding is idempotent (doesn't duplicate on subsequent launches)
- [x] Include characters at various levels (1, 5, 10) to test level-dependent features

## Test Data to Include

### Campaign: "The Lost Mine of Phandelver"
- Status: Active
- 2 Modules: "Goblin Ambush", "Cragmaw Hideout"
- Several sessions in various states (planned, completed)

### Players
1. Alice (alice@test.com)
2. Bob (bob@test.com)  
3. Charlie (charlie@test.com)
4. Diana (diana@test.com)

### Characters
1. **Thorin Ironforge** - Level 5 Dwarf Fighter (Champion)
   - Player: Alice
   - Equipment: Chainmail, Battleaxe, Shield
   - Background: Soldier

2. **Elara Moonwhisper** - Level 5 Elf Wizard (Evocation)
   - Player: Bob
   - Spells: Fireball, Shield, Magic Missile, etc.
   - Background: Sage

3. **Finn Lightfoot** - Level 1 Halfling Rogue (starting character)
   - Player: Charlie
   - Equipment: Leather armor, Shortsword, Thieves' tools
   - Background: Criminal

4. **Sister Helena** - Level 10 Human Cleric (Life Domain)
   - Player: Diana
   - Higher level for testing advanced features
   - Full spell list, Channel Divinity uses

## Implementation Notes

### Technical Approach

1. Create `dev_seeder.rs` module in `mimir-dm-core`
2. Add seeding call to app initialization (after migrations)
3. Check for existing "Test Campaign" to ensure idempotency
4. Use existing service layer for all data creation
5. Store seed data as embedded JSON or construct programmatically

### Key Files
- `crates/mimir-dm-core/src/seed/dev_seeder.rs` (new)
- `crates/mimir-dm/src/app_init.rs` - Add seeding call
- `crates/mimir-dm/src/embedded_test_book.rs` - Reference for pattern

### Character Data Structure
```rust
struct SeedCharacter {
    name: &'static str,
    player_name: &'static str,
    race: &'static str,
    class: &'static str,
    subclass: Option<&'static str>,
    level: u8,
    background: &'static str,
    equipment: Vec<&'static str>,
    // ... etc
}
```

### Dependencies
- Requires catalog data (classes, races, items, spells) to be seeded first
- Uses existing CharacterService, PlayerService, CampaignService

## Status Updates

### 2025-11-25: Implementation Complete

Created `dev_seeder.rs` module in `mimir-dm-core/src/seed/` with:
- Full campaign seeding ("The Lost Mine of Phandelver")
- 2 modules with sessions (Goblin Ambush, Cragmaw Hideout)
- 4 players (Alice, Bob, Charlie, Diana)
- 4 characters at levels 1, 5, 5, and 10:
  - Thorin Ironforge (Fighter 5)
  - Elara Moonwhisper (Wizard 5 with full spell loadout)
  - Finn Lightfoot (Rogue 1 - new character)
  - Sister Helena (Cleric 10 with extensive spell list)

Also fixed a bug where `create_character` wasn't setting `current_level` properly for characters above level 1.

Commit: cb8380d