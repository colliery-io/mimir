import { useCatalog } from '../composables/useCatalog'
import type { 
  SpellSummary, 
  ItemSummary, 
  MonsterSummary,
  ClassSummary,
  FeatSummary
} from '../composables/useCatalog'

export interface SearchFilters {
  spells: {
    school: string
    level: string
    ritual: boolean
    concentration: boolean
  }
  equipment: {
    type: string
    rarity: string
  }
  monsters: {
    sizes: string[]
    types: string[]
    minCr?: number
    maxCr?: number
  }
  magicItems: {
    rarity: string
  }
}

export interface SearchParams {
  query: string
  sources: string[]
  category: string
  filters: Partial<SearchFilters>
}

export interface DetailFetchParams {
  name: string
  source: string
  type: 'spell' | 'item' | 'monster' | 'class' | 'feat'
}

class SearchServiceClass {
  private catalog = useCatalog()
  
  async initialize(category: string): Promise<void> {
    switch (category) {
      case 'Spells':
        await this.catalog.initializeCatalog()
        break
      case 'Equipment':
      case 'Magic Items':
        await this.catalog.initializeItemCatalog()
        break
      case 'Monsters':
        await this.catalog.initializeMonsterCatalog()
        break
      case 'Classes':
        await this.catalog.initializeClassCatalog()
        break
      case 'Feats':
        await this.catalog.initializeFeatCatalog()
        break
    }
  }
  
  async search(params: Partial<SearchParams>): Promise<any[]> {
    const { query, sources, category, filters = {} } = params
    
    switch (category) {
      case 'Spells':
        return await this.searchSpells(query, sources, filters.spells)
      case 'Equipment':
        return await this.searchEquipment(query, sources, filters.equipment)
      case 'Magic Items':
        return await this.searchMagicItems(query, sources, filters.magicItems)
      case 'Monsters':
        return await this.searchMonsters(query, sources, filters.monsters)
      case 'Classes':
        return await this.searchClasses(query)
      case 'Feats':
        return await this.searchFeats(query)
      default:
        return []
    }
  }
  
  private async searchSpells(
    query?: string, 
    sources?: string[], 
    filters?: SearchFilters['spells']
  ): Promise<SpellSummary[]> {
    return await this.catalog.searchSpells({
      query: query || undefined,
      sources,
      schools: filters?.school ? [filters.school] : undefined,
      levels: filters?.level ? [parseInt(filters.level)] : undefined,
      ritual: filters?.ritual || undefined,
      concentration: filters?.concentration || undefined
    })
  }
  
  private async searchEquipment(
    query?: string,
    sources?: string[],
    filters?: SearchFilters['equipment']
  ): Promise<ItemSummary[]> {
    return await this.catalog.searchItems({
      query: query || undefined,
      sources,
      types: filters?.type ? [filters.type] : undefined
    })
  }
  
  private async searchMagicItems(
    query?: string,
    sources?: string[],
    filters?: SearchFilters['magicItems']
  ): Promise<ItemSummary[]> {
    const allItems = await this.catalog.searchItems({
      query: query || undefined,
      sources
    })
    
    return allItems.filter(item => 
      item.rarity && item.rarity !== 'none' &&
      (!filters?.rarity || item.rarity === filters.rarity)
    )
  }
  
  private async searchMonsters(
    query?: string,
    sources?: string[],
    filters?: SearchFilters['monsters']
  ): Promise<MonsterSummary[]> {
    return await this.catalog.searchMonsters({
      query: query || undefined,
      sources,
      sizes: filters?.sizes?.length ? filters.sizes : undefined,
      types: filters?.types?.length ? filters.types : undefined,
      min_cr: filters?.minCr,
      max_cr: filters?.maxCr
    })
  }
  
  private async searchClasses(query?: string): Promise<ClassSummary[]> {
    return await this.catalog.searchClasses({
      query: query || undefined
    })
  }
  
  private async searchFeats(query?: string): Promise<FeatSummary[]> {
    return await this.catalog.searchFeats({
      query: query || undefined
    })
  }
  
  async getDetails(params: DetailFetchParams): Promise<any> {
    const { name, source, type } = params
    
    switch (type) {
      case 'spell':
        return await this.catalog.getSpellDetails(name, source)
      case 'item':
        return await this.catalog.getItemDetails(name, source)
      case 'monster':
        return await this.catalog.getMonsterDetails(name, source)
      case 'class':
        return await this.catalog.getClassDetails(name, source)
      case 'feat':
        return await this.catalog.getFeatDetails(name, source)
      default:
        return null
    }
  }
  
  mapBookIdsToSources(bookIds: string[]): string[] {
    return bookIds.map(id => {
      const parts = id.split('-')
      return parts[parts.length - 1].toUpperCase()
    })
  }
  
  getClassSources(): string[] {
    return this.catalog.classSources.value
  }
}

export const SearchService = new SearchServiceClass()