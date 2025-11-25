---
id: implement-playwright-e2e-testing
level: task
title: "Implement Playwright E2E testing for GUI"
short_code: "MIMIR-T-0127"
created_at: 2025-11-25T13:13:58.995142+00:00
updated_at: 2025-11-25T13:13:58.995142+00:00
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

# Implement Playwright E2E testing for GUI

## Parent Initiative

[[MIMIR-I-0012]]

## Objective

Implement automated end-to-end testing for the GUI using Playwright to enable regression testing, catch UI bugs before release, and provide confidence when refactoring frontend code. Tests should cover critical user journeys and run in CI.

## Acceptance Criteria

- [ ] Playwright configured for Tauri application testing
- [ ] Test suite covers core user journeys (see below)
- [ ] Tests run in CI pipeline on PR/push
- [ ] Tests produce screenshots on failure for debugging
- [ ] Test data setup/teardown handled properly
- [ ] Documentation for running tests locally
- [ ] At least 80% of critical paths covered

## Critical User Journeys to Test

### Campaign Management
1. Create a new campaign
2. View campaign list
3. Open campaign detail view
4. Create a module within campaign
5. Create a session within module

### Character Management
1. Create a new player
2. Create a new character (wizard flow)
3. View character sheet
4. Level up a character
5. Add item to inventory
6. Manage spell list

### Catalog Browsing
1. Search for spells with filters
2. View spell details
3. Search for monsters
4. View monster stat block

### Chat/LLM (if LLM available)
1. Open chat panel
2. Send a message
3. Verify response appears

## Implementation Notes

### Technical Approach

1. Install Playwright and configure for Tauri
   - Use `@playwright/test` package
   - Configure to launch Tauri dev server or built app
   
2. Set up test structure
   ```
   e2e/
     playwright.config.ts
     tests/
       campaign.spec.ts
       character.spec.ts
       catalog.spec.ts
       chat.spec.ts
     fixtures/
       test-helpers.ts
     page-objects/
       campaign-page.ts
       character-page.ts
   ```

3. Use Page Object Model for maintainability

4. Configure test database isolation
   - Either use separate test database
   - Or reset state between tests

### Tauri + Playwright Setup

```typescript
// playwright.config.ts
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './e2e/tests',
  use: {
    baseURL: 'tauri://localhost',
  },
  webServer: {
    command: 'npm run tauri dev',
    url: 'http://localhost:1420',
    reuseExistingServer: !process.env.CI,
  },
});
```

### Key Considerations
- Tauri apps need special handling vs web apps
- May need `tauri-driver` for WebDriver protocol
- Consider using `@tauri-apps/api/mocks` for IPC mocking
- Screenshot comparison for visual regression testing

### Dependencies
- MIMIR-T-0125 (dev seed data) - Tests need data to work with
- Node.js test infrastructure in `ui/` directory

### CI Integration
- Add GitHub Actions workflow for E2E tests
- Run on PRs to main branch
- Store test artifacts (screenshots, videos) on failure

## Status Updates

*To be added during implementation*