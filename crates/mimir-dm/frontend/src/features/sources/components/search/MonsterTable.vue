<template>
  <table class="catalog-table">
    <thead>
      <tr>
        <th class="col-name sortable" @click="$emit('sort', 'name')">
          Name {{ sortIndicator('name') }}
        </th>
        <th class="col-size">
          <MultiSelectFilter
            label="Size"
            :options="sizeOptions"
            v-model="localFilters.sizes"
            @change="updateFilters"
          />
        </th>
        <th class="col-type">
          <MultiSelectFilter
            label="Type"
            :options="typeOptions"
            v-model="localFilters.types"
            @change="updateFilters"
          />
        </th>
        <th class="col-cr sortable" @click="$emit('sort', 'cr')">
          CR {{ sortIndicator('cr') }}
        </th>
        <th class="col-hp">HP</th>
        <th class="col-ac">AC</th>
        <th class="col-alignment">Alignment</th>
        <th class="col-source sortable" @click="$emit('sort', 'source')">
          Source {{ sortIndicator('source') }}
        </th>
      </tr>
    </thead>
    <tbody>
      <tr v-if="monsters.length === 0">
        <td colspan="8" class="no-results">
          <span v-if="searchPerformed">No monsters found matching your criteria</span>
          <span v-else>Search for monsters to see results</span>
        </td>
      </tr>
      <tr
        v-for="monster in sortedMonsters"
        :key="`${monster.name}-${monster.source}`"
        @click="$emit('select', monster)"
      >
        <td class="col-name">{{ monster.name }}</td>
        <td class="col-size">{{ formatSize(monster.size) }}</td>
        <td class="col-type">{{ monster.type }}</td>
        <td class="col-cr">{{ monster.cr }}</td>
        <td class="col-hp">{{ monster.hp }}</td>
        <td class="col-ac">{{ monster.ac }}</td>
        <td class="col-alignment">{{ monster.alignment }}</td>
        <td class="col-source">{{ monster.source }}</td>
      </tr>
    </tbody>
  </table>
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
.col-name { width: 25%; }
.col-size { width: 10%; }
.col-type { width: 15%; }
.col-cr { width: 8%; }
.col-hp { width: 8%; }
.col-ac { width: 8%; }
.col-alignment { width: 16%; }
.col-source { width: 10%; }
</style>