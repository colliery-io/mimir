<template>
  <div class="catalog-table">
    <div class="catalog-table__content">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th">Name</th>
            <th class="catalog-table__th">Source</th>
            <th class="catalog-table__th">Skills</th>
            <th class="catalog-table__th">Languages</th>
            <th class="catalog-table__th">Tools</th>
            <th class="catalog-table__th">Feature</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr v-for="background in backgrounds" :key="`${background.name}-${background.source}`"
              class="catalog-table__row catalog-table__row--clickable"
              @click="emit('select', background)">
            <td class="catalog-table__td catalog-table__name">{{ background.name }}</td>
            <td class="catalog-table__td catalog-table__source">{{ background.source }}</td>
            <td class="catalog-table__td catalog-table__secondary">{{ background.skills }}</td>
            <td class="catalog-table__td catalog-table__secondary">{{ background.languages }}</td>
            <td class="catalog-table__td catalog-table__secondary">{{ background.tools || 'None' }}</td>
            <td class="catalog-table__td catalog-table__secondary">{{ background.feature }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { BackgroundSummary } from '../../services/SearchService'

interface Props {
  backgrounds: BackgroundSummary[]
}

defineProps<Props>()

const emit = defineEmits<{
  select: [background: BackgroundSummary]
}>()
</script>

<!-- All styling now handled by consolidated CSS classes -->