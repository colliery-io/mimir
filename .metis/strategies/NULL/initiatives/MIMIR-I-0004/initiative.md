---
id: character-creation-and-management
level: initiative
title: "Character Creation and Management System"
short_code: "MIMIR-I-0004"
created_at: 2025-11-10T04:27:39.110957+00:00
updated_at: 2025-11-21T10:35:19.414779+00:00
parent: 
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/active"


exit_criteria_met: false
estimated_complexity: XL
strategy_id: NULL
initiative_id: character-creation-and-management
---

# Character Creation and Management System Initiative

*This template includes sections for various types of initiatives. Delete sections that don't apply to your specific use case.*

## Context **[REQUIRED]**

Mimir currently lacks character management functionality. DMs and players need the ability to create, manage, and track characters throughout a campaign. This includes character creation following D&D 5e rules, inventory management, character progression (leveling), and character sheet display.

This initiative focuses on building a simplified but functional character system that can:
- Create characters using D&D 5e core rules (races, classes, backgrounds, ability scores)
- Manage character state across sessions (HP, inventory, equipment, spells)
- Handle character progression (leveling, ASI/feats, multiclassing)
- Store character data in versioned markdown files with YAML frontmatter
- Display character information in a readable format
- **Be accessible from both GUI and chat interface via tools**

**Critical Design Constraint**: All character logic must reside in the backend/core services with clear, tool-accessible interfaces. The chat interface must be able to perform all character operations (create, level up, modify) through tool calls to the same service methods used by the GUI.

## Goals & Non-Goals **[REQUIRED]**

**Goals:**
- Enable character creation following D&D 5e rules (race, class, background, abilities)
- Support character progression through leveling (HP, ASI/feats, multiclassing)
- Track feats and spells with basic automation (spell slots, preparation limits)
- Manage character inventory and equipment between sessions
- Store character state as flexible JSON for future extensibility
- Display character sheets in a readable format
- Support multiple characters per campaign

**Non-Goals:**
- Full mechanical implementation of every feat effect and spell interaction
- Advanced character sheet calculations (auto-calculating AC from all sources, attack bonuses with all modifiers, etc.)
- Character import/export to other platforms (D&D Beyond, Roll20, etc.)
- Digital dice rolling integration
- Character portraits or custom artwork
- Mobile-optimized character sheets
- Real-time character sync across multiple users

## Requirements **[CONDITIONAL: Requirements-Heavy Initiative]**

### User Requirements
- **User Characteristics**: DMs and players familiar with D&D 5e rules who need to track characters across campaign sessions
- **System Functionality**: 
  - Create new characters with standard D&D 5e options
  - View and edit character information
  - Track character progression through levels
  - Manage inventory and equipment
  - View character sheets
- **User Interfaces**: 
  - Character creation wizard/form
  - Character list view for campaigns
  - Character sheet display
  - Inventory management interface

### System Requirements
- **Functional Requirements**:
  - REQ-001: System shall allow character creation with race, class, background, and ability score selection
  - REQ-002: System shall support character leveling with HP rolls and ASI/feat choices
  - REQ-003: System shall support multiclassing per D&D 5e rules
  - REQ-004: System shall track character inventory with add/remove/equip functionality
  - REQ-005: System shall store character data as JSON in SQLite database
  - REQ-006: System shall associate characters with campaigns and players
  - REQ-007: System shall support player management (create, edit, associate with campaigns)
  - REQ-008: System shall derive proficiencies from race/class/background choices
  - REQ-009: System shall track hit points (current and maximum)
  - REQ-010: System shall track experience points and character level
  - REQ-011: System shall support feat selection at ASI levels with feat stored from catalog
  - REQ-012: System shall track known/prepared spells for spellcasting classes
  - REQ-013: System shall calculate spell slots based on class and level
  - REQ-014: System shall track current and maximum spell slots per spell level
  - REQ-015: System shall enforce spell preparation limits based on class rules
  - REQ-016: System shall create versioned character files on level-up and manual snapshots
  - REQ-017: System shall store character data in database as YAML/JSON text
  - REQ-018: System shall generate human-readable markdown character sheets from database data (no frontmatter)
  - REQ-019: System shall organize character files in campaign folders following pattern: `characters/<name>/<name>-###.md`
  
