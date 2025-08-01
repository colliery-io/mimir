# D&D Campaign Core Workflows and Dependencies

## Workflow 1: Campaign Launch (One-Time)

```mermaid
graph TD
    A[Create Campaign Concept] -->|Required| B[Define Big Three<br/>Bad, Stakes, Connection]
    B -->|Required| C[Set Tone & Genre]
    C -->|Required| D[Create Starting Location<br/>One settlement/region]
    D -->|Required| E[Schedule Session Zero]
    E -->|Required| F[Run Session Zero]
    F -->|Output: Player Characters| G[Integrate Characters<br/>into World]
    G -->|Required| H[Create First Adventure Hook]
    H -->|Required| I[Prep Session 1]
    I -->|Required| J[Run First Session]
    J -->|Success!| K[Campaign is Active]
    
    style A fill:#f9f,stroke:#333,stroke-width:2px
    style K fill:#9f9,stroke:#333,stroke-width:2px
```

**Minimum Viable Campaign Launch:**
- Campaign pitch (30 min)
- Starting location (30 min)
- Session Zero (2-3 hours)
- Character integration (1 hour)
- First session prep (1 hour)
- **Total: ~6 hours to launch**

## Workflow 2: Session Cycle (Repeating)

```mermaid
graph TD
    A[Previous Session Ends] -->|Within 24hr| B[Write Session Recap]
    B -->|Within 3 days| C[Start Next Session Prep]
    C -->|Required Steps| D[Review Characters]
    D --> E[Create Strong Start]
    E --> F[Outline 3-5 Scenes]
    F --> G[Define Secrets/Clues]
    G --> H[Session Ready]
    H -->|Game Day| I[Run Session]
    I --> J{End of Module?}
    J -->|No| A
    J -->|Yes| K[Module Transition]
    K --> L[Start Next Module Prep]
    L --> A
    
    style H fill:#9f9,stroke:#333,stroke-width:2px
    style K fill:#ff9,stroke:#333,stroke-width:2px
```

**Maintenance Requirements:**
- Recap previous session (15 min)
- Prep next session (30-60 min)
- Track what happened (during play)
- Transition modules (when needed)

## Workflow 3: Content Pipeline (As Needed)

```mermaid
graph TD
    A[Campaign Active] --> B{Content Check}
    B -->|Low| C[Need More Content]
    B -->|Adequate| D[Continue Playing]
    C --> E{What's Needed?}
    E -->|Plot| F[Create New Module]
    E -->|World| G[Expand Region/NPCs]
    E -->|Character| H[Develop PC Backstory Elements]
    
    F --> I[Module Overview]
    I --> J[Key NPCs]
    J --> K[3 Main Locations]
    K --> L[Hook & Outcomes]
    L --> M[Module Ready]
    
    G --> N[Region/Settlement Details]
    N --> O[Local NPCs]
    O --> P[Available Hooks]
    P --> Q[Expansion Ready]
    
    M --> D
    Q --> D
    
    style C fill:#ff9,stroke:#333,stroke-width:2px
    style M fill:#9f9,stroke:#333,stroke-width:2px
    style Q fill:#9f9,stroke:#333,stroke-width:2px
```

## Workflow 4: Campaign Conclusion

```mermaid
graph TD
    A[Active Campaign] --> B{Ending Trigger}
    B -->|Natural| C[Main Conflict Resolved]
    B -->|Forced| D[External Factors<br/>Players leaving, etc]
    
    C --> E[Plan Final Module]
    E --> F[Resolve Plot Threads]
    F --> G[Character Epilogues]
    G --> H[Final Session]
    
    D --> I[Rapid Wrap-up]
    I --> J[Emergency Epilogue Session]
    
    H --> K[Campaign Complete]
    J --> K
    K --> L[Archive Campaign]
    L --> M[Post-Mortem/Feedback]
    
    style B fill:#ff9,stroke:#333,stroke-width:2px
    style K fill:#9f9,stroke:#333,stroke-width:2px
```

## Critical Dependencies Chart

