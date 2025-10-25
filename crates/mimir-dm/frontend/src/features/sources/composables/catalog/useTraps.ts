import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface TrapSummary {
  name: string
  source: string
  trap_type: string
  category: string
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

export function useTraps() {
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)

  async function initializeTrapCatalog() {
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
  }

  async function searchTraps(filters: {
    query?: string
    sources?: string[]
    categories?: string[]
    trap_types?: string[]
  }) {
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
  }

  async function getTrapDetails(name: string, source: string): Promise<TrapOrHazard | null> {
    try {
      const details = await invoke<TrapOrHazard>('get_trap_details', { name, source })
      return details
    } catch (e) {
      console.error(`Failed to get trap details: ${e}`)
      return null
    }
  }

  async function getTrapTypes(): Promise<string[]> {
    try {
      const types = await invoke<string[]>('get_trap_types')
      return types || []
    } catch (e) {
      console.error(`Failed to get trap types: ${e}`)
      return []
    }
  }

  return {
    isLoading,
    error,
    initializeTrapCatalog,
    searchTraps,
    getTrapDetails,
    getTrapTypes,
  }
}
