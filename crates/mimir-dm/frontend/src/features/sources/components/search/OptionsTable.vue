<template>
  <div class="options-table-container">
    <table class="options-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Type</th>
          <th>Prerequisites</th>
          <th>Source</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="option in options" :key="`${option.name}-${option.source}`"
            @click="emit('select', option)"
            class="option-row">
          <td class="name">
            {{ option.name }}
            <span v-if="option.grants_spells" class="spell-badge" title="Grants Additional Spells">🔮</span>
          </td>
          <td class="type">{{ option.feature_type_full }}</td>
          <td class="prerequisites">{{ option.prerequisite_text || '—' }}</td>
          <td class="source">
            {{ option.source }}
            <span v-if="option.is_srd" class="srd-badge">SRD</span>
          </td>
        </tr>
      </tbody>
    </table>
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
.options-table-container {
  width: 100%;
  overflow-x: auto;
}

.options-table {
  width: 100%;
  border-collapse: collapse;
}

.options-table th {
  text-align: left;
  padding: 8px;
  border-bottom: 2px solid var(--color-border, #333);
  color: var(--color-text-secondary, #999);
  font-weight: 600;
}

.option-row {
  cursor: pointer;
  border-bottom: 1px solid var(--color-border-light, #222);
  transition: background-color 0.2s;
}

.option-row:hover {
  background: rgba(74, 158, 255, 0.1);
}

.option-row td {
  padding: 10px 8px;
  vertical-align: top;
}

.name {
  color: var(--color-primary, #4a9eff);
  font-weight: 500;
  position: relative;
}

.spell-badge {
  margin-left: 4px;
  font-size: 0.85em;
  vertical-align: super;
}

.type {
  color: var(--color-text, #e0e0e0);
  font-size: 0.9em;
  white-space: nowrap;
}

.prerequisites {
  font-size: 0.9em;
  color: var(--color-text-secondary, #999);
  max-width: 300px;
}

.source {
  font-size: 0.85em;
  color: var(--color-text-secondary, #999);
  white-space: nowrap;
}

.srd-badge {
  margin-left: 4px;
  padding: 1px 4px;
  background: var(--color-background-tertiary, #262626);
  border-radius: 3px;
  font-size: 0.8em;
}
</style>