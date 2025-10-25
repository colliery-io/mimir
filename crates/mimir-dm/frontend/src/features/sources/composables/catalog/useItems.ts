import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ItemSummary {
  name: string
  itemType: string
  typeName: string
  source: string
  rarity: string
  value?: number
  weight?: number
  ac?: number
  damage?: string
  reqAttune?: string
  description: string
}

export interface ItemFilters {
  query?: string
  sources?: string[]
  types?: string[]
  rarities?: string[]
  min_value?: number
  max_value?: number
}

export interface Item {
  name: string
  source: string
  type: string
  rarity: string
  weight?: number
  value?: number
  entries?: string[]
}

export function useItems() {
  const isItemsInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)
  const items = ref<ItemSummary[]>([])

  async function initializeItemCatalog() {
    if (isItemsInitialized.value) return

    try {
      isLoading.value = true
      error.value = null
      await invoke('initialize_item_catalog')
      isItemsInitialized.value = true
    } catch (e) {
      error.value = `Failed to initialize item catalog: ${e}`
    } finally {
      isLoading.value = false
    }
  }

  async function searchItems(filters: ItemFilters): Promise<ItemSummary[]> {
    if (!isItemsInitialized.value) {
      await initializeItemCatalog()
    }

    try {
      isLoading.value = true
      error.value = null

      const results = await invoke<ItemSummary[]>('search_items', {
        query: filters.query || null,
        sources: filters.sources && filters.sources.length > 0 ? filters.sources : null,
        types: filters.types && filters.types.length > 0 ? filters.types : null,
        rarities: filters.rarities && filters.rarities.length > 0 ? filters.rarities : null,
        minValue: filters.min_value !== undefined ? filters.min_value : null,
        maxValue: filters.max_value !== undefined ? filters.max_value : null,
      })

      items.value = results
      return results
    } catch (e) {
      error.value = `Search failed: ${e}`
      return []
    } finally {
      isLoading.value = false
    }
  }

  async function getItemDetails(name: string, source: string): Promise<Item | null> {
    try {
      const item = await invoke<Item>('get_item_details', { name, source })
      return item
    } catch (e) {
      return null
    }
  }

  return {
    isItemsInitialized,
    isLoading,
    error,
    items,
    initializeItemCatalog,
    searchItems,
    getItemDetails,
  }
}
