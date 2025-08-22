<template>
  <table class="catalog-table">
    <thead>
      <tr>
        <th class="col-name sortable" @click="$emit('sort', 'name')">
          Name {{ sortIndicator('name') }}
        </th>
        <th class="col-hit-dice">Hit Dice</th>
        <th class="col-primary">Primary Ability</th>
        <th class="col-saves">Proficiencies</th>
        <th class="col-spellcasting">Spellcasting</th>
        <th class="col-subclass">Subclass</th>
        <th class="col-source">Source</th>
      </tr>
    </thead>
    <tbody>
      <tr v-if="classes.length === 0">
        <td colspan="7" class="no-results">
          <span v-if="searchPerformed">No classes found matching your criteria</span>
          <span v-else>Search for classes to see results</span>
        </td>
      </tr>
      <tr
        v-for="classItem in sortedClasses"
        :key="`${classItem.name}-${classItem.source}`"
        @click="$emit('select', classItem)"
      >
        <td class="col-name">{{ classItem.name }}</td>
        <td class="col-hit-dice">{{ classItem.hitDice }}</td>
        <td class="col-primary">{{ classItem.primaryAbility }}</td>
        <td class="col-saves">{{ classItem.proficiency }}</td>
        <td class="col-spellcasting">
          <span v-if="classItem.spellcastingAbility" class="tag-badge spellcaster">
            {{ formatSpellcasting(classItem.spellcastingAbility) }}
          </span>
          <span v-else class="text-dim">—</span>
        </td>
        <td class="col-subclass">
          <span v-if="classItem.subclassTitle">{{ classItem.subclassTitle }}</span>
          <span v-else class="text-dim">—</span>
        </td>
        <td class="col-source">{{ classItem.source }}</td>
      </tr>
    </tbody>
  </table>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ClassSummary } from '../../composables/useCatalog'

interface Props {
  classes: ClassSummary[]
  searchPerformed: boolean
  sortColumn: string
  sortDirection: 'asc' | 'desc'
  availableSources?: string[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  select: [classItem: ClassSummary]
  sort: [column: string]
}>()

const sortedClasses = computed(() => {
  let filtered = [...props.classes]
  
  // Apply sorting
  if (!['name', 'source', 'hitDice'].includes(props.sortColumn)) {
    return filtered
  }
  
  const sorted = [...filtered]
  sorted.sort((a, b) => {
    const aVal = a[props.sortColumn as keyof ClassSummary] as string
    const bVal = b[props.sortColumn as keyof ClassSummary] as string
    const comparison = aVal.localeCompare(bVal)
    return props.sortDirection === 'asc' ? comparison : -comparison
  })
  
  return sorted
})

function formatSpellcasting(ability: string): string {
  // Format spellcasting ability (e.g., "int" -> "INT")
  return ability.toUpperCase()
}

function sortIndicator(column: string) {
  if (props.sortColumn !== column) return ''
  return props.sortDirection === 'asc' ? '▲' : '▼'
}
</script>

<style scoped>
.catalog-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.875rem;
}

.catalog-table thead {
  position: sticky;
  top: 0;
  background: var(--color-surface, #1a1a1a);
  z-index: 10;
}

.catalog-table th {
  padding: var(--spacing-sm, 8px);
  text-align: left;
  color: var(--color-text, #e0e0e0);
  font-weight: 600;
  border-bottom: 2px solid var(--color-border, #333);
  white-space: nowrap;
}

.catalog-table th.sortable {
  cursor: pointer;
  user-select: none;
}

.catalog-table th.sortable:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
}

.catalog-table tbody tr {
  border-bottom: 1px solid var(--color-border-light, #222);
  transition: background-color 0.1s;
}

.catalog-table tbody tr:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
  cursor: pointer;
}

.catalog-table td {
  padding: var(--spacing-xs, 4px) var(--spacing-sm, 8px);
  color: var(--color-text-secondary, #999);
}

.no-results {
  text-align: center;
  padding: 2rem;
  color: var(--color-text-dim, #666);
  font-style: italic;
}

/* Column widths */
.col-name { width: 20%; }
.col-hit-dice { width: 10%; }
.col-primary { width: 15%; }
.col-saves { width: 15%; }
.col-spellcasting { width: 12%; }
.col-subclass { width: 18%; }
.col-source { width: 10%; }

/* Text styles */
.text-dim {
  color: var(--color-text-dim, #666);
}

/* Tag badges in cells */
.tag-badge {
  display: inline-block;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.75rem;
  font-weight: 500;
}

.tag-badge.spellcaster {
  background: rgba(168, 95, 230, 0.2);
  color: #a85fe6;
  border: 1px solid rgba(168, 95, 230, 0.3);
}
</style>