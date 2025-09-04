<template>
  <div class="catalog-table">
    <div class="catalog-table__content">
      <table class="catalog-table__table">
        <thead class="catalog-table__header">
          <tr>
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'name')">
              Name
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'name'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'rule_type')">
              Type
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'rule_type'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'source')">
              Source
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'source'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th">Page</th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr v-for="rule in rules" :key="`${rule.name}-${rule.source}`" 
              class="catalog-table__row catalog-table__row--clickable"
              @click="emit('select', rule)">
            <td class="catalog-table__td catalog-table__name">{{ rule.name }}</td>
            <td class="catalog-table__td">
              <span :class="['catalog-table__badge', getTypeClass(rule.rule_type)]">
                {{ rule.rule_type || 'General' }}
              </span>
            </td>
            <td class="catalog-table__td catalog-table__source">{{ rule.source }}</td>
            <td class="catalog-table__td catalog-table__page">{{ rule.page ? `p. ${rule.page}` : '—' }}</td>
          </tr>
        </tbody>
      </table>
    </div>
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
/* Custom variant rule type colors */
.type-general {
  background: var(--color-surface);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
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
</style>