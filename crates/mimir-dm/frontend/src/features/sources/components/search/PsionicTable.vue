<template>
  <div class="table-container">
    <table class="psionic-table">
      <thead>
        <tr>
          <th @click="$emit('sort', 'name')" class="sortable">
            Name
            <span v-if="sortColumn === 'name'" class="sort-indicator">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th @click="$emit('sort', 'type')" class="sortable">
            Type
            <span v-if="sortColumn === 'type'" class="sort-indicator">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th @click="$emit('sort', 'order')" class="sortable">
            Order
            <span v-if="sortColumn === 'order'" class="sort-indicator">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th>Source</th>
        </tr>
      </thead>
      <tbody>
        <tr 
          v-for="psionic in sortedPsionics" 
          :key="`${psionic.name}-${psionic.source}`"
          @click="$emit('select', psionic)"
          class="clickable"
        >
          <td class="psionic-name">{{ psionic.name }}</td>
          <td>
            <span :class="getTypeClass(psionic.psionic_type)" class="type-badge">
              {{ getTypeDisplay(psionic.psionic_type) }}
            </span>
          </td>
          <td>
            <span v-if="psionic.order" :class="getOrderClass(psionic.order)" class="order-badge">
              {{ psionic.order }}
            </span>
            <span v-else class="no-order">—</span>
          </td>
          <td>
            <span class="source">{{ psionic.source }}</span>
            <span v-if="psionic.page" class="page">, p. {{ psionic.page }}</span>
          </td>
        </tr>
      </tbody>
    </table>
    
    <div v-if="psionics.length === 0 && searchPerformed" class="no-results">
      No psionics found matching your criteria.
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { PsionicSummary } from '../../composables/useCatalog'

interface Props {
  psionics: PsionicSummary[]
  searchPerformed: boolean
  sortColumn: string
  sortDirection: 'asc' | 'desc'
}

const props = defineProps<Props>()

defineEmits<{
  'select': [psionic: PsionicSummary]
  'sort': [column: string]
}>()

const sortedPsionics = computed(() => {
  const sorted = [...props.psionics]
  
  sorted.sort((a, b) => {
    let aVal: any
    let bVal: any
    
    switch (props.sortColumn) {
      case 'name':
        aVal = a.name
        bVal = b.name
        break
      case 'type':
        aVal = a.psionic_type
        bVal = b.psionic_type
        break
      case 'order':
        aVal = a.order || 'zzz'
        bVal = b.order || 'zzz'
        break
      default:
        aVal = a.name
        bVal = b.name
    }
    
    const comparison = aVal < bVal ? -1 : aVal > bVal ? 1 : 0
    return props.sortDirection === 'asc' ? comparison : -comparison
  })
  
  return sorted
})

function getTypeDisplay(type: string): string {
  switch(type) {
    case 'D': return 'Discipline'
    case 'T': return 'Talent'
    default: return type
  }
}

function getTypeClass(type: string): string {
  switch(type) {
    case 'D': return 'type-discipline'
    case 'T': return 'type-talent'
    default: return 'type-default'
  }
}

function getOrderClass(order: string): string {
  const orderLower = order.toLowerCase()
  if (orderLower.includes('avatar')) return 'order-avatar'
  if (orderLower.includes('awakened')) return 'order-awakened'
  if (orderLower.includes('immortal')) return 'order-immortal'
  if (orderLower.includes('nomad')) return 'order-nomad'
  if (orderLower.includes('wu jen')) return 'order-wujen'
  return 'order-default'
}
</script>

<style scoped>
.table-container {
  width: 100%;
  overflow-x: auto;
}

.psionic-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9rem;
}

.psionic-table th {
  background: var(--color-surface, #1a1a1a);
  color: var(--color-text-secondary, #999);
  font-weight: 500;
  text-align: left;
  padding: var(--spacing-sm, 8px) var(--spacing-md, 12px);
  border-bottom: 2px solid var(--color-border, #333);
  white-space: nowrap;
  user-select: none;
}

.psionic-table th.sortable {
  cursor: pointer;
}

.psionic-table th.sortable:hover {
  background: var(--color-surface-hover, #242424);
}

.sort-indicator {
  margin-left: var(--spacing-xs, 4px);
  color: var(--color-primary, #4a9eff);
}

.psionic-table td {
  padding: var(--spacing-sm, 8px) var(--spacing-md, 12px);
  border-bottom: 1px solid var(--color-border-subtle, #1a1a1a);
  color: var(--color-text, #e0e0e0);
}

.psionic-table tbody tr.clickable {
  cursor: pointer;
}

.psionic-table tbody tr:hover {
  background: var(--color-surface-hover, #242424);
}

.psionic-name {
  font-weight: 500;
  color: var(--color-primary, #4a9eff);
}

.type-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 3px;
  font-size: 0.85rem;
  font-weight: 500;
}

.type-discipline {
  background: rgba(74, 158, 255, 0.2);
  color: #4a9eff;
}

.type-talent {
  background: rgba(76, 175, 80, 0.2);
  color: #4caf50;
}

.type-default {
  background: rgba(158, 158, 158, 0.2);
  color: #9e9e9e;
}

.order-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 3px;
  font-size: 0.85rem;
  font-weight: 500;
}

.order-avatar {
  background: rgba(156, 39, 176, 0.2);
  color: #9c27b0;
}

.order-awakened {
  background: rgba(33, 150, 243, 0.2);
  color: #2196f3;
}

.order-immortal {
  background: rgba(255, 152, 0, 0.2);
  color: #ff9800;
}

.order-nomad {
  background: rgba(0, 188, 212, 0.2);
  color: #00bcd4;
}

.order-wujen {
  background: rgba(244, 67, 54, 0.2);
  color: #f44336;
}

.order-default {
  background: rgba(158, 158, 158, 0.2);
  color: #9e9e9e;
}

.no-order {
  color: var(--color-text-dim, #666);
}

.source {
  color: var(--color-text-secondary, #999);
}

.page {
  color: var(--color-text-dim, #666);
  font-size: 0.85rem;
}

.no-results {
  padding: 2rem;
  text-align: center;
  color: var(--color-text-dim, #666);
  font-style: italic;
}
</style>