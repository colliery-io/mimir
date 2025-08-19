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
  item_type: string
  type_name: string
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
  source: string
  size: string
  creature_type: string
  alignment: string
  cr: string
  cr_numeric: number
  hp: number
  ac: number
  environment: string[]
}

export interface MonsterFilters {
  query?: string
  sources?: string[]
  sizes?: string[]
  types?: string[]
  min_cr?: number
  max_cr?: number
  environments?: string[]
}

export function useCatalog() {
  const isInitialized = ref(false)
  const isItemsInitialized = ref(false)
  const isMonstersInitialized = ref(false)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const spells = ref<SpellSummary[]>([])
  const items = ref<ItemSummary[]>([])
  const monsters = ref<MonsterSummary[]>([])

  // Initialize the spell catalog (load data from files)
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

  // Search spells with filters
  async function searchSpells(filters: SpellFilters): Promise<SpellSummary[]> {
    // Initialize if needed
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

  // Get detailed spell information
  async function getSpellDetails(name: string, source: string) {
    try {
      const spell = await invoke('get_spell_details', { name, source })
      return spell
    } catch (e) {
      console.error('Failed to get spell details:', e)
      return null
    }
  }

  // Search items with filters
  async function searchItems(filters: ItemFilters): Promise<ItemSummary[]> {
    // Initialize if needed
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

  // Get detailed item information
  async function getItemDetails(name: string, source: string) {
    try {
      const item = await invoke('get_item_details', { name, source })
      return item
    } catch (e) {
      console.error('Failed to get item details:', e)
      return null
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

  // Search monsters with filters
  async function searchMonsters(filters: MonsterFilters): Promise<MonsterSummary[]> {
    // Initialize if needed
    if (!isMonstersInitialized.value) {
      await initializeMonsterCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<MonsterSummary[]>('search_monsters', {
        query: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
        sizes: filters.sizes && filters.sizes.length > 0 ? filters.sizes : null,
        types: filters.types && filters.types.length > 0 ? filters.types : null,
        minCr: filters.min_cr !== undefined ? filters.min_cr : null,
        maxCr: filters.max_cr !== undefined ? filters.max_cr : null,
        environments: filters.environments && filters.environments.length > 0 ? filters.environments : null,
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

  // Get detailed monster information
  async function getMonsterDetails(name: string, source: string) {
    try {
      const monster = await invoke('get_monster_details', { name, source })
      return monster
    } catch (e) {
      console.error('Failed to get monster details:', e)
      return null
    }
  }

  return {
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