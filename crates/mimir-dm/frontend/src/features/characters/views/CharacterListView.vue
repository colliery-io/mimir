<template>
  <MainLayout>
    <div class="character-list-view">
      <div class="header">
        <h1 class="page-title">Characters</h1>
        <button @click="createCharacter" class="btn-primary">
          Create Character
        </button>
      </div>

      <!-- Filter Tabs -->
      <div class="filter-tabs">
        <button
          class="filter-tab"
          :class="{ active: characterFilter === 'all' }"
          @click="characterFilter = 'all'"
        >
          All ({{ allCharactersCount }})
        </button>
        <button
          class="filter-tab"
          :class="{ active: characterFilter === 'pc' }"
          @click="characterFilter = 'pc'"
        >
          Player Characters ({{ pcCount }})
        </button>
        <button
          class="filter-tab"
          :class="{ active: characterFilter === 'npc' }"
          @click="characterFilter = 'npc'"
        >
          NPCs ({{ npcCount }})
        </button>
      </div>

      <div v-if="characterStore.loading" class="loading">
        Loading characters...
      </div>

      <div v-else-if="characterStore.error" class="error-message">
        {{ characterStore.error }}
      </div>

      <EmptyState
        v-else-if="characters.length === 0"
        variant="characters"
        title="No characters yet"
        description="Create your first character to get started on your adventure"
      >
        <template #action>
          <button @click="createCharacter" class="btn-primary">
            Create Character
          </button>
        </template>
      </EmptyState>

      <div v-else class="character-sections">
        <!-- Unassigned Characters -->
        <div v-if="unassignedCharacters.length > 0" class="character-section">
          <h2 class="section-title">Unassigned Characters</h2>
          <div class="character-grid">
            <div
              v-for="character in unassignedCharacters"
              :key="character.id"
              class="character-card"
              :class="{ 'is-npc': character.is_npc }"
              @click="viewCharacter(character)"
            >
              <div class="character-header">
                <h3 class="character-name">{{ character.character_name }}</h3>
                <span v-if="character.is_npc" class="npc-badge">NPC</span>
              </div>
              <div class="character-class-race">
                Level {{ character.current_level }} {{ character.race || '' }} {{ character.class || '' }}
              </div>
              <div class="character-meta">
                <span class="character-player">{{ getPlayerName(character.player_id) }}</span>
              </div>
              <div class="character-actions" @click.stop>
                <select
                  class="campaign-select"
                  @change="assignToCampaign(character.id, $event)"
                >
                  <option value="">Add to Campaign...</option>
                  <option
                    v-for="campaign in campaignStore.campaigns"
                    :key="campaign.id"
                    :value="campaign.id"
                  >
                    {{ campaign.name }}
                  </option>
                </select>
              </div>
            </div>
          </div>
        </div>

        <!-- Campaign Characters -->
        <div v-for="(chars, campaignId) in charactersByCampaign" :key="campaignId" class="character-section">
          <h2 class="section-title">{{ getCampaignName(Number(campaignId)) }}</h2>
          <div class="character-grid">
            <div
              v-for="character in chars"
              :key="character.id"
              class="character-card"
              :class="{ 'is-npc': character.is_npc }"
              @click="viewCharacter(character)"
            >
              <div class="character-header">
                <h3 class="character-name">{{ character.character_name }}</h3>
                <span v-if="character.is_npc" class="npc-badge">NPC</span>
              </div>
              <div class="character-class-race">
                Level {{ character.current_level }} {{ character.race || '' }} {{ character.class || '' }}
              </div>
              <div class="character-meta">
                <span class="character-player">{{ getPlayerName(character.player_id) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Character Creation Wizard -->
    <CharacterCreationWizard
      :visible="showWizard"
      @close="handleWizardClose"
      @created="handleCharacterCreated"
    />
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import MainLayout from '../../../shared/components/layout/MainLayout.vue'
import CharacterCreationWizard from '../components/CharacterCreationWizard.vue'
import EmptyState from '../../../shared/components/ui/EmptyState.vue'
import { useCharacterStore } from '../../../stores/characters'
import { usePlayerStore } from '../../../stores/players'
import { useCampaignStore } from '../../../stores/campaigns'
import type { Character } from '../../../types/character'

const router = useRouter()
const characterStore = useCharacterStore()
const playerStore = usePlayerStore()
const campaignStore = useCampaignStore()

onMounted(async () => {
  // Load all data
  await Promise.all([
    playerStore.fetchPlayers(),
    campaignStore.fetchCampaigns(),
    characterStore.fetchAllCharacters()
  ])
})

// Filter state
type CharacterFilter = 'all' | 'pc' | 'npc'
const characterFilter = ref<CharacterFilter>('all')

// Counts for filter tabs
const allCharactersCount = computed(() => characterStore.characters.length)
const pcCount = computed(() => characterStore.characters.filter(c => !c.is_npc).length)
const npcCount = computed(() => characterStore.characters.filter(c => c.is_npc).length)

// Filtered characters based on selected filter
const characters = computed(() => {
  const all = characterStore.characters
  switch (characterFilter.value) {
    case 'pc':
      return all.filter(c => !c.is_npc)
    case 'npc':
      return all.filter(c => c.is_npc)
    default:
      return all
  }
})

const unassignedCharacters = computed(() =>
  characters.value.filter(c => c.campaign_id === null)
)

const charactersByCampaign = computed(() => {
  const grouped: Record<number, Character[]> = {}

  characters.value
    .filter(c => c.campaign_id !== null)
    .forEach(character => {
      const campaignId = character.campaign_id!
      if (!grouped[campaignId]) {
        grouped[campaignId] = []
      }
      grouped[campaignId].push(character)
    })

  return grouped
})

const getPlayerName = (playerId: number | null): string => {
  if (playerId === null) {
    return 'NPC'
  }
  const player = playerStore.players.find(p => p.id === playerId)
  return player?.name || 'Unknown Player'
}

const getCampaignName = (campaignId: number): string => {
  const campaign = campaignStore.campaigns.find(c => c.id === campaignId)
  return campaign?.name || 'Unknown Campaign'
}

const showWizard = ref(false)

const createCharacter = () => {
  showWizard.value = true
}

const handleWizardClose = () => {
  showWizard.value = false
}

const handleCharacterCreated = async () => {
  showWizard.value = false
  // Reload characters list
  await characterStore.fetchAllCharacters()
}

const viewCharacter = (character: Character) => {
  router.push(`/characters/${character.id}`)
}

const assignToCampaign = async (characterId: number, event: Event) => {
  const select = event.target as HTMLSelectElement
  const campaignId = parseInt(select.value)

  if (!campaignId) return

  try {
    await invoke('assign_character_to_campaign', {
      characterId,
      campaignId
    })
    // Reload characters to show updated list
    await characterStore.fetchAllCharacters()
  } catch (error) {
    console.error('Failed to assign character to campaign:', error)
    characterStore.error = `Failed to assign character: ${error}`
  }

  // Reset select
  select.value = ''
}
</script>

<style scoped>
.character-list-view {
  @apply space-y-6;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.page-title {
  font-size: 2rem;
  font-weight: 700;
  color: var(--color-text);
}

.loading,
.empty-state {
  text-align: center;
  padding: var(--spacing-xl) 0;
  color: var(--color-text-secondary);
}

.empty-state {
  @apply space-y-4;
}

.empty-subtitle {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.character-sections {
  @apply space-y-8;
}

.character-section {
  @apply space-y-4;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--color-text);
  padding-bottom: var(--spacing-sm);
  border-bottom: 2px solid var(--color-border);
}

.character-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--spacing-lg);
}

.character-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  cursor: pointer;
  transition: all var(--transition-base);
}

