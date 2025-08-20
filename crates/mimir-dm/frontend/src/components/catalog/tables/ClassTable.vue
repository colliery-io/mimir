<template>
  <table class="catalog-table">
    <thead>
      <tr>
        <th class="col-name sortable" @click="$emit('sort', 'name')">
          Name {{ sortIndicator('name') }}
        </th>
        <th class="col-hit-die">Hit Die</th>
        <th class="col-primary-ability">Primary Ability</th>
        <th class="col-saves">Saves</th>
        <th class="col-type">Type</th>
        <th class="col-source sortable" @click="$emit('sort', 'source')">
          Source {{ sortIndicator('source') }}
        </th>
      </tr>
    </thead>
    <tbody>
      <tr v-if="classes.length === 0">
        <td colspan="6" class="no-results">
          <span v-if="searchPerformed">No classes found matching your criteria</span>
          <span v-else>Search for classes to see results</span>
        </td>
      </tr>
      <tr
        v-for="cls in sortedClasses"
        :key="`${cls.name}-${cls.source}`"
        :class="{'class-row': true, 'subclass-row': cls.name.includes(':')}"
        @click="$emit('select', cls)"
      >
        <td class="col-name">
          <span v-if="cls.name.includes(':')" class="subclass-indent">{{ cls.name.split(': ')[1] }}</span>
          <span v-else class="class-name-main">{{ cls.name }}</span>
        </td>
        <td class="col-hit-die">{{ cls.hitDie }}</td>
        <td class="col-primary-ability">{{ cls.primaryAbility }}</td>
        <td class="col-saves">{{ cls.saves }}</td>
        <td class="col-type">{{ cls.description }}</td>
        <td class="col-source">{{ cls.source }}</td>
      </tr>
    </tbody>
  </table>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ClassSummary } from '@/composables/catalog/useCatalog'

interface Props {
  classes: ClassSummary[]
  searchPerformed: boolean
  sortColumn: string
  sortDirection: 'asc' | 'desc'
}

const props = defineProps<Props>()

const emit = defineEmits<{
  select: [cls: ClassSummary]
  sort: [column: string]
}>()

const sortedClasses = computed(() => {
  if (props.sortColumn !== 'name' && props.sortColumn !== 'source') {
    return props.classes
  }
  
  const sorted = [...props.classes]
  sorted.sort((a, b) => {
    let aVal = a[props.sortColumn as keyof ClassSummary] as string
    let bVal = b[props.sortColumn as keyof ClassSummary] as string
    
    const comparison = aVal.localeCompare(bVal)
    return props.sortDirection === 'asc' ? comparison : -comparison
  })
  
  return sorted
})

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

/* Subclass row styling */
.subclass-row {
  background: rgba(255, 255, 255, 0.02);
}

.subclass-row:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.07)) !important;
}

.subclass-indent {
  padding-left: 1.5rem;
  position: relative;
}

.subclass-indent::before {
  content: "└";
  position: absolute;
  left: 0.5rem;
  color: var(--color-text-dim, #666);
}

.class-name-main {
  font-weight: 500;
}

.no-results {
  text-align: center;
  padding: 2rem;
  color: var(--color-text-dim, #666);
  font-style: italic;
}

/* Column widths */
.col-name { width: 25%; }
.col-hit-die { width: 10%; }
.col-primary-ability { width: 20%; }
.col-saves { width: 20%; }
.col-type { width: 15%; }
.col-source { width: 10%; }
</style>