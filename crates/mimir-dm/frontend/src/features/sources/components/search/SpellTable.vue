<template>
  <table class="catalog-table">
    <thead>
      <tr>
        <th class="col-name sortable" @click="$emit('sort', 'name')">
          Name {{ sortIndicator('name') }}
        </th>
        <th class="col-level">
          <MultiSelectFilter
            label="Level"
            :options="levelOptions"
            v-model="localFilters.levels"
          />
        </th>
        <th class="col-school">
          <MultiSelectFilter
            label="School"
            :options="schoolOptions"
            v-model="localFilters.schools"
          />
        </th>
        <th class="col-cast-time">Cast Time</th>
        <th class="col-range">Range</th>
        <th class="col-components">Components</th>
        <th class="col-tags">
          <div class="tag-filters">
            <label class="tag-filter" title="Filter by Concentration spells">
              <input type="checkbox" v-model="localFilters.concentration" />
              <span>Conc</span>
            </label>
            <label class="tag-filter" title="Filter by Ritual spells">
              <input type="checkbox" v-model="localFilters.ritual" />
              <span>Ritual</span>
            </label>
          </div>
        </th>
        <th class="col-source">Source</th>
      </tr>
    </thead>
    <tbody>
      <tr v-if="spells.length === 0">
        <td colspan="8" class="no-results">
          <span v-if="searchPerformed">No spells found matching your criteria</span>
          <span v-else>Search for spells to see results</span>
        </td>
      </tr>
      <tr
        v-for="spell in sortedSpells"
        :key="`${spell.name}-${spell.source}`"
        @click="$emit('select', spell)"
      >
        <td class="col-name">{{ spell.name }}</td>
        <td class="col-level">{{ formatLevel(spell.level) }}</td>
        <td class="col-school">{{ spell.school }}</td>
        <td class="col-cast-time">{{ spell.casting_time }}</td>
        <td class="col-range">{{ spell.range }}</td>
        <td class="col-components">{{ spell.components }}</td>
        <td class="col-tags">
          <span v-if="spell.concentration" class="tag-badge concentration">Conc</span>
          <span v-if="spell.ritual" class="tag-badge ritual">Ritual</span>
        </td>
        <td class="col-source">{{ spell.source }}</td>
      </tr>
    </tbody>
  </table>
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
.col-name { width: 20%; }
.col-level { width: 10%; }
.col-school { width: 12%; }
.col-cast-time { width: 10%; }
.col-range { width: 10%; }
.col-components { width: 8%; }
.col-tags { width: 12%; }
.col-source { width: 8%; }

/* Tag filters */
.tag-filters {
  display: flex;
  gap: 8px;
  align-items: center;
}

.tag-filter {
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  font-size: 0.875rem;
  color: var(--color-text-secondary, #999);
}

.tag-filter input[type="checkbox"] {
  margin: 0;
  cursor: pointer;
}

.tag-filter span {
  font-weight: normal;
}

.tag-filter:hover {
  color: var(--color-text, #e0e0e0);
}

/* Tag badges in cells */
.tag-badge {
  display: inline-block;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.75rem;
  font-weight: 500;
  margin-right: 4px;
}

.tag-badge.concentration {
  background: rgba(74, 158, 255, 0.2);
  color: var(--color-primary, #4a9eff);
  border: 1px solid rgba(74, 158, 255, 0.3);
}

.tag-badge.ritual {
  background: rgba(168, 199, 255, 0.2);
  color: #a8c7ff;
  border: 1px solid rgba(168, 199, 255, 0.3);
}
</style>