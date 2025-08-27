<template>
  <div class="race-table-container">
    <table class="catalog-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Size</th>
          <th>Speed</th>
          <th>Ability Bonuses</th>
          <th>Traits</th>
          <th>Source</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="race in races" :key="`${race.name}-${race.source}`" @click="selectRace(race)" class="clickable">
          <td>{{ race.name }}</td>
          <td>{{ race.size }}</td>
          <td>{{ race.speed }} ft.</td>
          <td>{{ race.abilityBonuses }}</td>
          <td>{{ race.traitsCount }}</td>
          <td>{{ race.source }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import type { RaceSummary } from '../../composables/useCatalog'

defineProps<{
  races: RaceSummary[]
}>()

const emit = defineEmits<{
  select: [race: RaceSummary]
}>()

function selectRace(race: RaceSummary) {
  emit('select', race)
}
</script>

<style scoped>
.race-table-container {
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