import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface SpellSummary {
  name: string
  level: number
  school: string
  source: string
  concentration: boolean
  ritual: boolean
  casting_time: string
  range: string
  components: string
  classes: string[]
  description: string
}

export interface SpellFilters {
  query?: string
  sources?: string[]
  levels?: number[]
  schools?: string[]
  ritual?: boolean
  concentration?: boolean
}

export interface ItemSummary {
  name: string
  itemType: string
  typeName: string
  source: string
  rarity: string
  value?: number
  weight?: number
  ac?: number
  damage?: string
  reqAttune?: string
  description: string
}

export interface ItemFilters {
  query?: string
  sources?: string[]
  types?: string[]
  rarities?: string[]
  min_value?: number
  max_value?: number
}

export interface MonsterSummary {
  name: string
  size: string
  type: string
  alignment: string
  cr: string
  hp: string
  ac: string
  speed: string
  source: string
  str: number
  dex: number
  con: number
  int: number
  wis: number
  cha: number
  senses?: string
  languages?: string
  description?: string
  creature_type?: string  // Alternative field name
  environment?: string[]  // Environment tags
}

export interface MonsterFilters {
  query?: string
  sources?: string[]
  types?: string[]
  sizes?: string[]
  min_cr?: number
  max_cr?: number
}

// Type definitions for full details
export interface Spell {
  name: string
  source: string
  level: number
  school: string
  time: any[]
  range: any
  components: any
  duration: any[]
  entries: string[]
  scalingLevelDice?: any
  damageInflict?: string[]
  conditionInflict?: string[]
  savingThrow?: string[]
  miscTags?: string[]
  areaTags?: string[]
  classes?: any
}

export interface Item {
  name: string
  source: string
  type: string
  rarity: string
  weight?: number
  value?: number
  entries?: string[]
  // Add other properties as needed
}

export interface Monster {
  name: string
  source: string
  size: string[]
  type: any
  alignment?: any[]
  ac: any[]
  hp: any
  speed: any
  str: number
  dex: number
  con: number
  int: number
  wis: number
  cha: number
  save?: any
  skill?: any
  senses?: string[]
  languages?: string[]
  cr: string
  trait?: any[]
  action?: any[]
  legendary?: any[]
  immune?: any[]
  resist?: any[]
  vulnerable?: any[]
  conditionImmune?: string[]
  spellcasting?: any[]
  entries?: string[]
  // Fluff fields
  fluffEntries?: any[]
  fluffImages?: any[]
  fluff_images?: any[]
}

export function useCatalog() {
  // State
  const isInitialized = ref(false)
  const isItemsInitialized = ref(false)
  const isMonstersInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)
  
  // Data stores
  const spells = ref<SpellSummary[]>([])
  const items = ref<ItemSummary[]>([])
  const monsters = ref<MonsterSummary[]>([])

  // Initialize the spell catalog
  async function initializeCatalog() {
    if (isInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('initialize_spell_catalog')
      isInitialized.value = true
      console.log('Spell catalog initialized')
    } catch (e) {
      error.value = `Failed to initialize catalog: ${e}`
      console.error('Failed to initialize catalog:', e)
    } finally {
      isLoading.value = false
    }
  }

  // Initialize the item catalog
  async function initializeItemCatalog() {
    if (isItemsInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('initialize_item_catalog')
      isItemsInitialized.value = true
      console.log('Item catalog initialized')
    } catch (e) {
      error.value = `Failed to initialize item catalog: ${e}`
      console.error('Failed to initialize item catalog:', e)
    } finally {
      isLoading.value = false
    }
  }

  // Initialize the monster catalog
  async function initializeMonsterCatalog() {
    if (isMonstersInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('initialize_monster_catalog')
      isMonstersInitialized.value = true
      console.log('Monster catalog initialized')
    } catch (e) {
      error.value = `Failed to initialize monster catalog: ${e}`
      console.error('Failed to initialize monster catalog:', e)
    } finally {
      isLoading.value = false
    }
  }

  // Search spells with filters
  async function searchSpells(filters: SpellFilters): Promise<SpellSummary[]> {
    if (!isInitialized.value) {
      await initializeCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<SpellSummary[]>('search_spells', {
        query: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
        levels: filters.levels && filters.levels.length > 0 ? filters.levels : null,
        schools: filters.schools && filters.schools.length > 0 ? filters.schools : null,
        ritual: filters.ritual !== undefined ? filters.ritual : null,
        concentration: filters.concentration !== undefined ? filters.concentration : null,
      })
      
      spells.value = results
      return results
    } catch (e) {
      error.value = `Search failed: ${e}`
      console.error('Search failed:', e)
      return []
    } finally {
      isLoading.value = false
    }
  }

  // Search items with filters
  async function searchItems(filters: ItemFilters): Promise<ItemSummary[]> {
    if (!isItemsInitialized.value) {
      await initializeItemCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<ItemSummary[]>('search_items', {
        query: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
        types: filters.types && filters.types.length > 0 ? filters.types : null,
        rarities: filters.rarities && filters.rarities.length > 0 ? filters.rarities : null,
        minValue: filters.min_value !== undefined ? filters.min_value : null,
        maxValue: filters.max_value !== undefined ? filters.max_value : null,
      })
      
      items.value = results
      return results
    } catch (e) {
      error.value = `Search failed: ${e}`
      console.error('Search failed:', e)
      return []
    } finally {
      isLoading.value = false
    }
  }

  // Search monsters with filters
  async function searchMonsters(filters: MonsterFilters): Promise<MonsterSummary[]> {
    if (!isMonstersInitialized.value) {
      await initializeMonsterCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<MonsterSummary[]>('search_monsters', {
        query: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
        types: filters.types && filters.types.length > 0 ? filters.types : null,
        sizes: filters.sizes && filters.sizes.length > 0 ? filters.sizes : null,
        minCr: filters.min_cr !== undefined ? filters.min_cr : null,
        maxCr: filters.max_cr !== undefined ? filters.max_cr : null,
      })
      
      monsters.value = results
      return results
    } catch (e) {
      error.value = `Search failed: ${e}`
      console.error('Search failed:', e)
      return []
    } finally {
      isLoading.value = false
    }
  }

  // Get detailed spell information
  async function getSpellDetails(name: string, source: string): Promise<Spell | null> {
    try {
      const spell = await invoke<Spell>('get_spell_details', { name, source })
      return spell
    } catch (e) {
      console.error('Failed to get spell details:', e)
      return null
    }
  }

  // Get detailed item information
  async function getItemDetails(name: string, source: string): Promise<Item | null> {
    try {
      const item = await invoke<Item>('get_item_details', { name, source })
      return item
    } catch (e) {
      console.error('Failed to get item details:', e)
      return null
    }
  }

  // Get detailed monster information
  async function getMonsterDetails(name: string, source: string): Promise<Monster | null> {
    try {
      const monster = await invoke<Monster>('get_monster_details', { name, source })
      return monster
    } catch (e) {
      console.error('Failed to get monster details:', e)
      return null
    }
  }

  return {
    // State
    isInitialized,
    isItemsInitialized,
    isMonstersInitialized,
    isLoading,
    error,
    spells,
    items,
    monsters,
    initializeCatalog,
    initializeItemCatalog,
    initializeMonsterCatalog,
    searchSpells,
    searchItems,
    searchMonsters,
    getSpellDetails,
    getItemDetails,
    getMonsterDetails,
  }
}