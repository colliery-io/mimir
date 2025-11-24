<template>
  <div v-if="visible" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h2 class="modal-title">Manage Players</h2>
        <button @click="closeModal" class="close-button">×</button>
      </div>

      <div class="modal-body">
        <!-- Loading State -->
        <div v-if="playerStore.loading" class="loading-message">
          Loading players...
        </div>

        <!-- Error State -->
        <div v-else-if="playerStore.error" class="error-message">
          {{ playerStore.error }}
        </div>

        <!-- Empty State -->
        <div v-else-if="players.length === 0" class="empty-state">
          <p>No players yet</p>
          <p class="empty-subtitle">Add your first player to get started</p>
          <button @click="showAddPlayerDialog" class="primary-button">
            Add Player
          </button>
        </div>

        <!-- Player List -->
        <div v-else class="player-container">
          <div class="player-header">
            <h3>Players ({{ players.length }})</h3>
            <button @click="showAddPlayerDialog" class="primary-button">
              Add Player
            </button>
          </div>

          <div class="player-list">
            <div
              v-for="player in players"
              :key="player.id"
              class="player-item"
            >
              <div class="player-info">
                <div class="player-name">{{ player.name }}</div>
                <div class="player-meta">
                  <span v-if="player.email" class="player-email">
                    {{ player.email }}
                  </span>
                  <span class="player-date">
                    Added: {{ formatDate(player.created_at) }}
                  </span>
                </div>
                <div v-if="player.notes" class="player-notes">
                  {{ player.notes }}
                </div>
              </div>
              <div class="player-actions">
                <button
                  @click="editPlayer(player)"
                  class="edit-button"
                  title="Edit player"
                >
                  Edit
                </button>
                <button
                  @click="confirmDeletePlayer(player)"
                  class="delete-button"
                  title="Delete player"
                >
                  Delete
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Add/Edit Player Dialog -->
  <div v-if="showPlayerDialog" class="modal-overlay" @click="closePlayerDialog">
    <div class="dialog-content" @click.stop>
      <div class="dialog-header">
        <h3 class="dialog-title">
          {{ editingPlayer ? 'Edit Player' : 'Add Player' }}
        </h3>
        <button @click="closePlayerDialog" class="close-button">×</button>
      </div>

      <form @submit.prevent="savePlayer" class="dialog-body">
        <div class="form-group">
          <label for="player-name" class="form-label">
            Name <span class="required">*</span>
          </label>
          <input
            id="player-name"
            v-model="playerForm.name"
            type="text"
            class="form-input"
            placeholder="Enter player name"
            required
            autofocus
          />
        </div>

        <div class="form-group">
          <label for="player-email" class="form-label">Email</label>
          <input
            id="player-email"
            v-model="playerForm.email"
            type="email"
            class="form-input"
            placeholder="player@example.com"
          />
        </div>

        <div class="form-group">
          <label for="player-notes" class="form-label">Notes</label>
          <textarea
            id="player-notes"
            v-model="playerForm.notes"
            class="form-textarea"
            placeholder="Additional notes about the player"
            rows="3"
          ></textarea>
        </div>

        <div v-if="formError" class="form-error">
          {{ formError }}
        </div>

        <div class="dialog-actions">
          <button
            type="button"
            @click="closePlayerDialog"
            class="secondary-button"
          >
            Cancel
          </button>
          <button
            type="submit"
            class="primary-button"
            :disabled="!playerForm.name.trim() || saving"
          >
            {{ saving ? 'Saving...' : editingPlayer ? 'Save Changes' : 'Add Player' }}
          </button>
        </div>
      </form>
    </div>
  </div>

  <!-- Delete Confirmation Dialog -->
  <div v-if="showDeleteDialog" class="modal-overlay" @click="closeDeleteDialog">
    <div class="dialog-content confirm-dialog" @click.stop>
      <div class="dialog-header">
        <h3 class="dialog-title">Confirm Delete</h3>
        <button @click="closeDeleteDialog" class="close-button">×</button>
      </div>

      <div class="dialog-body">
        <p>
          Are you sure you want to delete
          <strong>{{ playerToDelete?.name }}</strong>?
        </p>
        <p class="warning-text">
          This will also delete all characters associated with this player. This
          action cannot be undone.
        </p>
      </div>

      <div class="dialog-actions">
        <button
          type="button"
          @click="closeDeleteDialog"
          class="secondary-button"
        >
          Cancel
        </button>
        <button
          @click="deletePlayer"
          class="danger-button"
          :disabled="deleting"
        >
          {{ deleting ? 'Deleting...' : 'Delete Player' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { usePlayerStore } from '../stores/players'
import type { Player } from '../types/character'

const props = defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

// Store
const playerStore = usePlayerStore()

// State
const showPlayerDialog = ref(false)
const showDeleteDialog = ref(false)
const editingPlayer = ref<Player | null>(null)
const playerToDelete = ref<Player | null>(null)
const saving = ref(false)
const deleting = ref(false)
const formError = ref<string | null>(null)

// Form data
const playerForm = ref({
  name: '',
  email: '',
  notes: ''
})

// Computed
const players = computed(() => playerStore.players)

// Methods
const handleOverlayClick = () => {
  closeModal()
}

const closeModal = () => {
  emit('close')
}

const showAddPlayerDialog = () => {
  editingPlayer.value = null
  playerForm.value = {
    name: '',
    email: '',
    notes: ''
  }
  formError.value = null
  showPlayerDialog.value = true
}

const editPlayer = (player: Player) => {
  editingPlayer.value = player
  playerForm.value = {
    name: player.name,
    email: player.email || '',
    notes: player.notes || ''
  }
  formError.value = null
  showPlayerDialog.value = true
}

const closePlayerDialog = () => {
  showPlayerDialog.value = false
  editingPlayer.value = null
  formError.value = null
}

const savePlayer = async () => {
  if (!playerForm.value.name.trim()) {
    formError.value = 'Player name is required'
    return
  }

  saving.value = true
  formError.value = null

  try {
    if (editingPlayer.value) {
      // Update existing player
      await playerStore.updatePlayer(editingPlayer.value.id, {
        name: playerForm.value.name.trim(),
        email: playerForm.value.email.trim() || null,
        notes: playerForm.value.notes.trim() || null
      })
    } else {
      // Create new player
      await playerStore.createPlayer(
        playerForm.value.name.trim(),
        playerForm.value.email.trim() || undefined,
        playerForm.value.notes.trim() || undefined
      )
    }

    closePlayerDialog()
  } catch (error) {
    formError.value =
      error instanceof Error ? error.message : 'Failed to save player'
  } finally {
    saving.value = false
  }
}

const confirmDeletePlayer = (player: Player) => {
  playerToDelete.value = player
  showDeleteDialog.value = true
}

const closeDeleteDialog = () => {
  showDeleteDialog.value = false
  playerToDelete.value = null
}

const deletePlayer = async () => {
  if (!playerToDelete.value) return

  deleting.value = true

  try {
    await playerStore.deletePlayer(playerToDelete.value.id)
    closeDeleteDialog()
  } catch (error) {
    console.error('Failed to delete player:', error)
    // Keep dialog open on error so user can see what happened
  } finally {
    deleting.value = false
  }
}

const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

// Load players when component becomes visible
watch(
  () => props.visible,
  async (isVisible) => {
    if (isVisible) {
      try {
        await playerStore.fetchPlayers()
      } catch (error) {
        console.error('Failed to load players:', error)
      }
    }
  },
  { immediate: true }
)

// Initial load
onMounted(async () => {
  if (props.visible) {
    try {
      await playerStore.fetchPlayers()
    } catch (error) {
      console.error('Failed to load players:', error)
    }
  }
})
</script>

<style scoped>
/* Modal Overlay */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

/* Modal Content */
.modal-content {
  background: var(--background);
  border-radius: 8px;
  width: 100%;
  max-width: 800px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid var(--border);
}

.modal-title {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: var(--text);
}

.close-button {
  background: none;
  border: none;
  font-size: 32px;
  line-height: 1;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.2s;
}

.close-button:hover {
  color: var(--text);
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

/* States */
.loading-message,
.error-message {
  text-align: center;
  padding: 40px;
  color: var(--text-secondary);
}

.error-message {
  color: var(--error);
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
}

.empty-state p {
  margin: 0 0 8px;
  font-size: 18px;
  color: var(--text);
}

.empty-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin-bottom: 24px !important;
}

/* Player Container */
.player-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.player-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.player-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text);
}

