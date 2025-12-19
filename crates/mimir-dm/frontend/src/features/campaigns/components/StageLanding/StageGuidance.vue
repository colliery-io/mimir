<template>
  <div class="stage-content-section">
    <!-- Concept Stage -->
    <div v-if="stage === 'concept'" class="stage-concept">
      <div class="activity-section">
        <div class="tip-card">
          <h4>Focus on Three Elements</h4>
          <ol>
            <li>
              <strong>Core Conflict:</strong> The fundamental tension driving your campaign
              <br><em>Example: "Ancient seals are weakening, releasing primordial titans" or "Two merchant houses vie for control of magical trade routes"</em>
            </li>
            <li>
              <strong>Unique Element:</strong> What makes this campaign special
              <br><em>Example: "Set in a city built on a massive dragon turtle's back" or "Magic only works during eclipses"</em>
            </li>
            <li>
              <strong>Player Role:</strong> How the PCs will shape the story
              <br><em>Example: "Agents of a secret order maintaining the balance" or "Founders of a new frontier settlement"</em>
            </li>
          </ol>
        </div>
      </div>
    </div>

    <!-- Session Zero Stage -->
    <div v-else-if="stage === 'session_zero'" class="stage-session-zero">
      <div class="activity-section">
        <h3>Prepare Your Session Zero</h3>
        <p>Create player-facing materials that bridge your vision with their expectations.</p>

        <div class="tip-card">
          <h4>Essential Session Zero Documents</h4>
          <p>Prepare these documents before meeting with your players:</p>
          <ol>
            <li>
              <strong>Starting Scenario:</strong> Where they are and what brings them together
              <br><em>Example: "You've been in the Foreign Quarter for two weeks when the Merchant Guild offers a job..."</em>
            </li>
            <li>
              <strong>World Primer:</strong> Essential setting knowledge their characters would have
              <br><em>Example: "The five major clans, why the forges are failing, common rumors..."</em>
            </li>
            <li>
              <strong>Table Expectations:</strong> How we'll play together safely and enjoyably
              <br><em>Example: "PG-13 rating, collaborative storytelling, attendance expectations, safety tools..."</em>
            </li>
            <li>
              <strong>Character Guidelines:</strong> How to build characters that fit the campaign
              <br><em>Example: "You're outsiders with unique skills, levels 1-3, standard array..."</em>
            </li>
            <li>
              <strong>Character Integration:</strong> Questions to connect characters to the world
              <br><em>Example: "Why did you come to Ironhold? What skills make you valuable? Who do you trust?"</em>
            </li>
          </ol>
        </div>
      </div>
    </div>

    <!-- Integration Stage -->
    <div v-else-if="stage === 'integration'" class="stage-integration">
      <div class="activity-section">
        <h3>Transform Session Zero Into Your Campaign</h3>
        <p>Take player contributions from Session Zero and weave them into your world.</p>

        <h4>Post-Session Zero Tasks</h4>
        <p>Review what your players created and make it official:</p>
        <ol>
          <li>
            <strong>Campaign Bible:</strong> Compile all world information into one master reference
            <br><em>Example: "Incorporate player-created NPCs, locations they mentioned, backstory elements..."</em>
          </li>
          <li>
            <strong>NPCs:</strong> Create important NPCs using the Character Wizard (for full character sheets) or document narrative NPCs
            <br><em>Example: "The merchant who cheated Tom's rogue, Marcus's old commander, Lisa's temple contacts..."</em>
            <br><small>Tip: Use Characters → Create Character → NPC for NPCs that need stats and equipment</small>
          </li>
          <li>
            <strong>First Module Hooks:</strong> Create personal stakes for each character
            <br><em>Example: "Sarah's wizard notices forge-magic anomalies, Tom's contacts go silent..."</em>
          </li>
        </ol>
        <p class="integration-note">This stage transforms collaborative ideas into playable content.</p>
      </div>
    </div>

    <!-- Concluding Stage -->
    <div v-else-if="stage === 'concluding'" class="stage-concluding">
      <div class="activity-section">
        <h3>Wrapping Up Your Campaign</h3>
        <p>Time to bring your epic story to a satisfying conclusion.</p>

        <div class="tip-card">
          <h4>Final Story Arcs</h4>
          <ul>
            <li>Resolve all major plot threads</li>
            <li>Give each character a meaningful ending</li>
            <li>Address unfinished business</li>
            <li>Provide closure for your players</li>
          </ul>
        </div>
      </div>
    </div>

    <!-- Completed Stage -->
    <div v-else-if="stage === 'completed'" class="stage-completed">
      <div class="activity-section">
        <div class="completion-card">
          <h3>🎉 Campaign Complete!</h3>
          <p>Congratulations on completing your campaign!</p>

          <div class="stats-summary mt-4">
            <h4>Campaign Statistics</h4>
            <div class="campaign-stats-card">
              <div class="campaign-stat">
                <span class="stat-value">{{ totalModules }}</span>
                <span class="stat-label">Modules</span>
              </div>
              <div class="campaign-stat">
                <span class="stat-value">{{ totalSessions }}</span>
                <span class="stat-label">Sessions</span>
              </div>
              <div class="campaign-stat">
                <span class="stat-value">{{ totalDocuments }}</span>
                <span class="stat-label">Documents</span>
              </div>
            </div>
          </div>
        </div>

        <div class="info-card mt-4">
          <p>All campaign materials remain available for reference. You can review session notes, documents, and campaign materials anytime.</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  stage: string
  totalModules: number
  totalSessions: number
  totalDocuments: number
}>()
</script>
