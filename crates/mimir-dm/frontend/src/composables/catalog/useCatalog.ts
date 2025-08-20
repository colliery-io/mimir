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

export interface ClassSummary {
  name: string
  source: string
  hitDie: string
  primaryAbility: string
  saves: string
  spellcaster: boolean
  description: string
  // Additional subclass information
  subclassFeatures?: string[]
  additionalSpells?: string[]
  spellcastingAbility?: string
  cantripProgression?: number[]
}

export interface CharacterClass {
  name: string
  source: string
  page?: number
  className?: string  // If present, this is a subclass
  classSource?: string
  subclassShortName?: string
  hd?: { faces: number, number: number }
  proficiency?: string[]
  startingProficiencies?: any
  startingEquipment?: any
  multiclassing?: any
  casterProgression?: string
  cantripProgression?: number[]
  spellSlotsProgression?: any
  spellsKnownProgression?: number[]
  spellcastingAbility?: string
  classFeatures?: any[]
  subclassFeatures?: string[]
  additionalSpells?: any[]
  subclassTitle?: string
  srd?: any
  basicRules?: boolean
}

export interface ClassFilters {
  query?: string
  sources?: string[]
  spellcaster?: boolean
}

export interface RaceSummary {
  name: string
  source: string
  size: string
  speed: number
  abilityBonuses: string
  traits: string[]
  description: string
}

export interface RaceFilters {
  query?: string
  sources?: string[]
  sizes?: string[]
}

export interface FeatSummary {
  name: string
  source: string
  prerequisite: string
  description: string
}

export interface FeatFilters {
  query?: string
  sources?: string[]
}

export interface BackgroundSummary {
  name: string
  source: string
  skills: string
  languages: string
  tools: string
  description: string
}

export interface BackgroundFilters {
  query?: string
  sources?: string[]
}

export function useCatalog() {
  const isInitialized = ref(false)
  const isItemsInitialized = ref(false)
  const isMonstersInitialized = ref(false)
  const isClassesInitialized = ref(false)
  const isRacesInitialized = ref(false)
  const isFeatsInitialized = ref(false)
  const isBackgroundsInitialized = ref(false)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const spells = ref<SpellSummary[]>([])
  const items = ref<ItemSummary[]>([])
  const monsters = ref<MonsterSummary[]>([])
  const classes = ref<ClassSummary[]>([])
  const races = ref<RaceSummary[]>([])
  const feats = ref<FeatSummary[]>([])
  const backgrounds = ref<BackgroundSummary[]>([])

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

  // Initialize the class catalog
  async function initializeClassCatalog() {
    if (isClassesInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('initialize_class_catalog')
      isClassesInitialized.value = true
      console.log('Class catalog initialized')
    } catch (e) {
      error.value = `Failed to initialize class catalog: ${e}`
      console.error('Failed to initialize class catalog:', e)
    } finally {
      isLoading.value = false
    }
  }

  // Search classes with filters
  async function searchClasses(filters: ClassFilters): Promise<ClassSummary[]> {
    if (!isClassesInitialized.value) {
      await initializeClassCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<ClassSummary[]>('search_classes', {
        query: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
        spellcaster: filters.spellcaster !== undefined ? filters.spellcaster : null,
      })
      
      classes.value = results
      return results
    } catch (e) {
      error.value = `Search failed: ${e}`
      console.error('Search failed:', e)
      return []
    } finally {
      isLoading.value = false
    }
  }
  
  // Get detailed class information
  async function getClassDetails(name: string, source: string): Promise<CharacterClass | null> {
    try {
      const classDetails = await invoke<CharacterClass | null>('get_class_details', { name, source })
      return classDetails
    } catch (e) {
      console.error('Failed to get class details:', e)
      return null
    }
  }

  // Initialize the race catalog
  async function initializeRaceCatalog() {
    if (isRacesInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('initialize_race_catalog')
      isRacesInitialized.value = true
      console.log('Race catalog initialized')
    } catch (e) {
      error.value = `Failed to initialize race catalog: ${e}`
      console.error('Failed to initialize race catalog:', e)
    } finally {
      isLoading.value = false
    }
  }

  // Search races with filters
  async function searchRaces(filters: RaceFilters): Promise<RaceSummary[]> {
    if (!isRacesInitialized.value) {
      await initializeRaceCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<RaceSummary[]>('search_races', {
        query: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
        sizes: filters.sizes && filters.sizes.length > 0 ? filters.sizes : null,
      })
      
      races.value = results
      return results
    } catch (e) {
      error.value = `Search failed: ${e}`
      console.error('Search failed:', e)
      return []
    } finally {
      isLoading.value = false
    }
  }

  // Initialize the feat catalog
  async function initializeFeatCatalog() {
    if (isFeatsInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('initialize_feat_catalog')
      isFeatsInitialized.value = true
      console.log('Feat catalog initialized')
    } catch (e) {
      error.value = `Failed to initialize feat catalog: ${e}`
      console.error('Failed to initialize feat catalog:', e)
    } finally {
      isLoading.value = false
    }
  }

  // Search feats with filters
  async function searchFeats(filters: FeatFilters): Promise<FeatSummary[]> {
    if (!isFeatsInitialized.value) {
      await initializeFeatCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<FeatSummary[]>('search_feats', {
        query: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
      })
      
      feats.value = results
      return results
    } catch (e) {
      error.value = `Search failed: ${e}`
      console.error('Search failed:', e)
      return []
    } finally {
      isLoading.value = false
    }
  }

  // Initialize the background catalog
  async function initializeBackgroundCatalog() {
    if (isBackgroundsInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('initialize_background_catalog')
      isBackgroundsInitialized.value = true
      console.log('Background catalog initialized')
    } catch (e) {
      error.value = `Failed to initialize background catalog: ${e}`
      console.error('Failed to initialize background catalog:', e)
    } finally {
      isLoading.value = false
    }
  }

  // Search backgrounds with filters
  async function searchBackgrounds(filters: BackgroundFilters): Promise<BackgroundSummary[]> {
    if (!isBackgroundsInitialized.value) {
      await initializeBackgroundCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<BackgroundSummary[]>('search_backgrounds', {
        query: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
      })
      
      backgrounds.value = results
      return results
    } catch (e) {
      error.value = `Search failed: ${e}`
      console.error('Search failed:', e)
      return []
    } finally {
      isLoading.value = false
    }
  }

  return {
    isInitialized,
    isItemsInitialized,
    isMonstersInitialized,
    isClassesInitialized,
    isRacesInitialized,
    isFeatsInitialized,
    isBackgroundsInitialized,
    isLoading,
    error,
    spells,
    items,
    monsters,
    classes,
    races,
    feats,
    backgrounds,
    initializeCatalog,
    initializeItemCatalog,
    initializeMonsterCatalog,
    initializeClassCatalog,
    initializeRaceCatalog,
    initializeFeatCatalog,
    initializeBackgroundCatalog,
    searchSpells,
    searchItems,
    searchMonsters,
    searchClasses,
    searchRaces,
    searchFeats,
    searchBackgrounds,
    getClassDetails,
    getSpellDetails,
    getItemDetails,
    getMonsterDetails,
  }
}