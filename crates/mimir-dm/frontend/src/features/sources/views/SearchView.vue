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
            <option value="Feats">Feats</option>
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
          :available-sources="classSources"
          @select="selectClass"
          @sort="handleSort"
        />
        
        <!-- Feat Table -->
        <FeatTable
          v-else-if="selectedCategory === 'Feats'"
          :feats="featResults"
          @select="selectFeat"
        />
        
        <!-- Placeholder for other categories -->
        <div v-else class="placeholder-message">
          {{ selectedCategory }} catalog coming soon...
        </div>
      </div>
    </div>
    
    <!-- Modal Stack -->
    <BaseModal
      v-for="(modal, index) in modalStack"
      :key="`modal-${index}`"
      :visible="modal.visible"
      :title="modal.title"
      :content="modal.content"
      :z-index="1000 + index * 10"
      @close="() => closeModal(index)"
      @reference-click="handleReferenceClick"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useCatalog } from '../composables/useCatalog'
import type { 
  SpellSummary, 
  ItemSummary, 
  MonsterSummary,
  ClassSummary,
  FeatSummary
} from '../composables/useCatalog'

// Import components
import BaseModal from '@/components/shared/BaseModal.vue'
import SpellTable from '../components/search/SpellTable.vue'
import ItemTable from '../components/search/ItemTable.vue'
import MonsterTable from '../components/search/MonsterTable.vue'
import ClassTable from '../components/search/ClassTable.vue'
import FeatTable from '../components/search/FeatTable.vue'

// Import formatters
import { formatSpellDetails } from '../formatters/spellFormatterEnhanced'
import { formatItemDetails } from '../formatters/itemFormatterEnhanced'
import { formatMonsterDetails } from '../formatters/monsterFormatterEnhanced'
import { formatClassDetails } from '../formatters/classFormatterEnhanced'
import { formatFeatDetails } from '../formatters/featFormatter'

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
  searchFeats,
  initializeCatalog,
  initializeItemCatalog,
  initializeMonsterCatalog,
  initializeClassCatalog,
  initializeFeatCatalog,
  getSpellDetails,
  getItemDetails,
  getMonsterDetails,
  getClassDetails,
  getFeatDetails,
  classSources
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
const featResults = ref<FeatSummary[]>([])

// Modal Stack
interface Modal {
  visible: boolean
  title: string
  content: string
}
const modalStack = ref<Modal[]>([])

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
    case 'Feats': return featResults.value.length
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
        query: searchQuery.value || undefined
        // Don't filter by source - backend already searches all loaded classes
      })
      break
      
    case 'Feats':
      featResults.value = await searchFeats({
        query: searchQuery.value || undefined
        // Don't filter by source - backend already searches all loaded feats
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
    modalStack.value.push({
      visible: true,
      title: spell.name,
      content: formatSpellDetails(fullSpell)
    })
  } else {
    // Fallback to summary if details fail
    modalStack.value.push({
      visible: true,
      title: spell.name,
      content: formatSpellDetails(spell)
    })
  }
}

async function selectItem(item: ItemSummary) {
  // Fetch full item details
  const fullItem = await getItemDetails(item.name, item.source)
  
  if (fullItem) {
    const formattedContent = await formatItemDetails(fullItem)
    modalStack.value.push({
      visible: true,
      title: item.name,
      content: formattedContent
    })
  } else {
    // Fallback to summary if details fail
    const formattedContent = await formatItemDetails(item)
    modalStack.value.push({
      visible: true,
      title: item.name,
      content: formattedContent
    })
  }
}

async function selectMonster(monster: MonsterSummary) {
  // Fetch full monster details
  const fullMonster = await getMonsterDetails(monster.name, monster.source)
  
  
  if (fullMonster) {
    const formattedContent = await formatMonsterDetails(fullMonster)
    modalStack.value.push({
      visible: true,
      title: monster.name,
      content: formattedContent
    })
  } else {
    // Fallback to summary if details fail
    const formattedContent = await formatMonsterDetails(monster)
    modalStack.value.push({
      visible: true,
      title: monster.name,
      content: formattedContent
    })
  }
}

