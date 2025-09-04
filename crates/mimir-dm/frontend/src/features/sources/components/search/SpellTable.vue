<template>
  <div class="catalog-table">
    <div class="catalog-table__header">
      <h2 class="catalog-table__title">Spells</h2>
      <div class="catalog-table__filters">
        <div class="catalog-table__filter-group">
          <MultiSelectFilter
            label="Level"
            :options="levelOptions"
            v-model="localFilters.levels"
          />
        </div>
        <div class="catalog-table__filter-group">
          <MultiSelectFilter
            label="School"
            :options="schoolOptions"
            v-model="localFilters.schools"
          />
        </div>
        <div class="catalog-table__filter-group">
          <label class="form-checkbox" title="Filter by Concentration spells">
            <input type="checkbox" class="form-checkbox__input" v-model="localFilters.concentration" />
            <span class="form-checkbox__box"></span>
            <span class="form-checkbox__label">Conc</span>
          </label>
          <label class="form-checkbox" title="Filter by Ritual spells">
            <input type="checkbox" class="form-checkbox__input" v-model="localFilters.ritual" />
            <span class="form-checkbox__box"></span>
            <span class="form-checkbox__label">Ritual</span>
          </label>
        </div>
      </div>
    </div>
    
    <div class="catalog-table__content">
      <div class="catalog-table__results-info">
        <span class="catalog-table__result-count">{{ spells.length }} spells</span>
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
              <th>Level</th>
              <th>School</th>
              <th>Cast Time</th>
              <th>Range</th>
              <th>Components</th>
              <th>Tags</th>
              <th>Source</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="spells.length === 0" class="catalog-table__empty-row">
              <td colspan="8">
                <div class="catalog-table__empty">
                  <h3 v-if="searchPerformed">No spells found</h3>
                  <h3 v-else>Search for spells</h3>
                  <p v-if="searchPerformed">No spells found matching your criteria</p>
                  <p v-else>Search for spells to see results</p>
                </div>
              </td>
            </tr>
            <tr
              v-for="spell in sortedSpells"
              :key="`${spell.name}-${spell.source}`"
              class="catalog-table__row"
              @click="$emit('select', spell)"
      >
              <td>
                <div class="catalog-table__cell-name">{{ spell.name }}</div>
              </td>
              <td>{{ formatLevel(spell.level) }}</td>
              <td>{{ spell.school }}</td>
              <td>{{ spell.casting_time }}</td>
              <td>{{ spell.range }}</td>
              <td>{{ spell.components }}</td>
              <td>
                <span v-if="spell.concentration" class="catalog-table__cell-badge catalog-table__cell-badge--primary">Conc</span>
                <span v-if="spell.ritual" class="catalog-table__cell-badge catalog-table__cell-badge--secondary">Ritual</span>
              </td>
              <td>{{ spell.source }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { SpellSummary } from '../../composables/useCatalog'
import MultiSelectFilter from '../../../../shared/components/ui/MultiSelectFilter.vue'

interface Props {
  spells: SpellSummary[]
  searchPerformed: boolean
  sortColumn: string
  sortDirection: 'asc' | 'desc'
}

const props = defineProps<Props>()

const emit = defineEmits<{
  select: [spell: SpellSummary]
  sort: [column: string]
}>()

// Local filter state
const localFilters = ref({
  levels: [] as string[],
  schools: [] as string[],
  concentration: false,
  ritual: false
})

const levelOptions = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']

const schoolOptions = [
  'Abjuration',
  'Conjuration',
  'Divination',
  'Enchantment',
  'Evocation',
  'Illusion',
  'Necromancy',
  'Transmutation'
]

const sortedSpells = computed(() => {
  let filtered = [...props.spells]
  
  // Apply filters
  if (localFilters.value.levels.length > 0) {
    filtered = filtered.filter(s => 
      localFilters.value.levels.includes(s.level.toString())
    )
  }
  if (localFilters.value.schools.length > 0) {
    filtered = filtered.filter(s => 
      localFilters.value.schools.includes(s.school)
    )
  }
  if (localFilters.value.concentration) {
    filtered = filtered.filter(s => s.concentration)
  }
  if (localFilters.value.ritual) {
    // Debug: Check if we have any ritual spells
    const ritualSpells = props.spells.filter(s => s.ritual)
    filtered = filtered.filter(s => s.ritual)
  }
  
  // Apply sorting
  if (!['name', 'level', 'source'].includes(props.sortColumn)) {
    return filtered
  }
  
  const sorted = [...filtered]
  sorted.sort((a, b) => {
    if (props.sortColumn === 'level') {
      const aLevel = a.level
      const bLevel = b.level
      const comparison = aLevel - bLevel
      return props.sortDirection === 'asc' ? comparison : -comparison
    } else {
      const aVal = a[props.sortColumn as keyof SpellSummary] as string
      const bVal = b[props.sortColumn as keyof SpellSummary] as string
      const comparison = aVal.localeCompare(bVal)
      return props.sortDirection === 'asc' ? comparison : -comparison
    }
  })
  
  return sorted
})

function formatLevel(level: number): string {
  if (level === 0) return 'Cantrip'
  if (level === 1) return '1st'
  if (level === 2) return '2nd'
  if (level === 3) return '3rd'
  return `${level}th`
}

function sortIndicator(column: string) {
  if (props.sortColumn !== column) return ''
  return props.sortDirection === 'asc' ? '▲' : '▼'
}
</script>

<!-- All styling now handled by consolidated CSS classes -->