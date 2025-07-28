---
id: core-agents-system
level: initiative
title: "Core Agents System"
created_at: 2025-07-28T16:26:45.812516+00:00
updated_at: 2025-07-28T16:26:45.812516+00:00
parent: initial-implementation
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: M
---

# Core Agents System Initiative

## Context

This initiative covers Phase 2 of the implementation plan (Weeks 3-4), building the agent framework and implementing the NPC Manager as the primary intelligent agent. This phase establishes the foundation for all AI-powered features and creates the RAG (Retrieval-Augmented Generation) pipeline that enables intelligent query responses.

The agent system uses a lightweight actor pattern with message passing between async Rust tasks. The NPC Manager serves as the flagship agent, demonstrating personality consistency tracking, relationship management, and intelligent information retrieval.

## Goals & Non-Goals

**Goals:**
- Implement lightweight actor pattern for agent communication using async Rust message passing
- Create NPC Manager agent with personality consistency and relationship tracking
- Build RAG pipeline combining FTS5 search with vector similarity for intelligent responses
- Develop hybrid search algorithm with configurable weights (0.5 FTS + 0.35 vector + 0.15 recency)
- Implement query router that chooses optimal search strategy based on query type
- Create agent trait system for extensible agent development
- Validate agent response times meet <2 second performance target

**Non-Goals:**
- User interface implementation (handled in Enhanced Capabilities phase)
- Additional agent types beyond NPC Manager (Plot, Session, Rules agents in Phase 3)
- Advanced AI features like consistency checking (handled in Intelligence Layer phase)
- Complex visualization or reporting features
- Multi-agent coordination or complex workflows

## Detailed Design

**Agent Framework Architecture:**
```rust
pub trait Agent: Send + Sync {
    fn id(&self) -> &str;
    async fn process(&mut self, request: Request, context: &Context) -> Result<Response>;
    fn capabilities(&self) -> Vec<Capability>;
}

pub struct Request {
    pub id: Uuid,
    pub intent: Intent,
    pub content: String,
    pub metadata: HashMap<String, Value>,
}
```

**NPC Manager Implementation:**
- Personality consistency tracking using embedding similarity
- Relationship graph storage and traversal
- Voice/behavior pattern recognition and reminder system
- Integration with hybrid search for NPC-related queries

**RAG Pipeline Design:**
- Query analysis: exact terms vs semantic intent detection
- FTS5 search for keyword matches (spell names, NPC names)
- Vector search for conceptual queries and relationship discovery
- Context assembly preserving complete rule sections
- Response generation with source attribution

**Message Passing System:**
- Async tokio channels for agent communication
- Request routing based on intent classification
- Context sharing between agents through Arc<Context>
- Error handling and timeout management

## Alternatives Considered

**Agent Communication Patterns:**
- Direct function calls: Rejected due to tight coupling and testing difficulties
- Event sourcing: Rejected as overly complex for single-user application
- Actor model with supervisors: Considered but deemed too heavy for initial implementation

**RAG Implementation Approaches:**
- LangChain integration: Rejected due to Python dependency and complexity
- Pure vector search: Rejected due to poor performance on exact term queries
- Pure FTS5 search: Rejected due to inability to handle semantic queries
- Custom hybrid approach: Selected for optimal balance of accuracy and performance

**NPC Data Modeling:**
- Graph database (Neo4j): Rejected due to deployment complexity
- Document store (MongoDB): Rejected due to poor SQL integration
- Relational with JSON fields: Selected for balance of structure and flexibility

## Implementation Plan

**Week 3:**
- Day 1-2: Implement core Agent trait and Request/Response types
- Day 3-4: Create message passing system with tokio channels
- Day 5-7: Build NPC Manager agent with basic personality tracking

**Week 4:**
- Day 1-3: Implement RAG pipeline with query analysis and routing
- Day 4-5: Integrate hybrid search algorithm with configurable weights
- Day 6-7: Add relationship tracking and context assembly for NPC queries

**Deliverables:**
- Functional agent framework with extensible trait system
- Working NPC Manager demonstrating personality consistency
- Complete RAG pipeline with <2s response times
- Integration tests showing end-to-end query processing

## Testing Strategy

**Unit Tests:**
- Agent trait implementations and message handling
- Query analysis and intent classification accuracy
- Hybrid search weight calculations and result merging
- NPC relationship graph operations

**Integration Tests:**
- End-to-end RAG pipeline with sample campaign data
- Agent communication through message passing system
- Context sharing and state management between agents

**Performance Tests:**
- Query response time under various load conditions
- Memory usage during concurrent agent operations
- Search relevance with different query types and weights

**Validation Criteria:**
- 95% accuracy on intent classification for common query types
- <2s response time for NPC-related queries
- Consistent personality tracking across multiple interactions
- Proper relationship inference from campaign context
