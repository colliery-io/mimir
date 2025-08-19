<template>
  <div class="catalog-panel">
    <div class="catalog-header">
      <div class="header-top">
        <div class="category-selector">
          <label for="category-select">Category:</label>
          <select 
            id="category-select" 
            v-model="selectedCategoryLocal"
            class="category-select"
          >
            <option value="Spells">Spells ✓</option>
            <option value="Monsters">Monsters</option>
            <option value="Equipment">Equipment</option>
            <option value="Magic Items">Magic Items (coming soon)</option>
            <option value="Classes">Classes (coming soon)</option>
            <option value="Races">Races (coming soon)</option>
            <option value="Feats">Feats (coming soon)</option>
            <option value="Backgrounds">Backgrounds (coming soon)</option>
            <option value="Conditions">Conditions (coming soon)</option>
          </select>
        </div>
        <div class="search-bar">
          <input 
            v-model="searchQuery"
            type="text"
            :placeholder="`Search ${selectedCategoryLocal.toLowerCase()} by name...`"
            class="search-input"
            @input="debouncedSearch"
          />
        </div>
      </div>
      <div class="header-info">
        <h2>{{ selectedCategoryLocal }}</h2>
        <span class="results-count" v-if="selectedCategoryLocal === 'Spells' && spellResults.length > 0">
          {{ filteredResults.length }} of {{ spellResults.length }} spells
        </span>
        <span class="results-count" v-else-if="selectedCategoryLocal === 'Equipment' && itemResults.length > 0">
          {{ filteredEquipment.length }} of {{ itemResults.length }} items
        </span>
      </div>
    </div>
    
    <div class="catalog-content">
      <!-- Loading state -->
      <div v-if="isLoading" class="loading-message">
        <p>Loading {{ selectedCategory.toLowerCase() }}...</p>
      </div>
      
      <!-- Error state -->
      <div v-else-if="error" class="error-message">
        <p>{{ error }}</p>
      </div>
      
      <!-- Spells Table -->
      <div v-else-if="selectedCategoryLocal === 'Spells'" class="table-container">
        <table class="catalog-table">
          <thead>
            <tr>
              <th class="col-name">
                Name
                <button class="sort-btn" @click="toggleSort('name')">
                  {{ getSortIcon('name') }}
                </button>
              </th>
              <th class="col-level">
                <select v-model="filters.level" @change="applyFilters" class="filter-select">
                  <option value="">All Levels</option>
                  <option value="0">Cantrip</option>
                  <option v-for="i in 9" :key="i" :value="i">{{ i }}</option>
                </select>
              </th>
              <th class="col-school">
                <select v-model="filters.school" @change="applyFilters" class="filter-select">
                  <option value="">All Schools</option>
                  <option v-for="school in schools" :key="school" :value="school">
                    {{ school }}
                  </option>
                </select>
              </th>
              <th class="col-time">Cast Time</th>
              <th class="col-range">Range</th>
              <th class="col-components">Comp.</th>
              <th class="col-tags">
                <div class="tag-filters">
                  <label class="tag-filter" title="Filter by Concentration spells">
                    <input type="checkbox" v-model="filters.concentration" @change="applyFilters" />
                    <span>Conc</span>
                  </label>
                  <label class="tag-filter" title="Filter by Ritual spells">
                    <input type="checkbox" v-model="filters.ritual" @change="applyFilters" />
                    <span>Ritual</span>
                  </label>
                </div>
              </th>
              <th class="col-source">Source</th>
            </tr>
          </thead>
          <tbody>
            <tr 
              v-for="spell in filteredResults" 
              :key="`${spell.name}-${spell.source}`"
              class="spell-row"
              @click="selectSpell(spell)"
            >
              <td class="col-name">
                <span class="spell-name">{{ spell.name }}</span>
              </td>
              <td class="col-level">
                <span class="level-badge" :class="`level-${spell.level}`">
                  {{ spell.level === 0 ? 'Cantrip' : `Level ${spell.level}` }}
                </span>
              </td>
              <td class="col-school">{{ spell.school }}</td>
              <td class="col-time">{{ spell.casting_time }}</td>
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
      </div>
      
      <!-- Equipment Table -->
      <div v-else-if="selectedCategoryLocal === 'Equipment'" class="table-container">
        <table class="catalog-table">
          <thead>
            <tr>
              <th class="col-name">
                Name
                <button class="sort-btn" @click="toggleSort('name')">
                  {{ getSortIcon('name') }}
                </button>
              </th>
              <th class="col-type">
                <select v-model="equipmentFilters.type" @change="applyEquipmentFilters" class="filter-select">
                  <option value="">Type</option>
                  <optgroup label="Weapons">
                    <option value="S">Simple Weapon</option>
                    <option value="M">Martial Weapon</option>
                    <option value="R">Ranged Weapon</option>
                    <option value="A">Ammunition</option>
                  </optgroup>
                  <optgroup label="Armor">
                    <option value="LA">Light Armor</option>
                    <option value="MA">Medium Armor</option>
                    <option value="HA">Heavy Armor</option>
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
              <th class="col-cost">Cost</th>
              <th class="col-weight">Weight</th>
              <th class="col-source">Source</th>
            </tr>
          </thead>
          <tbody>
            <tr 
              v-for="item in filteredEquipment" 
              :key="`${item.name}-${item.source}`"
              class="item-row"
              @click="selectItem(item)"
            >
              <td class="col-name">
                <span class="item-name">{{ item.name }}</span>
                <span v-if="item.damage" class="item-damage">{{ item.damage }}</span>
                <span v-if="item.ac" class="item-ac">AC {{ item.ac }}</span>
              </td>
              <td class="col-type">{{ item.type_name }}</td>
              <td class="col-cost">{{ formatCost(item.value) }}</td>
              <td class="col-weight">{{ formatWeight(item.weight) }}</td>
              <td class="col-source">{{ item.source }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      
      <!-- Monsters placeholder -->
      <!-- Monsters Table -->
      <div v-else-if="selectedCategoryLocal === 'Monsters'" class="table-container">
        <table class="catalog-table">
          <thead>
            <tr>
              <th class="col-name">
                <button class="sort-header" @click="sortBy('name')">
                  Name
                  {{ getSortIcon('name') }}
                </button>
              </th>
              <th class="col-size">
                <select v-model="monsterFilters.size" @change="applyMonsterFilters" class="filter-select">
                  <option value="">Size</option>
                  <option value="T">Tiny</option>
                  <option value="S">Small</option>
                  <option value="M">Medium</option>
                  <option value="L">Large</option>
                  <option value="H">Huge</option>
                  <option value="G">Gargantuan</option>
                </select>
              </th>
              <th class="col-type">
                <select v-model="monsterFilters.type" @change="applyMonsterFilters" class="filter-select">
                  <option value="">Type</option>
                  <option value="aberration">Aberration</option>
                  <option value="beast">Beast</option>
                  <option value="celestial">Celestial</option>
                  <option value="construct">Construct</option>
                  <option value="dragon">Dragon</option>
                  <option value="elemental">Elemental</option>
                  <option value="fey">Fey</option>
                  <option value="fiend">Fiend</option>
                  <option value="giant">Giant</option>
                  <option value="humanoid">Humanoid</option>
                  <option value="monstrosity">Monstrosity</option>
                  <option value="ooze">Ooze</option>
                  <option value="plant">Plant</option>
                  <option value="undead">Undead</option>
                </select>
              </th>
              <th class="col-cr">
                <button class="sort-header" @click="sortBy('cr_numeric')">
                  CR
                  {{ getSortIcon('cr_numeric') }}
                </button>
              </th>
              <th class="col-hp">HP</th>
              <th class="col-ac">AC</th>
              <th class="col-alignment">Alignment</th>
              <th class="col-source">Source</th>
            </tr>
          </thead>
          <tbody>
            <tr 
              v-for="monster in filteredMonsters"
              :key="`${monster.name}-${monster.source}`"
              class="monster-row"
              @click="selectMonster(monster)"
            >
              <td class="col-name">
                <span class="monster-name">{{ monster.name }}</span>
              </td>
              <td class="col-size">{{ monster.size }}</td>
              <td class="col-type">{{ monster.creature_type }}</td>
              <td class="col-cr">{{ monster.cr }}</td>
              <td class="col-hp">{{ monster.hp }}</td>
              <td class="col-ac">{{ monster.ac }}</td>
              <td class="col-alignment">{{ monster.alignment }}</td>
              <td class="col-source">{{ monster.source }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      
      <!-- Other categories placeholder -->
      <div v-else class="placeholder-text">
        <p>{{ selectedCategoryLocal }} catalog coming soon...</p>
      </div>
    </div>
    
    <!-- Spell detail modal -->
    <div v-if="modalContent.visible" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>{{ modalContent.title }}</h2>
          <button class="modal-close" @click="closeModal">×</button>
        </div>
        <div class="modal-body" v-html="modalContent.content"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, computed } from 'vue'
import { useCatalog, type SpellSummary, type ItemSummary, type MonsterSummary } from '../../composables/catalog/useCatalog'

interface Props {
  selectedCategory?: string
  selectedSources?: string[]
}

const props = withDefaults(defineProps<Props>(), {
  selectedCategory: 'Spells',
  selectedSources: () => []
})

const { 
  isLoading, 
  error, 
  searchSpells, 
  initializeCatalog, 
  getSpellDetails,
  searchItems,
  initializeItemCatalog,
  getItemDetails,
  searchMonsters,
  initializeMonsterCatalog,
  getMonsterDetails
} = useCatalog()

const searchQuery = ref('')
const spellResults = ref<SpellSummary[]>([])
const itemResults = ref<ItemSummary[]>([])
const monsterResults = ref<MonsterSummary[]>([])
const searchPerformed = ref(false)
let searchTimeout: NodeJS.Timeout | null = null

// Local category selection (since we don't have the middle panel anymore)
const selectedCategoryLocal = ref(props.selectedCategory)

// Modal state
const modalContent = ref({
  visible: false,
  title: '',
  content: ''
})

// Spell filtering state
const filters = ref({
  level: '',
  school: '',
  concentration: false,
  ritual: false
})

// Equipment filtering state
const equipmentFilters = ref({
  type: '',
  rarity: ''
})

// Monster filtering state
const monsterFilters = ref({
  size: '',
  type: '',
  minCr: undefined as number | undefined,
  maxCr: undefined as number | undefined
})

// Sorting state
const sortColumn = ref<string>('name')
const sortDirection = ref<'asc' | 'desc'>('asc')

// Available schools
const schools = [
  'Abjuration',
  'Conjuration',
  'Divination',
  'Enchantment',
  'Evocation',
  'Illusion',
  'Necromancy',
  'Transmutation'
]

// Initialize catalog when component mounts
onMounted(async () => {
  if (selectedCategoryLocal.value === 'Spells') {
    await initializeCatalog()
    // Load all spells by default
    await performSearch()
  } else if (selectedCategoryLocal.value === 'Equipment') {
    await initializeItemCatalog()
    await performSearch()
  }
})

// Watch for local category changes
watch(selectedCategoryLocal, async (newCategory) => {
  if (newCategory === 'Spells') {
    await initializeCatalog()
    // Always load spells when switching to spells category
    await performSearch()
  } else if (newCategory === 'Equipment') {
    await initializeItemCatalog()
    await performSearch()
  } else if (newCategory === 'Monsters') {
    await initializeMonsterCatalog()
    await performSearch()
  }
})

// Watch for source changes
watch(() => props.selectedSources, () => {
  if (selectedCategoryLocal.value === 'Spells') {
    performSearch()
  }
}, { deep: true })

// Debounced search function
function debouncedSearch() {
  if (searchTimeout) {
    clearTimeout(searchTimeout)
  }
  
  searchTimeout = setTimeout(() => {
    performSearch()
  }, 300)
}

// Perform the actual search
async function performSearch() {
  searchPerformed.value = true
  
  if (selectedCategoryLocal.value === 'Spells') {
    // Always perform search, even with empty query (to show all spells)
    const results = await searchSpells({
      query: searchQuery.value || undefined,
      sources: props.selectedSources.length > 0 ? mapBookIdsToSources(props.selectedSources) : undefined,
    })
    
    spellResults.value = results
  } else if (selectedCategoryLocal.value === 'Equipment') {
    const results = await searchItems({
      query: searchQuery.value || undefined,
      sources: props.selectedSources.length > 0 ? mapBookIdsToSources(props.selectedSources) : undefined,
      types: equipmentFilters.value.type ? [equipmentFilters.value.type] : undefined,
    })
    
    itemResults.value = results
  } else if (selectedCategoryLocal.value === 'Monsters') {
    const results = await searchMonsters({
      query: searchQuery.value || undefined,
      sources: props.selectedSources.length > 0 ? mapBookIdsToSources(props.selectedSources) : undefined,
      sizes: monsterFilters.value.size ? [monsterFilters.value.size] : undefined,
      types: monsterFilters.value.type ? [monsterFilters.value.type] : undefined,
      min_cr: monsterFilters.value.minCr,
      max_cr: monsterFilters.value.maxCr,
    })
    
    monsterResults.value = results
  }
}

// Computed filtered and sorted results
const filteredResults = computed(() => {
  let results = [...spellResults.value]
  
  // Apply level filter
  if (filters.value.level !== '') {
    const level = parseInt(filters.value.level)
    results = results.filter(spell => spell.level === level)
  }
  
  // Apply school filter
  if (filters.value.school !== '') {
    results = results.filter(spell => spell.school === filters.value.school)
  }
  
  // Apply concentration filter
  if (filters.value.concentration) {
    results = results.filter(spell => spell.concentration)
  }
  
  // Apply ritual filter
  if (filters.value.ritual) {
    results = results.filter(spell => spell.ritual)
  }
  
  // Apply sorting
  results.sort((a, b) => {
    let aVal: any = a[sortColumn.value as keyof SpellSummary]
    let bVal: any = b[sortColumn.value as keyof SpellSummary]
    
    // Handle different types
    if (typeof aVal === 'string') {
      aVal = aVal.toLowerCase()
      bVal = bVal.toLowerCase()
    }
    
    if (sortDirection.value === 'asc') {
      return aVal < bVal ? -1 : aVal > bVal ? 1 : 0
    } else {
      return aVal > bVal ? -1 : aVal < bVal ? 1 : 0
    }
  })
  
  return results
})

// Computed filtered equipment
const filteredEquipment = computed(() => {
  let results = [...itemResults.value]
  
  // Apply type filter (already applied in search, but can do client-side too)
  if (equipmentFilters.value.type) {
    results = results.filter(item => item.item_type === equipmentFilters.value.type)
  }
  
  // Apply sorting
  results.sort((a, b) => {
    let aVal: any = a[sortColumn.value as keyof ItemSummary]
    let bVal: any = b[sortColumn.value as keyof ItemSummary]
    
    // Handle different types
    if (typeof aVal === 'string') {
      aVal = aVal.toLowerCase()
      bVal = bVal.toLowerCase()
    }
    
    if (sortDirection.value === 'asc') {
      return aVal < bVal ? -1 : aVal > bVal ? 1 : 0
    } else {
      return aVal > bVal ? -1 : aVal < bVal ? 1 : 0
    }
  })
  
  return results
})

// Computed filtered and sorted monsters
const filteredMonsters = computed(() => {
  let results = [...monsterResults.value]
  
  // Apply size filter (already applied in search, but can do client-side too)
  if (monsterFilters.value.size) {
    results = results.filter(monster => {
      // Convert size code to match frontend filter
      const sizeMap: Record<string, string> = {
        'Tiny': 'T',
        'Small': 'S', 
        'Medium': 'M',
        'Large': 'L',
        'Huge': 'H',
        'Gargantuan': 'G'
      }
      const sizeCode = sizeMap[monster.size] || monster.size
      return sizeCode === monsterFilters.value.size
    })
  }
  
  // Apply type filter
  if (monsterFilters.value.type) {
    results = results.filter(monster => 
      monster.creature_type.toLowerCase().includes(monsterFilters.value.type.toLowerCase())
    )
  }
  
  // Apply sorting
  results.sort((a, b) => {
    let aVal: any = a[sortColumn.value as keyof MonsterSummary]
    let bVal: any = b[sortColumn.value as keyof MonsterSummary]
    
    // Handle different types
    if (typeof aVal === 'string') {
      aVal = aVal.toLowerCase()
      bVal = bVal.toLowerCase()
    }
    
    if (sortDirection.value === 'asc') {
      return aVal < bVal ? -1 : aVal > bVal ? 1 : 0
    } else {
      return aVal > bVal ? -1 : aVal < bVal ? 1 : 0
    }
  })
  
  return results
})

// Apply filters (triggers the computed properties)
function applyFilters() {
  // Filters are applied via computed properties
}

function applyEquipmentFilters() {
  performSearch()
}

function applyMonsterFilters() {
  performSearch()
}

// Toggle sort direction for a column
function toggleSort(column: string) {
  if (sortColumn.value === column) {
    sortDirection.value = sortDirection.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortColumn.value = column
    sortDirection.value = 'asc'
  }
}

// Sort by a column (alias for toggleSort)
function sortBy(column: string) {
  toggleSort(column)
}

// Get sort icon for a column
function getSortIcon(column: string): string {
  if (sortColumn.value !== column) return '↕'
  return sortDirection.value === 'asc' ? '↑' : '↓'
}

// Map book IDs to 5etools source codes
function mapBookIdsToSources(bookIds: string[]): string[] {
  // Extract source codes from book IDs
  // Book IDs are formatted as "book-{source}" where source is the 5etools code
  return bookIds
    .map(id => {
      // Remove "book-" prefix to get the source code
      if (id.startsWith('book-')) {
        return id.substring(5).toUpperCase()
      }
      return id.toUpperCase()
    })
    .filter(source => source !== 'TEST-BOOK')
}

async function selectSpell(spell: SpellSummary) {
  // Fetch full spell details
  const details = await getSpellDetails(spell.name, spell.source)
  
  if (details) {
    // Format the spell details for display
    modalContent.value = {
      visible: true,
      title: spell.name,
      content: formatSpellDetails(spell, details)
    }
  }
}

async function selectItem(item: ItemSummary) {
  // Fetch full item details
  const details = await getItemDetails(item.name, item.source)
  
  if (details) {
    // Format the item details for display
    modalContent.value = {
      visible: true,
      title: item.name,
      content: formatItemDetails(item, details)
    }
  }
}

async function selectMonster(monster: MonsterSummary) {
  // Fetch full monster details
  const details = await getMonsterDetails(monster.name, monster.source)
  
  if (details) {
    // Format the monster details for display
    modalContent.value = {
      visible: true,
      title: monster.name,
      content: formatMonsterDetails(monster, details)
    }
  }
}

function closeModal() {
  modalContent.value.visible = false
}

function formatMonsterDetails(summary: MonsterSummary, details: any): string {
  let html = '<div class="monster-details">'
  
  // Header info
  html += '<div class="monster-header">'
  html += `<div class="monster-type">${summary.size} ${summary.creature_type}, ${summary.alignment}</div>`
  html += '</div>'
  
  // Core stats
  html += '<div class="monster-stats">'
  html += `<div class="stat-row"><strong>Armor Class:</strong> ${summary.ac}`
  // Handle AC details (can be array or single value)
  if (details.ac) {
    if (Array.isArray(details.ac) && details.ac[0] && details.ac[0].from) {
      html += ` (${details.ac[0].from.join(', ')})`
    } else if (details.ac.from) {
      html += ` (${details.ac.from.join(', ')})`
    }
  }
  html += '</div>'
  
  html += `<div class="stat-row"><strong>Hit Points:</strong> ${summary.hp}`
  // Handle HP details (can be object or number)
  if (details.hp && typeof details.hp === 'object' && details.hp.formula) {
    html += ` (${details.hp.formula})`
  }
  html += '</div>'
  
  // Speed
  if (details.speed) {
    let speeds = []
    // Helper to extract speed value from number or object
    const getSpeed = (val: any) => {
      if (typeof val === 'number') return val
      if (val && typeof val === 'object' && val.number) return val.number
      return null
    }
    
    const walk = getSpeed(details.speed.walk)
    const fly = getSpeed(details.speed.fly)
    const swim = getSpeed(details.speed.swim)
    const climb = getSpeed(details.speed.climb)
    const burrow = getSpeed(details.speed.burrow)
    
    if (walk) speeds.push(`${walk} ft.`)
    if (fly) {
      let flyStr = `fly ${fly} ft.`
      if (details.speed.hover || details.speed.canHover) flyStr += ' (hover)'
      speeds.push(flyStr)
    }
    if (swim) speeds.push(`swim ${swim} ft.`)
    if (climb) speeds.push(`climb ${climb} ft.`)
    if (burrow) speeds.push(`burrow ${burrow} ft.`)
    
    if (speeds.length > 0) {
      html += `<div class="stat-row"><strong>Speed:</strong> ${speeds.join(', ')}</div>`
    }
  }
  html += '</div>'
  
  // Ability scores
  html += '<div class="ability-scores">'
  html += '<table class="abilities-table"><tr>'
  html += `<th>STR</th><th>DEX</th><th>CON</th><th>INT</th><th>WIS</th><th>CHA</th>`
  html += '</tr><tr>'
  html += `<td>${details.str || 10} (${getModifier(details.str || 10)})</td>`
  html += `<td>${details.dex || 10} (${getModifier(details.dex || 10)})</td>`
  html += `<td>${details.con || 10} (${getModifier(details.con || 10)})</td>`
  html += `<td>${details.int || 10} (${getModifier(details.int || 10)})</td>`
  html += `<td>${details.wis || 10} (${getModifier(details.wis || 10)})</td>`
  html += `<td>${details.cha || 10} (${getModifier(details.cha || 10)})</td>`
  html += '</tr></table>'
  html += '</div>'
  
  // Skills, senses, languages
  html += '<div class="monster-details-section">'
  if (details.skill) {
    const skills = Object.entries(details.skill).map(([skill, bonus]) => `${skill} ${bonus}`).join(', ')
    html += `<div><strong>Skills:</strong> ${skills}</div>`
  }
  if (details.senses && details.senses.length > 0) {
    html += `<div><strong>Senses:</strong> ${details.senses.join(', ')}, passive Perception ${details.passive || 10}</div>`
  }
  if (details.languages && details.languages.length > 0) {
    html += `<div><strong>Languages:</strong> ${details.languages.join(', ')}</div>`
  }
  html += `<div><strong>Challenge:</strong> ${summary.cr}</div>`
  html += '</div>'
  
  // Traits
  if (details.trait_entries && details.trait_entries.length > 0) {
    html += '<div class="monster-traits">'
    html += '<h4>Traits</h4>'
    for (const trait of details.trait_entries) {
      if (trait.name) {
        html += `<div class="trait"><strong>${trait.name}.</strong> `
        if (trait.entries) {
          for (const entry of trait.entries) {
            if (typeof entry === 'string') {
              html += parse5etoolsTags(entry)
            }
          }
        }
        html += '</div>'
      }
    }
    html += '</div>'
  }
  
  // Actions
  if (details.action && details.action.length > 0) {
    html += '<div class="monster-actions">'
    html += '<h4>Actions</h4>'
    for (const action of details.action) {
      if (action.name) {
        html += `<div class="action"><strong>${action.name}.</strong> `
        if (action.entries) {
          for (const entry of action.entries) {
            if (typeof entry === 'string') {
              html += parse5etoolsTags(entry)
            }
          }
        }
        html += '</div>'
      }
    }
    html += '</div>'
  }
  
  // Source
  html += `<div class="monster-source"><em>Source: ${summary.source}</em></div>`
  
  html += '</div>'
  return html
}

// Get ability modifier
function getModifier(score: number): string {
  const mod = Math.floor((score - 10) / 2)
  return mod >= 0 ? `+${mod}` : `${mod}`
}

// Format cost from copper pieces to standard notation
function formatCost(value?: number): string {
  if (!value && value !== 0) return '—'
  
  // Handle fractional copper (for items worth less than 1 cp)
  if (value < 1) {
    return `${value} cp`
  }
  
  // Convert copper to gold/silver/copper
  const gold = Math.floor(value / 100)
  const silver = Math.floor((value % 100) / 10)
  const copper = Math.floor(value % 10)
  
  if (gold > 0) {
    return `${gold} gp`
  } else if (silver > 0) {
    return `${silver} sp`
  } else {
    return `${copper} cp`
  }
}

// Format weight with proper decimal handling
function formatWeight(weight?: number): string {
  if (!weight) return '—'
  
  // If it's a whole number, show without decimal
  if (weight === Math.floor(weight)) {
    return `${weight} lb.`
  }
  
  // Otherwise show with appropriate decimals
  return `${weight} lb.`
}

function formatSpellDetails(summary: SpellSummary, details: any): string {
  let html = '<div class="spell-details">'
  
  // Header info
  html += '<div class="spell-header-info">'
  html += `<div class="spell-level-school">${summary.level === 0 ? 'Cantrip' : `Level ${summary.level}`} ${summary.school}</div>`
  
  // Tags
  if (summary.ritual || summary.concentration) {
    html += '<div class="spell-tags">'
    if (summary.ritual) html += '<span class="tag ritual">Ritual</span>'
    if (summary.concentration) html += '<span class="tag concentration">Concentration</span>'
    html += '</div>'
  }
  html += '</div>'
  
  // Casting details
  html += '<div class="spell-casting-info">'
  html += `<div><strong>Casting Time:</strong> ${summary.casting_time}</div>`
  html += `<div><strong>Range:</strong> ${summary.range}</div>`
  html += `<div><strong>Components:</strong> ${summary.components}`
  
  // Add material component details if present
  if (details.components?.m && typeof details.components.m === 'string') {
    html += ` (${details.components.m})`
  } else if (details.components?.m && typeof details.components.m === 'object' && details.components.m.text) {
    html += ` (${details.components.m.text})`
  }
  html += '</div>'
  
  // Duration
  if (details.duration && details.duration.length > 0) {
    const dur = details.duration[0]
    let durationText = dur.type
    if (dur.duration) {
      durationText = `${dur.duration.amount} ${dur.duration.type}`
    }
    if (dur.concentration) {
      durationText = `Concentration, up to ${durationText}`
    }
    html += `<div><strong>Duration:</strong> ${durationText}</div>`
  }
  html += '</div>'
  
  // Classes
  if (summary.classes && summary.classes.length > 0) {
    html += `<div class="spell-classes"><strong>Classes:</strong> ${summary.classes.join(', ')}</div>`
  }
  
  // Description/Entries
  html += '<div class="spell-description">'
  if (details.entries) {
    for (const entry of details.entries) {
      if (typeof entry === 'string') {
        html += `<p>${parse5etoolsTags(entry)}</p>`
      } else if (entry.type === 'list' && entry.items) {
        html += '<ul>'
        for (const item of entry.items) {
          html += `<li>${parse5etoolsTags(item)}</li>`
        }
        html += '</ul>'
      } else if (entry.type === 'entries' && entry.entries) {
        if (entry.name) {
          html += `<h4>${parse5etoolsTags(entry.name)}</h4>`
        }
        for (const subEntry of entry.entries) {
          if (typeof subEntry === 'string') {
            html += `<p>${parse5etoolsTags(subEntry)}</p>`
          }
        }
      }
    }
  }
  html += '</div>'
  
  // At Higher Levels
  if (details.entriesHigherLevel) {
    html += '<div class="spell-higher-levels">'
    html += '<h4>At Higher Levels</h4>'
    for (const entry of details.entriesHigherLevel) {
      if (typeof entry === 'string') {
        html += `<p>${parse5etoolsTags(entry)}</p>`
      } else if (entry.entries) {
        for (const subEntry of entry.entries) {
          if (typeof subEntry === 'string') {
            html += `<p>${parse5etoolsTags(subEntry)}</p>`
          }
        }
      }
    }
    html += '</div>'
  }
  
  // Source
  html += `<div class="spell-source"><em>Source: ${summary.source}</em></div>`
  
  html += '</div>'
  return html
}

function formatItemDetails(summary: ItemSummary, details: any): string {
  let html = '<div class="item-details">'
  
  // Header info
  html += '<div class="item-header-info">'
  html += `<div class="item-type">${summary.type_name}</div>`
  if (summary.rarity && summary.rarity !== 'none') {
    html += `<div class="item-rarity">${summary.rarity}</div>`
  }
  html += '</div>'
  
  // Item properties
  html += '<div class="item-properties">'
  if (summary.value) {
    html += `<div><strong>Cost:</strong> ${formatCost(summary.value)}</div>`
  }
  if (summary.weight) {
    html += `<div><strong>Weight:</strong> ${summary.weight} lb.</div>`
  }
  if (summary.ac) {
    html += `<div><strong>AC:</strong> ${summary.ac}</div>`
  }
  if (summary.damage) {
    html += `<div><strong>Damage:</strong> ${summary.damage}</div>`
  }
  html += '</div>'
  
  // Description/Entries
  html += '<div class="item-description">'
  if (details.entries) {
    for (const entry of details.entries) {
      if (typeof entry === 'string') {
        html += `<p>${parse5etoolsTags(entry)}</p>`
      } else if (entry.type === 'list' && entry.items) {
        html += '<ul>'
        for (const item of entry.items) {
          html += `<li>${parse5etoolsTags(item)}</li>`
        }
        html += '</ul>'
      } else if (entry.type === 'entries' && entry.entries) {
        if (entry.name) {
          html += `<h4>${parse5etoolsTags(entry.name)}</h4>`
        }
        for (const subEntry of entry.entries) {
          if (typeof subEntry === 'string') {
            html += `<p>${parse5etoolsTags(subEntry)}</p>`
          }
        }
      }
    }
  }
  
  // Additional entries (for tools, etc.)
  if (details.additionalEntries) {
    html += '<div class="item-additional">'
    for (const entry of details.additionalEntries) {
      if (typeof entry === 'string') {
        html += `<p>${parse5etoolsTags(entry)}</p>`
      } else if (entry.type === 'entries' && entry.entries) {
        if (entry.name) {
          html += `<h4>${parse5etoolsTags(entry.name)}</h4>`
        }
        for (const subEntry of entry.entries) {
          if (typeof subEntry === 'string') {
            html += `<p>${parse5etoolsTags(subEntry)}</p>`
          }
        }
      } else if (entry.type === 'table' && entry.rows) {
        html += '<table class="item-table">'
        if (entry.colLabels) {
          html += '<thead><tr>'
          for (const label of entry.colLabels) {
            html += `<th>${label}</th>`
          }
          html += '</tr></thead>'
        }
        html += '<tbody>'
        for (const row of entry.rows) {
          html += '<tr>'
          for (const cell of row) {
            html += `<td>${parse5etoolsTags(cell)}</td>`
          }
          html += '</tr>'
        }
        html += '</tbody></table>'
      }
    }
    html += '</div>'
  }
  html += '</div>'
  
  // Source
  html += `<div class="item-source"><em>Source: ${summary.source}</em></div>`
  
  html += '</div>'
  return html
}

function parse5etoolsTags(text: string): string {
  if (!text) return text
  
  // Replace common 5etools tags
  return text
    // Dice rolls: {@dice 1d4}, {@dice 2d6+3}, etc.
    .replace(/{@dice ([^}]+)}/gi, '<span class="dice-roll">$1</span>')
    
    // Damage: {@damage 1d6}, {@damage 2d8+2}, etc.
    .replace(/{@damage ([^}]+)}/gi, '<span class="damage">$1</span>')
    
    // Conditions: {@condition blinded}, {@condition poisoned}, etc.
    .replace(/{@condition ([^}]+)}/gi, '<span class="condition">$1</span>')
    
    // Status: {@status concentration}, etc.
    .replace(/{@status ([^}]+)}/gi, '<span class="status">$1</span>')
    
    // Skills: {@skill Athletics}, {@skill Perception}, etc.
    .replace(/{@skill ([^}]+)}/gi, '<span class="skill">$1</span>')
    
    // Actions: {@action Dash}, {@action Attack}, etc.
    .replace(/{@action ([^}]+)}/gi, '<span class="action">$1</span>')
    
    // Creatures: {@creature goblin}, {@creature dragon|MM|red dragon}, etc.
    .replace(/{@creature ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="creature">$1</span>')
    
    // Items: {@item longsword}, {@item potion of healing}, etc.
    .replace(/{@item ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="item">$1</span>')
    
    // Spells: {@spell fireball}, {@spell magic missile}, etc.
    .replace(/{@spell ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="spell-ref">$1</span>')
    
    // Classes: {@class fighter}, {@class wizard}, etc.
    .replace(/{@class ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="class-ref">$1</span>')
    
    // Filters: {@filter warlock|spells|level=0;1;2|class=warlock}
    .replace(/{@filter ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="filter">$1</span>')
    
    // Chance: {@chance 25}, {@chance 50|50% chance}, etc.
    .replace(/{@chance (\d+)(?:\|([^}]+))?}/gi, (match, num, text) => {
      return `<span class="chance">${text || num + '%'}</span>`
    })
    
    // DC: {@dc 15}, {@dc 13|Wisdom}, etc.
    .replace(/{@dc (\d+)(?:\|([^}]+))?}/gi, (match, dc, ability) => {
      return `<span class="dc">DC ${dc}${ability ? ' ' + ability : ''}</span>`
    })
    
    // Hit: {@hit 5}, {@hit +7}, etc.
    .replace(/{@hit ([^}]+)}/gi, '<span class="hit">$1</span>')
    
    // Recharge: {@recharge}, {@recharge 5}, etc.
    .replace(/{@recharge\s*(\d+)?}/gi, (match, num) => {
      return `<span class="recharge">(Recharge${num ? ' ' + num + '-6' : ''})</span>`
    })
    
    // Scaledice: {@scaledice 1d6|3-9|1d6}
    .replace(/{@scaledice ([^}]+)}/gi, '<span class="dice-roll">$1</span>')
    
    // Scaledamage: {@scaledamage 1d6|3-9|1d6}
    .replace(/{@scaledamage ([^}]+)}/gi, '<span class="damage">$1</span>')
    
    // Catch-all for any remaining tags
    .replace(/{@\w+ ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="tagged">$1</span>')
}
</script>

<style scoped>
.catalog-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--color-background, #0d0d0d);
}

/* Header styles */
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

.header-info {
  display: flex;
  align-items: baseline;
  gap: var(--spacing-lg, 16px);
}

.header-info h2 {
  color: var(--color-text, #e0e0e0);
  margin: 0;
  font-size: 1.25rem;
}

.results-count {
  color: var(--color-text-secondary, #999);
  font-size: 0.9rem;
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

/* Content area */
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

/* Table styles */
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

/* Column widths */
.col-name { 
  min-width: 150px; 
  max-width: 250px; 
}
.item-damage, .item-ac {
  margin-left: 8px;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.85em;
  font-weight: 500;
}
.item-damage {
  background: rgba(220, 38, 38, 0.2);
  color: #ef4444;
  border: 1px solid rgba(220, 38, 38, 0.3);
}
.item-ac {
  background: rgba(59, 130, 246, 0.2);
  color: #60a5fa;
  border: 1px solid rgba(59, 130, 246, 0.3);
}
.col-level { width: 80px; }
.col-school { width: 110px; }
.col-time { width: 100px; }
.col-range { width: 100px; }
.col-components { width: 70px; }
.col-tags { width: 120px; }
.col-source { width: 60px; }

/* Spell name styling */
.spell-name {
  color: var(--color-primary, #4a9eff);
  font-weight: 500;
}

/* Level badge */
.level-badge {
  display: inline-block;
  padding: 2px 6px;
  border-radius: 3px;
  background: var(--color-surface, #1a1a1a);
  color: var(--color-text, #e0e0e0);
  font-weight: 500;
  font-size: 0.8rem;
  text-align: center;
  white-space: nowrap;
}

.level-badge.level-0 {
  background: var(--color-info-alpha, rgba(74, 158, 255, 0.2));
  color: var(--color-info, #4a9eff);
}

/* Filter controls */
.filter-select {
  width: 100%;
  padding: 2px 4px;
  background: var(--color-background, #0d0d0d);
  border: 1px solid var(--color-border, #333);
  border-radius: 3px;
  color: var(--color-text, #e0e0e0);
  font-size: 0.85rem;
  cursor: pointer;
}

.filter-select:focus {
  outline: none;
  border-color: var(--color-primary, #4a9eff);
}

/* Sort button */
.sort-btn {
  background: none;
  border: none;
  color: var(--color-text-secondary, #999);
  cursor: pointer;
  padding: 2px 4px;
  margin-left: 4px;
  font-size: 0.9rem;
  vertical-align: middle;
}

.sort-btn:hover {
  color: var(--color-text, #e0e0e0);
}

/* Tag filters */
.tag-filters {
  display: flex;
  gap: var(--spacing-xs, 4px);
  justify-content: center;
}

.tag-filter {
  display: flex;
  align-items: center;
  gap: 2px;
  cursor: pointer;
}

.tag-filter input[type="checkbox"] {
  margin: 0;
  cursor: pointer;
}

.tag-filter span {
  font-weight: 600;
  font-size: 0.85rem;
}

/* Tag badges */
.tag-badge {
  display: inline-block;
  padding: 1px 4px;
  border-radius: 3px;
  font-size: 0.75rem;
  font-weight: 600;
  margin: 0 2px;
}

.tag-badge.concentration {
  background: var(--color-warning-alpha, rgba(255, 170, 0, 0.2));
  color: var(--color-warning, #ffaa00);
}

.tag-badge.ritual {
  background: var(--color-info-alpha, rgba(74, 158, 255, 0.2));
  color: var(--color-info, #4a9eff);
}

.placeholder-text {
  color: var(--color-text-secondary, #999);
  text-align: center;
  font-style: italic;
  padding: var(--spacing-xl, 24px);
}

/* Loading and error states */
.loading-message,
.error-message,
.no-results {
  text-align: center;
  padding: var(--spacing-lg, 16px);
  color: var(--color-text-secondary, #999);
}

.error-message {
  color: var(--color-danger, #ff4444);
}

/* Results list */
.results-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm, 8px);
}

.results-count {
  padding: var(--spacing-sm, 8px);
  color: var(--color-text-secondary, #999);
  font-size: 0.85rem;
  border-bottom: 1px solid var(--color-border, #333);
}

/* Spell item */
.spell-item {
  padding: var(--spacing-md, 12px);
  background: var(--color-background, #0d0d0d);
  border: 1px solid var(--color-border, #333);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.spell-item:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
  border-color: var(--color-primary, #4a9eff);
}

.spell-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: var(--spacing-xs, 4px);
}

.spell-name {
  font-weight: 600;
  color: var(--color-primary, #4a9eff);
  font-size: 1rem;
}

.spell-level {
  font-size: 0.85rem;
  color: var(--color-text-secondary, #999);
  font-weight: 500;
}

.spell-details {
  display: flex;
  gap: var(--spacing-sm, 8px);
  align-items: center;
  margin-bottom: var(--spacing-xs, 4px);
}

.spell-school {
  font-size: 0.85rem;
  color: var(--color-text, #e0e0e0);
  font-style: italic;
}

.spell-tag {
  padding: 1px 4px;
  border-radius: 3px;
  font-size: 0.75rem;
  font-weight: 600;
}

.spell-tag.concentration {
  background: var(--color-warning-alpha, rgba(255, 170, 0, 0.2));
  color: var(--color-warning, #ffaa00);
}

.spell-tag.ritual {
  background: var(--color-info-alpha, rgba(74, 158, 255, 0.2));
  color: var(--color-info, #4a9eff);
}

.spell-source {
  font-size: 0.75rem;
  color: var(--color-text-tertiary, #666);
  margin-left: auto;
}

.spell-info {
  display: flex;
  gap: var(--spacing-xs, 4px);
  font-size: 0.85rem;
  color: var(--color-text-secondary, #999);
  margin-bottom: var(--spacing-xs, 4px);
}

.spell-separator {
  color: var(--color-text-tertiary, #666);
}

.spell-time,
.spell-range,
.spell-components {
  white-space: nowrap;
}

.spell-classes {
  font-size: 0.85rem;
  color: var(--color-success, #44ff44);
  font-style: italic;
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--color-surface, #1a1a1a);
  border: 1px solid var(--color-border, #333);
  border-radius: 8px;
  max-width: 800px;
  max-height: 80vh;
  width: 90%;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-lg, 16px);
  border-bottom: 1px solid var(--color-border, #333);
}

.modal-header h2 {
  margin: 0;
  color: var(--color-primary, #4a9eff);
  font-size: 1.5rem;
}

.modal-close {
  background: none;
  border: none;
  color: var(--color-text-secondary, #999);
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-close:hover {
  color: var(--color-text, #e0e0e0);
}

.modal-body {
  padding: var(--spacing-lg, 16px);
  overflow-y: auto;
  flex: 1;
  color: var(--color-text, #e0e0e0);
}

/* Spell details in modal */
.modal-body :deep(.spell-details) {
  font-size: 0.95rem;
  line-height: 1.6;
}

.modal-body :deep(.spell-header-info) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md, 12px);
  padding-bottom: var(--spacing-sm, 8px);
  border-bottom: 1px solid var(--color-border, #333);
}

.modal-body :deep(.spell-level-school) {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--color-text, #e0e0e0);
}

.modal-body :deep(.spell-tags) {
  display: flex;
  gap: var(--spacing-xs, 4px);
}

.modal-body :deep(.spell-tags .tag) {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 0.85rem;
  font-weight: 600;
}

.modal-body :deep(.spell-tags .tag.ritual) {
  background: var(--color-info-alpha, rgba(74, 158, 255, 0.2));
  color: var(--color-info, #4a9eff);
}

.modal-body :deep(.spell-tags .tag.concentration) {
  background: var(--color-warning-alpha, rgba(255, 170, 0, 0.2));
  color: var(--color-warning, #ffaa00);
}

.modal-body :deep(.spell-casting-info) {
  margin-bottom: var(--spacing-md, 12px);
  padding: var(--spacing-sm, 8px);
  background: var(--color-background, #0d0d0d);
  border-radius: 4px;
}

.modal-body :deep(.spell-casting-info div) {
  margin-bottom: var(--spacing-xs, 4px);
}

.modal-body :deep(.spell-casting-info div:last-child) {
  margin-bottom: 0;
}

.modal-body :deep(.spell-classes) {
  margin-bottom: var(--spacing-md, 12px);
  color: var(--color-success, #44ff44);
  font-style: italic;
}

.modal-body :deep(.spell-description) {
  margin-bottom: var(--spacing-md, 12px);
}

.modal-body :deep(.spell-description p) {
  margin-bottom: var(--spacing-sm, 8px);
}

.modal-body :deep(.spell-description ul) {
  margin-left: var(--spacing-lg, 16px);
  margin-bottom: var(--spacing-sm, 8px);
}

.modal-body :deep(.spell-description h4) {
  color: var(--color-primary, #4a9eff);
  margin-top: var(--spacing-md, 12px);
  margin-bottom: var(--spacing-xs, 4px);
  font-size: 1rem;
}

.modal-body :deep(.spell-higher-levels) {
  margin-top: var(--spacing-lg, 16px);
  padding-top: var(--spacing-md, 12px);
  border-top: 1px solid var(--color-border, #333);
}

.modal-body :deep(.spell-higher-levels h4) {
  color: var(--color-primary, #4a9eff);
  margin-bottom: var(--spacing-sm, 8px);
}

.modal-body :deep(.spell-source) {
  margin-top: var(--spacing-lg, 16px);
  padding-top: var(--spacing-md, 12px);
  border-top: 1px solid var(--color-border, #333);
  color: var(--color-text-secondary, #999);
  font-size: 0.9rem;
}

/* 5etools tag styles */
.modal-body :deep(.dice-roll),
.modal-body :deep(.damage) {
  color: var(--color-warning, #ffaa00);
  font-weight: 600;
  font-family: monospace;
}

.modal-body :deep(.condition) {
  color: var(--color-danger, #ff4444);
  font-weight: 600;
  font-style: italic;
}

.modal-body :deep(.status) {
  color: var(--color-info, #4a9eff);
  font-weight: 600;
}

.modal-body :deep(.skill),
.modal-body :deep(.action) {
  color: var(--color-success, #44ff44);
  font-weight: 600;
}

.modal-body :deep(.creature),
.modal-body :deep(.item),
.modal-body :deep(.spell-ref),
.modal-body :deep(.class-ref) {
  color: var(--color-primary, #4a9eff);
  font-style: italic;
  cursor: pointer;
  text-decoration: underline;
  text-decoration-style: dotted;
}

.modal-body :deep(.creature:hover),
.modal-body :deep(.item:hover),
.modal-body :deep(.spell-ref:hover),
.modal-body :deep(.class-ref:hover) {
  text-decoration-style: solid;
}

.modal-body :deep(.dc),
.modal-body :deep(.hit) {
  color: var(--color-text, #e0e0e0);
  font-weight: 600;
}

.modal-body :deep(.chance) {
  color: var(--color-info, #4a9eff);
  font-style: italic;
}

.modal-body :deep(.recharge) {
  color: var(--color-warning, #ffaa00);
  font-style: italic;
  font-weight: 600;
}

.modal-body :deep(.filter),
.modal-body :deep(.tagged) {
  color: var(--color-text-secondary, #999);
  font-style: italic;
}

/* Monster-specific styles */
.col-size { width: 80px; }
.col-cr { width: 60px; }
.col-hp { width: 60px; }
.col-ac { width: 60px; }
.col-alignment { width: 120px; }

.monster-row {
  cursor: pointer;
  transition: background-color 0.2s;
}

.monster-row:hover {
  background: var(--color-surface-hover, #252525);
}

.monster-details .abilities-table {
  margin: 12px 0;
  width: 100%;
  border-collapse: collapse;
}

.monster-details .abilities-table th {
  background: var(--color-surface, #1a1a1a);
  padding: 4px 8px;
  border: 1px solid var(--color-border, #333);
  font-weight: 600;
}

.monster-details .abilities-table td {
  padding: 4px 8px;
  border: 1px solid var(--color-border, #333);
  text-align: center;
}

.monster-traits,
.monster-actions,
.monster-reactions,
.monster-legendary {
  margin-top: 16px;
}

.monster-traits h4,
.monster-actions h4,
.monster-reactions h4,
.monster-legendary h4 {
  color: var(--color-primary, #007bff);
  margin-bottom: 8px;
  border-bottom: 1px solid var(--color-border, #333);
  padding-bottom: 4px;
}

.trait,
.action,
.reaction,
.legendary {
  margin-bottom: 8px;
}
</style>