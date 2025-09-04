<template>
  <div class="catalog-table">
    <div class="catalog-table__content" v-if="traps.length > 0">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th catalog-table__th--sortable" @click="$emit('sort', 'name')">
              Name
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'name'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th">Category</th>
            <th class="catalog-table__th">Type</th>
            <th class="catalog-table__th">Source</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr 
            v-for="trap in traps" 
            :key="`${trap.name}-${trap.source}`"
            class="catalog-table__row catalog-table__row--clickable"
            @click="$emit('select', trap)"
          >
            <td class="catalog-table__td catalog-table__name">
              {{ trap.name }}
              <span v-if="trap.is_srd" class="catalog-table__badge catalog-table__badge--srd">SRD</span>
            </td>
            <td class="catalog-table__td">
              <span :class="['catalog-table__badge', getCategoryClass(trap.category)]">{{ trap.category }}</span>
            </td>
            <td class="catalog-table__td">{{ trap.trap_type }}</td>
            <td class="catalog-table__td">
              <span class="catalog-table__badge catalog-table__badge--source">{{ trap.source }}</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-else-if="searchPerformed" class="catalog-table__empty">
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
/* Custom category colors for traps/hazards */
.category-trap {
  background: rgba(231, 76, 60, 0.2);
  color: #e74c3c;
  border: 1px solid rgba(231, 76, 60, 0.4);
}

.category-hazard {
  background: rgba(243, 156, 18, 0.2);
  color: #f39c12;
  border: 1px solid rgba(243, 156, 18, 0.4);
}

.category-default {
  background: var(--color-surface);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
}
</style>