- **Non-Functional Requirements**:
  - NFR-001: Character creation should complete within 2 seconds
  - NFR-002: Character data must persist across application restarts
  - NFR-003: Character JSON schema must be extensible for future features
  - NFR-004: System must validate character choices against D&D 5e rules in catalog

## Use Cases **[CONDITIONAL: User-Facing Initiative]**

### Use Case 1: Create New Character
- **Actor**: Player or DM
- **Scenario**: 
  1. User navigates to campaign view
  2. Clicks "Create Character" button
  3. Enters character name and player name
  4. Selects race from catalog (optionally subrace)
  5. Selects class from catalog
  6. Selects background from catalog
  7. Assigns ability scores (rolling, standard array, or point buy)
  8. Chooses skill proficiencies based on class/background
  9. Reviews character summary
  10. Confirms creation
- **Expected Outcome**: Character is created and saved to database, user is redirected to character sheet view

### Use Case 2: Level Up Character
- **Actor**: Player or DM
- **Scenario**:
  1. User opens character sheet
  2. Clicks "Level Up" button
  3. System determines if multiclassing or continuing current class
  4. User rolls for HP or takes average
  5. If ASI/feat level (4, 8, 12, 16, 19), user chooses ability score increases or feat
  6. If subclass selection level, user chooses subclass
  7. System updates character level, HP, and abilities
  8. Confirms level up
- **Expected Outcome**: Character level increases, HP updated, ASI/feat applied, character data persisted

### Use Case 3: Manage Inventory
- **Actor**: Player or DM
- **Scenario**:
  1. User opens character sheet
  2. Navigates to inventory section
  3. Adds new items with name, quantity, weight
  4. Updates quantities for existing items
  5. Removes items as needed
  6. Equips items to specific slots (armor, shield, weapons)
- **Expected Outcome**: Inventory changes are saved to character data, equipped items are tracked

### Use Case 4: View Character List
- **Actor**: DM
- **Scenario**:
  1. User opens campaign
  2. Views list of all characters in campaign
  3. Sees character names, classes, levels, and player names
  4. Clicks on character to view full sheet
- **Expected Outcome**: User can see all characters associated with the campaign

## Architecture **[CONDITIONAL: Technically Complex Initiative]**

### Overview
The character system follows Mimir's existing layered architecture:
- **Database Layer**: SQLite with new `characters` table
- **Core Layer**: Character models, data structures, and business logic
- **Service Layer**: CharacterService for character operations
- **Command Layer**: Tauri commands exposing character functionality to frontend
- **Frontend Layer**: Vue components for character UI

### Component Structure

**Backend (Rust) - Core Services (Tool-Accessible)**:
```
crates/mimir-dm-core/
  src/
    models/
      player/
        mod.rs              - Player and CampaignPlayer models
      character/
        mod.rs              - Character and CharacterVersion models (metadata)
        data.rs             - CharacterData struct for YAML parsing
        inventory.rs        - Inventory and equipment types
        abilities.rs        - Ability scores and derived values
        spells.rs           - Spell tracking types
        proficiencies.rs    - Proficiency types
    services/
      player_service.rs     - Player management (CRUD, campaign association)
      character_service.rs  - Character management (core business logic)
        Methods (tool-accessible):
          - create_character(campaign_id, player_id, creation_data) -> Character
          - get_character(character_id) -> Character
          - get_character_version(character_id, version) -> CharacterData
          - list_campaign_characters(campaign_id) -> Vec<Character>
          - level_up_character(character_id, level_up_choices) -> CharacterVersion
          - create_snapshot(character_id, reason) -> CharacterVersion
          - update_inventory(character_id, inventory_update) -> CharacterVersion
          - update_spells(character_id, spell_update) -> CharacterVersion
          - parse_character_file(file_path) -> CharacterData
          - generate_character_markdown(character_data) -> String
    schema.rs              - Add players, campaign_players, characters, character_versions tables
```

