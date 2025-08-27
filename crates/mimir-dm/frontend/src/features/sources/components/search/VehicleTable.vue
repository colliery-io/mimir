<template>
  <div class="vehicle-table-container">
    <table class="vehicle-table">
      <thead>
        <tr>
          <th @click="emit('sort', 'name')" class="sortable">
            Name
            <span class="sort-indicator" v-if="sortColumn === 'name'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th @click="emit('sort', 'vehicle_type')" class="sortable">
            Type
            <span class="sort-indicator" v-if="sortColumn === 'vehicle_type'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
          <th>Size</th>
          <th>Crew/Passengers</th>
          <th>Speed/Pace</th>
          <th>Terrain</th>
          <th @click="emit('sort', 'source')" class="sortable">
            Source
            <span class="sort-indicator" v-if="sortColumn === 'source'">
              {{ sortDirection === 'asc' ? '▲' : '▼' }}
            </span>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="vehicle in vehicles" :key="`${vehicle.name}-${vehicle.source}`" 
            @click="emit('select', vehicle)" class="clickable-row">
          <td class="name-cell">{{ vehicle.name }}</td>
          <td class="type-cell">
            <span :class="['type-badge', getTypeClass(vehicle.vehicle_type)]">
              {{ vehicle.vehicle_type || 'Unknown' }}
            </span>
          </td>
          <td class="size-cell">{{ formatSize(vehicle.size) }}</td>
          <td class="capacity-cell">
            <span v-if="vehicle.cap_crew || vehicle.cap_passenger">
              {{ vehicle.cap_crew || 0 }} / {{ vehicle.cap_passenger || 0 }}
            </span>
            <span v-else>—</span>
          </td>
          <td class="speed-cell">{{ vehicle.speed || '—' }}</td>
          <td class="terrain-cell">
            <span v-if="vehicle.terrain" class="terrain-list">
              {{ vehicle.terrain.join(', ') }}
            </span>
            <span v-else>—</span>
          </td>
          <td class="source-cell">{{ vehicle.source }}</td>
        </tr>
      </tbody>
    </table>
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
.vehicle-table-container {
  width: 100%;
  overflow-x: auto;
}

.vehicle-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9rem;
}

.vehicle-table th {
  text-align: left;
  padding: var(--spacing-sm, 8px);
  border-bottom: 2px solid var(--color-border, #333);
  background: var(--color-surface, #1a1a1a);
  color: var(--color-text-secondary, #999);
  white-space: nowrap;
}

.vehicle-table th.sortable {
  cursor: pointer;
  user-select: none;
}

.vehicle-table th.sortable:hover {
  color: var(--color-text, #e0e0e0);
}

.sort-indicator {
  display: inline-block;
  margin-left: 4px;
  font-size: 0.8em;
}

.vehicle-table tbody tr {
  border-bottom: 1px solid var(--color-border-light, #262626);
  transition: background-color 0.15s ease;
}

.vehicle-table tbody tr:hover {
  background: var(--color-surface-hover, #262626);
}

.clickable-row {
  cursor: pointer;
}

.vehicle-table td {
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
  background: var(--color-surface, #1a1a1a);
  color: var(--color-text-secondary, #999);
  border: 1px solid var(--color-border, #333);
}

.size-cell {
  text-align: center;
  color: var(--color-text-secondary, #999);
}

.capacity-cell {
  text-align: center;
  font-family: monospace;
  font-size: 0.85rem;
  color: var(--color-accent, #ff6b6b);
}

.speed-cell {
  font-size: 0.85rem;
  color: var(--color-text-secondary, #999);
}

.terrain-cell {
  font-size: 0.85rem;
  color: var(--color-text-secondary, #999);
}

.terrain-list {
  text-transform: capitalize;
}

.source-cell {
  color: var(--color-text-secondary, #999);
  font-size: 0.85rem;
  white-space: nowrap;
}
</style>