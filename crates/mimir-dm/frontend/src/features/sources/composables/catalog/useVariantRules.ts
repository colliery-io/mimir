import { invoke } from '@tauri-apps/api/core'

export interface VariantRule {
  name: string
  source: string
  rule_type?: string
  page?: number
  entries?: any[]
}

export interface VariantRuleSummary {
  name: string
  source: string
  rule_type?: string
  page?: number
}

export function useVariantRules() {
  async function searchVariantRules(filters: {
    query?: string
    types?: string[]
    sources?: string[]
  }): Promise<VariantRuleSummary[]> {
    try {
      const results = await invoke<VariantRuleSummary[]>('search_variant_rules', {
        query: filters.query || null,
        rule_types: filters.types || null,
        sources: filters.sources || null
      })
      return results || []
    } catch (e) {
      console.error(`Failed to search variant rules: ${e}`)
      return []
    }
  }

  async function getVariantRuleDetails(name: string, source: string): Promise<VariantRule | null> {
    try {
      const details = await invoke<VariantRule>('get_variant_rule_details', { name, source })
      return details
    } catch (e) {
      console.error(`Failed to get variant rule details: ${e}`)
      return null
    }
  }

  async function getVariantRuleTypes(): Promise<string[]> {
    try {
      const types = await invoke<string[]>('get_variant_rule_types')
      return types || []
    } catch (e) {
      console.error(`Failed to get variant rule types: ${e}`)
      return []
    }
  }

  async function getVariantRuleSources(): Promise<string[]> {
    try {
      const sources = await invoke<string[]>('get_variant_rule_sources')
      return sources || []
    } catch (e) {
      console.error(`Failed to get variant rule sources: ${e}`)
      return []
    }
  }

  return {
    searchVariantRules,
    getVariantRuleDetails,
    getVariantRuleTypes,
    getVariantRuleSources,
  }
}
