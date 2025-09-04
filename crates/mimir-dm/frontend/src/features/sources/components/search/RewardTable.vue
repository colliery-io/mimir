<template>
  <div class="catalog-table">
    <div class="catalog-table__content">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'name')">
              Name
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'name'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'type')">
              Type
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'type'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th">Description</th>
            <th class="catalog-table__th">Prerequisites</th>
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'source')">
              Source
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'source'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr v-for="reward in rewards" :key="`${reward.name}-${reward.source}`" 
              class="catalog-table__row catalog-table__row--clickable"
              @click="emit('select', reward)">
            <td class="catalog-table__td catalog-table__name">{{ reward.name }}</td>
            <td class="catalog-table__td">
              <span :class="['catalog-table__badge', getTypeClass(reward.reward_type)]">
                {{ reward.reward_type }}
              </span>
            </td>
            <td class="catalog-table__td catalog-table__description">{{ reward.description }}</td>
            <td class="catalog-table__td catalog-table__center">
              <span v-if="reward.has_prerequisites" class="prereq-icon" title="Has prerequisites">
                ✓
              </span>
              <span v-else class="catalog-table__empty">—</span>
            </td>
            <td class="catalog-table__td catalog-table__source">{{ reward.source }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { RewardSummary } from '../../composables/useCatalog'

defineProps<{
  rewards: RewardSummary[]
  sortColumn: string
  sortDirection: 'asc' | 'desc'
}>()

const emit = defineEmits<{
  sort: [column: string]
  select: [reward: RewardSummary]
}>()

function getTypeClass(type: string): string {
  if (!type) return 'type-default'
  
  switch (type.toLowerCase()) {
    case 'blessing':
      return 'type-blessing'
    case 'epic boon':
    case 'boon':
      return 'type-boon'
    case 'charm':
      return 'type-charm'
    case 'feat':
      return 'type-feat'
    default:
      return 'type-default'
  }
}
</script>

<!-- Styles now handled by consolidated catalog-tables.css -->