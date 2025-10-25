---
id: split-usecatalog-ts-composable-by
level: task
title: "Split useCatalog.ts composable by entity type"
short_code: "MIMIR-T-0017"
created_at: 2025-10-24T11:53:49.767337+00:00
updated_at: 2025-10-24T11:53:49.767337+00:00
parent: MIMIR-I-0001
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0001
---

# Split useCatalog.ts composable by entity type

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective **[REQUIRED]**

Refactor the monolithic useCatalog.ts composable (1,873 lines) by splitting it into entity-specific composable files organized in a catalog/ subdirectory. This improves code navigability, maintainability, and follows the established pattern from previous refactoring tasks (T-0014, T-0015, T-0016).

## Type
- [x] Tech Debt - Code improvement or refactoring

## Technical Debt Impact
- **Current Problems**: 
  - Single 1,873-line file is difficult to navigate
  - Unrelated entity types are coupled in one file
  - Finding specific catalog functionality requires scrolling through entire file
  - Difficult to understand which parts of the catalog are being used
  
- **Benefits of Fixing**: 
  - Each entity type has its own focused file (<200 lines each)
  - Easier to find and modify specific catalog functionality
  - Clear separation of concerns by entity type
  - Consistent with other refactoring work in the codebase
  - Better developer experience when working with catalogs

- **Risk Assessment**: 
  - Low risk - composables are self-contained with clear interfaces
  - All usages can be updated to import from new locations
  - No runtime behavior changes, only file organization

## Acceptance Criteria **[REQUIRED]**

- [ ] Create composables/catalog/ directory structure
- [ ] Split useCatalog.ts into entity-specific files (useSpells.ts, useItems.ts, useMonsters.ts, etc.)
- [ ] Create index.ts that re-exports all catalog composables for backwards compatibility
- [ ] Update all imports throughout codebase to use new structure
- [ ] All entity-specific files are under 300 lines
- [ ] Application builds successfully without errors
- [ ] All catalog functionality works as before (no regressions)

## Implementation Notes **[CONDITIONAL: Technical Task]**

### Technical Approach

Following the pattern from previous refactoring tasks:

1. Create `composables/catalog/` directory
2. Split by entity type into separate files:
   - useSpells.ts - Spell catalog types and methods
   - useItems.ts - Item catalog types and methods
   - useMonsters.ts - Monster catalog types and methods
   - useClasses.ts - Class/Subclass catalog types and methods
   - useRaces.ts - Race/Subrace catalog types and methods
   - useFeats.ts - Feat catalog types and methods
   - useBackgrounds.ts - Background catalog types and methods
   - useActions.ts - Action catalog types and methods
   - useConditions.ts - Condition catalog types and methods
   - useOptionalFeatures.ts - Optional Feature catalog types and methods
   - useDeities.ts - Deity catalog types and methods
   - useObjects.ts - Object catalog types and methods
   - useTraps.ts - Trap catalog types and methods
   - useLanguages.ts - Language catalog types and methods
   - useRewards.ts - Reward catalog types and methods
   - useTables.ts - Table catalog types and methods
   - useVariantRules.ts - Variant Rule catalog types and methods
   - useVehicles.ts - Vehicle catalog types and methods
   - useCults.ts - Cult/Boon catalog types and methods
   - usePsionics.ts - Psionic catalog types and methods
3. Create index.ts to re-export everything
4. Update imports in components/views that use catalog functionality

### Dependencies
- Must maintain compatibility with all existing components that use useCatalog
- No changes to Rust backend required

### Risk Considerations
- Medium refactor scope but clear boundaries between entity types
- TypeScript compiler will catch any import issues
- Can verify no regressions by testing catalog functionality in UI

## Status Updates **[REQUIRED]**

*To be added during implementation*