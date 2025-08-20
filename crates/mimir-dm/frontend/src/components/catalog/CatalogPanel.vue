<template>
  <div class="catalog-panel">
    <!-- Header -->
    <div class="catalog-header">
      <div class="header-top">
        <div class="category-selector">
          <label for="category-select">Category:</label>
          <select 
            id="category-select" 
            v-model="selectedCategory" 
            class="category-select"
          >
            <option value="Spells">Spells</option>
            <option value="Equipment">Equipment</option>
            <option value="Monsters">Monsters/Bestiary</option>
            <option value="Magic Items">Magic Items</option>
            <option value="Classes">Classes</option>
            <option value="Races">Races</option>
            <option value="Feats">Feats</option>
            <option value="Backgrounds">Backgrounds</option>
            <option value="Conditions">Conditions</option>
          </select>
        </div>
        
        <div class="search-bar">
          <input 
            type="text" 
            v-model="searchQuery" 
            @input="debouncedSearch"
            placeholder="Search..."
            class="search-input"
          >
        </div>
        
        <div class="results-count" v-if="searchPerformed">
          {{ resultCount }} results
        </div>
      </div>
    </div>
    
    <!-- Content -->
    <div class="catalog-content">
      <div class="table-container">
        <!-- Spell Table -->
        <SpellTable
          v-if="selectedCategory === 'Spells'"
          :spells="spellResults"
          :search-performed="searchPerformed"
          :sort-column="sortColumn"
          :sort-direction="sortDirection"
          @select="selectSpell"
          @sort="handleSort"
        />
        
        <!-- Item Table (Equipment) -->
        <ItemTable
          v-else-if="selectedCategory === 'Equipment'"
          :items="itemResults"
          :search-performed="searchPerformed"
          :sort-column="sortColumn"
          :sort-direction="sortDirection"
          @select="selectItem"
          @sort="handleSort"
        />
        
        <!-- Item Table (Magic Items) -->
        <ItemTable
          v-else-if="selectedCategory === 'Magic Items'"
          :items="magicItemResults"
          :search-performed="searchPerformed"
          :sort-column="sortColumn"
          :sort-direction="sortDirection"
          :show-rarity="true"
          @select="selectItem"
          @sort="handleSort"
        />
        
        <!-- Monster Table -->
        <MonsterTable
          v-else-if="selectedCategory === 'Monsters'"
          :monsters="monsterResults"
          :search-performed="searchPerformed"
          :sort-column="sortColumn"
          :sort-direction="sortDirection"
          :filters="monsterFilters"
          @select="selectMonster"
          @sort="handleSort"
          @filter-update="updateMonsterFilters"
        />
        
        <!-- Class Table -->
        <ClassTable
          v-else-if="selectedCategory === 'Classes'"
          :classes="classResults"
          :search-performed="searchPerformed"
          :sort-column="sortColumn"
          :sort-direction="sortDirection"
          @select="selectClass"
          @sort="handleSort"
        />
        
        <!-- Placeholder for other categories -->
        <div v-else class="placeholder-message">
          {{ selectedCategory }} catalog coming soon...
        </div>
      </div>
    </div>
    
    <!-- Modal -->
    <CatalogModal
      :visible="modalContent.visible"
      :title="modalContent.title"
      :content="modalContent.content"
      @close="closeModal"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useCatalog } from '@/composables/catalog/useCatalog'
import type { 
  SpellSummary, 
  ItemSummary, 
  ClassSummary,
  MonsterSummary,
  RaceSummary,
  FeatSummary,
  BackgroundSummary
} from '@/composables/catalog/useCatalog'

// Import components
import CatalogModal from './CatalogModal.vue'
import SpellTable from './tables/SpellTable.vue'
import ItemTable from './tables/ItemTable.vue'
import ClassTable from './tables/ClassTable.vue'
import MonsterTable from './tables/MonsterTable.vue'

// Import formatters
import { formatSpellDetails } from '@/utils/catalog/spellFormatterEnhanced'
import { formatItemDetails } from '@/utils/catalog/itemFormatterEnhanced'
import { formatClassDetails, formatEnhancedClassDetails } from '@/utils/catalog/classFormatter'
import { formatMonsterDetails } from '@/utils/catalog/monsterFormatterEnhanced'

interface Props {
  selectedSources: string[]
  selectedCategory: string
}

const props = defineProps<Props>()

// Use the catalog composable
const {
  searchSpells,
  searchItems,
  searchMonsters,
  searchClasses,
  searchRaces,
  searchFeats,
  searchBackgrounds,
  initializeCatalog,
  initializeItemCatalog,
  initializeMonsterCatalog,
  initializeClassCatalog,
  initializeRaceCatalog,
  initializeFeatCatalog,
  initializeBackgroundCatalog,
  getClassDetails,
  getSpellDetails,
  getItemDetails,
  getMonsterDetails
} = useCatalog()