**Access Patterns**:
- **GUI**: Tauri commands → CharacterService methods
- **Chat Tools**: LLM tool definitions → CharacterService methods (same interface)
- **Shared Logic**: All business rules, validations, and file operations in service layer

**Frontend (Vue/TypeScript)**:
```
src/
  components/
    character/
      CharacterCreator.vue     - Character creation form
      CharacterSheet.vue       - Character sheet display
      CharacterList.vue        - List characters in campaign
      InventoryManager.vue     - Inventory management
      LevelUpDialog.vue        - Level up interface
  stores/
    characterStore.ts          - Character state management
  types/
    character.ts               - TypeScript interfaces
```

### Data Model

**players table**:
- id (INTEGER PRIMARY KEY)
- name (TEXT, required)
- email (TEXT, nullable)
- notes (TEXT, nullable)
- created_at (TIMESTAMP)

**campaign_players table** (join table):
- id (INTEGER PRIMARY KEY)
- campaign_id (INTEGER FK to campaigns)
- player_id (INTEGER FK to players)
- joined_at (TIMESTAMP)
- active (BOOLEAN, default true)

**characters table** (minimal metadata):
- id (INTEGER PRIMARY KEY)
- campaign_id (INTEGER FK to campaigns)
- player_id (INTEGER FK to players)
- character_name (TEXT, required)
- current_level (INTEGER, default 1)
- current_version (INTEGER, default 1)
- directory_path (TEXT, required) - e.g., "characters/Thorin_Ironforge/"
- created_at (TIMESTAMP)
- last_updated_at (TIMESTAMP)

**character_versions table** (version tracking with data storage):
- id (INTEGER PRIMARY KEY)
- character_id (INTEGER FK to characters)
- version_number (INTEGER, required)
- file_path (TEXT, required) - e.g., "characters/Thorin_Ironforge/Thorin_Ironforge-003.md"
- character_data (TEXT, required) - YAML/JSON blob of complete character state (source of truth)
- snapshot_reason (TEXT, nullable) - "Level up", "Manual snapshot", etc.
- level (INTEGER, required)
- created_at (TIMESTAMP)

**Character File Structure** (`characters/<name>/<name>-###.md`):

Each character version is stored in the database as YAML/JSON text. A corresponding markdown file is generated for human-readable viewing (no frontmatter, pure presentation).

**File Organization:**
```
campaigns/My_Campaign/
  characters/
    Thorin_Ironforge/
      Thorin_Ironforge-001.md  # Initial creation (level 1)
      Thorin_Ironforge-002.md  # After first session updates
      Thorin_Ironforge-003.md  # Level up to 3
    Elara_Moonwhisper/
      Elara_Moonwhisper-001.md  # Initial creation
      Elara_Moonwhisper-002.md  # Level up to 2
```

**Character Data Structure** (stored in `character_versions.character_data` as YAML/JSON):
```yaml
---
# Metadata
character_name: Thorin Ironforge
player_id: 1
level: 3
experience_points: 900
version: 3
snapshot_reason: "Leveled up to 3, chose Champion subclass"
created_at: 2025-01-15T10:30:00Z

# Core Identity
race: Mountain Dwarf
subrace: null
class: Fighter
subclass: Champion
background: Soldier
alignment: Lawful Good

# Abilities
abilities:
  strength: 16
  dexterity: 12
  constitution: 16
  intelligence: 10
  wisdom: 13
  charisma: 8

# HP and Resources
max_hp: 28
current_hp: 28
hit_dice_remaining: 3
hit_dice_type: d10

# Proficiencies
proficiencies:
  skills: [Athletics, Intimidation, Perception, Survival]
  saves: [Strength, Constitution]
  armor: [All armor, shields]
  weapons: [Simple weapons, martial weapons]
  tools: [Smith's tools, Vehicles (land)]
  languages: [Common, Dwarvish]

# Class Features
class_features:
  - Fighting Style (Defense)
  - Second Wind
  - Action Surge
  - Improved Critical

# Feats
feats: []

# Spells (if applicable)
spells:
  known_spells: []
  prepared_spells: []
  cantrips: []
  spell_slots:
    1: {max: 0, current: 0}
    2: {max: 0, current: 0}

# Inventory
inventory:
  - name: Rations
    quantity: 10
    weight: 20.0
    value: 5.0
    notes: null
  - name: Healing Potion
    quantity: 2
    weight: 1.0
    value: 50.0
    notes: Greater healing

# Equipment
equipped:
  armor: Chain Mail
  shield: Shield
  main_hand: Warhammer
  off_hand: null

# Personality
personality:
  traits: I'm always polite and respectful. I face problems head-on.
  ideals: Responsibility. I do what I must and obey authority.
  bonds: I would still lay down my life for the people I served with.
  flaws: I obey authority without question, even when it's wrong.

```

