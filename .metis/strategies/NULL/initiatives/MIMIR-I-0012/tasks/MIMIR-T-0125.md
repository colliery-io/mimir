---
id: seed-dev-build-with-test-campaign
level: task
title: "Seed dev build with test campaign, players, and characters"
short_code: "MIMIR-T-0125"
created_at: 2025-11-25T13:13:58.889724+00:00
updated_at: 2025-11-25T13:13:58.889724+00:00
parent: MIMIR-I-0012
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


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

- [ ] Dev builds automatically seed a "Test Campaign" on first launch
- [ ] Campaign includes at least 2 modules with sessions
- [ ] At least 3 test players are created with realistic names
- [ ] At least 4 characters created spanning different classes and levels
- [ ] Characters have equipment, spells (where applicable), and backstory
- [ ] Seed data only runs in debug builds or when MIMIR_DEV env var is set
- [ ] Seeding is idempotent (doesn't duplicate on subsequent launches)
- [ ] Include characters at various levels (1, 5, 10) to test level-dependent features

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

*To be added during implementation*