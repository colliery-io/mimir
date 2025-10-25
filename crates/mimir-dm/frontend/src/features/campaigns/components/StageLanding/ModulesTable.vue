<template>
  <div class="modules-section">
    <div class="section-header">
      <h3>{{ title }}</h3>
      <button @click="$emit('createModule')" class="btn btn-primary">
        New Module
      </button>
    </div>

    <div v-if="loading" class="loading-state">
      Loading modules...
    </div>

    <div v-else-if="modules.length === 0" class="empty-state">
      <p>{{ emptyMessage }}</p>
    </div>

    <table v-else class="modules-table">
      <thead>
        <tr>
          <th>Module</th>
          <th>Status</th>
          <th>Sessions</th>
          <th v-if="showProgress">Progress</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="module in modules" :key="module.id" class="module-row">
          <td class="module-name">
            <strong>Module #{{ module.module_number }}:</strong> {{ module.name }}
          </td>
          <td>
            <span class="status-badge" :class="module.status">
              {{ module.status }}
            </span>
          </td>
          <td class="sessions-info">
            {{ module.actual_sessions || 0 }} {{ showProgress ? `/ ${module.planned_sessions}` : 'sessions' }}
          </td>
          <td v-if="showProgress" class="progress-cell">
            <span v-if="module.progress_percentage > 0" class="progress-percentage">
              {{ module.progress_percentage }}%
            </span>
            <span v-else class="no-progress">—</span>
          </td>
          <td class="actions-cell">
            <router-link :to="`/modules/${module.id}/board`" class="btn btn-ghost btn-small">
              View →
            </router-link>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  modules: any[]
  loading: boolean
  title?: string
  emptyMessage?: string
  showProgress?: boolean
}>(), {
  title: 'Modules',
  emptyMessage: 'No modules yet. Create your first module to get started!',
  showProgress: true
})

defineEmits<{
  createModule: []
}>()
</script>
