<template>
  <table class="catalog-table">
    <thead>
      <tr>
        <th class="col-name sortable" @click="$emit('sort', 'name')">
          Name {{ sortIndicator('name') }}
        </th>
        <th class="col-type">
          <select v-model="localFilters.type" @change="updateFilters" class="filter-select">
            <option value="">Type</option>
            <optgroup label="Weapons">
              <option value="M">Melee Weapon</option>
              <option value="R">Ranged Weapon</option>
              <option value="A">Ammunition</option>
            </optgroup>
            <optgroup label="Armor">
              <option value="LA">Light Armor</option>
              <option value="MA">Medium Armor</option>
              <option value="HA">Heavy Armor</option>
              <option value="S">Shield</option>
            </optgroup>
            <optgroup label="Equipment">
              <option value="G">Adventuring Gear</option>
              <option value="AT">Artisan's Tools</option>
              <option value="T">Tools</option>
              <option value="GS">Gaming Set</option>
              <option value="SCF">Spellcasting Focus</option>
              <option value="INS">Instrument</option>
            </optgroup>
            <optgroup label="Transport">
              <option value="MNT">Mount</option>
              <option value="TAH">Tack & Harness</option>
              <option value="VEH">Vehicle</option>
            </optgroup>
            <optgroup label="Other">
              <option value="FD">Food & Drink</option>
              <option value="TG">Trade Good</option>
              <option value="$C">Treasure</option>
            </optgroup>
          </select>
        </th>
        <th class="col-cost sortable" @click="$emit('sort', 'value')">
          Cost {{ sortIndicator('value') }}
        </th>
        <th class="col-weight">Weight</th>
        <th class="col-rarity">
          <select v-model="localFilters.rarity" @change="updateFilters" class="filter-select">
            <option value="">Rarity</option>
            <option value="common">Common</option>
            <option value="uncommon">Uncommon</option>
            <option value="rare">Rare</option>
            <option value="very rare">Very Rare</option>
            <option value="legendary">Legendary</option>
            <option value="artifact">Artifact</option>
          </select>
        </th>
        <th class="col-source sortable" @click="$emit('sort', 'source')">
          Source {{ sortIndicator('source') }}
        </th>
      </tr>
    </thead>
    <tbody>
      <tr v-if="items.length === 0">
        <td :colspan="6" class="no-results">
          <span v-if="searchPerformed">No items found matching your criteria</span>
          <span v-else>Search for items to see results</span>
        </td>
      </tr>
      <tr
        v-for="item in sortedItems"
        :key="`${item.name}-${item.source}`"
        @click="$emit('select', item)"
      >
        <td class="col-name">{{ item.name }}</td>
        <td class="col-type">{{ item.typeName }}</td>
        <td class="col-cost">{{ formatCost(item.value) }}</td>
        <td class="col-weight">{{ formatWeight(item.weight) }}</td>
        <td class="col-rarity">{{ formatRarity(item.rarity) }}</td>
        <td class="col-source">{{ item.source }}</td>
      </tr>
    </tbody>
  </table>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { ItemSummary } from '../../composables/useCatalog'

interface Props {
  items: ItemSummary[]
  searchPerformed: boolean
  sortColumn: string
  sortDirection: 'asc' | 'desc'
  showRarity?: boolean  // For magic items
}

const props = withDefaults(defineProps<Props>(), {
  showRarity: false
})

const emit = defineEmits<{
  select: [item: ItemSummary]
  sort: [column: string]
}>()

// Local filter state
const localFilters = ref({
  type: '',
  rarity: ''
})

function updateFilters() {
  // Filters are applied locally in the computed property
}

const sortedItems = computed(() => {
  let filtered = [...props.items]
  
  // Apply type filter
  if (localFilters.value.type) {
    filtered = filtered.filter(item => item.itemType === localFilters.value.type)
  }
  
  // Apply rarity filter
  if (localFilters.value.rarity) {
    filtered = filtered.filter(item => item.rarity === localFilters.value.rarity)
  }
  
  // Apply sorting
  if (!['name', 'value', 'source'].includes(props.sortColumn)) {
    return filtered
  }
  
  const sorted = [...filtered]
  sorted.sort((a, b) => {
    if (props.sortColumn === 'value') {
      const aVal = a.value || 0
      const bVal = b.value || 0
      const comparison = aVal - bVal
      return props.sortDirection === 'asc' ? comparison : -comparison
    } else {
      const aVal = a[props.sortColumn as keyof ItemSummary] as string
      const bVal = b[props.sortColumn as keyof ItemSummary] as string
      const comparison = aVal.localeCompare(bVal)
      return props.sortDirection === 'asc' ? comparison : -comparison
    }
  })
  
  return sorted
})

function formatCost(value?: number): string {
  if (!value) return '—'
  if (value >= 100) {
    return `${value / 100} gp`
  } else if (value >= 10) {
    return `${value / 10} sp`
  } else {
    return `${value} cp`
  }
}

function formatWeight(weight?: number): string {
  if (!weight) return '—'
  return `${weight} lb`
}

function formatRarity(rarity?: string): string {
  if (!rarity || rarity === 'none') return '—'
  return rarity.charAt(0).toUpperCase() + rarity.slice(1)
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

.filter-select {
  width: 100%;
  padding: 2px 4px;
  background: var(--color-background, #0d0d0d);
  border: 1px solid var(--color-border, #333);
  border-radius: 3px;
  color: var(--color-text, #e0e0e0);
  font-size: 0.875rem;
  font-weight: normal;
  cursor: pointer;
}

.filter-select:focus {
  outline: none;
  border-color: var(--color-primary, #4a9eff);
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
.col-name { width: 35%; }
.col-type { width: 20%; }
.col-cost { width: 12%; }
.col-weight { width: 10%; }
.col-rarity { width: 13%; }
.col-source { width: 10%; }
</style>