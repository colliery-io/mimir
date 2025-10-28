import { invoke } from '@tauri-apps/api/core'

export interface PsionicSummary {
  name: string
  source: string
  psionic_type: string
  order?: string
  page?: number
}

export interface Psionic {
  name: string
  source: string
  psionic_type: string
  order?: string
  page?: number
  entries?: any[]
  focus?: string
  modes?: PsionicMode[]
}

export interface PsionicMode {
  name: string
  cost: {
    min: number
    max?: number
  }
  entries: any[]
  concentration?: {
    duration: number
    unit: string
  }
}

export function usePsionics() {
  async function searchPsionics(filters: {
    query?: string
    psionic_types?: string[]
    orders?: string[]
    sources?: string[]
  }): Promise<PsionicSummary[]> {
    try {
      const results = await invoke<PsionicSummary[]>('search_psionics', {
        query: filters.query || null,
        psionic_types: filters.psionic_types || null,
        orders: filters.orders || null,
        sources: filters.sources || null
      })
      return results || []
    } catch (e) {
      console.error(`Failed to search psionics: ${e}`)
      return []
    }
  }

  async function getPsionicDetails(name: string, source: string): Promise<Psionic | null> {
    try {
      const details = await invoke<Psionic>('get_psionic_details', { name, source })
      return details
    } catch (e) {
      console.error(`Failed to get psionic details: ${e}`)
      return null
    }
  }

  async function getPsionicOrders(): Promise<string[]> {
    try {
      const orders = await invoke<string[]>('get_psionic_orders')
      return orders || []
    } catch (e) {
      console.error(`Failed to get psionic orders: ${e}`)
      return []
    }
  }

  async function getPsionicSources(): Promise<string[]> {
    try {
      const sources = await invoke<string[]>('get_psionic_sources')
      return sources || []
    } catch (e) {
      console.error(`Failed to get psionic sources: ${e}`)
      return []
    }
  }

  return {
    searchPsionics,
    getPsionicDetails,
    getPsionicOrders,
    getPsionicSources,
  }
}
