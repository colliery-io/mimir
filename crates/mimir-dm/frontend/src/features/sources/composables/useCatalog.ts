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

export interface ClassSummary {
  name: string
  source: string
  page?: number
  hitDice: string
  proficiency: string
  primaryAbility: string
  spellcastingAbility?: string
  tableGroups?: any[]
  subclassTitle?: string
  description: string
}

export interface ClassFilters {
  query?: string
  source?: string
}

export interface Subclass {
  name: string
  source: string
  class_name: string
  class_source: string
  short_name?: string
  page?: number
  spellcasting_ability?: string
  caster_progression?: string
  subclass_features?: any
  subclass_table_groups?: any[]
}

export interface ClassFeature {
  name: string
  source: string
  class_name: string
  class_source: string
  level: number
  page?: number
  entries?: any[]
}

export interface SubclassFeature {
  name: string
  source: string
  class_name: string
  class_source: string
  subclass_short_name?: string
  subclass_source: string
  level: number
  page?: number
  entries?: any[]
}

export interface ClassFluff {
  name: string
  source: string
  entries: any[]
  images?: any[]
}

export interface SubclassFluff {
  name: string
  short_name?: string
  source: string
  class_name: string
  class_source: string
  entries: any[]
  images?: any[]
}

export interface ClassWithDetails {
  class: Class
  subclasses: Subclass[]
  features: ClassFeature[]
  subclass_features: SubclassFeature[]
  fluff?: ClassFluff
  subclass_fluff: SubclassFluff[]
}

// Race interfaces
export interface RaceSummary {
  name: string
  source: string
  size: string
  speed: number
  abilityBonuses: string
  traitsCount: number
  isSubrace: boolean
  parentRace?: string
}

export interface RaceFilters {
  query?: string
  sources?: string[]
  sizes?: string[]
  has_darkvision?: boolean
  has_flight?: boolean
}

export interface Race {
  name: string
  source: string
  page?: number
  size?: string[]
  speed?: any
  ability?: any[]
  age?: any
  darkvision?: number
  traitTags?: string[]
  languageProficiencies?: any[]
  skillProficiencies?: any[]
  weaponProficiencies?: any[]
  armorProficiencies?: any[]
  toolProficiencies?: any[]
  resist?: string[]
  immune?: string[]
  vulnerable?: string[]
  conditionImmune?: string[]
  entries: any[]
  soundClip?: any
  lineage?: string
  raceName?: string
  raceSource?: string
}

export interface Subrace {
  name: string
  source: string
  page?: number
  raceName: string
  raceSource: string
  ability?: any[]
  speed?: any
  darkvision?: number
  resist?: string[]
  traitTags?: string[]
  languageProficiencies?: any[]
  skillProficiencies?: any[]
  weaponProficiencies?: any[]
  armorProficiencies?: any[]
  toolProficiencies?: any[]
  entries: any[]
  overwrite?: any
}

export interface RaceWithDetails {
  race?: Race
  subrace?: Subrace
  relatedSubraces: Subrace[]
  fluff?: any
}

// Feat interfaces
export interface FeatSummary {
  name: string
  source: string
  page?: number
  prerequisites?: string
  brief?: string
}

export interface Feat {
  name: string
  source: string
  page?: number
  srd?: boolean
  entries: any[]
  prerequisite?: any[]
  ability?: any[]
  skill_proficiencies?: any[]
  language_proficiencies?: any[]
  tool_proficiencies?: any[]
  weapon_proficiencies?: any[]
  armor_proficiencies?: any[]
  saving_throw_proficiencies?: any[]
  expertise?: any[]
  resist?: any[]
  immune?: any[]
  senses?: any[]
  additional_spells?: any[]
  other_sources?: any[]
}

