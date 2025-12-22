<template>
  <Teleport to="body">
    <div v-if="visible" class="modal-overlay" @click.self="$emit('close')">
      <div class="modal-content">
        <div class="modal-header">
          <h2>Quick Add Token</h2>
          <button class="close-btn" @click="$emit('close')">×</button>
        </div>

        <div class="modal-body">
          <!-- Monster Search -->
          <div class="form-group">
            <label>Search Monsters</label>
            <input
              ref="searchInput"
              v-model="searchQuery"
              type="text"
              class="form-input"
              placeholder="Type to search monsters..."
              @input="handleSearch"
            />
          </div>

          <!-- Search Results -->
          <div v-if="loading" class="loading-state">
            Searching...
          </div>

          <div v-else-if="searchResults.length > 0" class="results-list">
            <button
              v-for="monster in searchResults"
              :key="monster.id"
              class="result-item"
              :class="{ selected: selectedMonster?.id === monster.id }"
              @click="selectMonster(monster)"
            >
              <div class="result-main">
                <span class="result-name">{{ monster.name }}</span>
                <span class="result-cr">CR {{ monster.cr }}</span>
              </div>
              <div class="result-meta">
                <span class="result-size">{{ monster.size }}</span>
                <span class="result-type">{{ monster.type }}</span>
              </div>
            </button>
          </div>

          <div v-else-if="searchQuery.length >= 2 && !loading" class="empty-state">
            No monsters found matching "{{ searchQuery }}"
          </div>

          <div v-else class="empty-state">
            Type at least 2 characters to search
          </div>

          <!-- Token Options (shown when monster selected) -->
          <div v-if="selectedMonster" class="token-options">
            <div class="selected-monster">
              <span class="selected-name">{{ selectedMonster.name }}</span>
              <button class="clear-btn" @click="clearSelection">×</button>
            </div>

            <div class="options-row">
              <div class="form-group">
                <label>Token Name</label>
                <input
                  v-model="tokenName"
                  type="text"
                  class="form-input"
                  :placeholder="selectedMonster.name"
                />
              </div>

              <div class="form-group">
                <label>Visible to Players</label>
                <label class="toggle">
                  <input v-model="visibleToPlayers" type="checkbox" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>
          </div>
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" @click="$emit('close')">Cancel</button>
          <button
            class="btn-primary"
            :disabled="!selectedMonster"
            @click="handleAdd"
          >
            Add to Map
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { TokenSize, CreateTokenRequest } from '@/types/api'

interface MonsterResult {
  id: number
  name: string
  cr: string
  size: string
  type: string
}

interface Props {
  visible: boolean
  mapId: number
  gridSizePx: number
}

const props = defineProps<Props>()

const emit = defineEmits<{
  close: []
  'add-token': [request: CreateTokenRequest]
}>()

const searchInput = ref<HTMLInputElement | null>(null)
const searchQuery = ref('')
const searchResults = ref<MonsterResult[]>([])
const loading = ref(false)
const selectedMonster = ref<MonsterResult | null>(null)
const tokenName = ref('')
const visibleToPlayers = ref(true)

let searchTimeout: ReturnType<typeof setTimeout> | null = null

// Focus search input when modal opens
watch(() => props.visible, async (visible) => {
  if (visible) {
    await nextTick()
    searchInput.value?.focus()
    // Reset state
    searchQuery.value = ''
    searchResults.value = []
    selectedMonster.value = null
    tokenName.value = ''
    visibleToPlayers.value = true
  }
})

function handleSearch() {
  if (searchTimeout) clearTimeout(searchTimeout)

  if (searchQuery.value.length < 2) {
    searchResults.value = []
    return
  }

  loading.value = true

  searchTimeout = setTimeout(async () => {
    try {
      const response = await invoke<{ success: boolean; data?: any[] }>('search_monsters', {
        request: { query: searchQuery.value, limit: 15 }
      })

      if (response.success && response.data) {
        searchResults.value = response.data.map(m => ({
          id: m.id,
          name: m.name,
          cr: m.challenge_rating || 'N/A',
          size: m.size || 'Medium',
          type: m.type || 'Unknown'
        }))
      }
    } catch (e) {
      console.error('Failed to search monsters:', e)
    } finally {
      loading.value = false
    }
  }, 300)
}

function selectMonster(monster: MonsterResult) {
  selectedMonster.value = monster
  tokenName.value = monster.name
}

function clearSelection() {
  selectedMonster.value = null
  tokenName.value = ''
}

