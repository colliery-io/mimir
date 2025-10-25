import { invoke } from '@tauri-apps/api/core'

export interface FeatSummary {
  name: string
  source: string
  page?: number
  prerequisites?: string
  brief?: string
}

export interface Feat {
  name: string
  source: string
  page?: number
  srd?: boolean
  entries: any[]
  prerequisite?: any[]
  ability?: any[]
  skill_proficiencies?: any[]
  language_proficiencies?: any[]
  tool_proficiencies?: any[]
  weapon_proficiencies?: any[]
  armor_proficiencies?: any[]
  saving_throw_proficiencies?: any[]
  expertise?: any[]
  resist?: any[]
  immune?: any[]
  senses?: any[]
  additional_spells?: any[]
  other_sources?: any[]
}

export function useFeats() {
  async function searchFeats(params: {
    query?: string
    sources?: string[]
    has_prerequisites?: boolean
  } = {}) {
    try {
      const results = await invoke<FeatSummary[]>('search_feats', {
        query: params.query,
        sources: params.sources,
        has_prerequisites: params.has_prerequisites
      })
      return results || []
    } catch (e) {
      console.error('Error searching feats:', e)
      return []
    }
  }

  async function getFeatDetails(name: string, source: string) {
    try {
      const feat = await invoke<Feat>('get_feat_details', { name, source })
      return feat
    } catch (e) {
      return null
    }
  }

  return {
    searchFeats,
    getFeatDetails,
  }
}
