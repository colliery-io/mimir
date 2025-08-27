<template>
  <div class="language-table">
    <table v-if="languages.length > 0">
      <thead>
        <tr>
          <th @click="$emit('sort', 'name')">
            Name
            <span class="sort-indicator" v-if="sortColumn === 'name'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th>Type</th>
          <th>Script</th>
          <th>Typical Speakers</th>
          <th>Source</th>
        </tr>
      </thead>
      <tbody>
        <tr 
          v-for="lang in languages" 
          :key="`${lang.name}-${lang.source}`"
          @click="$emit('select', lang)"
        >
          <td>
            <span class="language-name">{{ lang.name }}</span>
            <span v-if="lang.is_srd" class="srd-badge">SRD</span>
          </td>
          <td>
            <span :class="getTypeClass(lang.language_type)">{{ lang.language_type }}</span>
          </td>
          <td>{{ lang.script }}</td>
          <td class="speakers-cell">{{ lang.typical_speakers }}</td>
          <td>
            <span class="source-badge">{{ lang.source }}</span>
          </td>
        </tr>
      </tbody>
    </table>
    <div v-else-if="searchPerformed" class="no-results">
      No languages found matching your search criteria.
    </div>
  </div>
</template>

<script setup lang="ts">
import type { LanguageSummary } from '../../composables/useCatalog'

interface Props {
  languages: LanguageSummary[]
  searchPerformed: boolean
  sortColumn: string | null
  sortDirection: 'asc' | 'desc'
}

const props = defineProps<Props>()

const emit = defineEmits<{
  sort: [column: string]
  select: [lang: LanguageSummary]
}>()

function getTypeClass(type: string): string {
  if (!type) return 'type-standard'
  
  switch (type.toLowerCase()) {
    case 'standard':
      return 'type-standard'
    case 'exotic':
      return 'type-exotic'
    case 'secret':
      return 'type-secret'
    case 'dead':
      return 'type-dead'
    case 'primordial dialect':
      return 'type-primordial'
    default:
      return 'type-default'
  }
}
</script>

<style scoped>
.language-table {
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

.language-name {
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

.type-standard {
  color: var(--color-text, #e0e0e0);
}

.type-exotic {
  color: #f39c12;
  font-weight: 500;
}

.type-secret {
  color: #9b59b6;
  font-weight: 500;
}

.type-dead {
  color: #7f8c8d;
  font-style: italic;
}

.type-primordial {
  color: #3498db;
  font-weight: 500;
}

.type-default {
  color: var(--color-text-secondary, #999);
}

.speakers-cell {
  max-width: 300px;
  font-size: 0.85rem;
  color: var(--color-text-secondary, #999);
  line-height: 1.4;
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