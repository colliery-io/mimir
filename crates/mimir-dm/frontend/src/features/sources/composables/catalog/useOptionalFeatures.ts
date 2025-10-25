import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface OptionalFeatureSummary {
  name: string
  source: string
  feature_types: string[]
  feature_type_full: string
  prerequisite_text: string
  grants_spells: boolean
}

export interface OptionalFeatureFilters {
  query?: string
  sources?: string[]
  feature_types?: string[]
}

export function useOptionalFeatures() {
  const isOptionalFeaturesInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)
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
    isOptionalFeaturesInitialized,
    isLoading,
    error,
    optionalFeatures,
    initializeOptionalFeatureCatalog,
    searchOptionalFeatures,
    getOptionalFeatureDetails,
  }
}
