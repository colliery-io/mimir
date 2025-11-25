---
id: add-integration-tests-lore-and
level: task
title: "Add integration tests: Lore and rules services"
short_code: "MIMIR-T-0089"
created_at: 2025-11-24T20:28:59.690604+00:00
updated_at: 2025-11-25T00:23:18.160323+00:00
parent: MIMIR-I-0008
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0008
---

# Add integration tests: Lore and rules services

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0008]]

## Objective

Add integration tests for lore and rules catalog services used for world-building and optional rules.

## Services in Scope

- `deity_service.rs` - Deity catalog queries
- `cult_service.rs` - Cult catalog queries
- `language_service.rs` - Language catalog queries
- `psionic_service.rs` - Psionics system queries
- `variant_rule_service.rs` - Variant rule queries
- `optional_feature_service.rs` - Optional feature queries
- `document_service.rs` - Document operations

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] `deity_service` has tests for deity search by pantheon/domain
- [ ] `cult_service` has integration tests
- [ ] `language_service` has tests for language lookups
- [ ] `psionic_service` has integration tests
- [ ] `variant_rule_service` has integration tests
- [ ] `optional_feature_service` has integration tests
- [ ] `document_service` has integration tests for CRUD operations
- [ ] All tests use the existing `TestDatabase` fixture pattern
- [ ] Tests include both success and error scenarios

## Implementation Notes

### Test File Locations
- `crates/mimir-dm-core/tests/integrations/services/deity_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/cult_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/language_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/psionic_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/variant_rule_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/optional_feature_service.rs`
- `crates/mimir-dm-core/tests/integrations/services/document_service.rs`

### Key Test Cases

**Deity Service:**
- Search by pantheon
- Search by domain
- List all deities

**Language Service:**
- List all languages
- Search by script type
- Filter by typical speakers

**Document Service:**
- Create document
- Read document by ID
- Update document
- Delete document
- List documents by campaign/module

**Other Services:**
- Basic search functionality
- Filter combinations
- Empty results handling

### Dependencies
- Follow patterns established in MIMIR-T-0083
- Requires lore data seeded in test DB (coordinate with MIMIR-T-0095)

### Risk Considerations
- Low risk: Adding tests doesn't affect production code
- Some services may have limited test data available

## Status Updates **[REQUIRED]**

*To be added during implementation*