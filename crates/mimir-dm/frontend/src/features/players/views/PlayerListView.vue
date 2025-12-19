<template>
  <MainLayout>
    <div class="player-list-view">
      <div class="header">
        <h1 class="page-title">Players</h1>
        <button @click="showPlayerManager" class="btn-primary">
          Manage Players
        </button>
      </div>

      <div v-if="playerStore.loading" class="loading">
        Loading players...
      </div>

      <div v-else-if="playerStore.error" class="error-message">
        {{ playerStore.error }}
      </div>

      <div v-else-if="playerStore.players.length === 0" class="empty-state">
        <p>No players found.</p>
        <button @click="showPlayerManager" class="btn-primary">
          Add your first player
        </button>
      </div>

      <div v-else class="player-grid">
        <div
          v-for="player in playerStore.players"
          :key="player.id"
          class="player-card"
          @click="editPlayer(player)"
        >
          <h3 class="player-name">{{ player.name }}</h3>
          <div class="player-meta">
            <span v-if="player.email" class="player-email">
              {{ player.email }}
            </span>
            <span class="player-date">
              Added {{ formatDate(player.created_at) }}
            </span>
          </div>
          <p v-if="player.notes" class="player-notes">{{ player.notes }}</p>
        </div>
      </div>
    </div>

    <!-- Player Manager Modal -->
    <PlayerManager :visible="playerManagerVisible" @close="closePlayerManager" />
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import MainLayout from '../../../shared/components/layout/MainLayout.vue'
import PlayerManager from '../../../components/PlayerManager.vue'
import { usePlayerStore } from '../../../stores/players'
import type { Player } from '../../../types/character'

const playerStore = usePlayerStore()
const playerManagerVisible = ref(false)

onMounted(async () => {
  await playerStore.fetchPlayers()
})

const showPlayerManager = () => {
  playerManagerVisible.value = true
}

const closePlayerManager = () => {
  playerManagerVisible.value = false
}

const editPlayer = (player: Player) => {
  playerStore.setCurrentPlayer(player)
  showPlayerManager()
}

const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  return date.toLocaleDateString()
}
</script>

<style scoped>
.player-list-view {
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

.player-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: var(--spacing-lg);
}

.player-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  cursor: pointer;
  transition: all var(--transition-base);
}

.player-card:hover {
  border-color: var(--color-primary-500);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.player-name {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: var(--spacing-sm);
}

.player-meta {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  margin-bottom: var(--spacing-sm);
}

.player-email {
  font-size: 0.875rem;
  color: var(--color-primary-500);
}

.player-date {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.player-notes {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  font-style: italic;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
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
</style>
