import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ConditionSummary {
  name: string
  source: string
  item_type: 'Condition' | 'Disease'
  description: string
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

export function useConditions() {
  const isConditionsInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)
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

  return {
    isConditionsInitialized,
    isLoading,
    error,
    conditions,
    initializeConditionCatalog,
    searchConditions,
    getConditionDetails,
  }
}
