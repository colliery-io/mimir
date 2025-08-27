<template>
  <div class="cult-table-container">
    <table class="cult-table">
      <thead>
        <tr>
          <th @click="emit('sort', 'name')" class="sortable">
            Name
            <span class="sort-indicator" v-if="sortColumn === 'name'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th @click="emit('sort', 'item_type')" class="sortable">
            Category
            <span class="sort-indicator" v-if="sortColumn === 'item_type'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th @click="emit('sort', 'subtype')" class="sortable">
            Type
            <span class="sort-indicator" v-if="sortColumn === 'subtype'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th @click="emit('sort', 'source')" class="sortable">
            Source
            <span class="sort-indicator" v-if="sortColumn === 'source'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th>Page</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in items" :key="`${item.name}-${item.source}`" 
            @click="emit('select', item)" class="clickable-row">
          <td class="name-cell">{{ item.name }}</td>
          <td class="category-cell">
            <span :class="['category-badge', getCategoryClass(item.item_type)]">
              {{ item.item_type === 'cult' ? 'Cult' : 'Boon' }}
            </span>
          </td>
          <td class="type-cell">
            <span v-if="item.subtype" :class="['type-badge', getTypeClass(item.subtype)]">
              {{ item.subtype }}
            </span>
            <span v-else>—</span>
          </td>
          <td class="source-cell">{{ item.source }}</td>
          <td class="page-cell">{{ item.page ? `p. ${item.page}` : '—' }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import type { CultBoonSummary } from '../../composables/useCatalog'

defineProps<{
  items: CultBoonSummary[]
  sortColumn: string
  sortDirection: 'asc' | 'desc'
}>()

const emit = defineEmits<{
  sort: [column: string]
  select: [item: CultBoonSummary]
}>()

function getCategoryClass(category: string): string {
  return category === 'cult' ? 'category-cult' : 'category-boon'
}

function getTypeClass(type: string): string {
  const normalized = type.toLowerCase()
  switch (normalized) {
    case 'diabolical':
      return 'type-diabolical'
    case 'demonic':
      return 'type-demonic'
    case 'elder evil':
      return 'type-elder-evil'
    default:
      return 'type-other'
  }
}
</script>

<style scoped>
.cult-table-container {
  width: 100%;
  overflow-x: auto;
}

.cult-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9rem;
}

.cult-table th {
  text-align: left;
  padding: var(--spacing-sm, 8px);
  border-bottom: 2px solid var(--color-border, #333);
  background: var(--color-surface, #1a1a1a);
  color: var(--color-text-secondary, #999);
  white-space: nowrap;
}

.cult-table th.sortable {
  cursor: pointer;
  user-select: none;
}

.cult-table th.sortable:hover {
  color: var(--color-text, #e0e0e0);
}

.sort-indicator {
  display: inline-block;
  margin-left: 4px;
  font-size: 0.8em;
}

.cult-table tbody tr {
  border-bottom: 1px solid var(--color-border-light, #262626);
  transition: background-color 0.15s ease;
}

.cult-table tbody tr:hover {
  background: var(--color-surface-hover, #262626);
}

.clickable-row {
  cursor: pointer;
}

.cult-table td {
  padding: var(--spacing-xs, 4px) var(--spacing-sm, 8px);
  color: var(--color-text, #e0e0e0);
}

/* Cell-specific styles */
.name-cell {
  font-weight: 500;
  color: var(--color-primary, #4a9eff);
}

.category-cell, .type-cell {
  white-space: nowrap;
}

.category-badge, .type-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 0.85rem;
  font-weight: 500;
}

.category-cult {
  background: rgba(156, 39, 176, 0.2);
  color: #9c27b0;
  border: 1px solid rgba(156, 39, 176, 0.4);
}

.category-boon {
  background: rgba(76, 175, 80, 0.2);
  color: #4caf50;
  border: 1px solid rgba(76, 175, 80, 0.4);
}

.type-diabolical {
  background: rgba(244, 67, 54, 0.2);
  color: #f44336;
  border: 1px solid rgba(244, 67, 54, 0.4);
}

.type-demonic {
  background: rgba(255, 87, 34, 0.2);
  color: #ff5722;
  border: 1px solid rgba(255, 87, 34, 0.4);
}

.type-elder-evil {
  background: rgba(103, 58, 183, 0.2);
  color: #673ab7;
  border: 1px solid rgba(103, 58, 183, 0.4);
}

.type-other {
  background: var(--color-surface, #1a1a1a);
  color: var(--color-text-secondary, #999);
  border: 1px solid var(--color-border, #333);
}

.source-cell {
  color: var(--color-text-secondary, #999);
  font-size: 0.85rem;
  white-space: nowrap;
}

.page-cell {
  color: var(--color-text-secondary, #999);
  font-size: 0.85rem;
  text-align: center;
}
</style>