| Step | Must Have Before | Produces | Can't Skip Because |
|------|------------------|----------|-------------------|
| Session Zero | Campaign concept, basic world | Player characters | No PCs = No game |
| First Session | PCs, starting location, initial hook | Active campaign | Launches actual play |
| Session Prep | Previous recap, current status | Playable session | Improv has limits |
| Module Start | Completed previous, campaign status | New adventure arc | Need direction |
| Campaign End | Resolved/abandoned main conflict | Closure | Players need endings |

## Maintenance Activities Timeline

```mermaid
gantt
    title Weekly Campaign Maintenance
    dateFormat  YYYY-MM-DD
    section Immediate
    Session Recap           :done, recap, 2024-01-01, 1d
    section This Week
    Begin Session Prep      :active, prep, 2024-01-03, 1d
    Finalize Prep          :prep2, 2024-01-05, 1d
    Run Session            :crit, session, 2024-01-06, 1d
    section As Needed
    Module Planning        :module, 2024-01-02, 3d
    NPC Development        :npc, 2024-01-04, 2d
    World Expansion        :world, 2024-01-04, 2d
```

## Session Length Scaling

### How Session Length Affects Planning

| Session Length | Scenes | Prep Time | Module Length | Complexity |
|----------------|---------|-----------|---------------|------------|
| 1 hour (kids) | 1-2 | 15-20 min | 2-3 sessions | Simple |
| 2 hours | 2-3 | 30-40 min | 3-4 sessions | Moderate |
| 3 hours | 3-4 | 45-60 min | 3-5 sessions | Standard |
| 4+ hours | 4-6 | 60-90 min | 4-6 sessions | Full |

### Adjusted Workflows by Session Type

**Quick Sessions (1-2 hours):**
- Focus on single objective
- 1-2 simple encounters
- Linear scene progression
- Minimal prep between scenes

**Standard Sessions (3-4 hours):**
- Multiple objectives possible
- Mix of encounter types
- Room for exploration
- Player-driven tangents OK

**Marathon Sessions (5+ hours):**
- Complete story arcs
- Multiple locations
- Complex social encounters
- Significant progress expected

## "Done" Conditions

### Session is Done When:
- **Best**: Reached cliffhanger or natural stopping point
- **Good**: Energy flagging at scene boundary  
- **Acceptable**: Hit time limit mid-scene

### Design Sessions to Fit Time:
- **1 hour**: Design for 1 complete scene + cliffhanger
- **2 hours**: Design for 2-3 scenes with mini-arc
- **3 hours**: Design for complete adventure beat
- **4+ hours**: Design for multiple story beats

The goal is to structure content so natural endpoints align with time constraints.

### Module is Done When:
- Core conflict resolved
- Players ready to move on
- Usually 3-5 sessions

### Campaign is Done When:
- **Best Case**: Big Bad defeated, stakes resolved
- **Good Case**: Major arc completed, satisfaction achieved  
- **Acceptable**: Group decides to end, wrap-up provided
- **Emergency**: External factors, quick epilogue

## Minimum Viable System Must Support:

### Core Features (MVP)
1. **Campaign Creation**
   - Define concept and tone
   - Create starting location
   - Run session zero

2. **Session Management**
   - Prep upcoming session
   - Record what happened
   - Track who attended

3. **Content Pipeline**
   - Create/manage modules
   - Track NPCs (basic)
   - Note locations visited

4. **Progress Tracking**
   - Know what session we're on
   - See what's been played
   - Find things quickly

### Can Wait for v2:
- Detailed world building tools
- Complex relationship mapping
- Analytics and insights
- Player-facing features
- Advanced organization

## The Real Dependency: Time

```
Weekly Time Budget (Realistic):
- Post-session recap: 15 min
- Session prep: 45 min  
- Run session: 4 hours
- Module planning: 30 min every 2-3 weeks
- Total: ~5 hours/week when active

Crunch Periods:
- Campaign launch: 6 hours over 2-4 weeks
- Module transitions: +2 hours
- Campaign finale: +2-4 hours planning
```

The system succeeds if it:
1. Reduces prep time through templates/organization
2. Makes finding things instant vs hunting through folders
3. Tracks the boring stuff automatically
4. Never makes the DM do "homework" to use it