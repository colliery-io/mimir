---
id: mimir
level: vision
title: "mimir"
short_code: "MIMIR-V-0001"
created_at: 2025-10-24T11:47:20.429412+00:00
updated_at: 2025-10-24T11:59:51.812336+00:00
archived: false

tags:
  - "#vision"
  - "#phase/published"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# mimir Vision

## Purpose

Mimir exists to reduce the cognitive load on Dungeon Masters by providing intelligent, context-aware tools that handle the mechanical and organizational complexity of running D&D campaigns. Named after the wise floating skull of Planescape lore, Mimir serves as a knowledgeable companion that remembers everything, tracks complex narratives, and provides instant access to rules and campaign information, allowing DMs to focus on what matters most: creating memorable stories with their players.

## Current State

DMs currently juggle multiple tools, books, notes, and applications to run their campaigns. They struggle with:
- Maintaining narrative consistency across sessions
- Tracking dozens of NPCs with their personalities, goals, and relationships
- Managing complex plot threads and player actions
- Quick access to rules during gameplay
- Session preparation that often takes hours
- Remembering details from previous sessions
- Balancing encounters and managing game mechanics

This cognitive overload often leads to inconsistencies, forgotten plot threads, and DM burnout.

## Progress to Date

Mimir has completed foundational work establishing the core data and infrastructure capabilities:

**Data Foundation (Completed)**
- Phase 0 delivered a complete D&D 5e data ingestion pipeline from 5etools JSON
- Denormalized SQLite database design optimized for read-heavy queries
- Multi-ruleset architecture supporting 5e 2014, 5e 2024, and future systems
- JSON storage strategy for complex nested game data
- Sub-second query performance for all rules lookups

**Technical Decisions (Documented)**
- ADR-001: Denormalized Database Design for maximum query performance
- ADR-002: Multi-Ruleset Architecture for supporting multiple game systems
- ADR-003: JSON Storage Strategy for complex D&D data structures

**Infrastructure Established**
- Rust-based import pipeline processing 13+ core 5etools entity types
- Campaign management foundation with database schema
- Desktop application framework using Tauri + Vue 3
- Local-first SQLite storage with migration support

**Current Focus**
The project is currently implementing character creation integration (MIMIR-I-0002), which will complete the campaign-to-player workflow by enabling DMs to create and manage player characters with full rules validation and character sheet generation.

## Future State

A world where DMs have a single, intelligent assistant that:
- Instantly retrieves any rule, spell, or game mechanic with semantic understanding
- Maintains perfect narrative consistency by tracking all campaign events, NPCs, and plot threads
- Generates rich, personality-driven NPCs on demand that fit seamlessly into the world
- Provides intelligent session preparation with customized content based on campaign history
- Offers real-time assistance during gameplay without breaking immersion
- Learns from each session to provide better suggestions and maintain world consistency
- Operates entirely offline, ensuring privacy and reliability

DMs spend their energy on creative storytelling while Mimir handles the complexity.

## Success Criteria

- DMs report 50%+ reduction in session prep time
- Zero campaign inconsistencies caused by forgotten details
- Sub-second response time for any rules query
- 90%+ of generated NPCs require no manual adjustment
- DMs can run a full session using only Mimir and dice
- Community adoption shows measurable reduction in DM burnout
- Platform becomes the de facto standard for serious D&D campaign management

## Principles

- **Local-First**: All data stays on the DM's machine - no cloud dependencies
- **Speed Over Features**: Sub-second responses are non-negotiable
- **Narrative First**: Every feature must enhance storytelling, not complicate it
- **Opinionated Simplicity**: Make smart defaults that work for 90% of cases
- **Progressive Disclosure**: Simple for beginners, powerful for experts
- **Rules as Written**: Support official content accurately before homebrew
- **Keyboard-Driven**: TUI interface optimized for flow state during sessions

## Constraints

- **Rust-Only**: Performance and reliability requirements mandate Rust
- **Offline-First**: No internet requirement during sessions
- **Terminal UI**: No GUI complexity - focus on keyboard efficiency
- **5e Focus**: D&D 5th Edition only initially, other systems later
- **Legal Compliance**: Only use freely available SRD content and user-owned data
- **Single User**: Personal DM tool, not a virtual tabletop replacement
- **Resource Limits**: Must run smoothly on a 5-year-old laptop