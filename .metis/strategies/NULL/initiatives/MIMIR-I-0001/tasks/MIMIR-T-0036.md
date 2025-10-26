---
id: phase-3-standardize-frontend-error
level: task
title: "Phase 3: Standardize frontend error handling patterns by layer"
short_code: "MIMIR-T-0036"
created_at: 2025-10-26T11:21:04.910179+00:00
updated_at: 2025-10-26T11:21:04.910179+00:00
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

# Phase 3: Standardize frontend error handling patterns by layer

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective **[REQUIRED]**

Standardize error handling patterns in the TypeScript frontend by layer (services, composables, components, stores), ensuring consistent error propagation and user-facing error messages. This creates a maintainable and predictable error handling strategy across the application.

Part of error handling standardization effort (see MIMIR-T-0025 for full design). Can be done independently of or in parallel with MIMIR-T-0035.

### Type
- [ ] Bug - Production issue that needs fixing
- [ ] Feature - New functionality or enhancement  
- [x] Tech Debt - Code improvement or refactoring
- [ ] Chore - Maintenance or setup work

### Priority
- [ ] P0 - Critical (blocks users/revenue)
- [ ] P1 - High (important for user experience)
- [x] P2 - Medium (nice to have)
- [ ] P3 - Low (when time permits)

### Technical Debt Impact

**Current Problems**:
- 68+ files with try/catch blocks using inconsistent patterns
- Some services wrap errors, some don't, some swallow errors
- No clear guidelines on where errors should be handled
- Difficult to trace errors through the application layers
- Inconsistent user-facing error messages

**Benefits of Fixing**:
- Consistent error handling across all layers
- Clear guidelines for where to catch vs propagate
- Better error messages with appropriate context at each layer
- Easier debugging with predictable error flow
- Single pattern to follow for new code
- Foundation for centralized error tracking/logging

**Risk Assessment**:
- Low risk: Internal refactoring, no user-facing behavior changes
- Incremental approach: Can update one layer/feature at a time
- Easy to test: Verify error messages still display correctly
- Rollback friendly: Changes are localized to individual files

## Acceptance Criteria **[REQUIRED]**

- [ ] Error handling guidelines documented (which layer catches vs propagates)
- [ ] Service layer: All services wrap errors with operation context
- [ ] Composable layer: Minimal error handling, mostly propagate
- [ ] Component layer: Catch and display user-friendly messages
- [ ] Store layer: Critical ops throw, non-critical log and continue
- [ ] Services updated: ModuleService, DocumentService, CampaignService, SessionService
- [ ] Services updated: SearchService and catalog services
- [ ] Stores updated: sharedContext, theme, chat stores
- [ ] No changes to error message content (maintain user experience)
- [ ] TypeScript compilation successful with no errors
- [ ] Manual testing of error scenarios across updated files

## Implementation Notes

### Error Handling Guidelines by Layer

**1. Service Layer** (ModuleService, CampaignService, etc.)
- **Always wrap errors** with operation context
- **Pattern**: `throw new Error(\`Failed to ${operation} ${resource}: ${error}\`)`
- **Example**:
```typescript
async get(id: number): Promise<Module> {
  try {
    const response = await invoke<{ data: Module }>('get_module', { id })
    return response.data
  } catch (error) {
    throw new Error(`Failed to get module ${id}: ${error}`)
  }
}
```
- **Why**: Services are the boundary between frontend and backend - add context here

**2. Composable Layer** (useModuleStage, useCatalog, etc.)
- **Minimal error handling** - let errors propagate
- **Only catch if**: Adding valuable context or handling specially
- **Pattern**: Usually no try/catch
- **Example**:
```typescript
async function transitionToNextStage() {
  // No try/catch - let error propagate to component with service context
  await ModuleService.updateStatus(module.value.id, nextStageKey)
  window.location.reload()
}
```
- **Why**: Composables are business logic - let service errors bubble up

**3. Component Layer** (Vue components)
- **Always catch errors** from async operations
- **Display user-friendly messages**
- **Pattern**: Show error in UI (toast, alert, error message)
- **Example**:
```typescript
async function handleTransition() {
  try {
    await moduleComposable.transitionToNextStage()
    successMessage.value = 'Module transitioned successfully'
  } catch (error) {
    errorMessage.value = error.message || 'An error occurred'
  }
}
```
- **Why**: Components are user-facing - handle and display errors here

**4. Store Layer** (Pinia stores)
- **Critical operations**: Throw errors
- **Non-critical operations**: Log and continue
- **Pattern**: Decide based on operation criticality
- **Example - Critical**:
```typescript
async updateCampaign(data: CampaignContext) {
  try {
    await syncToBackend('campaign', data)
  } catch (error) {
    throw new Error(`Failed to update campaign: ${error}`)
  }
}
```
- **Example - Non-Critical**:
```typescript
async syncMetrics() {
  try {
    await invoke('sync_metrics')
  } catch (error) {
    console.warn('Failed to sync metrics:', error)
    // Don't throw - metrics sync is non-critical
  }
}
```
- **Why**: Stores manage state - some operations are critical, others aren't

### Implementation Strategy

**Phase 1: Create Guidelines Document**
1. Create `docs/ERROR_HANDLING.md` with patterns above
2. Include examples for each layer
3. Document when to catch vs propagate

**Phase 2: Update Services** (incremental)
1. ModuleService.ts - ensure all methods wrap errors
2. CampaignService.ts - add context to errors
3. SessionService.ts - standardize error messages
4. DocumentService.ts - consistent wrapping
5. SearchService.ts - add try/catch where missing
6. Catalog services (spells, items, monsters, etc.)

**Phase 3: Review Composables**
1. Audit composables for unnecessary try/catch
2. Remove error handling that just re-throws
3. Keep only valuable context additions

**Phase 4: Review Stores**
1. Identify critical vs non-critical operations
2. Ensure critical ops throw errors
3. Ensure non-critical ops log and continue

**Phase 5: Spot-Check Components**
1. Verify components catch and display errors
2. Ensure error messages are user-friendly
3. Check for proper error state management

### Testing Strategy
- Manual testing of error scenarios in each updated area
- Verify error messages are clear and helpful
- Ensure no user-facing regressions
- Check that errors propagate to UI correctly

### Dependencies

- **Not blocked by**: MIMIR-T-0034 or MIMIR-T-0035 (independent)
- **Can run in parallel with**: Phase 2 (command updates)
- **No external dependencies**: All frontend work

### Risk Considerations

**Very Low Risk**:
- Changes are internal to error handling logic
- No changes to success paths or business logic
- Error messages remain the same (just more consistent)
- Can be done incrementally one file at a time

**Mitigation**:
- Start with documentation/guidelines
- Update one service/layer at a time
- Test each area after updates
- Easy rollback if issues arise

## Status Updates **[REQUIRED]**

*To be added during implementation*