**Generated Markdown File** (pure presentation, no frontmatter):
```markdown
# Thorin Ironforge - Level 3 Fighter (Champion)
**Player:** Bjorn Ironside  
**Version:** 3 (Leveled up to 3, chose Champion subclass)  
**Created:** 2025-01-15

## Ability Scores
| STR | DEX | CON | INT | WIS | CHA |
|-----|-----|-----|-----|-----|-----|
| 16 (+3) | 12 (+1) | 16 (+3) | 10 (+0) | 13 (+1) | 8 (-1) |

## Combat Stats
- **HP:** 28 / 28
- **Hit Dice:** 3d10 remaining
- **Proficiency Bonus:** +2

## Proficiencies
**Skills:** Athletics, Intimidation, Perception, Survival  
**Saves:** Strength, Constitution  
**Armor:** All armor, shields  
**Weapons:** Simple weapons, martial weapons  
**Tools:** Smith's tools, Vehicles (land)  
**Languages:** Common, Dwarvish

## Class Features
- Fighting Style (Defense)
- Second Wind
- Action Surge
- Improved Critical

## Equipment
**Equipped:**
- Armor: Chain Mail
- Shield: Shield  
- Main Hand: Warhammer

**Inventory:**
| Item | Qty | Weight | Value | Notes |
|------|-----|--------|-------|-------|
| Rations | 10 | 20 lbs | 5 gp | |
| Healing Potion | 2 | 1 lb | 50 gp | Greater healing |

## Personality
**Traits:** I'm always polite and respectful. I face problems head-on.  
**Ideals:** Responsibility. I do what I must and obey authority.  
**Bonds:** I would still lay down my life for the people I served with.  
**Flaws:** I obey authority without question, even when it's wrong.
```

**Data Flow:**
- **Storage:** Character data stored in `character_versions.character_data` column (YAML/JSON text)
- **Generation:** On save, generate markdown file from database data
- **Loading (for editing):** Parse YAML/JSON from database column
- **Viewing (for players):** Open generated .md file
- **One-Way:** Database → Generated Markdown → File System → Players (read-only)

### Key Design Decisions
1. **Backend-First Architecture**: All character logic in core services with clear interfaces for both GUI and chat tool access
2. **Tool-Accessible Design**: Character operations exposed as discrete service methods callable from Tauri commands OR chat tools
3. **Database Storage with Generated Markdown**: Character data stored in database as YAML/JSON text (source of truth), with generated markdown files for presentation
4. **Version Tracking**: Each character snapshot creates a new numbered file (`<name>-###.md`) to track evolution over time
5. **Database as Source of Truth**: Database stores metadata in characters table, full character state in character_versions.character_data as YAML/JSON text
6. **Player Management**: Global players table with campaign_players join table for flexibility
7. **Campaign Folder Integration**: Characters stored in `campaigns/<name>/characters/<character_name>/` following existing Mimir patterns
8. **One-Way Data Flow**: Database → Generated Markdown Files → Players (read-only consumption)
9. **Versioning Triggers**: New versions created on level-up (automatic) or manual snapshot (user-triggered)
10. **Separate Tables**: Characters use dedicated tables instead of documents table due to distinct workflow and relationships

