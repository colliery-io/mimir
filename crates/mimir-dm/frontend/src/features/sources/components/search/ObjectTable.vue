<template>
  <div class="object-table">
    <table v-if="objects.length > 0">
      <thead>
        <tr>
          <th @click="$emit('sort', 'name')">
            Name
            <span class="sort-indicator" v-if="sortColumn === 'name'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th>Type</th>
          <th>Size</th>
          <th>AC</th>
          <th>HP</th>
          <th>Source</th>
        </tr>
      </thead>
      <tbody>
        <tr 
          v-for="obj in objects" 
          :key="`${obj.name}-${obj.source}`"
          @click="$emit('select', obj)"
        >
          <td>
            <span class="object-name">{{ obj.name }}</span>
            <span v-if="obj.is_srd" class="srd-badge">SRD</span>
          </td>
          <td>
            <span :class="getTypeClass(obj.object_type)">{{ obj.object_type }}</span>
          </td>
          <td>{{ obj.size }}</td>
          <td>
            <span class="stat-value">{{ obj.ac }}</span>
          </td>
          <td>
            <span class="stat-value">{{ obj.hp }}</span>
          </td>
          <td>
            <span class="source-badge">{{ obj.source }}</span>
          </td>
        </tr>
      </tbody>
    </table>
    <div v-else-if="searchPerformed" class="no-results">
      No objects found matching your search criteria.
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ObjectSummary } from '../../composables/useCatalog'

interface Props {
  objects: ObjectSummary[]
  searchPerformed: boolean
  sortColumn: string | null
  sortDirection: 'asc' | 'desc'
}

const props = defineProps<Props>()

const emit = defineEmits<{
  sort: [column: string]
  select: [obj: ObjectSummary]
}>()

function getTypeClass(type: string): string {
  switch (type) {
    case 'Siege Weapon':
      return 'type-siege'
    case 'Generic':
      return 'type-generic'
    default:
      return 'type-default'
  }
}
</script>

<style scoped>
.object-table {
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

.object-name {
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

.type-siege {
  color: #e74c3c;
  font-weight: 500;
}

.type-generic {
  color: #95a5a6;
  font-style: italic;
}

.type-default {
  color: var(--color-text, #e0e0e0);
}

.stat-value {
  font-family: monospace;
  font-weight: 500;
}

.source-badge {
  display: inline-block;
  padding: 2px 6px;
  background: var(--color-surface, #1a1a1a);
  border: 1px solid var(--color-border, #333);
  border-radius: 3px;
  font-size: 0.8rem;
  font-family: monospace;
}

.no-results {
  padding: var(--spacing-xl, 24px);
  text-align: center;
  color: var(--color-text-secondary, #999);
  font-style: italic;
}
</style>