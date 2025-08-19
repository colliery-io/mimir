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

export function useCatalog() {
  const isInitialized = ref(false)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const spells = ref<SpellSummary[]>([])

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