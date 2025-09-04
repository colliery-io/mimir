<template>
  <div class="catalog-table">
    <div class="catalog-table__content">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th">Name</th>
            <th class="catalog-table__th">Size</th>
            <th class="catalog-table__th">Speed</th>
            <th class="catalog-table__th">Ability Bonuses</th>
            <th class="catalog-table__th">Traits</th>
            <th class="catalog-table__th">Source</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr v-for="race in races" :key="`${race.name}-${race.source}`" 
              class="catalog-table__row catalog-table__row--clickable" 
              @click="selectRace(race)">
            <td class="catalog-table__td catalog-table__name">{{ race.name }}</td>
            <td class="catalog-table__td catalog-table__center">{{ race.size }}</td>
            <td class="catalog-table__td catalog-table__center">{{ race.speed }} ft.</td>
            <td class="catalog-table__td catalog-table__secondary">{{ race.abilityBonuses }}</td>
            <td class="catalog-table__td catalog-table__center">{{ race.traitsCount }}</td>
            <td class="catalog-table__td catalog-table__source">{{ race.source }}</td>
          </tr>
        </tbody>
      </table>
    </div>
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

<!-- All styling now handled by consolidated CSS classes -->