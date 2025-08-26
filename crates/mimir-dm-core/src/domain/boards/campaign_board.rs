//! Campaign board definition

use super::{BoardDefinition, StageMetadata};

pub struct CampaignBoard;

impl CampaignBoard {
    pub fn new() -> Self {
        Self
    }
}

impl BoardDefinition for CampaignBoard {
    fn board_type(&self) -> &str {
        "campaign"
    }
    
    fn stages(&self) -> Vec<&str> {
        vec!["concept", "session_zero", "integration", "active", "concluding", "completed"]
    }
    
    fn can_transition(&self, from: &str, to: &str) -> bool {
        match (from, to) {
            // Forward progression
            ("concept", "session_zero") => true,
            ("session_zero", "integration") => true,
            ("integration", "active") => true,
            ("active", "concluding") => true,
            ("concluding", "completed") => true,
                        
            _ => false,
        }
    }
    
    fn required_documents(&self, stage: &str) -> Vec<&str> {
        match stage {
            "concept" => vec!["campaign_pitch"],
            "session_zero" => vec![
                "starting_scenario",
                "world_primer",
                "character_guidelines",
                "table_expectations",
                "character_integration",
            ],
            "integration" => vec![
                "campaign_bible",
                "major_npc_tracker",
            ],
            "active" => vec![], // No required documents
            "concluding" => vec![],
            "completed" => vec![],
            _ => vec![],
        }
    }
    
    fn optional_documents(&self, stage: &str) -> Vec<&str> {
        match stage {
            "concept" => vec![],  // No optional documents - notes and inspiration are working tools, not artifacts
            "session_zero" => vec!["safety_tools", "house_rules"],
            "integration" => vec!["player_secrets", "faction_overview"],
            "active" => vec![],  // No documents in active stage - managed through session boards
            _ => vec![],
        }
    }
    
    fn next_stage(&self, current: &str) -> Option<&str> {
        match current {
            "concept" => Some("session_zero"),
            "session_zero" => Some("integration"),
            "integration" => Some("active"),
            "active" => Some("concluding"),
            "concluding" => Some("completed"),
            _ => None,
        }
    }
    
