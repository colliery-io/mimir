<template>
  <div class="background-table-container">
    <table class="background-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Source</th>
          <th>Skills</th>
          <th>Languages</th>
          <th>Tools</th>
          <th>Feature</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="background in backgrounds" :key="`${background.name}-${background.source}`"
            @click="emit('select', background)"
            class="background-row">
          <td class="name">{{ background.name }}</td>
          <td>{{ background.source }}</td>
          <td>{{ background.skills }}</td>
          <td>{{ background.languages }}</td>
          <td>{{ background.tools || 'None' }}</td>
          <td>{{ background.feature }}</td>
        </tr>
      </tbody>
    </table>
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

<style scoped>
.background-table-container {
  width: 100%;
  overflow-x: auto;
}

.background-table {
  width: 100%;
  border-collapse: collapse;
}

.background-table th {
  text-align: left;
  padding: 8px;
}

.background-row {
  cursor: pointer;
}

.background-row:hover {
  background: rgba(74, 158, 255, 0.1);
}

.background-row td {
  padding: 8px;
}

.name {
  color: var(--color-primary, #4a9eff);
}
</style>