<template>
  <div class="catalog-table">
    <div class="catalog-table__content">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th">Name</th>
            <th class="catalog-table__th">Prerequisites</th>
            <th class="catalog-table__th">Source</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr v-for="feat in feats" :key="`${feat.name}-${feat.source}`" 
              class="catalog-table__row catalog-table__row--clickable" 
              @click="selectFeat(feat)">
            <td class="catalog-table__td catalog-table__name">{{ feat.name }}</td>
            <td class="catalog-table__td catalog-table__secondary">{{ feat.prerequisites || feat.brief || '—' }}</td>
            <td class="catalog-table__td catalog-table__source">{{ feat.source }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { FeatSummary } from '../../composables/useCatalog'

defineProps<{
  feats: FeatSummary[]
}>()

const emit = defineEmits<{
  select: [feat: FeatSummary]
}>()

function selectFeat(feat: FeatSummary) {
  emit('select', feat)
}
</script>

<!-- All styling now handled by consolidated CSS classes -->