// Local state
const selectedCategory = ref(props.selectedCategory)
const searchQuery = ref('')
const searchPerformed = ref(false)
const sortColumn = ref('name')
const sortDirection = ref<'asc' | 'desc'>('asc')

// Results
const spellResults = ref<SpellSummary[]>([])
const itemResults = ref<ItemSummary[]>([])
const magicItemResults = ref<ItemSummary[]>([])
const monsterResults = ref<MonsterSummary[]>([])
const classResults = ref<ClassSummary[]>([])
const raceResults = ref<RaceSummary[]>([])
const featResults = ref<FeatSummary[]>([])
const backgroundResults = ref<BackgroundSummary[]>([])

// Modal
const modalContent = ref({
  visible: false,
  title: '',
  content: ''
})

// Filters
const spellFilters = ref({
  school: '',
  level: '',
  ritual: false,
  concentration: false
})

const equipmentFilters = ref({
  type: '',
  rarity: ''
})

const monsterFilters = ref({
  sizes: [] as string[],
  types: [] as string[],
  minCr: undefined as number | undefined,
  maxCr: undefined as number | undefined
})

const magicItemFilters = ref({
  rarity: ''
})

// Computed
const resultCount = computed(() => {
  switch (selectedCategory.value) {
    case 'Spells': return spellResults.value.length
    case 'Equipment': return itemResults.value.length
    case 'Magic Items': return magicItemResults.value.length
    case 'Monsters': return monsterResults.value.length
    case 'Classes': return classResults.value.length
    case 'Races': return raceResults.value.length
    case 'Feats': return featResults.value.length
    case 'Backgrounds': return backgroundResults.value.length
    default: return 0
  }
})

// Search timeout
let searchTimeout: NodeJS.Timeout | null = null

// Methods
async function performSearch() {
  searchPerformed.value = true
  
  // Map book IDs to source codes
  const sources = props.selectedSources.length > 0 
    ? mapBookIdsToSources(props.selectedSources) 
    : undefined
    
  switch (selectedCategory.value) {
    case 'Spells':
      spellResults.value = await searchSpells({
        query: searchQuery.value || undefined,
        sources,
        schools: spellFilters.value.school ? [spellFilters.value.school] : undefined,
        levels: spellFilters.value.level ? [parseInt(spellFilters.value.level)] : undefined,
        ritual: spellFilters.value.ritual || undefined,
        concentration: spellFilters.value.concentration || undefined
      })
      break
      
    case 'Equipment':
      itemResults.value = await searchItems({
        query: searchQuery.value || undefined,
        sources,
        types: equipmentFilters.value.type ? [equipmentFilters.value.type] : undefined
      })
      break
      
    case 'Magic Items':
      const allItems = await searchItems({
        query: searchQuery.value || undefined,
        sources
      })
      // Filter for magic items
      magicItemResults.value = allItems.filter(item => 
        item.rarity && item.rarity !== 'none' &&
        (!magicItemFilters.value.rarity || item.rarity === magicItemFilters.value.rarity)
      )
      break
      
    case 'Monsters':
      monsterResults.value = await searchMonsters({
        query: searchQuery.value || undefined,
        sources,
        sizes: monsterFilters.value.sizes.length > 0 ? monsterFilters.value.sizes : undefined,
        types: monsterFilters.value.types.length > 0 ? monsterFilters.value.types : undefined,
        min_cr: monsterFilters.value.minCr,
        max_cr: monsterFilters.value.maxCr
      })
      break
      
    case 'Classes':
      classResults.value = await searchClasses({
        query: searchQuery.value || undefined,
        sources
      })
      break
      
    // Add other categories as needed
  }
}

function debouncedSearch() {
  if (searchTimeout) {
    clearTimeout(searchTimeout)
  }
  searchTimeout = setTimeout(() => {
    performSearch()
  }, 300)
}

function updateMonsterFilters(newFilters: { sizes?: string[], types?: string[] }) {
  monsterFilters.value = { ...monsterFilters.value, ...newFilters }
  // No need to perform search - the table component filters locally
}

function handleSort(column: string) {
  if (sortColumn.value === column) {
    sortDirection.value = sortDirection.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortColumn.value = column
    sortDirection.value = 'asc'
  }
}

// Selection handlers
async function selectSpell(spell: SpellSummary) {
  // Fetch full spell details
  const fullSpell = await getSpellDetails(spell.name, spell.source)
  
  if (fullSpell) {
    modalContent.value = {
      visible: true,
      title: spell.name,
      content: formatSpellDetails(fullSpell)
    }
  } else {
    // Fallback to summary if details fail
    modalContent.value = {
      visible: true,
      title: spell.name,
      content: formatSpellDetails(spell)
    }
  }
}

async function selectItem(item: ItemSummary) {
  // Fetch full item details
  const fullItem = await getItemDetails(item.name, item.source)
  
  if (fullItem) {
    modalContent.value = {
      visible: true,
      title: item.name,
      content: formatItemDetails(fullItem)
    }
  } else {
    // Fallback to summary if details fail
    modalContent.value = {
      visible: true,
      title: item.name,
      content: formatItemDetails(item)
    }
  }
}

