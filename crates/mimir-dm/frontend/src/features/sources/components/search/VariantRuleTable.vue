<template>
  <div class="variant-rule-table-container">
    <table class="variant-rule-table">
      <thead>
        <tr>
          <th @click="emit('sort', 'name')" class="sortable">
            Name
            <span class="sort-indicator" v-if="sortColumn === 'name'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th @click="emit('sort', 'rule_type')" class="sortable">
            Type
            <span class="sort-indicator" v-if="sortColumn === 'rule_type'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th @click="emit('sort', 'source')" class="sortable">
            Source
            <span class="sort-indicator" v-if="sortColumn === 'source'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th>Page</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="rule in rules" :key="`${rule.name}-${rule.source}`" 
            @click="emit('select', rule)" class="clickable-row">
          <td class="name-cell">{{ rule.name }}</td>
          <td class="type-cell">
            <span :class="['type-badge', getTypeClass(rule.rule_type)]">
              {{ rule.rule_type || 'General' }}
            </span>
          </td>
          <td class="source-cell">{{ rule.source }}</td>
          <td class="page-cell">{{ rule.page ? `p. ${rule.page}` : '—' }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import type { VariantRuleSummary } from '../../composables/useCatalog'

defineProps<{
  rules: VariantRuleSummary[]
  sortColumn: string
  sortDirection: 'asc' | 'desc'
}>()

const emit = defineEmits<{
  sort: [column: string]
  select: [rule: VariantRuleSummary]
}>()

function getTypeClass(type: string | undefined): string {
  if (!type) return 'type-general'
  const normalized = type.toLowerCase().replace(/\s+/g, '-')
  return `type-${normalized}`
}
</script>

<style scoped>
.variant-rule-table-container {
  width: 100%;
  overflow-x: auto;
}

.variant-rule-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9rem;
}

.variant-rule-table th {
  text-align: left;
  padding: var(--spacing-sm, 8px);
  border-bottom: 2px solid var(--color-border, #333);
  background: var(--color-surface, #1a1a1a);
  color: var(--color-text-secondary, #999);
  white-space: nowrap;
}

.variant-rule-table th.sortable {
  cursor: pointer;
  user-select: none;
}

.variant-rule-table th.sortable:hover {
  color: var(--color-text, #e0e0e0);
}

.sort-indicator {
  display: inline-block;
  margin-left: 4px;
  font-size: 0.8em;
}

.variant-rule-table tbody tr {
  border-bottom: 1px solid var(--color-border-light, #262626);
  transition: background-color 0.15s ease;
}

.variant-rule-table tbody tr:hover {
  background: var(--color-surface-hover, #262626);
}

.clickable-row {
  cursor: pointer;
}

.variant-rule-table td {
  padding: var(--spacing-xs, 4px) var(--spacing-sm, 8px);
  color: var(--color-text, #e0e0e0);
}

/* Cell-specific styles */
.name-cell {
  font-weight: 500;
  color: var(--color-primary, #4a9eff);
}

.type-cell {
  white-space: nowrap;
}

.type-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 0.85rem;
  font-weight: 500;
}

.type-general {
  background: var(--color-surface, #1a1a1a);
  color: var(--color-text-secondary, #999);
  border: 1px solid var(--color-border, #333);
}

.type-action-options {
  background: rgba(255, 87, 34, 0.2);
  color: #ff5722;
  border: 1px solid rgba(255, 87, 34, 0.4);
}

.type-v {
  background: rgba(156, 39, 176, 0.2);
  color: #9c27b0;
  border: 1px solid rgba(156, 39, 176, 0.4);
}

.type-o {
  background: rgba(33, 150, 243, 0.2);
  color: #2196f3;
  border: 1px solid rgba(33, 150, 243, 0.4);
}

.source-cell {
  color: var(--color-text-secondary, #999);
  font-size: 0.85rem;
  white-space: nowrap;
}

.page-cell {
  color: var(--color-text-secondary, #999);
  font-size: 0.85rem;
  text-align: center;
}
</style>