## Detailed Design **[REQUIRED]**

### Phase 1: Database Schema and Core Models
1. Create migration for `characters` table
2. Add characters table to schema.rs
3. Create Character model with Diesel integration
4. Create CharacterData struct with all nested types (Abilities, ClassLevel, InventoryItem, EquippedItems)
5. Implement JSON serialization/deserialization

### Phase 2: Character Service Layer
1. Create CharacterService in mimir-dm-core
2. Implement character CRUD operations:
   - create_character() - validates choices against catalog, derives proficiencies
   - get_character() - retrieves by ID with JSON parsing
   - get_campaign_characters() - lists all characters for a campaign
   - update_character() - saves character changes
   - delete_character() - removes character
3. Implement character creation logic:
   - Validate race/class/background exist in catalog
   - Calculate starting HP from class hit die + CON modifier
   - Derive proficiencies from race/class/background
   - Apply racial ability bonuses
   - Set starting equipment based on class/background
4. Implement level up logic:
   - Handle HP increase (roll or average)
   - Apply ASI at levels 4/8/12/16/19
   - Support multiclassing rules
   - Track subclass selection at appropriate levels
5. Implement inventory management:
   - Add/remove items
   - Update quantities
   - Equip/unequip to slots
   - Track weight (optional)

### Phase 3: Tauri Command Layer
1. Create character commands module
2. Expose commands to frontend:
   - create_character
   - get_character
   - get_campaign_characters
   - update_character
   - level_up_character
   - update_character_inventory
   - delete_character

### Phase 4: Frontend Implementation
1. Create TypeScript interfaces matching Rust types
2. Create character store with Pinia
3. Implement CharacterCreator.vue:
   - Multi-step form (basic info → race → class → background → abilities → skills)
   - Load catalog options from existing catalog API
   - Ability score assignment (manual, standard array, or point buy)
   - Skill selection based on class/background choices
4. Implement CharacterList.vue:
   - Display characters in a campaign
   - Show key stats (name, class, level, player)
   - Navigate to character sheet
5. Implement CharacterSheet.vue:
   - Display all character information
   - Show abilities, proficiencies, equipment
   - Access to inventory and level up
6. Implement InventoryManager.vue:
   - Add/remove items
   - Update quantities
   - Equip items to slots
7. Implement LevelUpDialog.vue:
   - HP roll or average selection
   - ASI/feat selection at appropriate levels
   - Subclass selection when available
   - Multiclass option

### Phase 5: Integration
1. Add character section to campaign view
2. Wire up navigation between character list and sheets
3. Test character creation flow end-to-end
4. Test level up functionality
5. Test inventory management
6. Handle edge cases and validation

## UI/UX Design **[CONDITIONAL: Frontend Initiative]**

### User Interface Components

**CharacterCreator.vue**:
- Multi-step wizard interface
- Progress indicator showing current step
- Form sections: Basic Info → Race → Class → Background → Abilities → Skills → Review
- Catalog dropdowns for race/class/background selection
- Ability score input (number inputs or drag-drop for point buy)
- Skill checkboxes filtered by available choices
- Navigation: Next/Previous/Cancel buttons

**CharacterList.vue**:
- Table or card grid showing characters
- Columns: Character Name, Player Name, Race, Class, Level
- "Create Character" button at top
- Click row to view character sheet
- Optional: Filter/sort by level, class, player

**CharacterSheet.vue**:
- Header: Character name, class/level, player name
- Main sections:
  - Abilities (STR, DEX, CON, INT, WIS, CHA with modifiers)
  - Combat stats (HP, AC, proficiency bonus)
  - Proficiencies (skills, saves, armor, weapons, tools, languages)
  - Feats (list of acquired feats)
  - Spells (if spellcaster):
    - Spell slots by level (current/max with recovery button)
    - Known spells list
    - Prepared spells (highlighted/checkboxes)
    - Cantrips
  - Equipment (equipped items)
  - Inventory (collapsible list)
  - Class features (read-only list)
