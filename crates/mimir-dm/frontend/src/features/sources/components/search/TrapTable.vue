<template>
  <div class="trap-table">
    <table v-if="traps.length > 0">
      <thead>
        <tr>
          <th @click="$emit('sort', 'name')">
            Name
            <span class="sort-indicator" v-if="sortColumn === 'name'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th>Category</th>
          <th>Type</th>
          <th>Source</th>
        </tr>
      </thead>
      <tbody>
        <tr 
          v-for="trap in traps" 
          :key="`${trap.name}-${trap.source}`"
          @click="$emit('select', trap)"
        >
          <td>
            <span class="trap-name">{{ trap.name }}</span>
            <span v-if="trap.is_srd" class="srd-badge">SRD</span>
          </td>
          <td>
            <span :class="getCategoryClass(trap.category)">{{ trap.category }}</span>
          </td>
          <td>{{ trap.trap_type }}</td>
          <td>
            <span class="source-badge">{{ trap.source }}</span>
          </td>
        </tr>
      </tbody>
    </table>
    <div v-else-if="searchPerformed" class="no-results">
      No traps or hazards found matching your search criteria.
    </div>
  </div>
</template>

<script setup lang="ts">
import type { TrapSummary } from '../../composables/useCatalog'

interface Props {
  traps: TrapSummary[]
  searchPerformed: boolean
  sortColumn: string | null
  sortDirection: 'asc' | 'desc'
}

const props = defineProps<Props>()

const emit = defineEmits<{
  sort: [column: string]
  select: [trap: TrapSummary]
}>()

function getCategoryClass(category: string): string {
  switch (category.toLowerCase()) {
    case 'trap':
      return 'category-trap'
    case 'hazard':
      return 'category-hazard'
    default:
      return 'category-default'
  }
}
</script>

<style scoped>
.trap-table {
  width: 100%;
  overflow-x: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9rem;
}

th {
  background: var(--color-surface, #1a1a1a);
  color: var(--color-text-secondary, #999);
  text-align: left;
  padding: var(--spacing-sm, 8px) var(--spacing-md, 12px);
  border-bottom: 2px solid var(--color-border, #333);
  cursor: pointer;
  user-select: none;
  white-space: nowrap;
}

th:hover {
  background: var(--color-surface-hover, #252525);
}

.sort-indicator {
  margin-left: 4px;
  color: var(--color-primary, #4a9eff);
}

tbody tr {
  cursor: pointer;
  transition: background-color 0.2s;
  border-bottom: 1px solid var(--color-border, #333);
}

tbody tr:hover {
  background: rgba(74, 158, 255, 0.1);
}

td {
  padding: var(--spacing-sm, 8px) var(--spacing-md, 12px);
  color: var(--color-text, #e0e0e0);
  vertical-align: middle;
}

.trap-name {
  font-weight: 500;
  color: var(--color-primary, #4a9eff);
}

.srd-badge {
  display: inline-block;
  padding: 2px 4px;
  margin-left: 4px;
  background: var(--color-primary, #4a9eff);
  color: var(--color-background, #0d0d0d);
  border-radius: 3px;
  font-size: 0.7rem;
  font-weight: 600;
  vertical-align: middle;
}

.category-trap {
  color: #e74c3c;
  font-weight: 500;
}

.category-hazard {
  color: #f39c12;
  font-weight: 500;
}

.category-default {
  color: var(--color-text-secondary, #999);
}

.source-badge {
  display: inline-block;
  padding: 2px 6px;
  background: rgba(74, 158, 255, 0.15);
  color: var(--color-primary, #4a9eff);
  border-radius: 3px;
  font-size: 0.8rem;
  font-weight: 500;
}

.no-results {
  padding: var(--spacing-xl, 24px);
  text-align: center;
  color: var(--color-text-secondary, #999);
  font-style: italic;
}
</style>