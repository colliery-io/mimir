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
    
    <BackgroundTable
      v-else-if="category === 'Backgrounds'"
      :backgrounds="results"
      @select="$emit('select-background', $event)"
    />
    
    <ActionTable
      v-else-if="category === 'Actions'"
      :actions="results"
      @select="$emit('select-action', $event)"
    />
    
    <ConditionTable
      v-else-if="category === 'Conditions'"
      :conditions="results"
      @select="$emit('select-condition', $event)"
    />
    
    <OptionsTable
      v-else-if="category === 'Other Options & Features'"
      :options="results"
      :search-performed="searchPerformed"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      @select="$emit('select-option', $event)"
      @sort="$emit('sort', $event)"
    />
    
    <DeityTable
      v-else-if="category === 'Deities'"
      :deities="results"
      :search-performed="searchPerformed"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      @select="$emit('select-deity', $event)"
      @sort="$emit('sort', $event)"
    />
    
    <ObjectTable
      v-else-if="category === 'Objects'"
      :objects="results"
      :search-performed="searchPerformed"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      @select="$emit('select-object', $event)"
      @sort="$emit('sort', $event)"
    />
    
    <TrapTable
      v-else-if="category === 'Traps & Hazards'"
      :traps="results"
      :search-performed="searchPerformed"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      @select="$emit('select-trap', $event)"
      @sort="$emit('sort', $event)"
    />
    
    <LanguageTable
      v-else-if="category === 'Languages'"
      :languages="results"
      :search-performed="searchPerformed"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      @select="$emit('select-language', $event)"
      @sort="$emit('sort', $event)"
    />
    
    <RewardTable
      v-else-if="category === 'Rewards'"
      :rewards="results"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      @select="$emit('select-reward', $event)"
      @sort="$emit('sort', $event)"
    />
    
    <TablesList
      v-else-if="category === 'Tables'"
      :tables="results"
      :sort-column="sortColumn"
      :sort-direction="sortDirection"
      @select="$emit('select-table', $event)"
      @sort="$emit('sort', $event)"
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
import BackgroundTable from './BackgroundTable.vue'
import ActionTable from './ActionTable.vue'
import ConditionTable from './ConditionTable.vue'
import OptionsTable from './OptionsTable.vue'
import DeityTable from './DeityTable.vue'
import ObjectTable from './ObjectTable.vue'
import TrapTable from './TrapTable.vue'
import LanguageTable from './LanguageTable.vue'
import RewardTable from './RewardTable.vue'
import TablesList from './TablesList.vue'
import type { 
  SpellSummary, 
  ItemSummary, 
  MonsterSummary,
  ClassSummary,
  FeatSummary,
  RaceSummary,
  BackgroundSummary,
  ActionSummary,
  ConditionSummary,
  OptionalFeatureSummary,
  DeitySummary,
  ObjectSummary,
  TrapSummary,
  LanguageSummary,
  RewardSummary,
  TableSummary
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
  'select-background': [background: BackgroundSummary]
  'select-action': [action: ActionSummary]
  'select-condition': [condition: ConditionSummary]
  'select-option': [option: OptionalFeatureSummary]
  'select-deity': [deity: DeitySummary]
  'select-object': [obj: ObjectSummary]
  'select-trap': [trap: TrapSummary]
  'select-language': [lang: LanguageSummary]
  'select-reward': [reward: RewardSummary]
  'select-table': [table: TableSummary]
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