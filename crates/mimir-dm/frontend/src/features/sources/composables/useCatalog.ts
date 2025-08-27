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

// Background interfaces
export interface BackgroundSummary {
  name: string
  source: string
  skills: string
  languages: string
  tools: string
  feature: string
}

export interface BackgroundFilters {
  query?: string
  sources?: string[]
  has_tools?: boolean
}

// Action interfaces
export interface ActionSummary {
  name: string
  source: string
  time: string
  description: string
  see_also: string[]
}

export interface ActionFilters {
  query?: string
  sources?: string[]
  time_filter?: string
}

// Condition interfaces
export interface ConditionSummary {
  name: string
  source: string
  item_type: 'Condition' | 'Disease'
  description: string
  is_srd: boolean
}

export interface ConditionWithDetails {
  item: {
    type: 'Condition' | 'Disease'
    Condition?: any
    Disease?: any
  }
  fluff?: any
}

export interface ConditionFilters {
  query?: string
  sources?: string[]
  type_filter?: string
}

// Optional Feature interfaces  
export interface OptionalFeatureSummary {
  name: string
  source: string
  feature_types: string[]
  feature_type_full: string
  prerequisite_text: string
  is_srd: boolean
  grants_spells: boolean
}

export interface DeitySummary {
  name: string
  source: string
  title: string
  pantheon: string
  alignment: string
  domains: string[]
  symbol: string
  is_srd: boolean
}

export interface Deity {
  name: string
  source: string
  page?: number
  title?: string
  pantheon?: string
  alignment?: string[]
  domains?: string[]
  symbol?: string
  additionalSources?: any[]
  entries?: any[]
  srd?: boolean
  hasFluff?: boolean
  hasFluffImages?: boolean
}

export interface ObjectSummary {
  name: string
  source: string
  object_type: string
  size: string
  ac: string
  hp: string
  is_srd: boolean
}

export interface TrapSummary {
  name: string
  source: string
  trap_type: string
  category: string
  is_srd: boolean
}

export interface TrapOrHazard {
  name: string
  source: string
  page?: number
  trap_haz_type?: string
  entries?: any[]
  srd?: boolean
  has_fluff?: boolean
  has_fluff_images?: boolean
}

export interface DndObject {
  name: string
  source: string
  page?: number
  objectType?: string
  size?: string[]
  ac?: any
  hp?: number
  immune?: string[]
  resist?: string[]
  vulnerable?: string[]
  actionEntries?: any[]
  entries?: any[]
  hasToken?: boolean
  tokenCredit?: string
  srd?: boolean
  hasFluff?: boolean
  hasFluffImages?: boolean
}

export interface OptionalFeatureFilters {
  query?: string
  sources?: string[]
  feature_types?: string[]
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

  // Background catalog functions
  const isBackgroundsInitialized = ref(false)
  const backgrounds = ref<BackgroundSummary[]>([])

  async function initializeBackgroundCatalog() {
    if (isBackgroundsInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('init_background_catalog')
      isBackgroundsInitialized.value = true
    } catch (e) {
      error.value = `Failed to initialize background catalog: ${e}`
    } finally {
      isLoading.value = false
    }
  }

  async function searchBackgrounds(filters: BackgroundFilters = {}): Promise<BackgroundSummary[]> {
    if (!isBackgroundsInitialized.value) {
      await initializeBackgroundCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<BackgroundSummary[]>('search_backgrounds', {
        query: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
        hasTools: filters.has_tools !== undefined ? filters.has_tools : null
      })
      
      backgrounds.value = results
      return results
    } catch (e) {
      error.value = `Failed to search backgrounds: ${e}`
      return []
    } finally {
      isLoading.value = false
    }
  }

  async function getBackgroundDetails(name: string, source: string): Promise<any | null> {
    try {
      const details = await invoke('get_background_details', { name, source })
      return details
    } catch (e) {
      return null
    }
  }

  // Action catalog functions
  const isActionsInitialized = ref(false)
  const actions = ref<ActionSummary[]>([])

