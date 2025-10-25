import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface DeitySummary {
  name: string
  source: string
  title: string
  pantheon: string
  alignment: string
  domains: string[]
  symbol: string
}

export interface Deity {
  name: string
  source: string
  page?: number
  title?: string
  pantheon?: string
  alignment?: string[] | string
  domains?: string[]
  symbol?: string
  additionalSources?: any[]
  entries?: any[]
  srd?: boolean
  hasFluff?: boolean
  hasFluffImages?: boolean
}

export function useDeities() {
  const isDeitiesInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)

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

      const results = await invoke<DeitySummary[]>('search_deities_db', {
        filters: {
          name: filters.query || null,
          sources: filters.sources || null,
          pantheons: filters.pantheons || null,
          domains: filters.domains || null,
          alignments: null
        }
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
      const deity = await invoke<Deity>('get_deity_details_db', { deityName: name, deitySource: source })
      return deity
    } catch (e) {
      console.error('Failed to get deity details:', e)
      return null
    }
  }

  return {
    isDeitiesInitialized,
    isLoading,
    error,
    initializeDeityCatalog,
    searchDeities,
    getDeityDetails,
  }
}
