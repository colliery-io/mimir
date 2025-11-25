---
id: write-adr-for-service-pattern
level: task
title: "Write ADR for service pattern standardization"
short_code: "MIMIR-T-0101"
created_at: 2025-11-25T01:48:45.430737+00:00
updated_at: 2025-11-25T01:48:45.430737+00:00
parent: MIMIR-I-0010
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0010
---

# Write ADR for service pattern standardization

## Parent Initiative

[[MIMIR-I-0010]]

## Objective

Write an Architectural Decision Record (ADR) that documents the standardized service pattern for the codebase, addressing the current inconsistency between stateful services (with `&mut self`) and static method services.

## Acceptance Criteria

- [ ] ADR created in `.metis/adrs/` following existing ADR format
- [ ] Documents the current state (mixed patterns)
- [ ] Analyzes options: stateful vs stateless vs hybrid
- [ ] Recommends a standard pattern with rationale
- [ ] Provides migration guidance for existing services
- [ ] ADR transitioned to "decided" status

## Implementation Notes

### Technical Approach

Current state analysis:
- **Stateful pattern**: `SpellService<'a> { conn: &'a mut SqliteConnection }` with `&mut self` methods
- **Static pattern**: `ClassService::import_classes_from_book(conn, ...)` static methods
- **Mixed**: Some services use both patterns

Options to document:
1. **All stateful**: Services hold connection reference, instance methods only
2. **All stateless**: Static methods, pass connection each call
3. **Hybrid**: Instance methods for queries, static for imports

Recommendation factors:
- Testability (stateless is easier to test)
- Ergonomics (stateful is cleaner API)
- Connection pooling implications
- Tauri command handler patterns

### ADR Structure
```markdown
# ADR-004: Service Layer Pattern Standardization

## Status
Proposed

## Context
The codebase has 25+ services with inconsistent patterns...

## Decision
We will use [chosen pattern] because...

## Consequences
- Positive: ...
- Negative: ...
```

### Dependencies
None - this is a documentation task that informs other refactoring tasks

### Risk Considerations
- Need buy-in before implementing across codebase
- Migration effort could be significant

## Status Updates

*To be added during implementation*