async function selectClass(classItem: ClassSummary) {
  // Fetch full class details
  const fullClass = await getClassDetails(classItem.name, classItem.source)
  
  if (fullClass) {
    const formattedContent = await formatClassDetails(fullClass)
    modalStack.value.push({
      visible: true,
      title: classItem.name,
      content: formattedContent
    })
  } else {
    // Fallback to summary if details fail
    const formattedContent = await formatClassDetails(classItem)
    modalStack.value.push({
      visible: true,
      title: classItem.name,
      content: formattedContent
    })
  }
}

async function selectFeat(feat: FeatSummary) {
  // Fetch full feat details
  const fullFeat = await getFeatDetails(feat.name, feat.source)
  
  if (fullFeat) {
    const formattedContent = await formatFeatDetails(fullFeat)
    modalStack.value.push({
      visible: true,
      title: feat.name,
      content: formattedContent
    })
  } else {
    // Fallback to summary if details fail
    const formattedContent = await formatFeatDetails(feat)
    modalStack.value.push({
      visible: true,
      title: feat.name,
      content: formattedContent
    })
  }
}

function closeModal(index?: number) {
  if (index !== undefined) {
    // Close specific modal
    modalStack.value.splice(index, 1)
  } else {
    // Close top modal
    modalStack.value.pop()
  }
}

// Handle clicks on reference links in modals
async function handleReferenceClick(event: { type: string; name: string; source?: string }) {
  
  switch (event.type) {
    case 'creature':
    case 'monster': {
      // Try different capitalization formats for creature names
      const searchName = event.name
      const titleCaseName = searchName.split(' ')
        .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
        .join(' ')
      
      
      // Try with the exact name first
      let monster = await getMonsterDetails(searchName, event.source || 'MM')
      
      // If not found, try title case
      if (!monster) {
        monster = await getMonsterDetails(titleCaseName, event.source || 'MM')
      }
      
      if (monster) {
        const formattedContent = await formatMonsterDetails(monster)
        modalStack.value.push({
          visible: true,
          title: monster.name || event.name,
          content: formattedContent
        })
      } else {
      }
      break
    }
    case 'item': {
      const item = await getItemDetails(event.name, event.source || 'PHB')
      if (item) {
        const formattedContent = await formatItemDetails(item)
        modalStack.value.push({
          visible: true,
          title: event.name,
          content: formattedContent
        })
      } else {
      }
      break
    }
    case 'spell': {
      const spell = await getSpellDetails(event.name, event.source || 'PHB')
      if (spell) {
        modalStack.value.push({
          visible: true,
          title: event.name,
          content: formatSpellDetails(spell)
        })
      } else {
      }
      break
    }
    case 'class': {
      const classDetails = await getClassDetails(event.name, event.source || 'PHB')
      if (classDetails) {
        const formattedContent = await formatClassDetails(classDetails)
        modalStack.value.push({
          visible: true,
          title: event.name,
          content: formattedContent
        })
      } else {
      }
      break
    }
    case 'feat': {
      const feat = await getFeatDetails(event.name, event.source || 'PHB')
      if (feat) {
        const formattedContent = await formatFeatDetails(feat)
        modalStack.value.push({
          visible: true,
          title: event.name,
          content: formattedContent
        })
      } else {
      }
      break
    }
    case 'feature': {
      
      // Feature lookup not implemented yet
      let feature = null
      
      if (feature) {
        // Feature display not implemented yet
      } else {
      }
      break
    }
  }
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
    case 'Feats':
      await initializeFeatCatalog()
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
    case 'Monsters':
      await initializeMonsterCatalog()
      break
    case 'Classes':
      await initializeClassCatalog()
      break
    case 'Feats':
      await initializeFeatCatalog()
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