async function selectMonster(monster: MonsterSummary) {
  // Fetch full monster details
  const fullMonster = await getMonsterDetails(monster.name, monster.source)
  
  console.log('Full monster data received:', fullMonster)
  console.log('Has fluff_images?', fullMonster?.fluff_images)
  console.log('Has fluffImages?', fullMonster?.fluffImages)
  
  if (fullMonster) {
    const formattedContent = await formatMonsterDetails(fullMonster)
    modalContent.value = {
      visible: true,
      title: monster.name,
      content: formattedContent
    }
  } else {
    // Fallback to summary if details fail
    const formattedContent = await formatMonsterDetails(monster)
    modalContent.value = {
      visible: true,
      title: monster.name,
      content: formattedContent
    }
  }
}

async function selectClass(cls: ClassSummary) {
  const isSubclass = cls.name.includes(':')
  
  if (isSubclass && (cls.subclassFeatures || cls.additionalSpells)) {
    const parentClassName = cls.name.split(': ')[0]
    const parentClass = classResults.value.find(c => c.name === parentClassName)
    modalContent.value = {
      visible: true,
      title: cls.name,
      content: formatEnhancedClassDetails(cls, parentClass)
    }
  } else {
    const parentClass = isSubclass 
      ? classResults.value.find(c => c.name === cls.name.split(': ')[0])
      : undefined
    modalContent.value = {
      visible: true,
      title: cls.name,
      content: formatClassDetails(cls, parentClass)
    }
  }
}

function closeModal() {
  modalContent.value.visible = false
}

// Utility function to map book IDs to sources
function mapBookIdsToSources(bookIds: string[]): string[] {
  // This is a simplified mapping - adjust based on your actual book ID to source mapping
  return bookIds.map(id => {
    const parts = id.split('-')
    return parts[parts.length - 1].toUpperCase()
  })
}

// Initialize on mount
onMounted(async () => {
  switch (selectedCategory.value) {
    case 'Spells':
      await initializeCatalog()
      break
    case 'Equipment':
    case 'Magic Items':
      await initializeItemCatalog()
      break
    case 'Monsters':
      await initializeMonsterCatalog()
      break
    case 'Classes':
      await initializeClassCatalog()
      break
    case 'Races':
      await initializeRaceCatalog()
      break
    case 'Feats':
      await initializeFeatCatalog()
      break
    case 'Backgrounds':
      await initializeBackgroundCatalog()
      break
  }
  await performSearch()
})

// Watch for category changes
watch(selectedCategory, async (newCategory) => {
  // Initialize the appropriate catalog
  switch (newCategory) {
    case 'Spells':
      await initializeCatalog()
      break
    case 'Equipment':
    case 'Magic Items':
      await initializeItemCatalog()
      break
    case 'Classes':
      await initializeClassCatalog()
      break
    // Add other initializations as needed
  }
  await performSearch()
})

// Watch for source changes
watch(() => props.selectedSources, () => {
  performSearch()
}, { deep: true })
</script>

<style scoped>
.catalog-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--color-background, #0d0d0d);
}

.catalog-header {
  padding: var(--spacing-md, 12px) var(--spacing-lg, 16px);
  background: var(--color-surface, #1a1a1a);
  border-bottom: 1px solid var(--color-border, #333);
}

.header-top {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg, 16px);
  margin-bottom: var(--spacing-sm, 8px);
}

.category-selector {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 8px);
}

.category-selector label {
  color: var(--color-text-secondary, #999);
  font-size: 0.9rem;
  font-weight: 500;
}

.category-select {
  padding: var(--spacing-xs, 4px) var(--spacing-sm, 8px);
  background: var(--color-background, #0d0d0d);
  border: 1px solid var(--color-border, #333);
  border-radius: 4px;
  color: var(--color-text, #e0e0e0);
  font-size: 0.9rem;
  cursor: pointer;
}

.category-select:focus {
  outline: none;
  border-color: var(--color-primary, #4a9eff);
}

.search-bar {
  flex: 1;
  max-width: 400px;
}

.search-input {
  width: 100%;
  padding: var(--spacing-xs, 4px) var(--spacing-sm, 8px);
  background: var(--color-background, #0d0d0d);
  border: 1px solid var(--color-border, #333);
  border-radius: 4px;
  color: var(--color-text, #e0e0e0);
  font-size: 0.9rem;
}

.search-input:focus {
  outline: none;
  border-color: var(--color-primary, #4a9eff);
}

.results-count {
  color: var(--color-text-secondary, #999);
  font-size: 0.9rem;
}

.catalog-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.table-container {
  flex: 1;
  overflow: auto;
  padding: 0;
}

.placeholder-message {
  padding: 2rem;
  text-align: center;
  color: var(--color-text-dim, #666);
  font-style: italic;
}
</style>