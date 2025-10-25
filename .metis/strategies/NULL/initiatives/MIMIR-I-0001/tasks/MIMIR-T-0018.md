---
id: split-chat-store-by-concern
level: task
title: "Split chat store by concern (messages, session, tokens)"
short_code: "MIMIR-T-0018"
created_at: 2025-10-24T11:53:49.959380+00:00
updated_at: 2025-10-25T09:39:47.814990+00:00
parent: MIMIR-I-0001
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: MIMIR-I-0001
---

# Split chat store by concern (messages, session, tokens)

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[MIMIR-I-0001]]

## Objective **[REQUIRED]**

Split the monolithic chat store (860 lines) into three focused stores organized by concern: messages (message handling and display), session (session CRUD operations), and tokens (token tracking and system configuration). This improves code navigability, maintainability, and follows the separation of concerns principle.

### Technical Debt Impact

- **Current Problems**: 
  - 860-line monolithic store file is difficult to navigate
  - Three distinct concerns (messages, sessions, tokens) mixed together
  - Hard to locate specific functionality
  - Violates separation of concerns principle
  
- **Benefits of Fixing**: 
  - Each store under 300 lines, easier to understand
  - Clear boundaries between message, session, and token concerns
  - Improved code discoverability and maintainability
  - Follows established patterns from previous refactoring tasks
  
- **Risk Assessment**: 
  - Low risk - stores are well-tested through UI usage
  - No functional changes, pure refactoring
  - TypeScript will catch any import/reference issues

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria **[REQUIRED]**

- [ ] Created stores/chat/ directory with three store files: messages.ts, session.ts, tokens.ts
- [ ] Each store file is under 300 lines
- [ ] Created index.ts that exports all three stores with backwards-compatible API
- [ ] All imports updated throughout the codebase
- [ ] Original chat.ts deleted
- [ ] Application builds successfully with no TypeScript errors
- [ ] All chat functionality works as before (no functional changes)



## Implementation Notes

### Technical Approach

Split stores/chat.ts into three focused store files within a new stores/chat/ directory:

**1. messages.ts** (~300 lines)
- Types: ChatMessage, ChatResponseWithUsage, ActionDescription, ChangeDetail, LineEdit, DiffPreview, etc.
- State: messages, isLoading, isCancelling, error, pendingConfirmations
- Computed: lastMessage
- Actions: sendMessage, cancelMessage, deleteMessage, buildSystemMessage
- Tool confirmation: confirmToolAction, rejectToolAction, getConfirmationForMessage
- Event listeners: llm-intermediate-message, tool-result-message, tool-confirmation-request
- Todos: updateTodos, todos state, todoProgress, currentTodo, hasTodos, toggleTodosVisibility, clearTodos, loadTodosForSession, extractTodosFromMessage, configureTodoStorage

**2. session.ts** (~250 lines)
- Types: ChatSession, ChatSessionMetadata
- State: currentSessionId, sessions, sessionsLoading
- Actions: loadSessions, loadSession, saveCurrentSession, createNewSession, deleteSession, switchToSession, clearHistory
- Session persistence and CRUD operations

**3. tokens.ts** (~200 lines)
- Types: ModelInfo, SystemMessageConfig
- State: modelInfo, totalTokensUsed, maxResponseTokens, systemConfig
- Computed: conversationTokens, contextUsagePercentage
- Actions: setMaxResponseTokens, updateSystemConfig, toggleContext, setSystemInstructions, setCustomInstructions, resetToDefaultPrompt, setLlmEndpoint
- Configuration: saveSystemConfig, loadSystemConfig

**4. index.ts** (~50 lines)
- Main useChatStore that composes all three stores
- Provides backwards-compatible API
- Coordinates initialization across all three stores

### File Organization
```
stores/chat/
├── index.ts (main store that composes others)
├── messages.ts (message handling)
├── session.ts (session management)
└── tokens.ts (token tracking and config)
```

### Dependencies
- Follows the pattern from MIMIR-T-0017 (useCatalog split)
- Uses Pinia's defineStore for each concern
- May need to use storeToRefs for cross-store reactivity

### Risk Considerations
- Need to maintain cross-store dependencies (e.g., messages need currentSessionId from session)
- Event listeners in initialize() need to coordinate across stores
- Must preserve all existing functionality and API surface

## Status Updates **[REQUIRED]**

*To be added during implementation*