import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

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

export function useActions() {
  const isActionsInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)
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

  return {
    isActionsInitialized,
    isLoading,
    error,
    actions,
    initializeActionCatalog,
    searchActions,
    getActionDetails,
  }
}
