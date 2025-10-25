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

export function useSpells() {
  const isInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)
  const spells = ref<SpellSummary[]>([])

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

  async function getSpellDetails(name: string, source: string): Promise<Spell | null> {
    try {
      const spell = await invoke<Spell>('get_spell_details', { name, source })
      return spell
    } catch (e) {
      return null
    }
  }

  return {
    isInitialized,
    isLoading,
    error,
    spells,
    initializeCatalog,
    searchSpells,
    getSpellDetails,
  }
}
