<template>
  <div class="catalog-table">
    <div class="catalog-table__header">
      <h2 class="catalog-table__title">Classes</h2>
    </div>
    
    <div class="catalog-table__content">
      <div class="catalog-table__results-info">
        <span class="catalog-table__result-count">{{ sortedClasses.length }} classes</span>
      </div>
      
      <div class="catalog-table__scroll-container">
        <table class="catalog-table__table">
          <thead>
            <tr>
              <th>
                <div class="catalog-table__sort-header" @click="$emit('sort', 'name')">
                  Name
                  <span class="catalog-table__sort-icon">{{ sortIndicator('name') }}</span>
                </div>
              </th>
              <th>Hit Dice</th>
              <th>Primary Ability</th>
              <th>Proficiencies</th>
              <th>Spellcasting</th>
              <th>Subclass</th>
              <th>Source</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="sortedClasses.length === 0" class="catalog-table__empty-row">
              <td colspan="7">
                <div class="catalog-table__empty">
                  <h3 v-if="searchPerformed">No classes found</h3>
                  <h3 v-else>Search for classes</h3>
                  <p v-if="searchPerformed">No classes found matching your criteria</p>
                  <p v-else>Search for classes to see results</p>
                </div>
              </td>
            </tr>
            <tr
              v-for="classItem in sortedClasses"
              :key="`${classItem.name}-${classItem.source}`"
              class="catalog-table__row"
              @click="$emit('select', classItem)"
            >
              <td>
                <div class="catalog-table__cell-name">{{ classItem.name }}</div>
              </td>
              <td>{{ classItem.hitDice }}</td>
              <td>{{ classItem.primaryAbility }}</td>
              <td>{{ classItem.proficiency }}</td>
              <td>
                <span v-if="classItem.spellcastingAbility" class="catalog-table__cell-badge catalog-table__cell-badge--primary">
                  {{ formatSpellcasting(classItem.spellcastingAbility) }}
                </span>
                <span v-else class="catalog-table__cell-text-dim">—</span>
              </td>
              <td>
                <span v-if="classItem.subclassTitle">{{ classItem.subclassTitle }}</span>
                <span v-else class="catalog-table__cell-text-dim">—</span>
              </td>
              <td>{{ classItem.source }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ClassSummary } from '../../composables/useCatalog'

interface Props {
  classes: ClassSummary[]
  searchPerformed: boolean
  sortColumn: string
  sortDirection: 'asc' | 'desc'
  availableSources?: string[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  select: [classItem: ClassSummary]
  sort: [column: string]
}>()

const sortedClasses = computed(() => {
  let filtered = [...props.classes]
  
  // Apply sorting
  if (!['name', 'source', 'hitDice'].includes(props.sortColumn)) {
    return filtered
  }
  
  const sorted = [...filtered]
  sorted.sort((a, b) => {
    const aVal = a[props.sortColumn as keyof ClassSummary] as string
    const bVal = b[props.sortColumn as keyof ClassSummary] as string
    const comparison = aVal.localeCompare(bVal)
    return props.sortDirection === 'asc' ? comparison : -comparison
  })
  
  return sorted
})

function formatSpellcasting(ability: string): string {
  // Format spellcasting ability (e.g., "int" -> "INT")
  return ability.toUpperCase()
}

function sortIndicator(column: string) {
  if (props.sortColumn !== column) return ''
  return props.sortDirection === 'asc' ? '▲' : '▼'
}
</script>

<!-- All styling now handled by consolidated CSS classes -->
