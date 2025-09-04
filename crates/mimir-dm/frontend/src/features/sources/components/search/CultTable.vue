<template>
  <div class="catalog-table">
    <div class="catalog-table__content">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'name')">
              Name
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'name'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'item_type')">
              Category
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'item_type'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'subtype')">
              Type
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'subtype'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'source')">
              Source
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'source'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th">Page</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr v-for="item in items" :key="`${item.name}-${item.source}`" 
              class="catalog-table__row catalog-table__row--clickable" 
              @click="emit('select', item)">
            <td class="catalog-table__td catalog-table__name">{{ item.name }}</td>
            <td class="catalog-table__td">
              <span :class="['catalog-table__badge', getCategoryClass(item.item_type)]">
                {{ item.item_type === 'cult' ? 'Cult' : 'Boon' }}
              </span>
            </td>
            <td class="catalog-table__td">
              <span v-if="item.subtype" :class="['catalog-table__badge', getTypeClass(item.subtype)]">
                {{ item.subtype }}
              </span>
              <span v-else class="catalog-table__empty">—</span>
            </td>
            <td class="catalog-table__td catalog-table__source">{{ item.source }}</td>
            <td class="catalog-table__td catalog-table__page">{{ item.page ? `p. ${item.page}` : '—' }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { CultBoonSummary } from '../../composables/useCatalog'

defineProps<{
  items: CultBoonSummary[]
  sortColumn: string
  sortDirection: 'asc' | 'desc'
}>()

const emit = defineEmits<{
  sort: [column: string]
  select: [item: CultBoonSummary]
}>()

function getCategoryClass(category: string): string {
  return category === 'cult' ? 'category-cult' : 'category-boon'
}

function getTypeClass(type: string): string {
  const normalized = type.toLowerCase()
  switch (normalized) {
    case 'diabolical':
      return 'type-diabolical'
    case 'demonic':
      return 'type-demonic'
    case 'elder evil':
      return 'type-elder-evil'
    default:
      return 'type-other'
  }
}
</script>

<style scoped>
/* Custom badge colors for cult/boon categories */
.category-cult {
  background: rgba(156, 39, 176, 0.2);
  color: #9c27b0;
  border: 1px solid rgba(156, 39, 176, 0.4);
}

.category-boon {
  background: rgba(76, 175, 80, 0.2);
  color: #4caf50;
  border: 1px solid rgba(76, 175, 80, 0.4);
}

.type-diabolical {
  background: rgba(244, 67, 54, 0.2);
  color: #f44336;
  border: 1px solid rgba(244, 67, 54, 0.4);
}

.type-demonic {
  background: rgba(255, 87, 34, 0.2);
  color: #ff5722;
  border: 1px solid rgba(255, 87, 34, 0.4);
}

.type-elder-evil {
  background: rgba(103, 58, 183, 0.2);
  color: #673ab7;
  border: 1px solid rgba(103, 58, 183, 0.4);
}

.type-other {
  background: var(--color-surface);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
}
</style>