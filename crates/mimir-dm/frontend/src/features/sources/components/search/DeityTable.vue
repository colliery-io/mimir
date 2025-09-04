<template>
  <div class="catalog-table">
    <div class="catalog-table__content" v-if="deities.length > 0">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th catalog-table__th--sortable" @click="$emit('sort', 'name')">
              Name
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'name'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th">Title</th>
            <th class="catalog-table__th">Pantheon</th>
            <th class="catalog-table__th">Alignment</th>
            <th class="catalog-table__th">Domains</th>
            <th class="catalog-table__th">Symbol</th>
            <th class="catalog-table__th">Source</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr 
            v-for="deity in deities" 
            :key="`${deity.name}-${deity.source}`"
            class="catalog-table__row catalog-table__row--clickable"
            @click="$emit('select', deity)"
          >
            <td class="catalog-table__td catalog-table__name">
              {{ deity.name }}
              <span v-if="deity.is_srd" class="catalog-table__badge catalog-table__badge--srd">SRD</span>
            </td>
            <td class="catalog-table__td catalog-table__secondary">{{ deity.title }}</td>
            <td class="catalog-table__td catalog-table__secondary">{{ deity.pantheon }}</td>
            <td class="catalog-table__td catalog-table__center">{{ deity.alignment }}</td>
            <td class="catalog-table__td catalog-table__secondary">{{ formatDomains(deity.domains) }}</td>
            <td class="catalog-table__td catalog-table__secondary">{{ deity.symbol }}</td>
            <td class="catalog-table__td">
              <span class="catalog-table__badge catalog-table__badge--source">{{ deity.source }}</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-else-if="searchPerformed" class="catalog-table__empty">
      No deities found matching your search criteria.
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DeitySummary } from '../../composables/useCatalog'

interface Props {
  deities: DeitySummary[]
  searchPerformed: boolean
  sortColumn: string | null
  sortDirection: 'asc' | 'desc'
}

const props = defineProps<Props>()

const emit = defineEmits<{
  sort: [column: string]
  select: [deity: DeitySummary]
}>()

function formatDomains(domains: string[]): string {
  return domains.join(', ')
}
</script>

<style scoped>
/* Deity-specific table layout fixes */
.catalog-table__table {
  table-layout: fixed;
}

.catalog-table__table th:nth-child(1) { /* Name */
  width: 15%;
  min-width: 120px;
}

.catalog-table__table th:nth-child(2) { /* Title */
  width: 20%;
  min-width: 150px;
}

.catalog-table__table th:nth-child(3) { /* Pantheon */
  width: 15%;
  min-width: 120px;
}

.catalog-table__table th:nth-child(4) { /* Alignment */
  width: 12%;
  min-width: 100px;
}

.catalog-table__table th:nth-child(5) { /* Domains */
  width: 20%;
  min-width: 150px;
}

.catalog-table__table th:nth-child(6) { /* Symbol */
  width: 8%;
  min-width: 80px;
}

.catalog-table__table th:nth-child(7) { /* Source */
  width: 10%;
  min-width: 80px;
}

/* Ensure domains text wraps properly */
.catalog-table__td:nth-child(5) {
  word-wrap: break-word;
  line-height: 1.3;
}
</style>