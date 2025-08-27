<template>
  <div class="search-results">
    <SpellTable
      v-if="category === 'Spells'"
      :spells="results"
      :search-performed="searchPerformed"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      @select="$emit('select-spell', $event)"
      @sort="$emit('sort', $event)"
    />
    
    <ItemTable
      v-else-if="category === 'Equipment'"
      :items="results"
      :search-performed="searchPerformed"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      @select="$emit('select-item', $event)"
      @sort="$emit('sort', $event)"
    />
    
    <ItemTable
      v-else-if="category === 'Magic Items'"
      :items="results"
      :search-performed="searchPerformed"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      :show-rarity="true"
      @select="$emit('select-item', $event)"
      @sort="$emit('sort', $event)"
    />
    
    <MonsterTable
      v-else-if="category === 'Monsters'"
      :monsters="results"
      :search-performed="searchPerformed"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      :filters="monsterFilters"
      @select="$emit('select-monster', $event)"
      @sort="$emit('sort', $event)"
      @filter-update="$emit('update-monster-filters', $event)"
    />
    
    <ClassTable
      v-else-if="category === 'Classes'"
      :classes="results"
      :search-performed="searchPerformed"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      :available-sources="availableSources"
      @select="$emit('select-class', $event)"
      @sort="$emit('sort', $event)"
    />
    
    <FeatTable
      v-else-if="category === 'Feats'"
      :feats="results"
      @select="$emit('select-feat', $event)"
    />
    
    <RaceTable
      v-else-if="category === 'Races'"
      :races="results"
      @select="$emit('select-race', $event)"
    />
    
    <div v-else class="placeholder-message">
      {{ category }} catalog coming soon...
    </div>
  </div>
</template>

<script setup lang="ts">
import SpellTable from './SpellTable.vue'
import ItemTable from './ItemTable.vue'
import MonsterTable from './MonsterTable.vue'
import ClassTable from './ClassTable.vue'
import FeatTable from './FeatTable.vue'
import RaceTable from './RaceTable.vue'
import type { 
  SpellSummary, 
  ItemSummary, 
  MonsterSummary,
  ClassSummary,
  FeatSummary,
  RaceSummary
} from '../../composables/useCatalog'

interface Props {
  category: string
  results: any[]
  searchPerformed: boolean
  sortColumn: string
  sortDirection: 'asc' | 'desc'
  monsterFilters?: {
    sizes: string[]
    types: string[]
    minCr?: number
    maxCr?: number
  }
  availableSources?: string[]
}

defineProps<Props>()

defineEmits<{
  'select-spell': [spell: SpellSummary]
  'select-item': [item: ItemSummary]
  'select-monster': [monster: MonsterSummary]
  'select-class': [classItem: ClassSummary]
  'select-feat': [feat: FeatSummary]
  'select-race': [race: RaceSummary]
  'sort': [column: string]
  'update-monster-filters': [filters: { sizes?: string[], types?: string[] }]
}>()
</script>

<style scoped>
.search-results {
  height: 100%;
  overflow: auto;
}

.placeholder-message {
  padding: 2rem;
  text-align: center;
  color: var(--color-text-dim, #666);
  font-style: italic;
}
</style>