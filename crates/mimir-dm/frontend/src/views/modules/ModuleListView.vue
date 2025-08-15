<template>
  <MainLayout>
    <div class="module-list-view">
      <div class="header">
        <h1>Modules</h1>
        <button @click="showCreateModal = true" class="btn btn-primary">
          + New Module
        </button>
      </div>
      
      <!-- Create Module Modal -->
      <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
        <div class="modal-content">
          <h2>Create New Module</h2>
          <div class="form-group">
            <label for="module-name">Module Name:</label>
            <input 
              id="module-name"
              v-model="newModuleName" 
              type="text" 
              placeholder="Enter module name"
              @keyup.enter="confirmCreateModule"
            />
          </div>
          <div class="form-group">
            <label for="module-type">Module Type:</label>
            <select id="module-type" v-model="newModuleType">
              <option value="standard">Standard Adventure</option>
              <option value="mystery">Mystery</option>
              <option value="dungeon">Dungeon Crawl</option>
              <option value="heist">Heist</option>
              <option value="horror">Horror</option>
              <option value="political">Political Intrigue</option>
            </select>
          </div>
          <div class="form-group">
            <label for="module-sessions">Expected Sessions:</label>
            <input 
              id="module-sessions"
              v-model.number="newModuleSessions" 
              type="number" 
              min="1"
              placeholder="4"
              @keyup.enter="confirmCreateModule"
            />
          </div>
          <div class="modal-actions">
            <button @click="showCreateModal = false" class="btn btn-secondary">
              Cancel
            </button>
            <button @click="confirmCreateModule" class="btn btn-primary">
              Create Module
            </button>
          </div>
        </div>
      </div>

      <div v-if="loading" class="loading-state">
        Loading modules...
      </div>

      <div v-else-if="modules.length === 0" class="empty-state">
        <p>No modules yet. Create your first module to get started!</p>
      </div>

      <div v-else class="modules-grid">
        <div v-for="module in modules" :key="module.id" class="module-card">
          <div class="module-header">
            <h3>Module #{{ module.module_number }}: {{ module.name }}</h3>
            <span class="status-badge" :class="module.status">
              {{ formatStatus(module.status) }}
            </span>
          </div>
          
          <div class="module-info">
            <div class="info-item">
              <span class="label">Sessions:</span>
              <span class="value">{{ module.actual_sessions }} / {{ module.expected_sessions }}</span>
            </div>
            <div v-if="module.started_at" class="info-item">
              <span class="label">Started:</span>
              <span class="value">{{ formatDate(module.started_at) }}</span>
            </div>
            <div v-if="getProgress(module) > 0" class="progress-bar">
              <div class="progress-fill" :style="{ width: getProgress(module) + '%' }"></div>
            </div>
          </div>

          <div class="module-actions">
            <router-link :to="`/modules/${module.id}/board`" class="btn btn-primary">
              Open Board
            </router-link>
            <button @click="deleteModule(module.id)" class="btn btn-danger">
              Delete
            </button>
          </div>
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import MainLayout from '../../components/layout/MainLayout.vue'

const route = useRoute()
const campaignId = parseInt(route.params.id as string)

// Log for debugging
console.log('ModuleListView - Campaign ID from route:', route.params.id, 'Parsed:', campaignId)

interface Module {
  id: number
  campaign_id: number
  name: string
  module_number: number
  status: string
  expected_sessions: number
  actual_sessions: number
  created_at: string
  started_at: string | null
  completed_at: string | null
}

const modules = ref<Module[]>([])
const loading = ref(false)
const showCreateModal = ref(false)
const newModuleName = ref('')
const newModuleType = ref('standard')
const newModuleSessions = ref(4)

const loadModules = async () => {
  loading.value = true
  console.log('Loading modules for campaign:', campaignId)
  try {
    const response = await invoke<{ data: Module[] }>('list_campaign_modules', {
      request: {
        campaign_id: campaignId
      }
    })
    console.log('Modules loaded:', response)
    modules.value = response.data || []
    console.log('Modules state updated:', modules.value)
  } catch (error) {
    console.error('Failed to load modules:', error)
  } finally {
    loading.value = false
  }
}