    fn stage_metadata(&self, stage: &str) -> StageMetadata {
        match stage {
            "concept" => StageMetadata {
                display_name: "Concept".to_string(),
                description: "Initial campaign planning and pitch development".to_string(),
                completion_message: Some(
                    "Great! Your campaign pitch is complete. Next, you'll prepare materials for Session Zero."
                        .to_string()
                ),
                transition_prompt: Some(
                    "You can always edit this document later, but make sure your players have a chance to read the pitch and provide initial feedback before Session Zero."
                        .to_string()
                ),
                help_text: None,
                content: Some(
                    r#"<h3>Phase 1: The Spark - Transform Your Idea into a Campaign</h3>
<p>Every campaign begins with a spark—that initial idea that excites you enough to build a world around it. This phase captures and refines that inspiration into a workable campaign concept. Total time investment: 5-8 hours across a week.</p>

<h4>Finding and Refining Your Spark</h4>
<p>Your spark might come from:</p>
<ul>
  <li><strong>Media Inspiration</strong>: "What if Game of Thrones met Aliens?"</li>
  <li><strong>Mechanical Interest</strong>: "I want to run a naval exploration campaign"</li>
  <li><strong>Thematic Question</strong>: "What does it mean to be heroic in a morally gray world?"</li>
  <li><strong>Visual Imagery</strong>: "A city built on the back of a massive, sleeping dragon"</li>
  <li><strong>Player Request</strong>: "We want to play sky pirates!"</li>
</ul>

<h4>Define Your Big Three</h4>
<p>Transform your spark into concrete campaign elements:</p>
<ol>
  <li><strong>Core Conflict</strong>
    <p>The fundamental tension driving the campaign. What's at stake? What happens if no one acts? This should create urgency and agency.</p>
  </li>
  <li><strong>Unique Element</strong>
    <p>What makes this different from generic fantasy? (Though there's nothing wrong with generic fantasy—sometimes you just need to save a princess from a dragon!)</p>
  </li>
  <li><strong>Player Role</strong>
    <p>How do the PCs fit into this world? Are they hired investigators, reluctant heroes, ambitious mercenaries? Give them a clear starting position.</p>
  </li>
</ol>

<h4>Create Your Starting Point</h4>
<p>Define where and how the campaign begins:</p>
<ul>
  <li><strong>Physical Location</strong>: Specific place where Session 1 occurs</li>
  <li><strong>Initial Situation</strong>: The immediate problem or opportunity</li>
  <li><strong>Party Connection</strong>: Why these specific PCs are together</li>
  <li><strong>First Adventure</strong>: Plan a 3-4 session mini-arc to establish tone</li>
</ul>

<h4>Your Campaign Pitch Document</h4>
<p>Write a one-page pitch that excites potential players while setting expectations:</p>
<ul>
  <li><strong>The Hook</strong>: 2-3 sentences that sell the concept</li>
  <li><strong>What Makes It Special</strong>: 3-4 unique aspects</li>
  <li><strong>The Tone</strong>: Reference media or themes they'll recognize</li>
  <li><strong>What You Need From Players</strong>: Commitment level, play style, character requirements</li>
  <li><strong>Practical Details</strong>: System, starting level, session frequency</li>
</ul>

<h4>Phase 1 Timeline</h4>
<ul>
  <li><strong>Days 1-2</strong>: Brainstorm and refine your spark (2 hours)</li>
  <li><strong>Day 3</strong>: Define your Big Three (1 hour)</li>
  <li><strong>Day 4</strong>: Create starting point details (1 hour)</li>
  <li><strong>Day 5</strong>: Write campaign pitch (1 hour)</li>
  <li><strong>Day 6</strong>: Outline first adventure (2 hours)</li>
  <li><strong>Day 7</strong>: Review and polish (1 hour)</li>
</ul>

<p class="integration-note">Remember: Your spark should excite you enough to sustain months or years of play. If you're not enthusiastic about it, your players won't be either. But also remember—campaigns evolve. Your initial concept is a starting point, not a prison.</p>"#
                        .to_string()
                ),
            },
            "session_zero" => StageMetadata {
                display_name: "Session Zero".to_string(),
                description: "Preparing materials for the collaborative session zero".to_string(),
                completion_message: Some(
                    "Excellent! Your Session Zero materials are ready. After your Session Zero, you'll move to the Integration stage."
                        .to_string()
                ),
                transition_prompt: Some(
                    "Remember to share these documents with your players before Session Zero. Take notes during the session as you'll need them for the Integration stage."
                        .to_string()
                ),
                help_text: None,
                content: Some(
                    r#"<h3>Phase 2: Session Zero Preparation</h3>
<p>Session Zero creates the bridge between your vision and your players' expectations. This phase prepares you to run an effective Session Zero that aligns everyone and generates excitement. Time investment: 6-9 hours across a week.</p>

<h4>The Session Zero Packet - Progressive Disclosure</h4>
<p>Create player-facing documents using information layering to prevent overload:</p>

<ol>
  <li><strong>Starting Scenario (1-2 pages)</strong>
    <p>Send T-7 days before Session Zero. Sets the immediate situation:</p>
    <ul>
      <li>Where the characters are right now</li>
      <li>Recent events they might have witnessed</li>
      <li>The immediate opportunity that brings them together</li>
      <li>Questions to consider for character creation</li>
    </ul>
  </li>
  
  <li><strong>World Primer (2-3 pages)</strong>
    <p>Send with full packet at T-3 days. Provides deeper context:</p>
    <ul>
      <li>Brief history (5-6 key events maximum)</li>
      <li>Current situation (2-3 paragraphs)</li>
      <li>Major factions/powers (3-4 with one-line descriptions)</li>
      <li>Common knowledge vs. mysteries</li>
      <li>Geography and important locations</li>
    </ul>
  </li>
  
  <li><strong>Character Guidelines</strong>
    <p>Include with packet. Helps players create appropriate characters:</p>
    <ul>
      <li>Allowed races, classes, and backgrounds</li>
      <li>Starting level and equipment</li>
      <li>Required connections to the setting</li>
      <li>Character concept examples</li>
      <li>What makes a good fit for this campaign</li>
    </ul>
  </li>
  
  <li><strong>Table Expectations</strong>
    <p>Essential for alignment. Cover:</p>
    <ul>
      <li>Tone and themes (with content warnings if needed)</li>
      <li>Player agency vs. railroad expectations</li>
      <li>Combat vs. roleplay vs. exploration balance</li>
      <li>House rules and table etiquette</li>
      <li>Scheduling and attendance expectations</li>
    </ul>
  </li>
  
  <li><strong>Character Integration Worksheet</strong>
    <p>Collaborative tool for Session Zero:</p>
    <ul>
      <li>Three NPCs your character knows</li>
      <li>One secret about the setting</li>
      <li>A rumor they've heard (true or false)</li>
      <li>Connection to at least one other PC</li>
      <li>Personal stake in the opening scenario</li>
    </ul>
  </li>
</ol>

<h4>Session Zero Structure Plan</h4>
<p>Design your 3-4 hour session:</p>
<ul>
  <li><strong>Hour 1</strong>: Introductions, expectations, safety tools</li>
  <li><strong>Hour 2</strong>: Character creation/refinement</li>
  <li><strong>Hour 3</strong>: Party connections and dynamics</li>
  <li><strong>Hour 4</strong>: World building contributions, Q&A</li>
</ul>

<h4>Preparation Timeline</h4>
<ul>
  <li><strong>T-7 days</strong>: Send Starting Scenario to generate excitement</li>
  <li><strong>T-6 days</strong>: Create World Primer (2 hours)</li>
  <li><strong>T-5 days</strong>: Write Character Guidelines (1 hour)</li>
  <li><strong>T-4 days</strong>: Develop Table Expectations (1 hour)</li>
  <li><strong>T-3 days</strong>: Send complete packet to players</li>
  <li><strong>T-2 days</strong>: Create integration worksheets (1 hour)</li>
  <li><strong>T-1 day</strong>: Review and prepare session materials</li>
</ul>

<p class="integration-note">Session Zero is not about playing the game—it's about ensuring everyone wants to play the same game. Take time to listen to player ideas and concerns. The best campaigns grow from this collaborative foundation.</p>"#
                        .to_string()
                ),
            },
            "integration" => StageMetadata {
                display_name: "Integration".to_string(),
                description: "Integrating player feedback and characters into the campaign".to_string(),
                completion_message: Some(
                    "Perfect! Your campaign is fully integrated and ready to begin. Time to start your adventure!"
                        .to_string()
                ),
                transition_prompt: Some(
                    "These documents will be your reference throughout the campaign. Make sure everything from Session Zero has been incorporated."
                        .to_string()
                ),
                help_text: None,
                content: Some(
                    r#"<h3>Phase 4: Integration and Launch</h3>
<p>Transform Session Zero output into your first module. This phase weaves player contributions into playable content while observing what excites your specific group. Time investment: 6-9 hours.</p>

<h4>Your First Module Is Special</h4>
<p>Design it as a testing ground:</p>
<ul>
  <li><strong>Short Duration</strong>: 2-3 sessions (6-10 hours of play)</li>
  <li><strong>Flexible Paths</strong>: Multiple approaches to explore player preferences</li>
  <li><strong>Observable Moments</strong>: Built-in decision points reveal what players enjoy</li>
  <li><strong>Integrated Elements</strong>: Weaves in Session Zero discoveries</li>
</ul>

<h4>Character Integration Process</h4>
<p>For each PC, identify and document:</p>
<ol>
  <li><strong>Immediate Connection</strong>
    <p>Why does this opening problem matter to them personally? Use their backstory, goals, or relationships from Session Zero.</p>
  </li>
  <li><strong>Unique Advantage</strong>
    <p>What skills, knowledge, or connections do they bring that others lack? Plan at least one scene where this shines.</p>
  </li>
  <li><strong>Personal Stakes</strong>
    <p>What do they gain or lose from success or failure? Make it specific to their character, not generic rewards.</p>
  </li>
  <li><strong>Growth Opportunity</strong>
    <p>How does this adventure challenge their beliefs or push them toward their goals?</p>
  </li>
</ol>

<h4>Mining Session Zero</h4>
<p>Catalog player contributions to weave into your module:</p>
<ul>
  <li><strong>NPCs Created</strong>: List every NPC mentioned in backstories or during Session Zero</li>
  <li><strong>Locations Mentioned</strong>: Note places from character histories that could become scenes</li>
  <li><strong>Backstory Elements</strong>: Track rivals, mentors, debts, and obligations to reference</li>
  <li><strong>Stated Interests</strong>: Remember what excited players during discussion</li>
</ul>

<h4>First Module Design Goals</h4>
<ul>
  <li><strong>Test Engagement</strong>: Include combat, investigation, and social scenes to see what resonates</li>
  <li><strong>Branch Early</strong>: Offer meaningful choices by end of first session</li>
  <li><strong>Feature Everyone</strong>: Each PC gets at least one spotlight moment per session</li>
  <li><strong>Plant Seeds</strong>: Introduce 3-4 potential future plots without committing</li>
  <li><strong>Stay Flexible</strong>: Don't lock into one campaign direction yet</li>
</ul>

<h4>Creating Your Campaign Bible</h4>
<p>Consolidate everything into your reference document:</p>
<ul>
  <li><strong>Character Profiles</strong>: One page per PC with connections, goals, and likely approaches</li>
  <li><strong>NPC Roster</strong>: Quick reference of all NPCs with relationships mapped</li>
  <li><strong>Location Guide</strong>: Key places with who controls them and what happens there</li>
  <li><strong>Faction Summary</strong>: Who wants what and how PCs relate to each</li>
  <li><strong>Open Threads</strong>: List of mysteries, conflicts, and opportunities to develop</li>
</ul>

<h4>Major NPC Development</h4>
<p>For each significant NPC, define:</p>
<ul>
  <li><strong>Appearance</strong>: One memorable physical trait</li>
  <li><strong>Voice</strong>: Speech pattern or verbal tic</li>
  <li><strong>Want</strong>: What they're trying to achieve</li>
  <li><strong>Leverage</strong>: What they can offer or threaten</li>
  <li><strong>Secret</strong>: Something not immediately obvious</li>
</ul>

<h4>Integration Timeline</h4>
<ul>
  <li><strong>T+1 day after Session Zero</strong>: Review notes while fresh (1 hour)</li>
  <li><strong>Day 2</strong>: Create character integration profiles (2 hours)</li>
  <li><strong>Day 3</strong>: Develop Campaign Bible structure (2 hours)</li>
  <li><strong>Day 4</strong>: Design major NPCs (2 hours)</li>
  <li><strong>Day 5-6</strong>: Create first module using Module Creation process</li>
  <li><strong>Day 7</strong>: Final review and polish</li>
</ul>

<p class="integration-note">Your first module is a testing ground, not a commitment. Watch what makes players lean forward, take notes on what they engage with, and use these observations to shape the campaign's true direction. The best campaigns grow from this patient observation.</p>"#
                        .to_string()
                ),
            },
            "active" => StageMetadata {
                display_name: "Active".to_string(),
                description: "Campaign is actively being played".to_string(),
                completion_message: Some(
                    "Your campaign has been an amazing journey! Time to bring it to a conclusion."
                        .to_string()
                ),
                transition_prompt: Some(
                    "Is your campaign approaching its finale? Move to concluding when you're ready to wrap up the story."
                        .to_string()
                ),
                help_text: None,
                content: Some(
                    r#"<h3>Campaign Active: Sustainable Long-Term Play</h3>
<p>Your campaign is now active! This is where preparation becomes play, where your world comes alive through modules and sessions. The active phase can last months or years, sustained by good habits and systematic approaches.</p>

<h4>The Module Rhythm</h4>
<p>Campaigns thrive on a sustainable cycle:</p>
<ul>
  <li><strong>Module Planning</strong>: 8-12 hours creates 12-20 hours of play</li>
  <li><strong>Session Preparation</strong>: 1 hour creates 4 hours of play</li>
  <li><strong>Between Modules</strong>: Take a week to assess and plan</li>
  <li><strong>Campaign Breaks</strong>: Schedule them before you need them</li>
</ul>

<h4>Managing Multiple Story Threads</h4>
<p>Active campaigns juggle various narrative elements:</p>
<ul>
  <li><strong>A-Plot</strong>: The main campaign arc (develops slowly)</li>
  <li><strong>B-Plots</strong>: Module-specific stories (2-6 sessions each)</li>
  <li><strong>C-Plots</strong>: Character personal arcs (weave throughout)</li>
  <li><strong>D-Plots</strong>: World events and consequences (background motion)</li>
</ul>

<h4>When to Create New Modules</h4>
<p>Natural triggers for module creation:</p>
<ul>
  <li><strong>Current Module Completing</strong>: Start planning when 1-2 sessions remain</li>
  <li><strong>Narrative Momentum</strong>: Story demands new direction or location</li>
  <li><strong>Player Goals</strong>: Character objectives require dedicated content</li>
  <li><strong>Campaign Phase Shift</strong>: Moving between major acts or themes</li>
</ul>

<h4>Living World Management</h4>
<p>The world continues moving between sessions:</p>
<ul>
  <li><strong>NPC Actions</strong>: Major NPCs pursue goals regardless of PC involvement</li>
  <li><strong>Faction Progress</strong>: Organizations advance their agendas</li>
  <li><strong>Consequence Ripples</strong>: PC actions have ongoing effects</li>
  <li><strong>Timeline Events</strong>: Scheduled events occur unless prevented</li>
</ul>

<h4>Campaign Health Indicators</h4>
<p>Signs your campaign is thriving:</p>
<ul>
  <li>Players discuss the game between sessions</li>
  <li>Character goals drive story direction</li>
  <li>Past events get referenced naturally</li>
  <li>Players make long-term plans</li>
  <li>You're excited to prep each week</li>
</ul>

<h4>Warning Signs to Address</h4>
<p>Issues to catch early:</p>
<ul>
  <li><strong>Aimless Sessions</strong>: Players unsure of objectives → Create clearer module goals</li>
  <li><strong>DM Burnout</strong>: Prep feels like work → Take a break or run a one-shot</li>
  <li><strong>Scattered Focus</strong>: Too many unrelated threads → Consolidate or resolve some</li>
  <li><strong>Player Disengagement</strong>: Interest waning → Check in privately, adjust focus</li>
  <li><strong>Attendance Issues</strong>: Regular cancellations → Address scheduling honestly</li>
</ul>

<h4>Sustainable Practices</h4>
<ul>
  <li><strong>Prep Limits</strong>: Never prep more than 2 sessions ahead</li>
  <li><strong>Player Notes</strong>: Rotate who takes session notes</li>
  <li><strong>Regular Check-ins</strong>: Monthly "how's the game?" discussions</li>
  <li><strong>Flexible Scheduling</strong>: Build in skip weeks for holidays/life</li>
  <li><strong>Document Everything</strong>: Future you will thank present you</li>
</ul>

<h4>Using the Module System</h4>
<p>Each module follows the same lifecycle:</p>
<ol>
  <li><strong>Planning</strong>: Define concept, stakes, and scope</li>
  <li><strong>Development</strong>: Create NPCs, locations, encounters</li>
  <li><strong>Ready</strong>: Pressure test and finalize</li>
  <li><strong>Active</strong>: Run sessions with weekly prep cycle</li>
  <li><strong>Completed</strong>: Archive and mine for future content</li>
</ol>

<p class="integration-note">The best campaigns balance structure with flexibility. Use modules to maintain momentum while leaving room for player-driven stories. Remember: the campaign that reaches a satisfying conclusion is better than the perfect campaign that burns out.</p>"#
                        .to_string()
                ),
            },
            "concluding" => StageMetadata {
                display_name: "Concluding".to_string(),
                description: "Campaign is wrapping up its final story arcs".to_string(),
                completion_message: Some(
                    "Congratulations! Your campaign has reached its epic conclusion."
                        .to_string()
                ),
                transition_prompt: Some(
                    "Have all story arcs been resolved? Mark the campaign as completed to archive it."
                        .to_string()
                ),
                help_text: None,
                content: Some(
                    r#"<h3>Campaign Concluding: Crafting the Finale</h3>
<p>Your campaign is approaching its end. This is a special time—the culmination of months or years of shared storytelling. A well-crafted conclusion transforms a good campaign into an unforgettable legend.</p>

<h4>Recognizing When to Conclude</h4>
<p>Natural ending points include:</p>
<ul>
  <li><strong>Narrative Completion</strong>: Main antagonist defeated, quest fulfilled</li>
  <li><strong>Character Arcs Resolved</strong>: PCs achieved or failed personal goals</li>
  <li><strong>Natural Breaking Point</strong>: Players graduating, moving, life changes</li>
  <li><strong>Energy Depletion</strong>: Better to end strong than fade away</li>
  <li><strong>Story Escalation</strong>: Stakes can't meaningfully go higher</li>
</ul>

<h4>The Conclusion Timeline</h4>
<p>Plan your ending 4-8 sessions in advance:</p>
<ul>
  <li><strong>T-8 sessions</strong>: Announce the campaign is concluding</li>
  <li><strong>T-6 sessions</strong>: Begin resolving B and C plots</li>
  <li><strong>T-4 sessions</strong>: Focus entirely on main arc</li>
  <li><strong>T-2 sessions</strong>: Set up the finale</li>
  <li><strong>T-0</strong>: The climactic session</li>
  <li><strong>T+1</strong>: Epilogue session (optional but recommended)</li>
</ul>

<h4>Resolving Story Threads</h4>
<p>Not every thread needs resolution, but be intentional:</p>
<ul>
  <li><strong>A-Plot</strong>: Must reach satisfying conclusion</li>
  <li><strong>Character Arcs</strong>: Each PC gets personal resolution</li>
  <li><strong>Major NPCs</strong>: Show their fates or final positions</li>
  <li><strong>Minor Threads</strong>: Can remain open for sequel campaigns</li>
  <li><strong>World Events</strong>: Show how PC actions changed things</li>
</ul>

<h4>Designing the Final Module</h4>
<p>Your last module should:</p>
<ul>
  <li><strong>Callback Earlier Events</strong>: NPCs return, old decisions matter</li>
  <li><strong>Showcase Growth</strong>: Challenges that once threatened now manageable</li>
  <li><strong>Personal Stakes</strong>: Make it about characters, not just the world</li>
  <li><strong>Multiple Climaxes</strong>: Action climax, emotional climax, revelation climax</li>
  <li><strong>Player Agency</strong>: Their choices determine the ending</li>
</ul>

<h4>The Climactic Session</h4>
<p>Make the finale memorable:</p>
<ul>
  <li><strong>Extended Time</strong>: Consider running 5-6 hours instead of usual 4</li>
  <li><strong>No Held Punches</strong>: Use everything—all resources, all stakes</li>
  <li><strong>Callback Moments</strong>: Reference Session 1, early adventures</li>
  <li><strong>Spotlight Rotation</strong>: Everyone gets hero moments</li>
  <li><strong>Meaningful Consequences</strong>: Victory might require sacrifice</li>
</ul>

<h4>The Epilogue Session</h4>
<p>Often the most memorable session:</p>
<ul>
  <li><strong>Time Skip</strong>: Jump forward months or years</li>
  <li><strong>Character Futures</strong>: Each player describes their PC's fate</li>
  <li><strong>World Changes</strong>: Show how their actions reshaped things</li>
  <li><strong>Open Questions</strong>: Leave some mystery for imagination</li>
  <li><strong>Emotional Closure</strong>: Celebrate the journey together</li>
</ul>

<h4>Common Finale Pitfalls</h4>
<ul>
  <li><strong>Introducing New Elements</strong>: Stick with established story</li>
  <li><strong>Deus Ex Machina</strong>: Let PCs be the heroes</li>
  <li><strong>Anticlimax</strong>: Don't overthink—trust your preparation</li>
  <li><strong>Rushing</strong>: Take time needed for satisfying conclusion</li>
  <li><strong>Ignoring Characters</strong>: Plot serves characters, not vice versa</li>
</ul>

<h4>Preserving the Campaign</h4>
<p>While concluding, capture the campaign for posterity:</p>
<ul>
  <li><strong>Campaign Summary</strong>: Write 2-3 pages of highlights</li>
  <li><strong>Character Epilogues</strong>: Document each PC's ending</li>
  <li><strong>Memorable Quotes</strong>: That table joke that became canon</li>
  <li><strong>Key Decisions</strong>: Major choices and consequences</li>
  <li><strong>Photo Gallery</strong>: Screenshots, character art, maps</li>
</ul>

<p class="integration-note">A campaign conclusion is not just an ending—it's a transformation. Your players will carry these memories for years. Take time to craft an ending worthy of the journey you've shared. Some groups frame character sheets, others create photo books, many simply share one last meal together. Honor what you've built.</p>"#
                        .to_string()
                ),
            },
            "completed" => StageMetadata {
                display_name: "Completed".to_string(),
                description: "Campaign has been completed and archived".to_string(),
                completion_message: None,
                transition_prompt: None,
                help_text: None,
                content: Some(
                    r#"<h3>Campaign Complete: The Archive of Legends</h3>
<p>Your campaign has reached its conclusion. The stories have been told, the dragons slain (or befriended), and the world forever changed by your players' actions. This archive preserves the memories and materials of your shared journey.</p>

<h4>What's Preserved</h4>
<p>Your complete campaign archive contains:</p>
<ul>
  <li><strong>All Modules</strong>: Every adventure from first to last</li>
  <li><strong>Session Records</strong>: Notes and outcomes from actual play</li>
  <li><strong>Character Documents</strong>: PC progression and story arcs</li>
  <li><strong>World Materials</strong>: Maps, NPCs, factions, lore</li>
  <li><strong>Campaign Bible</strong>: Your master reference document</li>
</ul>

<h4>Mining for Future Campaigns</h4>
<p>This archive is a goldmine for future games:</p>
<ul>
  <li><strong>Successful Modules</strong>: Can be reskinned for new campaigns</li>
  <li><strong>Beloved NPCs</strong>: Might appear in other worlds</li>
  <li><strong>World Building</strong>: Locations and lore for sequel campaigns</li>
  <li><strong>Lessons Learned</strong>: What worked, what didn't, what surprised you</li>
  <li><strong>Player Preferences</strong>: Now you know what your group loves</li>
</ul>

<h4>The Campaign Summary</h4>
<p>Consider creating a brief campaign summary including:</p>
<ul>
  <li><strong>The Premise</strong>: What was the campaign about?</li>
  <li><strong>The Heroes</strong>: Who were the PCs and what did they achieve?</li>
  <li><strong>Key Events</strong>: 5-10 major story moments</li>
  <li><strong>The Ending</strong>: How did it all conclude?</li>
  <li><strong>Campaign Stats</strong>: Sessions played, real-world duration, levels gained</li>
</ul>

<h4>Sequel Campaign Seeds</h4>
<p>Elements that could spawn new campaigns:</p>
<ul>
  <li><strong>Unresolved Mysteries</strong>: Questions left deliberately open</li>
  <li><strong>Next Generation</strong>: Children or students of the PCs</li>
  <li><strong>Consequences</strong>: What happened because of PC actions?</li>
  <li><strong>Different Perspective</strong>: Same events, different viewpoint</li>
  <li><strong>The New Threat</strong>: What fills the power vacuum?</li>
</ul>

<h4>Sharing Your Campaign</h4>
<p>Ways to share your completed campaign:</p>
<ul>
  <li><strong>Campaign Journal</strong>: Blog or document for players to revisit</li>
  <li><strong>Module Library</strong>: Share successful adventures with other DMs</li>
  <li><strong>Actual Play</strong>: If recorded, edit highlights for posterity</li>
  <li><strong>Campaign Reunion</strong>: Annual one-shot returning to the world</li>
  <li><strong>New Player Resource</strong>: "Previously, in our last campaign..."</li>
</ul>

<h4>Personal Reflection</h4>
<p>Questions for your DM journey:</p>
<ul>
  <li>What moments made you proudest as a DM?</li>
  <li>Which player actions surprised you most?</li>
  <li>What would you do differently?</li>
  <li>What will you carry forward to future campaigns?</li>
  <li>Which memories will you treasure?</li>
</ul>

<h4>The Next Campaign</h4>
<p>When you're ready to begin again:</p>
<ul>
  <li>Take a break first—let this campaign settle into memory</li>
  <li>Discuss with players what they want next</li>
  <li>Consider a different genre or system for variety</li>
  <li>Apply lessons learned from this campaign</li>
  <li>Start fresh with Phase 1: The Spark</li>
</ul>

<p class="integration-note">Completing a campaign is a rare achievement in tabletop gaming. Many campaigns start, but few reach meaningful conclusions. You've given your players something precious: a complete story with beginning, middle, and end. That shared narrative will bond your group forever, referenced in years to come with "Remember when..." Take a moment to appreciate what you've accomplished together.</p>"#
                        .to_string()
                ),
            },
            _ => StageMetadata {
                display_name: stage.to_string(),
                description: format!("The {} stage", stage),
                completion_message: None,
                transition_prompt: None,
                help_text: None,
                content: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_type() {
        let board = CampaignBoard::new();
        assert_eq!(board.board_type(), "campaign");
    }

    #[test]
    fn test_stages_order() {
        let board = CampaignBoard::new();
        let stages = board.stages();
        
        assert_eq!(stages.len(), 6);
        assert_eq!(stages[0], "concept");
        assert_eq!(stages[1], "session_zero");
        assert_eq!(stages[2], "integration");
        assert_eq!(stages[3], "active");
        assert_eq!(stages[4], "concluding");
        assert_eq!(stages[5], "completed");
    }

    #[test]
    fn test_valid_forward_transitions() {
        let board = CampaignBoard::new();
        
        // Test all valid forward transitions
        assert!(board.can_transition("concept", "session_zero"));
        assert!(board.can_transition("session_zero", "integration"));
        assert!(board.can_transition("integration", "active"));
        assert!(board.can_transition("active", "concluding"));
        assert!(board.can_transition("concluding", "completed"));
    }

    #[test]
    fn test_invalid_transitions() {
        let board = CampaignBoard::new();
        
        // Test backward transitions (not allowed)
        assert!(!board.can_transition("session_zero", "concept"));
        assert!(!board.can_transition("integration", "session_zero"));
        assert!(!board.can_transition("active", "integration"));
        
        // Test skip transitions (not allowed)
        assert!(!board.can_transition("concept", "integration"));
        assert!(!board.can_transition("concept", "active"));
        assert!(!board.can_transition("session_zero", "active"));
        
        // Test self-transitions (not allowed)
        assert!(!board.can_transition("concept", "concept"));
        assert!(!board.can_transition("active", "active"));
        
        // Test from completed (no transitions allowed)
        assert!(!board.can_transition("completed", "concept"));
        assert!(!board.can_transition("completed", "active"));
        
        // Test invalid stage names
        assert!(!board.can_transition("invalid", "concept"));
        assert!(!board.can_transition("concept", "invalid"));
    }

    #[test]
    fn test_required_documents_per_stage() {
        let board = CampaignBoard::new();
        
        // Concept stage
        let concept_docs = board.required_documents("concept");
        assert_eq!(concept_docs.len(), 1);
        assert_eq!(concept_docs[0], "campaign_pitch");
        
        // Session Zero stage
        let session_zero_docs = board.required_documents("session_zero");
        assert_eq!(session_zero_docs.len(), 5);
        assert!(session_zero_docs.contains(&"starting_scenario"));
        assert!(session_zero_docs.contains(&"world_primer"));
        assert!(session_zero_docs.contains(&"character_guidelines"));
        assert!(session_zero_docs.contains(&"table_expectations"));
        assert!(session_zero_docs.contains(&"character_integration"));
        
        // Integration stage
        let integration_docs = board.required_documents("integration");
        assert_eq!(integration_docs.len(), 2);
        assert!(integration_docs.contains(&"campaign_bible"));
        assert!(integration_docs.contains(&"major_npc_tracker"));
        
        // Active stage (no required documents)
        assert_eq!(board.required_documents("active").len(), 0);
        assert_eq!(board.required_documents("concluding").len(), 0);
        assert_eq!(board.required_documents("completed").len(), 0);
        
        // Invalid stage
        assert_eq!(board.required_documents("invalid").len(), 0);
    }

    #[test]
    fn test_optional_documents_per_stage() {
        let board = CampaignBoard::new();
        
        // Concept stage - no optional documents (notes and inspiration are tools, not artifacts)
        let concept_optional = board.optional_documents("concept");
        assert_eq!(concept_optional.len(), 0);
        
        // Session Zero stage
        let session_zero_optional = board.optional_documents("session_zero");
        assert_eq!(session_zero_optional.len(), 2);
        assert!(session_zero_optional.contains(&"safety_tools"));
        assert!(session_zero_optional.contains(&"house_rules"));
        
        // Integration stage
        let integration_optional = board.optional_documents("integration");
        assert_eq!(integration_optional.len(), 2);
        assert!(integration_optional.contains(&"player_secrets"));
        assert!(integration_optional.contains(&"faction_overview"));
        
        // Active stage - no documents (managed through session boards)
        let active_optional = board.optional_documents("active");
        assert_eq!(active_optional.len(), 0);
        
        // Stages with no optional documents
        assert_eq!(board.optional_documents("concluding").len(), 0);
        assert_eq!(board.optional_documents("completed").len(), 0);
        assert_eq!(board.optional_documents("invalid").len(), 0);
    }

    #[test]
    fn test_next_stage_progression() {
        let board = CampaignBoard::new();
        
        assert_eq!(board.next_stage("concept"), Some("session_zero"));
        assert_eq!(board.next_stage("session_zero"), Some("integration"));
        assert_eq!(board.next_stage("integration"), Some("active"));
        assert_eq!(board.next_stage("active"), Some("concluding"));
        assert_eq!(board.next_stage("concluding"), Some("completed"));
        assert_eq!(board.next_stage("completed"), None);
        assert_eq!(board.next_stage("invalid"), None);
    }

    #[test]
    fn test_stage_metadata_completeness() {
        let board = CampaignBoard::new();
        
        // Test that all stages have metadata
        for stage in board.stages() {
            let metadata = board.stage_metadata(stage);
            assert!(!metadata.display_name.is_empty());
            assert!(!metadata.description.is_empty());
        }
        
        // Test specific metadata for concept stage
        let concept_meta = board.stage_metadata("concept");
        assert_eq!(concept_meta.display_name, "Concept");
        assert!(concept_meta.description.contains("planning"));
        assert!(concept_meta.completion_message.is_some());
        assert!(concept_meta.transition_prompt.is_some());
        assert!(concept_meta.help_text.is_some());
        
        // Test specific metadata for session_zero stage
        let session_zero_meta = board.stage_metadata("session_zero");
        assert_eq!(session_zero_meta.display_name, "Session Zero");
        assert!(session_zero_meta.description.contains("collaborative"));
        assert!(session_zero_meta.completion_message.is_some());
        assert!(session_zero_meta.transition_prompt.is_some());
        assert!(session_zero_meta.help_text.is_some());
        
        // Test specific metadata for integration stage
        let integration_meta = board.stage_metadata("integration");
        assert_eq!(integration_meta.display_name, "Integration");
        assert!(integration_meta.description.contains("player feedback"));
        assert!(integration_meta.completion_message.is_some());
        assert!(integration_meta.transition_prompt.is_some());
        assert!(integration_meta.help_text.is_some());
        
        // Test specific metadata for active stage
        let active_meta = board.stage_metadata("active");
        assert_eq!(active_meta.display_name, "Active");
        assert!(active_meta.description.contains("actively being played"));
        assert!(active_meta.completion_message.is_none());
        assert!(active_meta.transition_prompt.is_none());
        assert!(active_meta.help_text.is_some());
        
        // Test fallback metadata for unknown stage
        let unknown_meta = board.stage_metadata("unknown");
        assert_eq!(unknown_meta.display_name, "unknown");
        assert_eq!(unknown_meta.description, "The unknown stage");
        assert!(unknown_meta.completion_message.is_none());
        assert!(unknown_meta.transition_prompt.is_none());
        assert!(unknown_meta.help_text.is_none());
    }

    #[test]
    fn test_stage_progression_completeness() {
        let board = CampaignBoard::new();
        let stages = board.stages();
        
        // Verify that each stage (except the last) has a next stage
        for i in 0..stages.len() - 1 {
            let current = stages[i];
            let expected_next = stages[i + 1];
            assert_eq!(board.next_stage(current), Some(expected_next));
        }
        
        // Verify the last stage has no next stage
        assert_eq!(board.next_stage(stages[stages.len() - 1]), None);
    }

    #[test]
    fn test_transition_consistency_with_next_stage() {
        let board = CampaignBoard::new();
        
        // For each stage that has a next stage, verify can_transition agrees
        for stage in board.stages() {
            if let Some(next) = board.next_stage(stage) {
                assert!(
                    board.can_transition(stage, next),
                    "Stage {} should be able to transition to next stage {}",
                    stage,
                    next
                );
            }
        }
    }

    #[test]
    fn test_no_orphaned_transitions() {
        let board = CampaignBoard::new();
        let valid_stages: Vec<&str> = board.stages();
        
        // Test that can_transition only returns true for valid stage pairs
        for from in &valid_stages {
            for to in &valid_stages {
                if board.can_transition(from, to) {
                    // If transition is allowed, verify it matches next_stage
                    assert_eq!(
                        board.next_stage(from),
                        Some(*to),
                        "Transition from {} to {} is allowed but doesn't match next_stage",
                        from,
                        to
                    );
                }
            }
        }
    }
}