- Action buttons: "Level Up", "Edit", "Manage Inventory", "Manage Spells" (if spellcaster)

**InventoryManager.vue**:
- List of inventory items with quantity and weight
- Add item form (name, quantity, weight, notes)
- Delete button per item
- Equip/unequip buttons for appropriate items
- Equipment slots display (armor, shield, main hand, off hand)

**LevelUpDialog.vue**:
- Modal dialog
- Shows current level → new level
- HP roll section (roll button or "take average" checkbox)
- ASI/Feat section (if applicable):
  - Radio buttons: "Ability Score Improvement" or "Feat"
  - If ASI: two dropdowns to select abilities
  - If Feat: dropdown of available feats (future)
- Subclass selection (if applicable level)
- Multiclass option (checkbox and class dropdown)
- "Confirm Level Up" button

### User Flows

**Character Creation Flow**:
1. Campaign view → "Create Character" button
2. Step 1: Enter character name, player name → Next
3. Step 2: Select race (and subrace if applicable) → Next
4. Step 3: Select class → Next
5. Step 4: Select background → Next
6. Step 5: Assign ability scores → Next
7. Step 6: Choose skill proficiencies → Next
8. Step 7: Review character → Confirm
9. Redirect to character sheet

**Level Up Flow**:
1. Character sheet → "Level Up" button
2. Level up dialog opens
3. Roll HP or take average
4. If ASI level: choose ASI or feat
5. If subclass level: choose subclass
6. Confirm → Character sheet updates

**Inventory Management Flow**:
1. Character sheet → "Manage Inventory" button
2. Inventory manager opens
3. Add/remove/equip items
4. Close → Return to character sheet

### Design System Integration
- Use existing Mimir component library (buttons, inputs, modals)
- Match campaign management UI patterns
- Use consistent typography and spacing
- Follow existing color scheme for primary/secondary actions
- Integrate with existing navigation structure

## Testing Strategy **[CONDITIONAL: Separate Testing Initiative]**

### Unit Testing
- **Strategy**: Test core character logic in isolation
- **Key Areas**:
  - Character creation validation
  - HP calculation (starting and level up)
  - Proficiency derivation from race/class/background
  - ASI application logic
  - Inventory operations
  - JSON serialization/deserialization
- **Tools**: Rust built-in test framework, cargo test

### Integration Testing
- **Strategy**: Test character service with database
- **Test Environment**: In-memory SQLite database for tests
- **Key Areas**:
  - CRUD operations on characters table
  - Character creation with catalog lookups
  - Level up with state persistence
  - Inventory updates with state persistence
- **Data Management**: Test fixtures with sample races/classes/backgrounds

### Manual Testing
- **User Flows**: Test all use cases end-to-end through UI
- **Edge Cases**:
  - Invalid race/class/background selections
  - Multiclassing at level 2
  - ASI at levels 4, 8, 12, 16, 19
  - Subclass selection at appropriate levels
  - Maximum ability scores (20 cap)
  - Inventory weight limits (if implemented)
  - Delete character with confirmation

### Test Selection
Focus testing on:
1. Character creation flow (most critical path)
2. Level up logic (complex business rules)
3. Data persistence (ensure no data loss)
4. Catalog integration (validate against real D&D data)

### Bug Tracking
- Use GitHub issues for bug tracking
- Priority levels: Critical (data loss), High (broken functionality), Medium (UX issues), Low (cosmetic)
- Test in phases as features are completed

## Alternatives Considered **[REQUIRED]**

### Alternative 1: Fully Normalized Database Schema
**Approach**: Store character data across multiple related tables (character_classes, character_inventory, character_proficiencies, etc.)

**Rejected Because**:
- Conflicts with ADR-001 (Denormalized Database Design)
- Adds complexity for character data that doesn't need relational queries
- Character data is primarily read/written as a unit
- JSON approach provides better flexibility for future D&D rule expansions

