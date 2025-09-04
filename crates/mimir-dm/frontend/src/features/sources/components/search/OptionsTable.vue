<template>
  <div class="catalog-table">
    <div class="catalog-table__content">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th">Name</th>
            <th class="catalog-table__th">Type</th>
            <th class="catalog-table__th">Prerequisites</th>
            <th class="catalog-table__th">Source</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr v-for="option in options" :key="`${option.name}-${option.source}`"
              class="catalog-table__row catalog-table__row--clickable"
              @click="emit('select', option)">
            <td class="catalog-table__td catalog-table__name">
              {{ option.name }}
              <span v-if="option.grants_spells" class="spell-badge" title="Grants Additional Spells">🔮</span>
            </td>
            <td class="catalog-table__td catalog-table__secondary">{{ option.feature_type_full }}</td>
            <td class="catalog-table__td catalog-table__secondary">{{ option.prerequisite_text || '—' }}</td>
            <td class="catalog-table__td catalog-table__source">
              {{ option.source }}
              <span v-if="option.is_srd" class="catalog-table__badge catalog-table__badge--srd">SRD</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { OptionalFeatureSummary } from '../../services/SearchService'

interface Props {
  options: OptionalFeatureSummary[]
}

defineProps<Props>()

const emit = defineEmits<{
  select: [option: OptionalFeatureSummary]
}>()
</script>

<style scoped>
/* Custom badge for spell indication */
.spell-badge {
  margin-left: 4px;
  font-size: 0.85em;
  vertical-align: super;
}
</style>