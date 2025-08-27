import { useCatalog } from '../composables/useCatalog'
import type { 
  SpellSummary, 
  ItemSummary, 
  MonsterSummary,
  ClassSummary,
  FeatSummary,
  RaceSummary,
  BackgroundSummary,
  ActionSummary
} from '../composables/useCatalog'

export type { BackgroundSummary, ActionSummary }

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
  type: 'spell' | 'item' | 'monster' | 'class' | 'feat' | 'race' | 'background' | 'action'
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
      case 'Races':
        console.log('Initializing race catalog...')
        await this.catalog.initializeRaceCatalog()
        console.log('Race catalog initialized')
        break
      case 'Backgrounds':
        await this.catalog.initializeBackgroundCatalog()
        break
      case 'Actions':
        await this.catalog.initializeActionCatalog()
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
      case 'Races':
        return await this.searchRaces(query, sources)
      case 'Backgrounds':
        return await this.searchBackgrounds(query, sources)
      case 'Actions':
        return await this.searchActions(query, sources)
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
  
  private async searchRaces(query?: string, sources?: string[]): Promise<RaceSummary[]> {
    console.log('SearchService.searchRaces called with:', { query, sources })
    const results = await this.catalog.searchRaces({
      query: query || undefined,
      sources: sources || undefined
    })
    console.log('SearchService.searchRaces results:', results)
    return results
  }
  
  private async searchBackgrounds(query?: string, sources?: string[]): Promise<BackgroundSummary[]> {
    const results = await this.catalog.searchBackgrounds({
      query: query || undefined,
      sources: sources || undefined
    })
    return results
  }
  
  private async searchActions(query?: string, sources?: string[]): Promise<ActionSummary[]> {
    const results = await this.catalog.searchActions({
      query: query || undefined,
      sources: sources || undefined
    })
    return results
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
      case 'race':
        return await this.catalog.getRaceDetails(name, source)
      case 'background':
        return await this.catalog.getBackgroundDetails(name, source)
      case 'action':
        return await this.catalog.getActionDetails(name, source)
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