### Alternative 2: Full Character Sheet Builder
**Approach**: Implement comprehensive D&D 5e character creation with all options (all feats, spell management, complex class features)

**Rejected Because**:
- Scope too large for initial implementation
- Many features (spells, feats) require additional catalog infrastructure
- Can be added incrementally after core system is working
- MVP focuses on essential character tracking, not complete rulebook implementation

### Alternative 3: Character Import from D&D Beyond/Roll20
**Approach**: Allow users to import existing characters from other platforms

**Rejected Because**:
- Requires parsing external formats and APIs
- Not essential for initial MVP
- Can be added as enhancement after core system works
- Manual character creation is sufficient for launch

### Alternative 4: Calculated Fields in Database
**Approach**: Store derived values (modifiers, proficiency bonus, AC) as database columns

**Rejected Because**:
- Increases chance of data inconsistency
- D&D rules can change with errata/houserules
- Better to calculate on-demand from base values
- Simplifies database schema

### Alternative 5: Separate Service per Operation
**Approach**: Create separate services for CharacterCreation, CharacterLeveling, InventoryManagement

**Rejected Because**:
- Character operations are tightly coupled
- Single CharacterService is simpler and follows existing Mimir patterns
- All operations share same data model and validation logic
- Can refactor later if service becomes too large

## Implementation Plan **[REQUIRED]**

### Phase 1: Foundation (Database & Models)
**Tasks**:
- Create database migrations for players, campaign_players, characters, and character_versions tables
- Add new tables to schema.rs
- Create Player model with Diesel integration
- Create Character model with Diesel integration (minimal metadata)
- Create CharacterVersion model with Diesel integration
- Create CharacterData struct for YAML parsing (Abilities, ClassLevel, InventoryItem, EquippedItems, Proficiencies, Spells)
- Implement YAML serialization/deserialization (using serde_yaml)
- Implement markdown template renderer for character sheets
- Write unit tests for models and YAML parsing

**Dependencies**: None
**Estimated Effort**: Medium

### Phase 2: Player and Character Services (Backend Logic - Tool-Accessible Core)
**Tasks**:
- Create PlayerService with database connection
  - Implement player CRUD operations
  - Implement campaign player association
  - **Design for dual access**: Methods callable from both Tauri commands and chat tools
- Create CharacterService with database connection
  - **Design principle**: Pure business logic, no UI dependencies
  - Implement character CRUD operations (metadata only)
  - Implement character file management (create directory, write versioned files)
  - Implement character data serialization to YAML/JSON for database storage
  - Implement character data deserialization from YAML/JSON
  - Implement markdown generation from character data (no frontmatter)