  async function initializeActionCatalog() {
    if (isActionsInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('init_action_catalog')
      isActionsInitialized.value = true
    } catch (e) {
      error.value = `Failed to initialize action catalog: ${e}`
    } finally {
      isLoading.value = false
    }
  }

  async function searchActions(filters: ActionFilters = {}): Promise<ActionSummary[]> {
    if (!isActionsInitialized.value) {
      await initializeActionCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<ActionSummary[]>('search_actions', {
        query: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
        timeFilter: filters.time_filter || null
      })
      
      actions.value = results
      return results
    } catch (e) {
      error.value = `Failed to search actions: ${e}`
      return []
    } finally {
      isLoading.value = false
    }
  }

  async function getActionDetails(name: string, source: string): Promise<any | null> {
    try {
      const details = await invoke('get_action_details', { name, source })
      return details
    } catch (e) {
      return null
    }
  }

  // Condition catalog functions
  const isConditionsInitialized = ref(false)
  const conditions = ref<ConditionSummary[]>([])

  async function initializeConditionCatalog() {
    if (isConditionsInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('init_condition_catalog')
      isConditionsInitialized.value = true
    } catch (e) {
      error.value = `Failed to initialize condition catalog: ${e}`
    } finally {
      isLoading.value = false
    }
  }

  async function searchConditions(filters: ConditionFilters = {}): Promise<ConditionSummary[]> {
    if (!isConditionsInitialized.value) {
      await initializeConditionCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      const results = await invoke<ConditionSummary[]>('search_conditions', {
        query: filters.query,
        sources: filters.sources,
        typeFilter: filters.type_filter
      })
      conditions.value = results
      return results
    } catch (e) {
      error.value = `Failed to search conditions: ${e}`
      return []
    } finally {
      isLoading.value = false
    }
  }

  async function getConditionDetails(name: string, source: string): Promise<ConditionWithDetails | null> {
    try {
      const details = await invoke<ConditionWithDetails>('get_condition_details', { name, source })
      return details
    } catch (e) {
      return null
    }
  }

  // Optional Feature catalog functions
  const isOptionalFeaturesInitialized = ref(false)
  const isDeitiesInitialized = ref(false)
  const optionalFeatures = ref<OptionalFeatureSummary[]>([])

  async function initializeOptionalFeatureCatalog() {
    if (isOptionalFeaturesInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('init_optional_feature_catalog')
      isOptionalFeaturesInitialized.value = true
    } catch (e) {
      error.value = `Failed to initialize optional feature catalog: ${e}`
    } finally {
      isLoading.value = false
    }
  }

  // Deity catalog methods
  async function initializeDeityCatalog() {
    if (isDeitiesInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('init_deity_catalog')
      isDeitiesInitialized.value = true
    } catch (e) {
      error.value = `Failed to initialize deity catalog: ${e}`
    } finally {
      isLoading.value = false
    }
  }

  async function searchDeities(filters: { query?: string, sources?: string[], pantheons?: string[], domains?: string[] }) {
    if (!isDeitiesInitialized.value) {
      await initializeDeityCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<DeitySummary[]>('search_deities', {
        query: filters.query || null,
        sources: filters.sources || null,
        pantheons: filters.pantheons || null,
        domains: filters.domains || null
      })
      
      return results || []
    } catch (e) {
      error.value = `Failed to search deities: ${e}`
      return []
    } finally {
      isLoading.value = false
    }
  }

  async function getDeityDetails(name: string, source: string): Promise<Deity | null> {
    try {
      const deity = await invoke<Deity>('get_deity_details', { name, source })
      return deity
    } catch (e) {
      console.error('Failed to get deity details:', e)
      return null
    }
  }

  // Object catalog methods
  const isObjectsInitialized = ref(false)
  
  async function initializeObjectCatalog() {
    if (isObjectsInitialized.value) return
    
    try {
      isLoading.value = true
      error.value = null
      await invoke('init_object_catalog')
      isObjectsInitialized.value = true
    } catch (e) {
      error.value = `Failed to initialize object catalog: ${e}`
    } finally {
      isLoading.value = false
    }
  }

