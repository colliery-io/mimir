<template>
  <div class="catalog-table" data-table="action">
    <div class="catalog-table__content">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th">Name</th>
            <th class="catalog-table__th">Time</th>
            <th class="catalog-table__th">Description</th>
            <th class="catalog-table__th">See Also</th>
            <th class="catalog-table__th">Source</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr v-for="action in actions" :key="`${action.name}-${action.source}`"
              class="catalog-table__row catalog-table__row--clickable"
              @click="emit('select', action)">
            <td class="catalog-table__td catalog-table__name">{{ action.name }}</td>
            <td class="catalog-table__td catalog-table__center">{{ action.time }}</td>
            <td class="catalog-table__td catalog-table__description">{{ action.description }}</td>
            <td class="catalog-table__td catalog-table__secondary">
              <span v-if="action.see_also && action.see_also.length > 0">
                {{ action.see_also.join(', ') }}
              </span>
              <span v-else class="catalog-table__empty">—</span>
            </td>
            <td class="catalog-table__td catalog-table__source">{{ action.source }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ActionSummary } from '../../services/SearchService'

interface Props {
  actions: ActionSummary[]
}

defineProps<Props>()

const emit = defineEmits<{
  select: [action: ActionSummary]
}>()
</script>

<!-- Styles now handled by consolidated catalog-tables.css -->