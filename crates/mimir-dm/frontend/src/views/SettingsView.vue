<template>
  <MainLayout>
    <div class="settings-view">
      <h1 class="page-title">Settings</h1>
      
      <div class="settings-sections">
        <!-- Administrative Tools Section -->
        <section class="settings-section">
          <h2 class="section-title">Administrative Tools</h2>
          <div class="settings-cards">
            <div class="settings-card" @click="handleImportRuleSets">
              <div class="card-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                  <polyline points="7 10 12 15 17 10"></polyline>
                  <line x1="12" y1="15" x2="12" y2="3"></line>
                </svg>
              </div>
              <h3 class="card-title">Import Books</h3>
              <p class="card-description">Import D&D rule sets, modules, and content from book archives</p>
            </div>
            
            <div class="settings-card" @click="handleManageTemplates">
              <div class="card-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
                  <line x1="9" y1="9" x2="15" y2="9"></line>
                  <line x1="9" y1="13" x2="15" y2="13"></line>
                  <line x1="9" y1="17" x2="11" y2="17"></line>
                </svg>
              </div>
              <h3 class="card-title">Manage Templates</h3>
              <p class="card-description">Create and manage campaign and session templates</p>
            </div>
            
            <div class="settings-card" @click="handleDatabaseManagement">
              <div class="card-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <ellipse cx="12" cy="5" rx="9" ry="3"></ellipse>
                  <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"></path>
                  <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"></path>
                </svg>
              </div>
              <h3 class="card-title">Database Management</h3>
              <p class="card-description">Backup, restore, and manage your campaign database</p>
            </div>
          </div>
        </section>
        
        <!-- AI Assistant Configuration Section -->
        <section class="settings-section">
          <h2 class="section-title">AI Assistant Configuration</h2>
          
          <SystemPromptEditor
            :model-value="chatStore.systemConfig.baseInstructions || ''"
            @update:model-value="chatStore.setSystemInstructions"
          />
          
        </section>
        
        <!-- Application Settings Section -->
        <section class="settings-section">
          <h2 class="section-title">Application Settings</h2>
          <div class="settings-form">
            <div class="form-group">
              <label class="form-label">Default Campaign Directory</label>
              <div class="input-group">
                <input 
                  type="text" 
                  v-model="defaultCampaignDir" 
                  class="form-input"
                  readonly
                />
                <button @click="selectDefaultDirectory" class="button button-secondary">
                  Browse
                </button>
              </div>
            </div>
            
            <div class="form-group">
              <label class="form-label">Theme</label>
              <ThemeSelector />
            </div>
          </div>
        </section>
      </div>
    </div>
    
    <!-- Book Management Modal -->
    <BookManagementModal 
      :visible="showBookManagementModal"
      @close="showBookManagementModal = false"
    />
  </MainLayout>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { open } from '@tauri-apps/plugin-dialog'
import MainLayout from '../shared/components/layout/MainLayout.vue'
import ThemeSelector from '../shared/components/ui/ThemeSelector.vue'
import SystemPromptEditor from '@/components/SystemPromptEditor.vue'
import BookManagementModal from '@/components/BookManagementModal.vue'
import { useChatStore } from '@/stores/chat'

const router = useRouter()
const chatStore = useChatStore()
const defaultCampaignDir = ref('')
const showBookManagementModal = ref(false)

const handleImportRuleSets = () => {
  showBookManagementModal.value = true
}

const handleManageTemplates = () => {
  router.push('/templates')
}

const handleDatabaseManagement = () => {
  // TODO: Navigate to database management page
  router.push('/settings/database')
}

const selectDefaultDirectory = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Select Default Campaign Directory'
  })
  
  if (selected && typeof selected === 'string') {
    defaultCampaignDir.value = selected
    // TODO: Save to application settings
  }
}

</script>

<style scoped>
.settings-view {
  @apply space-y-8;
}

.page-title {
  font-size: 2rem;
  font-weight: 700;
  color: var(--color-text);
  margin-bottom: var(--spacing-xl);
}

.settings-sections {
  @apply space-y-12;
}

.settings-section {
  @apply space-y-4;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: var(--spacing-lg);
}

.settings-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--spacing-lg);
}

.settings-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  cursor: pointer;
  transition: all var(--transition-base);
  text-align: center;
}

.settings-card:hover {
  border-color: var(--color-primary-500);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.card-icon {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 48px;
  height: 48px;
  margin: 0 auto var(--spacing-md);
  background-color: var(--color-primary-100);
  border-radius: var(--radius-lg);
  color: var(--color-primary-600);
}

.theme-dark .card-icon {
  background-color: var(--color-primary-900);
  color: var(--color-primary-400);
}

.card-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: var(--spacing-sm);
}

.card-description {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  line-height: 1.5;
}

.settings-form {
  @apply space-y-6;
  max-width: 600px;
}

.form-group {
  @apply space-y-2;
}

.form-label {
  display: block;
  font-weight: 500;
  color: var(--color-text);
  margin-bottom: var(--spacing-sm);
}

.form-help {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.form-input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  color: var(--color-text);
  transition: border-color var(--transition-fast);
}

.form-input:focus {
  outline: none;
  border-color: var(--color-primary-500);
}

.input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.button {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  border: none;
}

.button-secondary {
  background-color: var(--color-surface-variant);
  color: var(--color-text);
  border: 1px solid var(--color-border);
}

.button-secondary:hover {
  background-color: var(--color-gray-200);
  border-color: var(--color-border-hover);
}

.theme-dark .button-secondary:hover {
  background-color: var(--color-gray-700);
}

</style>