export interface Class {
  name: string
  source: string
  page?: number
  hd?: any // Hit dice object
  proficiency?: any
  startingProficiencies?: any
  spellcastingAbility?: string
  classTableGroups?: any[]
  subclassTitle?: string
  entries?: any[]
  classFeatures?: any[]
  multiclassing?: any
  casterProgression?: string
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
  const isClassesInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)
  
  // Data stores
  const spells = ref<SpellSummary[]>([])
  const items = ref<ItemSummary[]>([])
  const monsters = ref<MonsterSummary[]>([])
  const classes = ref<ClassSummary[]>([])
  const classSources = ref<string[]>([])

  // Initialize the spell catalog
  async function initializeCatalog() {
    if (isInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('initialize_spell_catalog')
      isInitialized.value = true
    } catch (e) {
      error.value = `Failed to initialize catalog: ${e}`
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
    } catch (e) {
      error.value = `Failed to initialize item catalog: ${e}`
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
    } catch (e) {
      error.value = `Failed to initialize monster catalog: ${e}`
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
      return null
    }
  }

  // Get detailed item information
  async function getItemDetails(name: string, source: string): Promise<Item | null> {
    try {
      const item = await invoke<Item>('get_item_details', { name, source })
      return item
    } catch (e) {
      return null
    }
  }

  // Get detailed monster information
  async function getMonsterDetails(name: string, source: string): Promise<Monster | null> {
    try {
      const monster = await invoke<Monster>('get_monster_details', { name, source })
      return monster
    } catch (e) {
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
      
      // Load class sources
      try {
        const sources = await invoke<string[]>('get_class_sources')
        classSources.value = sources
      } catch (e) {
      }
    } catch (e) {
      error.value = `Failed to initialize class catalog: ${e}`
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
        source: filters.source || null,
      })
      
      classes.value = results
      return results
    } catch (e) {
      error.value = `Search failed: ${e}`
      return []
    } finally {
      isLoading.value = false
    }
  }

  // Get detailed class information
  async function getClassDetails(name: string, source: string): Promise<ClassWithDetails | null> {
    try {
      const classDetails = await invoke<ClassWithDetails>('get_class_details', { name, source })
      return classDetails
    } catch (e) {
      return null
    }
  }

  // Get all subclasses for a class
  async function getClassSubclasses(className: string, classSource: string): Promise<Subclass[]> {
    try {
      const subclasses = await invoke<Subclass[]>('get_class_subclasses', { 
        className, 
        classSource 
      })
      return subclasses
    } catch (e) {
      return []
    }
  }

  // Feat catalog functions
  async function initializeFeatCatalog() {
    try {
      await invoke('initialize_feat_catalog')
    } catch (e) {
      throw e
    }
  }

  async function searchFeats(params: { query?: string; source?: string } = {}) {
    try {
      const results = await invoke<FeatSummary[]>('search_feats', params)
      return results || []
    } catch (e) {
      return []
    }
  }

  async function getFeatDetails(name: string, source: string) {
    try {
      const feat = await invoke<Feat>('get_feat_details', { name, source })
      return feat
    } catch (e) {
      return null
    }
  }

  // Race catalog functions
  const isRacesInitialized = ref(false)
  const races = ref<RaceSummary[]>([])

  async function initializeRaceCatalog() {
    if (isRacesInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('initialize_race_catalog')
      isRacesInitialized.value = true
    } catch (e) {
      error.value = `Failed to initialize race catalog: ${e}`
    } finally {
      isLoading.value = false
    }
  }

  async function searchRaces(filters: RaceFilters = {}): Promise<RaceSummary[]> {
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
        hasDarkvision: filters.has_darkvision !== undefined ? filters.has_darkvision : null,
        hasFlight: filters.has_flight !== undefined ? filters.has_flight : null,
      })
      
      races.value = results
      return results
    } catch (e) {
      error.value = `Search failed: ${e}`
      return []
    } finally {
      isLoading.value = false
    }
  }

  async function getRaceDetails(name: string, source: string): Promise<RaceWithDetails | null> {
    try {
      const details = await invoke<RaceWithDetails>('get_race_details', { name, source })
      return details
    } catch (e) {
      return null
    }
  }

  return {
    // State
    isInitialized,
    isItemsInitialized,
    isMonstersInitialized,
    isClassesInitialized,
    isRacesInitialized,
    isLoading,
    error,
    spells,
    items,
    monsters,
    classes,
    races,
    classSources,
    initializeCatalog,
    initializeItemCatalog,
    initializeMonsterCatalog,
    initializeClassCatalog,
    searchSpells,
    searchItems,
    searchMonsters,
    searchClasses,
    getSpellDetails,
    getItemDetails,
    getMonsterDetails,
    getClassDetails,
    getClassSubclasses,
    initializeFeatCatalog,
    searchFeats,
    getFeatDetails,
    initializeRaceCatalog,
    searchRaces,
    getRaceDetails,
  }
}