/* Player List */
.player-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.player-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 16px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  transition: border-color 0.2s;
}

.player-item:hover {
  border-color: var(--primary);
}

.player-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.player-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--text);
}

.player-meta {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
  font-size: 13px;
  color: var(--text-secondary);
}

.player-email {
  color: var(--primary);
}

.player-notes {
  font-size: 14px;
  color: var(--text-secondary);
  margin-top: 4px;
  font-style: italic;
}

.player-actions {
  display: flex;
  gap: 8px;
  margin-left: 16px;
}

/* Dialog */
.dialog-content {
  background: var(--background);
  border-radius: 8px;
  width: 100%;
  max-width: 500px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.dialog-content.confirm-dialog {
  max-width: 400px;
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid var(--border);
}

.dialog-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--text);
}

.dialog-body {
  padding: 20px;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--border);
}

/* Form */
.form-group {
  margin-bottom: 20px;
}

.form-label {
  display: block;
  margin-bottom: 6px;
  font-size: 14px;
  font-weight: 500;
  color: var(--text);
}

.required {
  color: var(--error);
}

.form-input,
.form-textarea {
  width: 100%;
  padding: 10px 12px;
  font-size: 14px;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--surface);
  color: var(--text);
  font-family: inherit;
  transition: border-color 0.2s;
}

.form-input:focus,
.form-textarea:focus {
  outline: none;
  border-color: var(--primary);
}

.form-textarea {
  resize: vertical;
  min-height: 60px;
}

.form-error {
  margin-top: 12px;
  padding: 10px;
  background: rgba(220, 38, 38, 0.1);
  border: 1px solid var(--error);
  border-radius: 4px;
  color: var(--error);
  font-size: 14px;
}

.warning-text {
  margin-top: 12px;
  color: var(--warning);
  font-size: 14px;
}

/* Buttons */
.primary-button,
.secondary-button,
.danger-button,
.edit-button,
.delete-button {
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.primary-button {
  background: var(--primary);
  color: white;
}

.primary-button:hover:not(:disabled) {
  background: var(--primary-dark);
}

.primary-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.secondary-button {
  background: var(--surface);
  color: var(--text);
  border: 1px solid var(--border);
}

.secondary-button:hover {
  border-color: var(--text-secondary);
}

.danger-button {
  background: var(--error);
  color: white;
}

.danger-button:hover:not(:disabled) {
  background: var(--error-dark);
}

.danger-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.edit-button {
  background: var(--primary);
  color: white;
  font-size: 13px;
  padding: 6px 12px;
}

.edit-button:hover {
  background: var(--primary-dark);
}

.delete-button {
  background: transparent;
  color: var(--error);
  border: 1px solid var(--error);
  font-size: 13px;
  padding: 6px 12px;
}

.delete-button:hover {
  background: var(--error);
  color: white;
}
</style>
