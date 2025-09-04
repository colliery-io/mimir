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
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'vehicle_type')">
              Type
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'vehicle_type'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
            <th class="catalog-table__th">Size</th>
            <th class="catalog-table__th">Crew/Passengers</th>
            <th class="catalog-table__th">Speed/Pace</th>
            <th class="catalog-table__th">Terrain</th>
            <th class="catalog-table__th catalog-table__th--sortable" @click="emit('sort', 'source')">
              Source
              <span class="catalog-table__sort-indicator" v-if="sortColumn === 'source'">
                {{ sortDirection === 'asc' ? '▲' : '▼' }}
              </span>
            </th>
          </tr>
        </thead>
        <tbody class="catalog-table__body">
          <tr v-for="vehicle in vehicles" :key="`${vehicle.name}-${vehicle.source}`" 
              class="catalog-table__row catalog-table__row--clickable" 
              @click="emit('select', vehicle)">
            <td class="catalog-table__td catalog-table__name">{{ vehicle.name }}</td>
            <td class="catalog-table__td">
              <span :class="['catalog-table__badge', getTypeClass(vehicle.vehicle_type)]">
                {{ vehicle.vehicle_type || 'Unknown' }}
              </span>
            </td>
            <td class="catalog-table__td catalog-table__center">{{ formatSize(vehicle.size) }}</td>
            <td class="catalog-table__td catalog-table__center">
              <span v-if="vehicle.cap_crew || vehicle.cap_passenger" class="capacity-display">
                {{ vehicle.cap_crew || 0 }} / {{ vehicle.cap_passenger || 0 }}
              </span>
              <span v-else class="catalog-table__empty">—</span>
            </td>
            <td class="catalog-table__td catalog-table__secondary">{{ vehicle.speed || '—' }}</td>
            <td class="catalog-table__td catalog-table__secondary">
              <span v-if="vehicle.terrain" class="terrain-list">
                {{ vehicle.terrain.join(', ') }}
              </span>
              <span v-else class="catalog-table__empty">—</span>
            </td>
            <td class="catalog-table__td catalog-table__source">{{ vehicle.source }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { VehicleSummary } from '../../composables/useCatalog'

defineProps<{
  vehicles: VehicleSummary[]
  sortColumn: string
  sortDirection: 'asc' | 'desc'
}>()

const emit = defineEmits<{
  sort: [column: string]
  select: [vehicle: VehicleSummary]
}>()

function getTypeClass(type: string | undefined): string {
  if (!type) return 'type-unknown'
  const normalized = type.toLowerCase()
  switch (normalized) {
    case 'ship':
      return 'type-ship'
    case 'object':
      return 'type-object'
    case 'infernal war machine':
      return 'type-infernal'
    default:
      return 'type-other'
  }
}

function formatSize(size: string | undefined): string {
  if (!size) return '—'
  switch (size) {
    case 'T': return 'Tiny'
    case 'S': return 'Small'
    case 'M': return 'Medium'
    case 'L': return 'Large'
    case 'H': return 'Huge'
    case 'G': return 'Gargantuan'
    default: return size
  }
}
</script>

<style scoped>
/* Custom vehicle type colors */
.type-ship {
  background: rgba(33, 150, 243, 0.2);
  color: #2196f3;
  border: 1px solid rgba(33, 150, 243, 0.4);
}

.type-object {
  background: rgba(156, 39, 176, 0.2);
  color: #9c27b0;
  border: 1px solid rgba(156, 39, 176, 0.4);
}

.type-infernal {
  background: rgba(244, 67, 54, 0.2);
  color: #f44336;
  border: 1px solid rgba(244, 67, 54, 0.4);
}

.type-other, .type-unknown {
  background: var(--color-surface);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
}

/* Vehicle-specific styling */
.capacity-display {
  font-family: monospace;
  color: var(--color-accent, #ff6b6b);
}

.terrain-list {
  text-transform: capitalize;
}
</style>