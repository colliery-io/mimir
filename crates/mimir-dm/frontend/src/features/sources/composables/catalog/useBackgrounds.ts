import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

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

export function useBackgrounds() {
  const isBackgroundsInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)
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

  return {
    isBackgroundsInitialized,
    isLoading,
    error,
    backgrounds,
    initializeBackgroundCatalog,
    searchBackgrounds,
    getBackgroundDetails,
  }
}
