<template>
  <div class="catalog-table">
    <div class="catalog-table__header">
      <h2 class="catalog-table__title">Monsters</h2>
      <div class="catalog-table__filters">
        <div class="catalog-table__filter-group">
          <MultiSelectFilter
            label="Size"
            :options="sizeOptions"
            v-model="localFilters.sizes"
            @change="updateFilters"
          />
        </div>
        <div class="catalog-table__filter-group">
          <MultiSelectFilter
            label="Type"
            :options="typeOptions"
            v-model="localFilters.types"
            @change="updateFilters"
          />
        </div>
      </div>
    </div>
    
    <div class="catalog-table__content">
      <div class="catalog-table__results-info">
        <span class="catalog-table__result-count">{{ sortedMonsters.length }} monsters</span>
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
              <th>Size</th>
              <th>Type</th>
              <th>
                <div class="catalog-table__sort-header" @click="$emit('sort', 'cr')">
                  CR
                  <span class="catalog-table__sort-icon">{{ sortIndicator('cr') }}</span>
                </div>
              </th>
              <th>HP</th>
              <th>AC</th>
              <th>Alignment</th>
              <th>
                <div class="catalog-table__sort-header" @click="$emit('sort', 'source')">
                  Source
                  <span class="catalog-table__sort-icon">{{ sortIndicator('source') }}</span>
                </div>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="sortedMonsters.length === 0" class="catalog-table__empty-row">
              <td colspan="8">
                <div class="catalog-table__empty">
                  <h3 v-if="searchPerformed">No monsters found</h3>
                  <h3 v-else>Search for monsters</h3>
                  <p v-if="searchPerformed">No monsters found matching your criteria</p>
                  <p v-else>Search for monsters to see results</p>
                </div>
              </td>
            </tr>
            <tr
              v-for="monster in sortedMonsters"
              :key="`${monster.name}-${monster.source}`"
              class="catalog-table__row"
              @click="$emit('select', monster)"
            >
              <td>
                <div class="catalog-table__cell-name">{{ monster.name }}</div>
              </td>
              <td>{{ formatSize(monster.size) }}</td>
              <td>{{ monster.type }}</td>
              <td>{{ monster.cr }}</td>
              <td>{{ monster.hp }}</td>
              <td>{{ monster.ac }}</td>
              <td>{{ monster.alignment }}</td>
              <td>{{ monster.source }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { MonsterSummary } from '../../composables/useCatalog'
import MultiSelectFilter from '../../../../shared/components/ui/MultiSelectFilter.vue'

interface Props {
  monsters: MonsterSummary[]
  searchPerformed: boolean
  sortColumn: string
  sortDirection: 'asc' | 'desc'
  filters?: {
    sizes?: string[]
    types?: string[]
  }
}

const props = withDefaults(defineProps<Props>(), {
  filters: () => ({})
})

const emit = defineEmits<{
  select: [monster: MonsterSummary]
  sort: [column: string]
  filterUpdate: [filters: { sizes?: string[], types?: string[] }]
}>()

const sizeOptions = ['T', 'S', 'M', 'L', 'H', 'G']

const typeOptions = [
  'aberration',
  'beast',
  'celestial',
  'construct',
  'dragon',
  'elemental',
  'fey',
  'fiend',
  'giant',
  'humanoid',
  'monstrosity',
  'ooze',
  'plant',
  'undead'
]

const localFilters = ref({
  sizes: props.filters?.sizes || [],
  types: props.filters?.types || []
})

watch(() => props.filters, (newFilters) => {
  if (newFilters) {
    localFilters.value = {
      sizes: newFilters.sizes || [],
      types: newFilters.types || []
    }
  }
}, { deep: true })

const sortedMonsters = computed(() => {
  let filtered = [...props.monsters]
  
  // Apply filters
  if (localFilters.value.sizes && localFilters.value.sizes.length > 0) {
    filtered = filtered.filter(m => {
      // Backend returns full names like "Tiny", but filter uses codes like "T"
      // Create a mapping to convert filter codes to full names
      const sizeMap: Record<string, string> = {
        'T': 'Tiny',
        'S': 'Small',
        'M': 'Medium',
        'L': 'Large',
        'H': 'Huge',
        'G': 'Gargantuan'
      }
      const filterSizeNames = localFilters.value.sizes.map(code => sizeMap[code] || code)
      const monsterSize = Array.isArray(m.size) ? m.size[0] : m.size
      return filterSizeNames.includes(monsterSize)
    })
  }
  if (localFilters.value.types && localFilters.value.types.length > 0) {
    filtered = filtered.filter(m => localFilters.value.types.includes(m.type))
  }
  
  // Apply sorting
  if (['name', 'cr', 'source'].includes(props.sortColumn)) {
    filtered.sort((a, b) => {
      if (props.sortColumn === 'cr') {
        // Parse CR for numeric sorting
        const crToNum = (cr: string) => {
          if (cr === '1/8') return 0.125
          if (cr === '1/4') return 0.25
          if (cr === '1/2') return 0.5
          return parseFloat(cr) || 0
        }
        const comparison = crToNum(a.cr) - crToNum(b.cr)
        return props.sortDirection === 'asc' ? comparison : -comparison
      } else {
        const aVal = a[props.sortColumn as keyof MonsterSummary] as string
        const bVal = b[props.sortColumn as keyof MonsterSummary] as string
        const comparison = aVal.localeCompare(bVal)
        return props.sortDirection === 'asc' ? comparison : -comparison
      }
    })
  }
  
  return filtered
})

function formatSize(size: string | string[]): string {
  const sizeValue = Array.isArray(size) ? size[0] : size
  const sizeMap: Record<string, string> = {
    'T': 'Tiny',
    'S': 'Small',
    'M': 'Medium',
    'L': 'Large',
    'H': 'Huge',
    'G': 'Gargantuan'
  }
  return sizeMap[sizeValue] || sizeValue
}

function updateFilters() {
  emit('filterUpdate', { ...localFilters.value })
}

function sortIndicator(column: string) {
  if (props.sortColumn !== column) return ''
  return props.sortDirection === 'asc' ? '▲' : '▼'
}
</script>

<!-- All styling now handled by consolidated CSS classes -->
