<template>
  <div class="feat-table-container">
    <table class="catalog-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Prerequisites</th>
          <th>Source</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="feat in feats" :key="`${feat.name}-${feat.source}`" @click="selectFeat(feat)" class="clickable">
          <td>{{ feat.name }}</td>
          <td>{{ feat.prerequisites || feat.brief || '—' }}</td>
          <td>{{ feat.source }}</td>
        </tr>
      </tbody>
    </table>
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

<style scoped>
.feat-table-container {
  width: 100%;
  overflow-x: auto;
}

.catalog-table {
  width: 100%;
  border-collapse: collapse;
}

.catalog-table th {
  background: var(--color-surface-variant);
  padding: var(--spacing-sm);
  text-align: left;
  font-weight: 600;
  border-bottom: 2px solid var(--color-border);
}

.catalog-table td {
  padding: var(--spacing-sm);
  border-bottom: 1px solid var(--color-border-light);
}

.catalog-table tbody tr:hover {
  background: var(--color-surface-hover);
}

.clickable {
  cursor: pointer;
}
</style>