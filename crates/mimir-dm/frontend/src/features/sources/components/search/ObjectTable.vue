<template>
  <div class="catalog-table">
    <div class="catalog-table__content" v-if="objects.length > 0">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th catalog-table__th--sortable" @click="$emit('sort', 'name')">
              Name
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'name'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th">Type</th>
            <th class="catalog-table__th">Size</th>
            <th class="catalog-table__th">AC</th>
            <th class="catalog-table__th">HP</th>
            <th class="catalog-table__th">Source</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr 
            v-for="obj in objects" 
            :key="`${obj.name}-${obj.source}`"
            class="catalog-table__row catalog-table__row--clickable"
            @click="$emit('select', obj)"
          >
            <td class="catalog-table__td catalog-table__name">
              {{ obj.name }}
              <span v-if="obj.is_srd" class="catalog-table__badge catalog-table__badge--srd">SRD</span>
            </td>
            <td class="catalog-table__td">
              <span :class="['catalog-table__badge', getTypeClass(obj.object_type)]">{{ obj.object_type }}</span>
            </td>
            <td class="catalog-table__td catalog-table__center">{{ obj.size }}</td>
            <td class="catalog-table__td catalog-table__center">
              <span class="stat-value">{{ obj.ac }}</span>
            </td>
            <td class="catalog-table__td catalog-table__center">
              <span class="stat-value">{{ obj.hp }}</span>
            </td>
            <td class="catalog-table__td">
              <span class="catalog-table__badge catalog-table__badge--source">{{ obj.source }}</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-else-if="searchPerformed" class="catalog-table__empty">
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
/* Custom object type colors */
.type-siege {
  background: rgba(231, 76, 60, 0.2);
  color: #e74c3c;
  border: 1px solid rgba(231, 76, 60, 0.4);
}

.type-generic {
  background: rgba(149, 165, 166, 0.2);
  color: #95a5a6;
  border: 1px solid rgba(149, 165, 166, 0.4);
  font-style: italic;
}

.type-default {
  background: var(--color-surface);
  color: var(--color-text);
  border: 1px solid var(--color-border);
}

/* Object-specific styling */
.stat-value {
  font-family: monospace;
  font-weight: 500;
}
</style>