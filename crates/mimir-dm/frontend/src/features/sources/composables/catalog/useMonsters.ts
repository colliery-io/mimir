import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

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
  creature_type?: string
  environment?: string[]
}

export interface MonsterFilters {
  query?: string
  sources?: string[]
  types?: string[]
  sizes?: string[]
  min_cr?: number
  max_cr?: number
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
  fluffEntries?: any[]
  fluffImages?: any[]
  fluff_images?: any[]
}

export function useMonsters() {
  const isMonstersInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)
  const monsters = ref<MonsterSummary[]>([])

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

  async function getMonsterDetails(name: string, source: string): Promise<Monster | null> {
    try {
      const monster = await invoke<Monster>('get_monster_details', { name, source })
      return monster
    } catch (e) {
      return null
    }
  }

  return {
    isMonstersInitialized,
    isLoading,
    error,
    monsters,
    initializeMonsterCatalog,
    searchMonsters,
    getMonsterDetails,
  }
}
