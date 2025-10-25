import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

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

export function useRaces() {
  const isRacesInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)
  const races = ref<RaceSummary[]>([])

  async function initializeRaceCatalog() {
    if (isRacesInitialized.value) return

    try {
      isLoading.value = true
      error.value = null
      await invoke('init_race_catalog')
      isRacesInitialized.value = true
    } catch (e) {
      error.value = `Failed to initialize race catalog: ${e}`
    } finally {
      isLoading.value = false
    }
  }

  async function searchRaces(filters: RaceFilters = {}): Promise<RaceSummary[]> {
    try {
      isLoading.value = true
      error.value = null

      const results = await invoke<RaceSummary[]>('search_races', {
        search: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
        sizes: filters.sizes && filters.sizes.length > 0 ? filters.sizes : null,
        has_darkvision: filters.has_darkvision !== undefined ? filters.has_darkvision : null,
        has_flight: filters.has_flight !== undefined ? filters.has_flight : null,
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
      const jsonString = await invoke<string | null>('get_race_details', { name, source })
      if (!jsonString) {
        return null
      }

      const raceData = JSON.parse(jsonString)

      return {
        race: raceData.name ? raceData : null,
        subrace: raceData.race_name ? raceData : null,
        relatedSubraces: [],
        fluff: null
      } as RaceWithDetails
    } catch (e) {
      console.error('Failed to get race details:', e)
      return null
    }
  }

  return {
    isRacesInitialized,
    isLoading,
    error,
    races,
    initializeRaceCatalog,
    searchRaces,
    getRaceDetails,
  }
}
