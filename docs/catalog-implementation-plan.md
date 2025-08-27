# Catalog Implementation Plan

## Overview
This document tracks the implementation of remaining catalog sections for the Mimir DM application. The catalog provides searchable reference content from D&D 5e sources.

## Current Implementation Status

### ✅ Fully Implemented (5 types)
These catalog types have complete implementation across all layers:

| Type | Backend File | Frontend Component | Status |
|------|--------------|-------------------|---------|
| Spells | `catalog.rs` | `SpellTable.vue` | ✅ Complete |
| Items | `catalog.rs` | `ItemTable.vue` | ✅ Complete |
| Monsters | `catalog.rs` | `MonsterTable.vue` | ✅ Complete |
| Classes | `catalog_class.rs` | `ClassTable.vue` | ✅ Complete |
| Feats | `catalog_feat.rs` | `FeatTable.vue` | ✅ Complete |

### 📦 Data Extracted but Not Implemented (15 types)
These types are already being extracted by the splitter but lack UI/backend implementation:

| Type | Priority | Splitter Function | Status |
|------|----------|------------------|---------|
| Races | High | `collect_filtered_races()` | ⏳ Pending |
| Backgrounds | High | `collect_filtered_backgrounds()` | ⏳ Pending |
| Optional Features | High | `collect_filtered_optfeatures()` | ⏳ Pending |
| Actions | Medium | `collect_filtered_generic()` | ⏳ Pending |
| Conditions/Diseases | Medium | `collect_filtered_generic()` | ⏳ Pending |
| Deities | Medium | `collect_filtered_generic()` | ⏳ Pending |
| Objects | Medium | `collect_filtered_generic()` | ⏳ Pending |
| Traps/Hazards | Medium | `collect_filtered_generic()` | ⏳ Pending |
| Languages | Low | `collect_filtered_generic()` | ⏳ Pending |
| Rewards | Low | `collect_filtered_generic()` | ⏳ Pending |
| Tables | Low | `collect_filtered_generic()` | ⏳ Pending |
| Variant Rules | Low | `collect_filtered_generic()` | ⏳ Pending |
| Vehicles | Low | `collect_filtered_generic()` | ⏳ Pending |
| Cults/Boons | Low | `collect_filtered_generic()` | ⏳ Pending |
| Psionics | Low | Not currently extracted | ⏳ Pending |

## Implementation Workflow

### Step 1: Data Pipeline Verification
**Location:** `crates/mimir-5etools-splitter/src/collector.rs`

For each catalog type:
- [ ] Verify data extraction in splitter
- [ ] Check if fluff data needs to be collected
- [ ] Test extraction with sample book (PHB)
- [ ] Verify output structure in archives
- [ ] Add/fix extraction if missing or broken

### Step 2: Core Models
**Location:** `crates/mimir-dm-core/src/models/catalog/`

- [ ] Create `[type].rs` file with:
  - Main data structures (matching 5etools schema)
  - Summary structure for list views
  - Parsing traits/implementations
  - Serde attributes for JSON serialization
- [ ] Add module exports to `catalog/mod.rs`
- [ ] Test deserialization with sample data

### Step 3: Backend Commands
**Location:** `crates/mimir-dm/src/commands/`

- [ ] Create `catalog_[type].rs` with:
  - Catalog state struct
  - `load_from_books_directory()` method
  - `search()` method with appropriate filters
  - `get_details()` method
  - Tauri command functions:
    - `initialize_[type]_catalog()`
    - `search_[types]()`
    - `get_[type]_details()`
- [ ] Register commands in `main.rs`
- [ ] Add state management in `main.rs`

### Step 4: Frontend Components
**Location:** `crates/mimir-dm/frontend/src/features/sources/components/search/`

- [ ] Create `[Type]Table.vue` component with:
  - Search input
  - Filter controls
  - Results table
  - Detail view modal/panel
- [ ] Add API bindings in `useCatalog.ts`
- [ ] Integrate into main catalog search interface
- [ ] Add appropriate styling

### Step 5: Testing & Validation
- [ ] Load test data from multiple sources
- [ ] Verify search functionality
- [ ] Test all filters
- [ ] Check detail views
- [ ] Performance testing with large datasets

## Implementation Priority & Schedule

### Phase 1: Character Creation Essentials (Week 1)
Essential for players creating characters.

#### 1. Races
- [ ] Data pipeline verification
- [ ] Core models (`race.rs`)
- [ ] Backend commands (`catalog_race.rs`)
- [ ] Frontend (`RaceTable.vue`)
- [ ] Testing

#### 2. Backgrounds
- [ ] Data pipeline verification
- [ ] Core models (`background.rs`)
- [ ] Backend commands (`catalog_background.rs`)
- [ ] Frontend (`BackgroundTable.vue`)
- [ ] Testing