  async function searchObjects(filters: { query?: string, sources?: string[], object_types?: string[], sizes?: string[] }) {
    if (!isObjectsInitialized.value) {
      await initializeObjectCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      
      const results = await invoke<ObjectSummary[]>('search_objects', {
        query: filters.query || null,
        sources: filters.sources || null,
        object_types: filters.object_types || null,
        sizes: filters.sizes || null
      })
      
      return results || []
    } catch (e) {
      error.value = `Failed to search objects: ${e}`
      return []
    } finally {
      isLoading.value = false
    }
  }

  async function getObjectDetails(name: string, source: string): Promise<DndObject | null> {
    try {
      const obj = await invoke<DndObject>('get_object_details', { name, source })
      return obj
    } catch (e) {
      console.error('Failed to get object details:', e)
      return null
    }
  }

  async function searchOptionalFeatures(filters: OptionalFeatureFilters = {}): Promise<OptionalFeatureSummary[]> {
    if (!isOptionalFeaturesInitialized.value) {
      await initializeOptionalFeatureCatalog()
    }
    
    try {
      isLoading.value = true
      error.value = null
      const results = await invoke<OptionalFeatureSummary[]>('search_optional_features', {
        query: filters.query,
        sources: filters.sources,
        featureTypes: filters.feature_types
      })
      optionalFeatures.value = results
      return results
    } catch (e) {
      error.value = `Failed to search optional features: ${e}`
      return []
    } finally {
      isLoading.value = false
    }
  }

  async function getOptionalFeatureDetails(name: string, source: string): Promise<any | null> {
    try {
      const details = await invoke('get_optional_feature_details', { name, source })
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
    isBackgroundsInitialized,
    isActionsInitialized,
    isConditionsInitialized,
    isOptionalFeaturesInitialized,
    isLoading,
    error,
    spells,
    items,
    monsters,
    classes,
    races,
    backgrounds,
    actions,
    conditions,
    optionalFeatures,
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
    initializeBackgroundCatalog,
    searchBackgrounds,
    getBackgroundDetails,
    initializeActionCatalog,
    searchActions,
    getActionDetails,
    initializeConditionCatalog,
    searchConditions,
    getConditionDetails,
    initializeOptionalFeatureCatalog,
    searchOptionalFeatures,
    getOptionalFeatureDetails,
    // Deity catalog methods
    initializeDeityCatalog,
    searchDeities,
    getDeityDetails,
    // Object catalog methods
    initializeObjectCatalog,
    searchObjects,
    getObjectDetails,
    // Trap catalog methods
    initializeTrapCatalog: async () => {
      const isTrapsInitialized = ref(false)
      if (isTrapsInitialized.value) return
      
      try {
        isLoading.value = true
        error.value = null
        await invoke('init_trap_catalog')
        isTrapsInitialized.value = true
      } catch (e) {
        error.value = `Failed to initialize trap catalog: ${e}`
      } finally {
        isLoading.value = false
      }
    },
    searchTraps: async (filters: { 
      query?: string, 
      sources?: string[], 
      categories?: string[],
      trap_types?: string[] 
    }) => {
      try {
        isLoading.value = true
        error.value = null
        
        const results = await invoke<TrapSummary[]>('search_traps', {
          query: filters.query || null,
          sources: filters.sources || null,
          categories: filters.categories || null,
          trap_types: filters.trap_types || null
        })
        
        return results || []
      } catch (e) {
        error.value = `Failed to search traps: ${e}`
        return []
      } finally {
        isLoading.value = false
      }
    },
    getTrapDetails: async (name: string, source: string): Promise<TrapOrHazard | null> => {
      try {
        const details = await invoke<TrapOrHazard>('get_trap_details', { name, source })
        return details
      } catch (e) {
        console.error(`Failed to get trap details: ${e}`)
        return null
      }
    },
    getTrapTypes: async (): Promise<string[]> => {
      try {
        const types = await invoke<string[]>('get_trap_types')
        return types || []
      } catch (e) {
        console.error(`Failed to get trap types: ${e}`)
        return []
      }
    }
  }
}