// Map monster size to token size
function getTokenSize(monsterSize: string): TokenSize {
  const sizeMap: Record<string, TokenSize> = {
    'Tiny': 'tiny',
    'Small': 'small',
    'Medium': 'medium',
    'Large': 'large',
    'Huge': 'huge',
    'Gargantuan': 'gargantuan'
  }
  return sizeMap[monsterSize] || 'medium'
}

function handleAdd() {
  if (!selectedMonster.value) return

  const request: CreateTokenRequest = {
    map_id: props.mapId,
    name: tokenName.value || selectedMonster.value.name,
    token_type: 'monster',
    size: getTokenSize(selectedMonster.value.size),
    x: 0, // Will be set by parent for placement
    y: 0,
    visible_to_players: visibleToPlayers.value,
    monster_id: selectedMonster.value.id
  }

  emit('add-token', request)
  emit('close')
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--color-surface);
  border-radius: var(--radius-lg);
  width: 90%;
  max-width: 480px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--color-border);
}

.modal-header h2 {
  font-size: 1.125rem;
  font-weight: 600;
  margin: 0;
}

.close-btn {
  width: 28px;
  height: 28px;
  border: none;
  background: none;
  color: var(--color-text-muted);
  font-size: 1.25rem;
  cursor: pointer;
  border-radius: var(--radius-sm);
}

.close-btn:hover {
  background: var(--color-base-200);
  color: var(--color-text);
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-lg);
}

.form-group {
  margin-bottom: var(--spacing-md);
}

.form-group label {
  display: block;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--color-text-muted);
  text-transform: uppercase;
  margin-bottom: var(--spacing-xs);
}

.form-input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  background: var(--color-background);
  color: var(--color-text);
  font-size: 0.875rem;
}

.form-input:focus {
  outline: none;
  border-color: var(--color-primary-500);
}

.loading-state,
.empty-state {
  text-align: center;
  padding: var(--spacing-lg);
  color: var(--color-text-muted);
  font-size: 0.875rem;
}

.results-list {
  max-height: 250px;
  overflow-y: auto;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  margin-bottom: var(--spacing-md);
}

.result-item {
  display: block;
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  border: none;
  border-bottom: 1px solid var(--color-border);
  background: var(--color-background);
  text-align: left;
  cursor: pointer;
  transition: background var(--transition-fast);
}

.result-item:last-child {
  border-bottom: none;
}

.result-item:hover {
  background: var(--color-base-200);
}

.result-item.selected {
  background: var(--color-primary-100);
  border-color: var(--color-primary-500);
}

.result-main {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2px;
}

.result-name {
  font-weight: 500;
  color: var(--color-text);
}

.result-cr {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  background: var(--color-base-200);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
}

.result-meta {
  font-size: 0.75rem;
  color: var(--color-text-muted);
}

.result-size {
  margin-right: var(--spacing-sm);
}

.token-options {
  border-top: 1px solid var(--color-border);
  padding-top: var(--spacing-md);
  margin-top: var(--spacing-md);
}

.selected-monster {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--color-primary-100);
  border: 1px solid var(--color-primary-500);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-md);
  margin-bottom: var(--spacing-md);
}

.selected-name {
  font-weight: 600;
  color: var(--color-primary-700);
}

.clear-btn {
  width: 20px;
  height: 20px;
  border: none;
  background: none;
  color: var(--color-primary-700);
  font-size: 1rem;
  cursor: pointer;
  border-radius: var(--radius-sm);
}

.clear-btn:hover {
  background: var(--color-primary-200);
}

.options-row {
  display: flex;
  gap: var(--spacing-md);
}

.options-row .form-group:first-child {
  flex: 1;
}

/* Toggle switch */
.toggle {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
  cursor: pointer;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  inset: 0;
  background: var(--color-base-300);
  border-radius: 12px;
  transition: background var(--transition-fast);
}

.toggle-slider::before {
  content: '';
  position: absolute;
  width: 18px;
  height: 18px;
  left: 3px;
  bottom: 3px;
  background: white;
  border-radius: 50%;
  transition: transform var(--transition-fast);
}

.toggle input:checked + .toggle-slider {
  background: var(--color-primary-500);
}

.toggle input:checked + .toggle-slider::before {
  transform: translateX(20px);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--color-border);
}

.btn-secondary,
.btn-primary {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.btn-secondary {
  border: 1px solid var(--color-border);
  background: var(--color-background);
  color: var(--color-text);
}

.btn-secondary:hover {
  background: var(--color-base-200);
}

.btn-primary {
  border: none;
  background: var(--color-primary-500);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--color-primary-600);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
