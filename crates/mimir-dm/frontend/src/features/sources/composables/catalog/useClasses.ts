import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ClassSummary {
  name: string
  source: string
  page?: number
  hitDice: string
  proficiency: string
  primaryAbility: string
  spellcastingAbility?: string
  tableGroups?: any[]
  subclassTitle?: string
  description: string
  subclassName?: string
  rowType: string
}

export interface ClassFilters {
  name?: string
  sources?: string[]
  has_spellcasting?: boolean
  primary_abilities?: string[]
}

export interface Subclass {
  name: string
  source: string
  className: string
  classSource: string
  shortName?: string
  page?: number
  spellcastingAbility?: string
  casterProgression?: string
  subclassFeatures?: any
  subclassTableGroups?: any[]
  fluff?: SubclassFluff
  introDescription?: string
}

export interface ClassFeature {
  name: string
  source: string
  class_name: string
  class_source: string
  level: number
  page?: number
  entries?: any[]
}

export interface SubclassFeature {
  name: string
  source: string
  class_name: string
  class_source: string
  subclass_short_name?: string
  subclass_source: string
  level: number
  page?: number
  entries?: any[]
}

export interface ClassFluff {
  name: string
  source: string
  entries: any[]
  images?: any[]
}

export interface SubclassFluff {
  name: string
  short_name?: string
  source: string
  class_name: string
  class_source: string
  entries: any[]
  images?: any[]
}

export interface ClassWithDetails {
  class: Class
  subclasses: Subclass[]
  features: ClassFeature[]
  subclass_features: SubclassFeature[]
  fluff?: ClassFluff
  subclass_fluff: SubclassFluff[]
}

export interface Class {
  name: string
  source: string
  page?: number
  hd?: any
  proficiency?: any
  startingProficiencies?: any
  spellcastingAbility?: string
  classTableGroups?: any[]
  subclassTitle?: string
  entries?: any[]
  classFeatures?: any[]
  multiclassing?: any
  casterProgression?: string
  fluff?: ClassFluff
}

export function useClasses() {
  const isClassesInitialized = ref(false)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)
  const classes = ref<ClassSummary[]>([])
  const classSources = ref<string[]>([])

  async function initializeClassCatalog() {
    isClassesInitialized.value = true
  }

  async function searchClasses(filters: ClassFilters): Promise<ClassSummary[]> {
    if (!isClassesInitialized.value) {
      await initializeClassCatalog()
    }

    try {
      isLoading.value = true
      error.value = null

      const results = await invoke<ClassSummary[]>('search_classes_db', {
        filters: {
          name: filters.name || null,
          sources: filters.sources || null,
          has_spellcasting: filters.has_spellcasting || null,
          primary_abilities: filters.primary_abilities || null,
        }
      })

      console.log('search_classes_db returned:', results)
      console.log('First 5 results detail:', results.slice(0, 5))

      classes.value = results
      return results
    } catch (e) {
      error.value = `Search failed: ${e}`
      return []
    } finally {
      isLoading.value = false
    }
  }

  async function getClassDetails(name: string, source: string): Promise<ClassWithDetails | null> {
    try {
      console.log('getClassDetails called with:', { name, source })
      const classDetails = await invoke<ClassWithDetails>('get_class_details_db', {
        className: name,
        classSource: source
      })
      console.log('getClassDetails returned:', classDetails)
      return classDetails
    } catch (e) {
      console.error('getClassDetails error:', e)
      return null
    }
  }

  async function getSubclassDetails(subclassName: string, className: string, classSource: string): Promise<Subclass | null> {
    try {
      console.log('getSubclassDetails called with:', { subclassName, className, classSource })
      const subclassDetails = await invoke<Subclass>('get_subclass_details_db', {
        subclassName: subclassName,
        className: className,
        classSource: classSource
      })
      console.log('getSubclassDetails returned:', subclassDetails)
      return subclassDetails
    } catch (e) {
      console.error('getSubclassDetails error:', e)
      return null
    }
  }

  async function getClassSubclasses(className: string, classSource: string): Promise<Subclass[]> {
    try {
      const subclasses = await invoke<Subclass[]>('get_class_subclasses_db', {
        class_name: className,
        class_source: classSource
      })
      return subclasses
    } catch (e) {
      return []
    }
  }

  return {
    isClassesInitialized,
    isLoading,
    error,
    classes,
    classSources,
    initializeClassCatalog,
    searchClasses,
    getClassDetails,
    getSubclassDetails,
    getClassSubclasses,
  }
}
