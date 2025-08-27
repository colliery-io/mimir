<template>
  <div class="action-table-container">
    <table class="action-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Time</th>
          <th>Description</th>
          <th>See Also</th>
          <th>Source</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="action in actions" :key="`${action.name}-${action.source}`"
            @click="emit('select', action)"
            class="action-row">
          <td class="name">{{ action.name }}</td>
          <td class="time">{{ action.time }}</td>
          <td class="description">{{ action.description }}</td>
          <td class="see-also">
            <span v-if="action.see_also && action.see_also.length > 0">
              {{ action.see_also.join(', ') }}
            </span>
            <span v-else class="none">—</span>
          </td>
          <td class="source">{{ action.source }}</td>
        </tr>
      </tbody>
    </table>
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

<style scoped>
.action-table-container {
  width: 100%;
  overflow-x: auto;
}

.action-table {
  width: 100%;
  border-collapse: collapse;
}

.action-table th {
  text-align: left;
  padding: 8px;
}

.action-row {
  cursor: pointer;
}

.action-row:hover {
  background: rgba(74, 158, 255, 0.1);
}

.action-row td {
  padding: 8px;
}

.name {
  color: var(--color-primary, #4a9eff);
  font-weight: 500;
}

.time {
  white-space: nowrap;
}

.description {
  font-size: 0.9em;
  color: var(--color-text-secondary, #999);
}

.see-also {
  font-size: 0.85em;
}

.see-also .none {
  color: var(--color-text-secondary, #999);
}

.source {
  font-size: 0.85em;
  color: var(--color-text-secondary, #999);
}
</style>