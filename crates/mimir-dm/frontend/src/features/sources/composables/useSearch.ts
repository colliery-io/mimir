import { ref, computed, watch } from 'vue'
import { SearchService, type SearchFilters } from '../services/SearchService'
import type { 
  SpellSummary, 
  ItemSummary, 
  MonsterSummary,
  ClassSummary,
  FeatSummary
} from './useCatalog'
import { formatSpellDetails } from '../formatters/spellFormatterEnhanced'
import { formatItemDetails } from '../formatters/itemFormatterEnhanced'
import { formatMonsterDetails } from '../formatters/monsterFormatterEnhanced'
import { formatClassDetails } from '../formatters/classFormatterEnhanced'
import { formatFeatDetails } from '../formatters/featFormatter'

export function useSearch(initialCategory: string, selectedSources: string[]) {
  const selectedCategory = ref(initialCategory)
  const searchQuery = ref('')
  const searchPerformed = ref(false)
  const sortColumn = ref('name')
  const sortDirection = ref<'asc' | 'desc'>('asc')
  
  const results = ref<any[]>([])
  const filters = ref<SearchFilters>({
    spells: {
      school: '',
      level: '',
      ritual: false,
      concentration: false
    },
    equipment: {
      type: '',
      rarity: ''
    },
    monsters: {
      sizes: [],
      types: [],
      minCr: undefined,
      maxCr: undefined
    },
    magicItems: {
      rarity: ''
    }
  })
  
  const modalStack = ref<Array<{
    visible: boolean
    title: string
    content: string
  }>>([])
  
  let searchTimeout: NodeJS.Timeout | null = null
  
  const resultCount = computed(() => results.value.length)
  
  const classSources = computed(() => SearchService.getClassSources())
  
  async function performSearch() {
    searchPerformed.value = true
    
    const sources = selectedSources.length > 0 
      ? SearchService.mapBookIdsToSources(selectedSources) 
      : undefined
    
    results.value = await SearchService.search({
      query: searchQuery.value,
      sources,
      category: selectedCategory.value,
      filters: filters.value
    })
  }
  
  function debouncedSearch() {
    if (searchTimeout) {
      clearTimeout(searchTimeout)
    }
    searchTimeout = setTimeout(() => {
      performSearch()
    }, 300)
  }
  
  function handleSort(column: string) {
    if (sortColumn.value === column) {
      sortDirection.value = sortDirection.value === 'asc' ? 'desc' : 'asc'
    } else {
      sortColumn.value = column
      sortDirection.value = 'asc'
    }
  }
  
  function updateMonsterFilters(newFilters: { sizes?: string[], types?: string[] }) {
    filters.value.monsters = { ...filters.value.monsters, ...newFilters }
  }
  
  async function selectSpell(spell: SpellSummary) {
    const fullSpell = await SearchService.getDetails({
      name: spell.name,
      source: spell.source,
      type: 'spell'
    })
    
    modalStack.value.push({
      visible: true,
      title: spell.name,
      content: formatSpellDetails(fullSpell || spell)
    })
  }
  
  async function selectItem(item: ItemSummary) {
    const fullItem = await SearchService.getDetails({
      name: item.name,
      source: item.source,
      type: 'item'
    })
    
    const formattedContent = await formatItemDetails(fullItem || item)
    modalStack.value.push({
      visible: true,
      title: item.name,
      content: formattedContent
    })
  }
  
  async function selectMonster(monster: MonsterSummary) {
    const fullMonster = await SearchService.getDetails({
      name: monster.name,
      source: monster.source,
      type: 'monster'
    })
    
    const formattedContent = await formatMonsterDetails(fullMonster || monster)
    modalStack.value.push({
      visible: true,
      title: monster.name,
      content: formattedContent
    })
  }
  
  async function selectClass(classItem: ClassSummary) {
    const fullClass = await SearchService.getDetails({
      name: classItem.name,
      source: classItem.source,
      type: 'class'
    })
    
    const formattedContent = await formatClassDetails(fullClass || classItem)
    modalStack.value.push({
      visible: true,
      title: classItem.name,
      content: formattedContent
    })
  }
  
  async function selectFeat(feat: FeatSummary) {
    const fullFeat = await SearchService.getDetails({
      name: feat.name,
      source: feat.source,
      type: 'feat'
    })
    
    const formattedContent = await formatFeatDetails(fullFeat || feat)
    modalStack.value.push({
      visible: true,
      title: feat.name,
      content: formattedContent
    })
  }
  
  function closeModal(index?: number) {
    if (index !== undefined) {
      modalStack.value.splice(index, 1)
    } else {
      modalStack.value.pop()
    }
  }
  
  async function handleReferenceClick(event: { type: string; name: string; source?: string }) {
    let details: any = null
    let formattedContent: string = ''
    
    switch (event.type) {
      case 'creature':
      case 'monster': {
        const searchName = event.name
        const titleCaseName = searchName.split(' ')
          .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
          .join(' ')
        
        details = await SearchService.getDetails({
          name: searchName,
          source: event.source || 'MM',
          type: 'monster'
        })
        
        if (!details) {
          details = await SearchService.getDetails({
            name: titleCaseName,
            source: event.source || 'MM',
            type: 'monster'
          })
        }
        
        if (details) {
          formattedContent = await formatMonsterDetails(details)
          modalStack.value.push({
            visible: true,
            title: details.name || event.name,
            content: formattedContent
          })
        }
        break
      }
      case 'item': {
        details = await SearchService.getDetails({
          name: event.name,
          source: event.source || 'PHB',
          type: 'item'
        })
        
        if (details) {
          formattedContent = await formatItemDetails(details)
          modalStack.value.push({
            visible: true,
            title: event.name,
            content: formattedContent
          })
        }
        break
      }
      case 'spell': {
        details = await SearchService.getDetails({
          name: event.name,
          source: event.source || 'PHB',
          type: 'spell'
        })
        
        if (details) {
          modalStack.value.push({
            visible: true,
            title: event.name,
            content: formatSpellDetails(details)
          })
        }
        break
      }
      case 'class': {
        details = await SearchService.getDetails({
          name: event.name,
          source: event.source || 'PHB',
          type: 'class'
        })
        
        if (details) {
          formattedContent = await formatClassDetails(details)
          modalStack.value.push({
            visible: true,
            title: event.name,
            content: formattedContent
          })
        }
        break
      }
      case 'feat': {
        details = await SearchService.getDetails({
          name: event.name,
          source: event.source || 'PHB',
          type: 'feat'
        })
        
        if (details) {
          formattedContent = await formatFeatDetails(details)
          modalStack.value.push({
            visible: true,
            title: event.name,
            content: formattedContent
          })
        }
        break
      }
    }
  }
  
  async function initialize() {
    await SearchService.initialize(selectedCategory.value)
    await performSearch()
  }
  
  watch(selectedCategory, async (newCategory) => {
    await SearchService.initialize(newCategory)
    await performSearch()
  })
  
  watch(() => selectedSources, () => {
    performSearch()
  }, { deep: true })
  
  return {
    selectedCategory,
    searchQuery,
    searchPerformed,
    sortColumn,
    sortDirection,
    results,
    filters,
    modalStack,
    resultCount,
    classSources,
    performSearch,
    debouncedSearch,
    handleSort,
    updateMonsterFilters,
    selectSpell,
    selectItem,
    selectMonster,
    selectClass,
    selectFeat,
    closeModal,
    handleReferenceClick,
    initialize
  }
}