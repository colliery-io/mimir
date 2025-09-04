<template>
  <div class="catalog-table">
    <div class="catalog-table__header">
      <h2 class="catalog-table__title">Items</h2>
      <div class="catalog-table__filters">
        <div class="catalog-table__filter-group">
          <select v-model="localFilters.type" @change="updateFilters" class="catalog-table__filter-select">
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
        </div>
        <div class="catalog-table__filter-group">
          <select v-model="localFilters.rarity" @change="updateFilters" class="catalog-table__filter-select">
            <option value="">Rarity</option>
            <option value="common">Common</option>
            <option value="uncommon">Uncommon</option>
            <option value="rare">Rare</option>
            <option value="very rare">Very Rare</option>
            <option value="legendary">Legendary</option>
            <option value="artifact">Artifact</option>
          </select>
        </div>
      </div>
    </div>
    
    <div class="catalog-table__content">
      <div class="catalog-table__results-info">
        <span class="catalog-table__result-count">{{ sortedItems.length }} items</span>
      </div>
      
      <div class="catalog-table__scroll-container">
        <table class="catalog-table__table">
          <thead>
            <tr>
              <th>
                <div class="catalog-table__sort-header" @click="$emit('sort', 'name')">
                  Name
                  <span class="catalog-table__sort-icon">{{ sortIndicator('name') }}</span>
                </div>
              </th>
              <th>Type</th>
              <th>
                <div class="catalog-table__sort-header" @click="$emit('sort', 'value')">
                  Cost
                  <span class="catalog-table__sort-icon">{{ sortIndicator('value') }}</span>
                </div>
              </th>
              <th>Weight</th>
              <th>Rarity</th>
              <th>
                <div class="catalog-table__sort-header" @click="$emit('sort', 'source')">
                  Source
                  <span class="catalog-table__sort-icon">{{ sortIndicator('source') }}</span>
                </div>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="sortedItems.length === 0" class="catalog-table__empty-row">
              <td colspan="6">
                <div class="catalog-table__empty">
                  <h3 v-if="searchPerformed">No items found</h3>
                  <h3 v-else>Search for items</h3>
                  <p v-if="searchPerformed">No items found matching your criteria</p>
                  <p v-else>Search for items to see results</p>
                </div>
              </td>
            </tr>
            <tr
              v-for="item in sortedItems"
              :key="`${item.name}-${item.source}`"
              class="catalog-table__row"
              @click="$emit('select', item)"
            >
              <td>
                <div class="catalog-table__cell-name">{{ item.name }}</div>
              </td>
              <td>{{ item.typeName }}</td>
              <td>{{ formatCost(item.value) }}</td>
              <td>{{ formatWeight(item.weight) }}</td>
              <td>{{ formatRarity(item.rarity) }}</td>
              <td>{{ item.source }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
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

<!-- All styling now handled by consolidated CSS classes -->
