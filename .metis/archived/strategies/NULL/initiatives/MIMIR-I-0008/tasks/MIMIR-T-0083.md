---
id: add-integration-tests-core-catalog
level: task
title: "Add integration tests: Core catalog services"
short_code: "MIMIR-T-0083"
created_at: 2025-11-24T20:28:58.395138+00:00
updated_at: 2025-11-24T23:56:43.302502+00:00
parent: MIMIR-I-0008
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0008
---

# Add integration tests: Core catalog services

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Add integration tests for core catalog services that are critical for character creation. These services currently have zero test coverage.

## Services in Scope

- `spell_service.rs` (462 lines) - Highest priority, used heavily in character creation
- `class_service.rs` (785 lines) - Complex search with subclass handling
- `race_service.rs` (168 lines) - Race selection for character creation
- `background_service.rs` - Background selection
- `feat_service.rs` - Feat selection and filtering

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] `spell_service` has integration tests covering search with filters
- [ ] `spell_service` has tests for edge cases (empty results, invalid filters)
- [ ] `class_service` has tests for base class and subclass queries
- [ ] `class_service` has tests for complex sorting/filtering
- [ ] `race_service` has integration tests for race catalog queries
- [ ] `background_service` has integration tests
- [ ] `feat_service` has integration tests
- [ ] All tests use the existing `TestDatabase` fixture pattern
- [ ] Tests include both success and error scenarios

## Implementation Notes

### Test File Locations
Create new test files following existing patterns:
- `crates/mimir-dm-core/tests/integrations/services/spell_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/class_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/race_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/background_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/feat_service.rs`

### Test Patterns to Follow
Reference existing test patterns in:
- `tests/integrations/services/campaign_service.rs`
- `tests/integrations/services/character_service.rs`

### Test Cases Per Service
Each service should have tests for:
1. Search with no filters (returns all)
2. Search with single filter
3. Search with multiple filters combined
4. Search with filters that return empty results
5. Get by ID/name (if applicable)
6. Error handling for invalid inputs

### Example Test Structure
```rust
#[test]
fn test_search_spells_by_level() {
    let db = TestDatabase::new();
    let mut conn = db.get_connection();
    
    let filters = SpellFilters {
        level: Some(vec![1]),
        ..Default::default()
    };
    
    let results = SpellService::search_spells(&mut conn, filters)
        .expect("Search should succeed");
    
    assert!(!results.is_empty());
    assert!(results.iter().all(|s| s.level == 1));
}
```

### Dependencies
- Requires catalog data to be seeded in test database
- May need to coordinate with MIMIR-T-0095 (Seed missing catalog data)

### Risk Considerations
- Low risk: Adding tests doesn't affect production code
- Ensure test data is representative of real-world usage

## Status Updates **[REQUIRED]**

*To be added during implementation*