#### 3. Optional Features
- [ ] Data pipeline verification
- [ ] Core models (`optionalfeature.rs`)
- [ ] Backend commands (`catalog_optionalfeature.rs`)
- [ ] Frontend (`OptionalFeatureTable.vue`)
- [ ] Testing

### Phase 2: Combat & Rules References (Week 2)
Core gameplay references for DMs and players.

#### 4. Actions
- [ ] Data pipeline verification
- [ ] Core models (`action.rs`)
- [ ] Backend commands (`catalog_action.rs`)
- [ ] Frontend (`ActionTable.vue`)
- [ ] Testing

#### 5. Conditions & Diseases
- [ ] Data pipeline verification
- [ ] Core models (`condition.rs`)
- [ ] Backend commands (`catalog_condition.rs`)
- [ ] Frontend (`ConditionTable.vue`)
- [ ] Testing

### Phase 3: World Building (Week 3)
DM tools for campaign creation.

#### 6. Deities
- [ ] Data pipeline verification
- [ ] Core models (`deity.rs`)
- [ ] Backend commands (`catalog_deity.rs`)
- [ ] Frontend (`DeityTable.vue`)
- [ ] Testing

#### 7. Objects
- [ ] Data pipeline verification
- [ ] Core models (`object.rs`)
- [ ] Backend commands (`catalog_object.rs`)
- [ ] Frontend (`ObjectTable.vue`)
- [ ] Testing

#### 8. Traps & Hazards
- [ ] Data pipeline verification
- [ ] Core models (`trap.rs`)
- [ ] Backend commands (`catalog_trap.rs`)
- [ ] Frontend (`TrapTable.vue`)
- [ ] Testing

### Phase 4: Additional Content (Week 4)
Lower priority but useful references.

#### 9-15. Remaining Types
- Languages
- Rewards
- Tables
- Variant Rules
- Vehicles
- Cults & Boons
- Psionics (requires adding to splitter)

## Technical Notes

### Data Structure Patterns
Most catalog types follow similar patterns:
```rust
// Main data structure
pub struct [Type] {
    pub name: String,
    pub source: String,
    pub page: Option<i32>,
    // Type-specific fields
}

// Summary for list views
pub struct [Type]Summary {
    pub name: String,
    pub source: String,
    // Key fields for display
}

// Container for JSON parsing
pub struct [Type]Data {
    pub [type]: Vec<[Type]>,
}
```

### Search Pattern
```rust
pub fn search(&self, 
    query: Option<String>,
    sources: Vec<String>,
    // Type-specific filters
) -> Vec<[Type]Summary>
```

### Common Filters
- **All types:** Name search, source filter
- **Races:** Size, ability scores
- **Backgrounds:** Skills, features
- **Actions:** Action type (action/bonus/reaction)
- **Conditions:** Type (condition/disease)
- **Objects:** Size, type
- **Traps:** Danger level, type

## Resources

### File Locations
- **5etools source data:** `/Users/dstorey/Desktop/colliery/mimir/data/5etools-2014-src-v1.210.46/data/`
- **Splitter code:** `/Users/dstorey/Desktop/colliery/mimir/crates/mimir-5etools-splitter/src/`
- **Book archives:** `/Users/dstorey/Desktop/colliery/mimir/data/books-output/`
- **Core models:** `/Users/dstorey/Desktop/colliery/mimir/crates/mimir-dm-core/src/models/catalog/`
- **Backend commands:** `/Users/dstorey/Desktop/colliery/mimir/crates/mimir-dm/src/commands/`
- **Frontend components:** `/Users/dstorey/Desktop/colliery/mimir/crates/mimir-dm/frontend/src/features/sources/components/search/`

### Testing Books
- **PHB** - Player's Handbook (core rules, good for all types)
- **XGE** - Xanathar's Guide (optional features, subclasses)
- **TCE** - Tasha's Cauldron (optional features, variant rules)
- **MM** - Monster Manual (creatures only)
- **DMG** - Dungeon Master's Guide (items, variant rules)

## Progress Tracking

### Overall Progress
- Total Types: 20
- Implemented: 5 (25%)
- In Progress: 0 (0%)
- Pending: 15 (75%)

### Last Updated
- Date: 2025-08-26
- Current Phase: Planning
- Next Action: Begin Phase 1 - Races implementation

## Notes & Decisions

### Design Decisions
1. **Unified Search Interface:** All catalog types will be accessible through a single search interface with type tabs
2. **Lazy Loading:** Catalogs are initialized on first access to reduce startup time
3. **Memory Caching:** All catalog data is loaded into memory for fast searching
4. **Source Filtering:** All types support filtering by source book

### Known Issues
- Psionics data not currently extracted by splitter
- Some fluff data (descriptions, lore) may need additional extraction
- Image references may need path adjustments

### Future Enhancements
- Full-text search across all content
- Advanced filtering UI
- Bookmarking/favorites system
- Quick reference cards
- Integration with campaign tools