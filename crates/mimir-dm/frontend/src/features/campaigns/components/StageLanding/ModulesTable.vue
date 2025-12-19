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
          <td class="actions-cell">
            <router-link
              :to="`/modules/${module.id}/play`"
              class="btn btn-play btn-small"
              :class="{ disabled: module.status !== 'ready' && module.status !== 'active' }"
            >
              Play
            </router-link>
            <router-link :to="`/modules/${module.id}/board`" class="btn btn-ghost btn-small">
              Open
            </router-link>
            <button
              class="btn btn-pdf btn-small"
              @click="exportModule(module)"
              :disabled="exportingModuleId === module.id"
              title="Export module as PDF"
            >
              {{ exportingModuleId === module.id ? '...' : 'PDF' }}
            </button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { PrintService } from '../../../../services/PrintService'

const props = withDefaults(defineProps<{
  modules: any[]
  loading: boolean
  title?: string
  emptyMessage?: string
}>(), {
  title: 'Modules',
  emptyMessage: 'No modules yet. Create your first module to get started!'
})

defineEmits<{
  createModule: []
}>()

const exportingModuleId = ref<number | null>(null)

const exportModule = async (module: any) => {
  if (!module?.id) return

  exportingModuleId.value = module.id

  try {
    const result = await PrintService.exportModuleDocuments(module.id)

    const filename = `Module_${module.module_number}_${module.name || 'module'}.pdf`
      .replace(/[^a-z0-9\s\-_.]/gi, '')
      .replace(/\s+/g, '_')

    await PrintService.savePdf(result, filename)
  } catch (e) {
    console.error('Failed to export module:', e)
  } finally {
    exportingModuleId.value = null
  }
}
</script>

<style scoped>
.modules-table {
  width: 100%;
  table-layout: fixed;
  border-collapse: collapse;
}

.modules-table th:nth-child(1) { width: 55%; } /* Module name */
.modules-table th:nth-child(2) { width: 15%; } /* Status */
.modules-table th:last-child { width: 30%; }   /* Actions */

.modules-table td {
  vertical-align: middle;
}

.actions-cell {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  justify-content: flex-end;
}

.actions-cell .btn {
  min-width: 4rem;
  height: 2rem;
  line-height: 2rem;
  padding: 0 0.75rem;
  text-align: center;
  box-sizing: border-box;
}

.btn-play {
  background: var(--color-accent, #e67e22);
  color: white;
  border: none;
  font-weight: 600;
}

.btn-play:hover:not(.disabled) {
  background: var(--color-accent-dark, #d35400);
}

.btn-play.disabled {
  background: var(--color-border, #ddd);
  color: var(--color-text-secondary, #999);
  cursor: not-allowed;
  pointer-events: none;
}

.btn-pdf {
  background: var(--color-secondary, #95a5a6);
  color: white;
  border: none;
}

.btn-pdf:hover {
  background: var(--color-secondary-dark, #7f8c8d);
}

.btn-pdf:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