.character-card:hover {
  border-color: var(--color-primary-500);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.character-name {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: var(--spacing-xs);
}

.character-class-race {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-primary-500);
  margin-bottom: var(--spacing-sm);
}

.character-meta {
  display: flex;
  gap: var(--spacing-md);
}

.character-player {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.character-actions {
  margin-top: var(--spacing-sm);
  padding-top: var(--spacing-sm);
  border-top: 1px solid var(--color-border);
}

.campaign-select {
  width: 100%;
  padding: var(--spacing-xs) var(--spacing-sm);
  background-color: var(--color-background);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  color: var(--color-text);
  cursor: pointer;
}

.campaign-select:hover {
  border-color: var(--color-primary-500);
}

.campaign-select:focus {
  outline: none;
  border-color: var(--color-primary-500);
  box-shadow: 0 0 0 2px var(--color-primary-500) / 0.2;
}

.btn-primary {
  padding: var(--spacing-sm) var(--spacing-lg);
  background-color: var(--color-primary-500);
  color: var(--color-background);
  border-radius: var(--radius-md);
  border: none;
  font-weight: 500;
  transition: all var(--transition-fast);
  cursor: pointer;
}

.btn-primary:hover {
  background-color: var(--color-primary-600);
  transform: translateY(-1px);
}

.error-message {
  padding: var(--spacing-md);
  background-color: var(--color-error) / 0.1;
  border: 1px solid var(--color-error) / 0.2;
  border-radius: var(--radius-md);
  color: var(--color-error);
}

/* Filter Tabs */
.filter-tabs {
  display: flex;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background-color: var(--color-surface);
  border-radius: var(--radius-lg);
  border: 1px solid var(--color-border);
}

.filter-tab {
  padding: var(--spacing-sm) var(--spacing-md);
  border: none;
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.filter-tab:hover {
  background-color: var(--color-background);
  color: var(--color-text);
}

.filter-tab.active {
  background-color: var(--color-primary-500);
  color: var(--color-background);
}

/* Character Card Header */
.character-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--spacing-sm);
}

.character-header .character-name {
  margin-bottom: 0;
}

/* NPC Badge */
.npc-badge {
  display: inline-block;
  padding: 2px 8px;
  font-size: 0.625rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  background-color: var(--color-warning, #f59e0b);
  color: white;
  border-radius: var(--radius-sm);
  flex-shrink: 0;
}

/* NPC card styling */
.character-card.is-npc {
  border-left: 3px solid var(--color-warning, #f59e0b);
}
</style>
