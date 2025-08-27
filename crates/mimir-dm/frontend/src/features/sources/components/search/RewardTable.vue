<template>
  <div class="reward-table-container">
    <table class="reward-table">
      <thead>
        <tr>
          <th @click="emit('sort', 'name')" class="sortable">
            Name
            <span class="sort-indicator" v-if="sortColumn === 'name'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th @click="emit('sort', 'type')" class="sortable">
            Type
            <span class="sort-indicator" v-if="sortColumn === 'type'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th>Description</th>
          <th>Prerequisites</th>
          <th @click="emit('sort', 'source')" class="sortable">
            Source
            <span class="sort-indicator" v-if="sortColumn === 'source'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="reward in rewards" :key="`${reward.name}-${reward.source}`" 
            @click="emit('select', reward)" class="clickable-row">
          <td class="name-cell">{{ reward.name }}</td>
          <td class="type-cell">
            <span :class="['type-badge', getTypeClass(reward.reward_type)]">
              {{ reward.reward_type }}
            </span>
          </td>
          <td class="description-cell">{{ reward.description }}</td>
          <td class="prereq-cell">
            <span v-if="reward.has_prerequisites" class="prereq-icon" title="Has prerequisites">
              ✓
            </span>
            <span v-else class="no-prereq">—</span>
          </td>
          <td class="source-cell">{{ reward.source }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import type { RewardSummary } from '../../composables/useCatalog'

defineProps<{
  rewards: RewardSummary[]
  sortColumn: string
  sortDirection: 'asc' | 'desc'
}>()

const emit = defineEmits<{
  sort: [column: string]
  select: [reward: RewardSummary]
}>()

function getTypeClass(type: string): string {
  if (!type) return 'type-default'
  
  switch (type.toLowerCase()) {
    case 'blessing':
      return 'type-blessing'
    case 'epic boon':
    case 'boon':
      return 'type-boon'
    case 'charm':
      return 'type-charm'
    case 'feat':
      return 'type-feat'
    default:
      return 'type-default'
  }
}
</script>

<style scoped>
.reward-table-container {
  width: 100%;
  overflow-x: auto;
}

.reward-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9rem;
}

.reward-table th {
  text-align: left;
  padding: var(--spacing-sm, 8px);
  border-bottom: 2px solid var(--color-border, #333);
  background: var(--color-surface, #1a1a1a);
  color: var(--color-text-secondary, #999);
  white-space: nowrap;
}

.reward-table th.sortable {
  cursor: pointer;
  user-select: none;
}

.reward-table th.sortable:hover {
  color: var(--color-text, #e0e0e0);
}

.sort-indicator {
  display: inline-block;
  margin-left: 4px;
  font-size: 0.8em;
}

.reward-table tbody tr {
  border-bottom: 1px solid var(--color-border-light, #262626);
  transition: background-color 0.15s ease;
}

.reward-table tbody tr:hover {
  background: var(--color-surface-hover, #262626);
}

.clickable-row {
  cursor: pointer;
}

.reward-table td {
  padding: var(--spacing-xs, 4px) var(--spacing-sm, 8px);
  color: var(--color-text, #e0e0e0);
}

/* Cell-specific styles */
.name-cell {
  font-weight: 500;
  color: var(--color-primary, #4a9eff);
}

.type-cell {
  white-space: nowrap;
}

.type-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 0.85rem;
  font-weight: 500;
}

.type-blessing {
  background: rgba(255, 215, 0, 0.2);
  color: #ffd700;
  border: 1px solid rgba(255, 215, 0, 0.4);
}

.type-boon {
  background: rgba(147, 112, 219, 0.2);
  color: #9370db;
  border: 1px solid rgba(147, 112, 219, 0.4);
}

.type-charm {
  background: rgba(255, 105, 180, 0.2);
  color: #ff69b4;
  border: 1px solid rgba(255, 105, 180, 0.4);
}

.type-feat {
  background: rgba(70, 130, 180, 0.2);
  color: #4682b4;
  border: 1px solid rgba(70, 130, 180, 0.4);
}

.type-default {
  background: var(--color-surface, #1a1a1a);
  color: var(--color-text-secondary, #999);
  border: 1px solid var(--color-border, #333);
}

.description-cell {
  max-width: 400px;
  font-size: 0.85rem;
  color: var(--color-text-secondary, #999);
  line-height: 1.4;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.prereq-cell {
  text-align: center;
  width: 60px;
}

.prereq-icon {
  color: var(--color-success, #4caf50);
  font-weight: bold;
}

.no-prereq {
  color: var(--color-text-secondary, #999);
  opacity: 0.5;
}

.source-cell {
  color: var(--color-text-secondary, #999);
  font-size: 0.85rem;
  white-space: nowrap;
}
</style>