const confirmCreateModule = async () => {
  console.log('confirmCreateModule called')
  console.log('Module name:', newModuleName.value)
  console.log('Module sessions:', newModuleSessions.value)
  console.log('Campaign ID:', campaignId)
  
  if (!newModuleName.value.trim()) {
    console.log('No module name provided')
    console.warn('Please enter a module name')
    return
  }
  
  if (newModuleSessions.value < 1) {
    console.log('Invalid session count:', newModuleSessions.value)
    console.warn('Expected sessions must be at least 1')
    return
  }
  
  if (isNaN(campaignId)) {
    console.log('Invalid campaign ID:', campaignId)
    console.warn('Invalid campaign ID. Please navigate from a campaign.')
    return
  }
  
  try {
    console.log('Invoking create_module with:', { 
      campaign_id: campaignId, 
      name: newModuleName.value, 
      expected_sessions: newModuleSessions.value,
      module_type: newModuleType.value
    })
    
    const response = await invoke('create_module', {
      request: {
        campaign_id: campaignId,
        name: newModuleName.value,
        expected_sessions: newModuleSessions.value,
        module_type: newModuleType.value
      }
    })
    
    console.log('Module created successfully:', response)
    
    // Reset form and close modal
    newModuleName.value = ''
    newModuleType.value = 'standard'
    newModuleSessions.value = 4
    showCreateModal.value = false
    
    // Reload modules
    console.log('Reloading modules after creation...')
    await loadModules()
    console.log('Modules after reload:', modules.value)
  } catch (error) {
    console.error('Failed to create module - Full error:', error)
    alert(`Failed to create module: ${error}`)
  }
}

const deleteModule = async (id: number) => {
  if (!confirm('Are you sure you want to delete this module?')) return
  
  try {
    await invoke('delete_module', { id })
    await loadModules()
  } catch (error) {
    console.error('Failed to delete module:', error)
    alert('Failed to delete module')
  }
}

const formatStatus = (status: string): string => {
  return status.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase())
}

const formatDate = (dateString: string): string => {
  if (!dateString) return ''
  return new Date(dateString).toLocaleDateString()
}

const getProgress = (module: Module): number => {
  if (module.expected_sessions === 0) return 0
  return Math.round((module.actual_sessions / module.expected_sessions) * 100)
}

onMounted(() => {
  loadModules()
})
</script>

<style scoped>
.module-list-view {
  padding: var(--spacing-xl);
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-xl);
}

.header h1 {
  margin: 0;
}

.loading-state,
.empty-state {
  text-align: center;
  padding: var(--spacing-xl);
  color: var(--color-text-secondary);
}

.modules-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: var(--spacing-lg);
}

.module-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-lg);
}

.module-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--spacing-md);
}

.module-header h3 {
  margin: 0;
  font-size: 1.125rem;
}

.status-badge {
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
}

.status-badge.backlog {
  background-color: var(--color-gray-100);
  color: var(--color-gray-700);
}

.status-badge.planning {
  background-color: var(--color-info-bg);
  color: var(--color-info);
}

.status-badge.development {
  background-color: var(--color-warning-bg);
  color: var(--color-warning);
}

.status-badge.ready {
  background-color: var(--color-success-bg);
  color: var(--color-success);
}

.status-badge.active {
  background-color: var(--color-primary-100);
  color: var(--color-primary-600);
}

.status-badge.completed {
  background-color: var(--color-success);
  color: white;
}

.module-info {
  margin-bottom: var(--spacing-lg);
}

.info-item {
  display: flex;
  justify-content: space-between;
  margin-bottom: var(--spacing-sm);
}

.info-item .label {
  color: var(--color-text-secondary);
  font-size: 0.875rem;
}

.info-item .value {
  font-weight: 600;
}

.progress-bar {
  height: 8px;
  background-color: var(--color-surface-variant);
  border-radius: 4px;
  overflow: hidden;
  margin-top: var(--spacing-sm);
}

.progress-fill {
  height: 100%;
  background-color: var(--color-primary-400);
  transition: width var(--transition-base);
}

.module-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.btn {
  padding: var(--spacing-sm) var(--spacing-md);
  border: none;
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-base);
  text-decoration: none;
  display: inline-block;
}

.btn-primary {
  background-color: var(--color-primary-500);
  color: white;
}

.btn-primary:hover {
  background-color: var(--color-primary-600);
}

.btn-danger {
  background-color: var(--color-error);
  color: white;
}

.btn-danger:hover {
  background-color: var(--color-error-dark);
}

/* Modal Styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--color-surface);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
  width: 90%;
  max-width: 500px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
}

.modal-content h2 {
  margin: 0 0 var(--spacing-lg) 0;
  color: var(--color-text);
}

.form-group {
  margin-bottom: var(--spacing-lg);
}

.form-group label {
  display: block;
  margin-bottom: var(--spacing-sm);
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  font-weight: 600;
}

.form-group input,
.form-group select {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--color-background);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  color: var(--color-text);
  font-size: 1rem;
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: var(--color-primary-400);
}

.modal-actions {
  display: flex;
  gap: var(--spacing-md);
  justify-content: flex-end;
  margin-top: var(--spacing-xl);
}

.btn-secondary {
  background-color: var(--color-surface-variant);
  color: var(--color-text);
  border: 1px solid var(--color-border);
}

.btn-secondary:hover {
  background-color: var(--color-surface);
  border-color: var(--color-primary-300);
}
</style>