- Implement character creation logic:
  - Validate race/class/background against catalog
  - Calculate starting HP from class hit die + CON modifier
  - Derive proficiencies from race/class/background
  - Apply racial ability bonuses
  - Set starting equipment based on class/background
  - Create initial version file (###-001.md)
- Implement level up logic:
  - Handle HP increase (roll or average)
  - Apply ASI at levels 4/8/12/16/19
  - Support multiclassing rules
  - Track subclass selection at appropriate levels
  - Create new version file
- Implement feat selection and tracking
- Implement spell management (known, prepared, slots)
- Implement spell slot calculation based on class/level
- Implement inventory management
- Implement manual snapshot creation
- **Document service API**: Clear method signatures and return types for tool integration
- Write unit and integration tests for services

**Dependencies**: Phase 1
**Estimated Effort**: X-Large

### Phase 3: Command and Tool Integration Layer
**Tasks**:
- Create player commands module (Tauri):
  - create_player, get_player, update_player, delete_player
  - get_campaign_players, add_player_to_campaign, remove_player_from_campaign
- Create character commands module (Tauri):
  - create_character, get_character, get_campaign_characters
  - level_up_character, create_character_snapshot
  - update_character_inventory, update_character_spells
  - get_character_versions, get_character_version_file
- **Create chat tool definitions** (for LLM access):
  - Define tool schemas that map to CharacterService methods
  - Implement tool handlers that call same service methods as Tauri commands
  - Tools: create_character, level_up_character, update_inventory, etc.
- Add commands to main.rs
- Test commands with Tauri dev tools
- **Test tool integration** with chat interface

**Dependencies**: Phase 2
**Estimated Effort**: Medium

### Phase 4: Frontend Foundation (Types & Stores)
**Tasks**:
- Create TypeScript interfaces matching Rust types:
  - Player, CampaignPlayer
  - Character, CharacterVersion
  - CharacterData (YAML structure)
- Create player store with Pinia:
  - Player CRUD operations
  - Campaign player management
- Create character store with Pinia:
  - Character CRUD operations
  - Version management
  - Character data manipulation
- Add error handling

**Dependencies**: Phase 3
**Estimated Effort**: Small

### Phase 5: Player and Character Management UI
**Tasks**:
- Create PlayerManager.vue:
  - Player list view
  - Create/edit player form
  - Associate players with campaigns
- Create CharacterCreator.vue with multi-step form:
  - Step 1: Player selection (existing or new)
  - Step 2: Character name and basic info
  - Step 3: Race selection from catalog (with subrace)
  - Step 4: Class selection from catalog
  - Step 5: Background selection from catalog
  - Step 6: Ability score assignment (manual, standard array, point buy)
  - Step 7: Skill proficiency selection
  - Step 8: Starting equipment
  - Step 9: Personality traits (optional)
  - Step 10: Review and confirm
- Integrate with catalog API for race/class/background
- Add form validation
- Test character creation flow

**Dependencies**: Phase 4
**Estimated Effort**: Large

### Phase 6: Character Display UI
**Tasks**:
- Create CharacterList.vue:
  - Display characters in campaign with player, class, level
  - Filter by player
  - Version history dropdown per character
  - "Create Character" button
- Create CharacterSheet.vue:
  - Display current character version
  - Show all character data (abilities, proficiencies, inventory, spells, feats)
  - Version selector dropdown
  - "Level Up", "Save Snapshot", "Manage Inventory", "Manage Spells" buttons
  - Link to open markdown file
- Add navigation between list and sheet
- Integrate with campaign view

**Dependencies**: Phase 5
**Estimated Effort**: Medium

### Phase 7: Character Progression UI
**Tasks**:
- Create LevelUpDialog.vue
- Implement HP roll/average selection
- Implement ASI/feat selection with feat catalog integration
- Implement subclass selection
- Implement multiclass option
- Test level up flow

**Dependencies**: Phase 6
**Estimated Effort**: Medium

### Phase 7b: Spell Management UI
**Tasks**:
- Create SpellManager.vue component
- Implement spell selection from catalog for known spells
- Implement spell preparation interface
- Implement spell slot tracking (current/max per level)
- Add spell slot recovery (short/long rest)
- Display cantrips separately
- Integrate spell UI into character sheet
- Test spell management flow

**Dependencies**: Phase 6
**Estimated Effort**: Medium

### Phase 8: Inventory Management UI
**Tasks**:
- Create InventoryManager.vue
- Implement add/remove/update items
- Implement equip/unequip functionality
- Test inventory management flow

**Dependencies**: Phase 6
**Estimated Effort**: Small

### Phase 9: Testing & Polish
**Tasks**:
- End-to-end testing of all flows (creation, leveling, inventory, spells)
- Edge case testing (spell slot limits, feat restrictions, multiclassing spell slots)
- Bug fixes
- UI polish and consistency
- Documentation

**Dependencies**: Phases 5-8
**Estimated Effort**: Medium

### Overall Timeline
- Total Estimated Effort: ~5-6 weeks of focused development
- Can be parallelized: Frontend work (Phases 4-8) can start once Phase 3 is complete
- Critical path: Phase 1 → Phase 2 → Phase 3
- Incremental delivery: Each phase produces testable artifacts
- New additions vs. original scope:
  - Player management system (+3-4 days)
  - YAML/markdown file generation (+4-5 days)
  - Character versioning system (+3-4 days)
  - Spell/feat features (+1 week)