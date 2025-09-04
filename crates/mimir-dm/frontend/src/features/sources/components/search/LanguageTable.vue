<template>
  <div class="catalog-table">
    <div class="catalog-table__content" v-if="languages.length > 0">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th catalog-table__th--sortable" @click="$emit('sort', 'name')">
              Name
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'name'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th">Type</th>
            <th class="catalog-table__th">Script</th>
            <th class="catalog-table__th">Typical Speakers</th>
            <th class="catalog-table__th">Source</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr 
            v-for="lang in languages" 
            :key="`${lang.name}-${lang.source}`"
            class="catalog-table__row catalog-table__row--clickable"
            @click="$emit('select', lang)"
          >
            <td class="catalog-table__td catalog-table__name">
              {{ lang.name }}
              <span v-if="lang.is_srd" class="catalog-table__badge catalog-table__badge--srd">SRD</span>
            </td>
            <td class="catalog-table__td">
              <span :class="['catalog-table__badge', getTypeClass(lang.language_type)]">{{ lang.language_type }}</span>
            </td>
            <td class="catalog-table__td">{{ lang.script }}</td>
            <td class="catalog-table__td catalog-table__description">{{ lang.typical_speakers }}</td>
            <td class="catalog-table__td">
              <span class="catalog-table__badge catalog-table__badge--source">{{ lang.source }}</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-else-if="searchPerformed" class="catalog-table__empty">
      No languages found matching your search criteria.
    </div>
  </div>
</template>

<script setup lang="ts">
import type { LanguageSummary } from '../../composables/useCatalog'

interface Props {
  languages: LanguageSummary[]
  searchPerformed: boolean
  sortColumn: string | null
  sortDirection: 'asc' | 'desc'
}

const props = defineProps<Props>()

const emit = defineEmits<{
  sort: [column: string]
  select: [lang: LanguageSummary]
}>()

function getTypeClass(type: string): string {
  if (!type) return 'type-standard'
  
  switch (type.toLowerCase()) {
    case 'standard':
      return 'type-standard'
    case 'exotic':
      return 'type-exotic'
    case 'secret':
      return 'type-secret'
    case 'dead':
      return 'type-dead'
    case 'primordial dialect':
      return 'type-primordial'
    default:
      return 'type-default'
  }
}
</script>

<!-- Styles now handled by consolidated catalog-tables.css -->