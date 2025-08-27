import { ref, computed, watch } from 'vue'
import { SearchService, type SearchFilters } from '../services/SearchService'
import type { 
  SpellSummary, 
  ItemSummary, 
  MonsterSummary,
  ClassSummary,
  FeatSummary,
  RaceSummary,
  BackgroundSummary,
  ActionSummary,
  ConditionSummary,
  OptionalFeatureSummary,
  DeitySummary,
  ObjectSummary,
  DndObject,
  TrapSummary,
  TrapOrHazard,
  LanguageSummary,
  Language,
  RewardSummary,
  Reward,
  TableSummary,
  Table,
  PsionicSummary
} from './useCatalog'
import { formatSpellDetails } from '../formatters/spellFormatterEnhanced'
import { formatItemDetails } from '../formatters/itemFormatterEnhanced'
import { formatMonsterDetails } from '../formatters/monsterFormatterEnhanced'
import { formatClassDetails } from '../formatters/classFormatterEnhanced'
import { formatFeatDetails } from '../formatters/featFormatter'
import { formatRaceDetails } from '../formatters/raceFormatter'
import { formatBackgroundDetails } from '../formatters/backgroundFormatter'
import { formatActionDetails } from '../formatters/actionFormatter'

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
  
  async function selectRace(race: RaceSummary) {
    const fullRace = await SearchService.getDetails({
      name: race.name,
      source: race.source,
      type: 'race'
    })
    
    const formattedContent = await formatRaceDetails(fullRace || race)
    modalStack.value.push({
      visible: true,
      title: race.name,
      content: formattedContent
    })
  }
  
  async function selectBackground(background: BackgroundSummary) {
    const fullBackground = await SearchService.getDetails({
      name: background.name,
      source: background.source,
      type: 'background'
    })
    
    const formattedContent = await formatBackgroundDetails(fullBackground || background)
    modalStack.value.push({
      visible: true,
      title: background.name,
      content: formattedContent
    })
  }
  
  async function selectAction(action: ActionSummary) {
    const fullAction = await SearchService.getDetails({
      name: action.name,
      source: action.source,
      type: 'action'
    })
    
    const formattedContent = await formatActionDetails(fullAction || action)
    modalStack.value.push({
      visible: true,
      title: action.name,
      content: formattedContent
    })
  }
  
  async function selectCondition(condition: ConditionSummary) {
    const fullCondition = await SearchService.getDetails({
      name: condition.name,
      source: condition.source,
      type: 'condition'
    })
    
    const { formatConditionDetails } = await import('../formatters/conditionFormatter')
    const formattedContent = formatConditionDetails(fullCondition || condition)
    modalStack.value.push({
      visible: true,
      title: condition.name,
      content: formattedContent
    })
  }
  
  async function selectOption(option: OptionalFeatureSummary) {
    const fullOption = await SearchService.getDetails({
      name: option.name,
      source: option.source,
      type: 'option'
    })
    
    const { formatOptionalFeatureDetails } = await import('../formatters/optionalFeatureFormatter')
    const formattedContent = formatOptionalFeatureDetails(fullOption || option)
    modalStack.value.push({
      visible: true,
      title: option.name,
      content: formattedContent
    })
  }
  
  async function selectDeity(deity: DeitySummary) {
    const fullDeity = await SearchService.getDetails({
      name: deity.name,
      source: deity.source,
      type: 'deity'
    })
    
    const { formatDeityContent } = await import('../formatters/deityFormatter')
    const formattedContent = formatDeityContent(fullDeity || deity)
    modalStack.value.push({
      visible: true,
      title: deity.name,
      content: formattedContent
    })
  }
  
  async function selectObject(obj: ObjectSummary) {
    const fullObject = await SearchService.getDetails({
      name: obj.name,
      source: obj.source,
      type: 'object'
    }) as DndObject
    
    const { formatObjectDetails } = await import('../formatters/objectFormatter')
    const formattedContent = await formatObjectDetails(fullObject || obj)
    modalStack.value.push({
      visible: true,
      title: obj.name,
      content: formattedContent
    })
  }
  
  async function selectTrap(trap: TrapSummary) {
    const fullTrap = await SearchService.getDetails({
      name: trap.name,
      source: trap.source,
      type: 'trap'
    }) as TrapOrHazard
    
    const { formatTrapDetails } = await import('../formatters/trapFormatter')
    const formattedContent = await formatTrapDetails(fullTrap || trap)
    modalStack.value.push({
      visible: true,
      title: trap.name,
      content: formattedContent
    })
  }
  
  async function selectLanguage(lang: LanguageSummary) {
    const fullLang = await SearchService.getDetails({
      name: lang.name,
      source: lang.source,
      type: 'language'
    }) as Language
    
    const { formatLanguageDetails } = await import('../formatters/languageFormatter')
    const formattedContent = await formatLanguageDetails(fullLang || lang)
    modalStack.value.push({
      visible: true,
      title: lang.name,
      content: formattedContent
    })
  }
  
  async function selectReward(reward: RewardSummary) {
    const fullReward = await SearchService.getDetails({
      name: reward.name,
      source: reward.source,
      type: 'reward'
    })
    
    const { formatRewardDetails } = await import('../formatters/rewardFormatter')
    const formattedContent = await formatRewardDetails(fullReward || reward)
    modalStack.value.push({
      visible: true,
      title: reward.name,
      content: formattedContent
    })
  }
  
  async function selectTable(table: TableSummary) {
    const fullTable = await SearchService.getDetails({
      name: table.name,
      source: table.source,
      type: 'table'
    })
    
    const { formatTableDetails } = await import('../formatters/tableFormatter')
    const formattedContent = await formatTableDetails(fullTable || table)
    modalStack.value.push({
      visible: true,
      title: table.name,
      content: formattedContent
    })
  }
  
  async function selectVariantRule(rule: any) {
    const fullRule = await SearchService.getDetails({
      name: rule.name,
      source: rule.source,
      type: 'variantrule'
    })
    
    const { formatVariantRuleDetails } = await import('../formatters/variantRuleFormatter')
    const formattedContent = formatVariantRuleDetails(fullRule || rule)
    modalStack.value.push({
      visible: true,
      title: rule.name,
      content: formattedContent
    })
  }
  
  async function selectVehicle(vehicle: any) {
    const fullVehicle = await SearchService.getDetails({
      name: vehicle.name,
      source: vehicle.source,
      type: 'vehicle'
    })
    
    const { formatVehicleDetails } = await import('../formatters/vehicleFormatter')
    const formattedContent = formatVehicleDetails(fullVehicle || vehicle)
    modalStack.value.push({
      visible: true,
      title: vehicle.name,
      content: formattedContent
    })
  }
  
  async function selectCult(item: any) {
    let details: any
    let formattedContent: string
    
    if (item.item_type === 'cult') {
      details = await SearchService.getDetails({
        name: item.name,
        source: item.source,
        type: 'cult'
      })
      const { formatCultDetails } = await import('../formatters/cultFormatter')
      formattedContent = formatCultDetails(details || item)
    } else {
      details = await SearchService.getDetails({
        name: item.name,
        source: item.source,
        type: 'boon'
      })
      const { formatBoonDetails } = await import('../formatters/cultFormatter')
      formattedContent = formatBoonDetails(details || item)
    }
    
    modalStack.value.push({
      visible: true,
      title: item.name,
      content: formattedContent
    })
  }
  
  async function selectPsionic(psionic: PsionicSummary) {
    const fullPsionic = await SearchService.getDetails({
      name: psionic.name,
      source: psionic.source,
      type: 'psionic'
    })
    
    const { formatPsionicDetails } = await import('../formatters/psionicFormatter')
    const formattedContent = formatPsionicDetails(fullPsionic || psionic)
    modalStack.value.push({
      visible: true,
      title: psionic.name,
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
    console.log('Reference clicked:', event)
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
      case 'race': {
        details = await SearchService.getDetails({
          name: event.name,
          source: event.source || 'PHB',
          type: 'race'
        })
        
        if (details) {
          formattedContent = await formatRaceDetails(details)
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
    selectRace,
    selectBackground,
    selectAction,
    selectCondition,
    selectOption,
    selectDeity,
    selectObject,
    selectTrap,
    selectLanguage,
    selectReward,
    selectTable,
    selectVariantRule,
    selectVehicle,
    selectCult,
    selectPsionic,
    closeModal,
    handleReferenceClick,
    initialize,
    // Direct trap catalog functions
    searchTraps: async (params: {
      query?: string
      sources?: string[]
      categories?: string[]
      trap_types?: string[]
    }) => SearchService.searchTraps(params),
    getTrapDetails: async (name: string, source: string) => 
      SearchService.getTrapDetails(name, source),
    getTrapTypes: async () => SearchService.getTrapTypes(),
    // Direct language catalog functions
    searchLanguages: async (params: {
      query?: string
      sources?: string[]
      types?: string[]
      scripts?: string[]
    }) => SearchService.searchLanguages(params),
    getLanguageDetails: async (name: string, source: string) => 
      SearchService.getLanguageDetails(name, source),
    getLanguageTypes: async () => SearchService.getLanguageTypes(),
    getLanguageScripts: async () => SearchService.getLanguageScripts()
  }
}