<template>
  <div class="deity-table">
    <table v-if="deities.length > 0">
      <thead>
        <tr>
          <th @click="$emit('sort', 'name')">
            Name
            <span class="sort-indicator" v-if="sortColumn === 'name'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th>Title</th>
          <th>Pantheon</th>
          <th>Alignment</th>
          <th>Domains</th>
          <th>Symbol</th>
          <th>Source</th>
        </tr>
      </thead>
      <tbody>
        <tr 
          v-for="deity in deities" 
          :key="`${deity.name}-${deity.source}`"
          @click="$emit('select', deity)"
        >
          <td>
            <span class="deity-name">{{ deity.name }}</span>
            <span v-if="deity.is_srd" class="srd-badge">SRD</span>
          </td>
          <td>{{ deity.title }}</td>
          <td>{{ deity.pantheon }}</td>
          <td>{{ deity.alignment }}</td>
          <td>{{ formatDomains(deity.domains) }}</td>
          <td>{{ deity.symbol }}</td>
          <td>
            <span class="source-badge">{{ deity.source }}</span>
          </td>
        </tr>
      </tbody>
    </table>
    <div v-else-if="searchPerformed" class="no-results">
      No deities found matching your search criteria.
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DeitySummary } from '../../composables/useCatalog'

interface Props {
  deities: DeitySummary[]
  searchPerformed: boolean
  sortColumn: string | null
  sortDirection: 'asc' | 'desc'
}

const props = defineProps<Props>()

const emit = defineEmits<{
  sort: [column: string]
  select: [deity: DeitySummary]
}>()

function formatDomains(domains: string[]): string {
  return domains.join(', ')
}
</script>

<style scoped>
.deity-table {
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

.deity-name {
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