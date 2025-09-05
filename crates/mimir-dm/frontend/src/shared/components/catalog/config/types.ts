// Column definition types
export interface ColumnDef {
  key: string
  label: string
  sortable?: boolean
  width?: string
  type?: 'text' | 'badge' | 'badges' | 'number' | 'array'
  formatter?: (value: any) => string | string[]
  badgeType?: string
  className?: string
}

// Filter definition types
export interface FilterConfig {
  type: 'text' | 'select' | 'multiselect' | 'checkbox' | 'range'
  key: string
  label: string
  options?: (string | SelectOption)[]
  groupedOptions?: GroupedOption[]
  placeholder?: string
  tooltip?: string
  min?: number
  max?: number
  step?: number
  apiSource?: string
}

export interface SelectOption {
  value: string
  label: string
}

export interface GroupedOption {
  label: string
  options: SelectOption[]
}

// Main catalog configuration
export interface CatalogConfig {
  name: string
  title: string
  columns: ColumnDef[]
  filters: FilterConfig[]
  searchCommands?: {
    search: string
    details: string
    sources?: string
    timeTypes?: string
  }
  emptyMessage?: {
    title: string
    subtitle: string
    noResults: string
  }
  searchPlaceholder?: string
}

// Filter values for different types
export interface FilterValues {
  [key: string]: any
}

// Range value type
export interface RangeValue {
  min?: number
  max?: number
}