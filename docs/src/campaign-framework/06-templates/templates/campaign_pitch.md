---
id: campaign_pitch
title: Campaign Pitch Template
type: campaign_pitch
level: campaign
purpose: Create a one-page pitch to excite players about your campaign concept
author: Mimir Team
variables:
  - name: campaign_name
    type: string
    description: The name of your campaign
    default: "[Campaign Name]"
    required: true
  - name: genre
    type: string
    description: Primary genre and tone (e.g., Dark Fantasy, Pulp Adventure)
    default: "Fantasy Adventure"
    required: true
  - name: hook
    type: string
    description: One exciting sentence that captures the essence
    default: "[One exciting sentence that captures the essence of your campaign - make them lean forward]"
    required: true
  - name: big_bad
    type: string
    description: Who or what threatens the world
    default: "[Who or what threatens the world - one compelling sentence]"
    required: true
  - name: stakes
    type: string
    description: What happens if the heroes fail
    default: "[What happens if the heroes fail - make it personal AND epic]"
    required: true
  - name: connection
    type: string
    description: Why the PCs specifically must act
    default: "[Why the PCs specifically are the ones who must act]"
    required: true
  - name: starting_location
    type: string
    description: Where the campaign begins
    default: "[Location and circumstances]"
    required: true
  - name: initial_goal
    type: string
    description: What brings the party together
    default: "[What brings the party together]"
    required: true
  - name: session_length
    type: string
    description: How long are your sessions
    default: "[X hours]"
    required: true
  - name: schedule
    type: string
    description: How often you play
    default: "[Frequency and day]"
    required: true
---

# Campaign Pitch: {{ campaign_name }}

*One page to excite your players*

---

## The Hook
{{ hook }}

---

## Core Concept

**Genre & Tone:** {{ genre }}  
**Inspiration:** ["Like X meets Y" - use familiar media]  
**What Makes This Special:** [What sets your campaign apart in 2-3 bullets]

---

## The Big Three

### 1. The Big Bad
{{ big_bad }}

### 2. The Stakes  
{{ stakes }}

### 3. Your Connection
{{ connection }}

---

## Campaign Pillars
*Rate each pillar's emphasis (1-5 stars)*

**Combat:** ☆☆☆☆☆ - [Brief description of combat style]  
**Exploration:** ☆☆☆☆☆ - [What they'll discover]  
**Social:** ☆☆☆☆☆ - [Types of interactions]  
**Mystery:** ☆☆☆☆☆ - [If applicable]  
**Other:** ☆☆☆☆☆ - [Unique pillar for your campaign]

---

## Starting Situation

**Where You Begin:** {{ starting_location }}  
**Your Initial Goal:** {{ initial_goal }}  
**The Opening Scene:** [A glimpse of session 1 to build anticipation]

---

## Player Buy-In

*"To enjoy this campaign, I agree to..."*
- [Requirement 1 - e.g., "Work with the party, not against it"]
- [Requirement 2 - e.g., "Engage with the central mystery"]  
- [Requirement 3 - e.g., "Build a character who cares about X"]
- [Optional requirements specific to your campaign]

---

## Campaign Logistics

**Estimated Length:** □ Short (5-10) □ Medium (10-25) □ Long (25-50) □ Epic (50+ sessions)  
**Session Length:** {{ session_length }} with [break structure]  
**Schedule:** {{ schedule }}  
**Style:** □ Railroad □ Guided □ Sandbox □ Player-Driven  
**Lethality:** □ Heroic □ Dangerous □ Deadly □ Meat Grinder

---

## What I Promise as Your DM

- [Promise 1 - e.g., "Your choices will matter"]
- [Promise 2 - e.g., "NPCs will feel real and memorable"]
- [Promise 3 - e.g., "Epic moments balanced with personal stories"]
- [Specific promise related to your campaign theme]

---

## Questions to Consider for Your Character

- [Question that ties to Big Bad]
- [Question that ties to Stakes]  
- [Question that ties to Starting Situation]
- [Question that encourages party bonds]

---

*"So... are you in?"*