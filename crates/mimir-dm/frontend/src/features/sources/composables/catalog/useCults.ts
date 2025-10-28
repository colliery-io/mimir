import { invoke } from '@tauri-apps/api/core'

export interface CultBoonSummary {
  name: string
  source: string
  item_type: string
  subtype?: string
  page?: number
}

export interface Cult {
  name: string
  source: string
  cult_type?: string
  page?: number
  entries?: any[]
  cultists?: { entry: string }
  goal?: { entry: string }
  signature_spells?: { entry: string }
}

export interface Boon {
  name: string
  source: string
  boon_type?: string
  page?: number
  entries?: any[]
  ability?: { entry: string }
  signature_spells?: { entry: string }
}

export function useCults() {
  async function initializeCultCatalog() {
    // Database-backed system - no initialization required
  }

  async function searchCults(filters: {
    query?: string
    item_types?: string[]
    subtypes?: string[]
    sources?: string[]
  }): Promise<CultBoonSummary[]> {
    try {
      const results = await invoke<CultBoonSummary[]>('search_cults', {
        query: filters.query || null,
        item_types: filters.item_types || null,
        subtypes: filters.subtypes || null,
        sources: filters.sources || null
      })
      return results || []
    } catch (e) {
      console.error(`Failed to search cults: ${e}`)
      return []
    }
  }

  async function getCultDetails(name: string, source: string): Promise<Cult | null> {
    try {
      const details = await invoke<Cult>('get_cult_details', { name, source })
      return details
    } catch (e) {
      console.error(`Failed to get cult details: ${e}`)
      return null
    }
  }

  async function getBoonDetails(name: string, source: string): Promise<Boon | null> {
    try {
      const details = await invoke<Boon>('get_boon_details', { name, source })
      return details
    } catch (e) {
      console.error(`Failed to get boon details: ${e}`)
      return null
    }
  }

  async function getCultTypes(): Promise<string[]> {
    try {
      const types = await invoke<string[]>('get_cult_types')
      return types || []
    } catch (e) {
      console.error(`Failed to get cult types: ${e}`)
      return []
    }
  }

  async function getCultSources(): Promise<string[]> {
    try {
      const sources = await invoke<string[]>('get_cult_sources')
      return sources || []
    } catch (e) {
      console.error(`Failed to get cult sources: ${e}`)
      return []
    }
  }

  return {
    initializeCultCatalog,
    searchCults,
    getCultDetails,
    getBoonDetails,
    getCultTypes,
    getCultSources,
  }
}
