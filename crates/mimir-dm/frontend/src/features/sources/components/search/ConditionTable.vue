<template>
  <div class="catalog-table">
    <div class="catalog-table__content">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th">Name</th>
            <th class="catalog-table__th">Type</th>
            <th class="catalog-table__th">Description</th>
            <th class="catalog-table__th">Source</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr v-for="condition in conditions" :key="`${condition.name}-${condition.source}`"
              class="catalog-table__row catalog-table__row--clickable"
              @click="emit('select', condition)">
            <td class="catalog-table__td catalog-table__name">{{ condition.name }}</td>
            <td class="catalog-table__td">
              <span :class="['catalog-table__badge', (condition.item_type || 'unknown').toLowerCase()]">
                {{ condition.item_type || 'Unknown' }}
              </span>
            </td>
            <td class="catalog-table__td catalog-table__description">{{ condition.description }}</td>
            <td class="catalog-table__td catalog-table__source">
              {{ condition.source }}
              <span v-if="condition.is_srd" class="catalog-table__badge catalog-table__badge--srd">SRD</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ConditionSummary } from '../../services/SearchService'

interface Props {
  conditions: ConditionSummary[]
}

defineProps<Props>()

const emit = defineEmits<{
  select: [condition: ConditionSummary]
}>()
</script>

<!-- Styles now handled by consolidated catalog-tables.css -->