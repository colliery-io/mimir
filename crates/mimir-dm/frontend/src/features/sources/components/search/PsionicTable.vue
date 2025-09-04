<template>
  <div class="catalog-table">
    <div class="catalog-table__content">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th catalog-table__th--sortable" @click="$emit('sort', 'name')">
              Name
              <span v-if="sortColumn === 'name'" class="catalog-table__sort-indicator">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th catalog-table__th--sortable" @click="$emit('sort', 'type')">
              Type
              <span v-if="sortColumn === 'type'" class="catalog-table__sort-indicator">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th catalog-table__th--sortable" @click="$emit('sort', 'order')">
              Order
              <span v-if="sortColumn === 'order'" class="catalog-table__sort-indicator">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th">Source</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr 
            v-for="psionic in sortedPsionics" 
            :key="`${psionic.name}-${psionic.source}`"
            class="catalog-table__row catalog-table__row--clickable"
            @click="$emit('select', psionic)"
          >
            <td class="catalog-table__td catalog-table__name">{{ psionic.name }}</td>
            <td class="catalog-table__td">
              <span :class="['catalog-table__badge', getTypeClass(psionic.psionic_type)]">
                {{ getTypeDisplay(psionic.psionic_type) }}
              </span>
            </td>
            <td class="catalog-table__td">
              <span v-if="psionic.order" :class="['catalog-table__badge', getOrderClass(psionic.order)]">
                {{ psionic.order }}
              </span>
              <span v-else class="catalog-table__empty">—</span>
            </td>
            <td class="catalog-table__td catalog-table__source">
              {{ psionic.source }}<span v-if="psionic.page">, p. {{ psionic.page }}</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    
    <div v-if="psionics.length === 0 && searchPerformed" class="catalog-table__empty">
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
/* Custom psionic type colors */
.type-discipline {
  background: rgba(74, 158, 255, 0.2);
  color: #4a9eff;
  border: 1px solid rgba(74, 158, 255, 0.4);
}

.type-talent {
  background: rgba(76, 175, 80, 0.2);
  color: #4caf50;
  border: 1px solid rgba(76, 175, 80, 0.4);
}

.type-default {
  background: rgba(158, 158, 158, 0.2);
  color: #9e9e9e;
  border: 1px solid rgba(158, 158, 158, 0.4);
}

/* Custom order colors */
.order-avatar {
  background: rgba(156, 39, 176, 0.2);
  color: #9c27b0;
  border: 1px solid rgba(156, 39, 176, 0.4);
}

.order-awakened {
  background: rgba(33, 150, 243, 0.2);
  color: #2196f3;
  border: 1px solid rgba(33, 150, 243, 0.4);
}

.order-immortal {
  background: rgba(255, 152, 0, 0.2);
  color: #ff9800;
  border: 1px solid rgba(255, 152, 0, 0.4);
}

.order-nomad {
  background: rgba(0, 188, 212, 0.2);
  color: #00bcd4;
  border: 1px solid rgba(0, 188, 212, 0.4);
}

.order-wujen {
  background: rgba(244, 67, 54, 0.2);
  color: #f44336;
  border: 1px solid rgba(244, 67, 54, 0.4);
}

.order-default {
  background: rgba(158, 158, 158, 0.2);
  color: #9e9e9e;
  border: 1px solid rgba(158, 158, 158, 0.4);
}
</style>