import type { CatalogConfig } from './types'

export const raceConfig: CatalogConfig = {
  name: 'races',
  title: 'Races',
  searchCommands: {
    search: 'search_races',
    details: 'get_race_details',
    sources: 'get_race_sources'
  },
  columns: [
    {
      key: 'name',
      label: 'Name',
      sortable: true,
      className: 'catalog-table__cell-name'
    },
    {
      key: 'size',
      label: 'Size',
      type: 'text',
      className: 'catalog-table__cell-center'
    },
    {
      key: 'speed',
      label: 'Speed',
      type: 'number',
      className: 'catalog-table__cell-center',
      formatter: (value: number) => `${value} ft.`
    },
    {
      key: 'ability_bonuses',
      label: 'Ability Bonuses',
      type: 'text',
      className: 'catalog-table__cell-secondary'
    },
    {
      key: 'traits_count',
      label: 'Traits',
      type: 'number',
      className: 'catalog-table__cell-center'
    },
    {
      key: 'source',
      label: 'Source',
      sortable: true,
      type: 'text',
      className: 'catalog-table__cell-source'
    }
  ],
  filters: [
    {
      type: 'text',
      key: 'search',
      label: 'Search',
      placeholder: 'Search races...'
    },
    {
      type: 'multiselect',
      key: 'sources',
      label: 'Source',
      options: [], // Will be populated dynamically from API
      apiSource: 'get_race_sources'
    },
    {
      type: 'multiselect',
      key: 'sizes',
      label: 'Size',
      options: [], // Will be populated dynamically from API
      apiSource: 'get_race_sizes'
    },
    {
      type: 'checkbox',
      key: 'has_darkvision',
      label: 'Has Darkvision',
      tooltip: 'Show only races with darkvision ability'
    },
    {
      type: 'checkbox',
      key: 'has_flight',
      label: 'Has Flight',
      tooltip: 'Show only races with flying speed'
    }
  ],
  emptyMessage: {
    title: 'No races found',
    subtitle: 'Search for races to see results',
    noResults: 'No races found matching your criteria'
  }
}