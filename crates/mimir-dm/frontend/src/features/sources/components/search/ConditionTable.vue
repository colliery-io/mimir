<template>
  <div class="condition-table-container">
    <table class="condition-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Type</th>
          <th>Description</th>
          <th>Source</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="condition in conditions" :key="`${condition.name}-${condition.source}`"
            @click="emit('select', condition)"
            class="condition-row">
          <td class="name">{{ condition.name }}</td>
          <td class="type">
            <span :class="['type-badge', (condition.item_type || 'unknown').toLowerCase()]">
              {{ condition.item_type || 'Unknown' }}
            </span>
          </td>
          <td class="description">{{ condition.description }}</td>
          <td class="source">
            {{ condition.source }}
            <span v-if="condition.is_srd" class="srd-badge">SRD</span>
          </td>
        </tr>
      </tbody>
    </table>
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

<style scoped>
.condition-table-container {
  width: 100%;
  overflow-x: auto;
}

.condition-table {
  width: 100%;
  border-collapse: collapse;
}

.condition-table th {
  text-align: left;
  padding: 8px;
}

.condition-row {
  cursor: pointer;
}

.condition-row:hover {
  background: rgba(74, 158, 255, 0.1);
}

.condition-row td {
  padding: 8px;
}

.name {
  color: var(--color-primary, #4a9eff);
  font-weight: 500;
}

.type-badge {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 0.85em;
  font-weight: 500;
}

.type-badge.condition {
  background: rgba(74, 158, 255, 0.2);
  color: var(--color-primary, #4a9eff);
}

.type-badge.disease {
  background: rgba(255, 107, 107, 0.2);
  color: #ff6b6b;
}

.type-badge.unknown {
  background: rgba(128, 128, 128, 0.2);
  color: #888;
}

.description {
  font-size: 0.9em;
  color: var(--color-text-secondary, #999);
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