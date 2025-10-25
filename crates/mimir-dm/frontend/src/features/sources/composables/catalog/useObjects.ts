import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ObjectSummary {
  name: string
  source: string
  object_type: string
  size: string
  ac: string
  hp: string
}

export interface DndObject {
  name: string
  source: string
  page?: number
  objectType?: string
  size?: string[]
  ac?: any
  hp?: number
  immune?: string[]
  resist?: string[]
  vulnerable?: string[]
  actionEntries?: any[]
  entries?: any[]
  hasToken?: boolean
  tokenCredit?: string
  srd?: boolean
  hasFluff?: boolean
  hasFluffImages?: boolean
}

export function useObjects() {
  const isObjectsInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)

  async function initializeObjectCatalog() {
    if (isObjectsInitialized.value) return

    try {
      isLoading.value = true
      error.value = null
      await invoke('init_object_catalog')
      isObjectsInitialized.value = true
    } catch (e) {
      error.value = `Failed to initialize object catalog: ${e}`
    } finally {
      isLoading.value = false
    }
  }

  async function searchObjects(filters: { query?: string, sources?: string[], object_types?: string[], sizes?: string[] }) {
    try {
      isLoading.value = true
      error.value = null

      const results = await invoke<ObjectSummary[]>('search_objects', {
        search: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
        object_types: filters.object_types && filters.object_types.length > 0 ? filters.object_types : null,
        sizes: filters.sizes && filters.sizes.length > 0 ? filters.sizes : null
      })

      return results || []
    } catch (e) {
      error.value = `Failed to search objects: ${e}`
      return []
    } finally {
      isLoading.value = false
    }
  }

  async function getObjectDetails(name: string, source: string): Promise<DndObject | null> {
    try {
      const jsonString = await invoke<string | null>('get_object_details', { name, source })
      if (!jsonString) {
        return null
      }

      const objectData = JSON.parse(jsonString)
      return objectData as DndObject
    } catch (e) {
      console.error('Failed to get object details:', e)
      return null
    }
  }

  return {
    isObjectsInitialized,
    isLoading,
    error,
    initializeObjectCatalog,
    searchObjects,
    getObjectDetails,
  }
}
