<template>
  <MainLayout>
    <div class="settings-view">
      <h1 class="page-title">Settings</h1>
      
      <div class="settings-layout">
        <!-- Sidebar Navigation -->
        <nav class="settings-sidebar">
          <div class="sidebar-section">
            <h3 class="sidebar-section-title">Admin Tools</h3>
            <ul class="sidebar-nav">
              <li>
                <button 
                  @click="activeSection = 'manage-campaigns'"
                  :class="['nav-item', { active: activeSection === 'manage-campaigns' }]"
                >
                  Manage Campaigns
                </button>
              </li>
              <li>
                <button 
                  @click="activeSection = 'import-books'"
                  :class="['nav-item', { active: activeSection === 'import-books' }]"
                >
                  Import Books
                </button>
              </li>
            </ul>
          </div>
          
          <div class="sidebar-section">
            <h3 class="sidebar-section-title">AI Assistant</h3>
            <ul class="sidebar-nav">
              <li>
                <button 
                  @click="activeSection = 'system-prompt'"
                  :class="['nav-item', { active: activeSection === 'system-prompt' }]"
                >
                  System Prompt
                </button>
              </li>
            </ul>
          </div>
          
          <div class="sidebar-section">
            <h3 class="sidebar-section-title">Application</h3>
            <ul class="sidebar-nav">
              <li>
                <button 
                  @click="activeSection = 'theme'"
                  :class="['nav-item', { active: activeSection === 'theme' }]"
                >
                  Theme
                </button>
              </li>
            </ul>
          </div>
        </nav>
        
        <!-- Content Area -->
        <main class="settings-content">
          <!-- System Prompt -->
          <div v-if="activeSection === 'system-prompt'" class="content-section">
            <h2 class="content-title">System Prompt</h2>
            <p class="content-description">Configure the AI assistant's behavior and instructions</p>
            <SystemPromptEditor
              :model-value="chatStore.systemConfig.baseInstructions || ''"
              @update:model-value="chatStore.setSystemInstructions"
            />
          </div>
          
          <!-- Theme -->
          <div v-else-if="activeSection === 'theme'" class="content-section">
            <h2 class="content-title">Theme</h2>
            <p class="content-description">Customize the application appearance</p>
            <div class="form-group">
              <ThemeSelector />
            </div>
          </div>
          
        </main>
      </div>
    </div>
    
    <!-- Book Management Modal -->
    <BookManagementModal 
      :visible="showBookManagementModal"
      @close="handleBookModalClose"
    />
    
    <!-- Campaign Management Modal -->
    <CampaignManagementModal 
      :visible="showCampaignManagementModal"
      @close="handleCampaignModalClose"
    />
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import MainLayout from '../shared/components/layout/MainLayout.vue'
import ThemeSelector from '../shared/components/ui/ThemeSelector.vue'
import SystemPromptEditor from '@/components/SystemPromptEditor.vue'
import BookManagementModal from '@/components/BookManagementModal.vue'
import CampaignManagementModal from '@/components/CampaignManagementModal.vue'
import { useChatStore } from '@/stores/chat'

const chatStore = useChatStore()
const showBookManagementModal = ref(false)
const showCampaignManagementModal = ref(false)
const activeSection = ref('theme')

// Open modals based on section selection
watch(activeSection, (newSection) => {
  if (newSection === 'import-books') {
    showBookManagementModal.value = true
  } else if (newSection === 'manage-campaigns') {
    showCampaignManagementModal.value = true
  }
})

// When modals close, switch to a different section (theme)  
const handleBookModalClose = () => {
  showBookManagementModal.value = false
  activeSection.value = 'theme'
}

const handleCampaignModalClose = () => {
  showCampaignManagementModal.value = false
  activeSection.value = 'theme'
}

</script>

<style scoped>
.settings-view {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.page-title {
  font-size: 2rem;
  font-weight: 700;
  color: var(--color-text);
  margin-bottom: var(--spacing-xl);
}

.settings-layout {
  flex: 1;
  display: flex;
  gap: var(--spacing-xl);
  min-height: 0;
}

/* Sidebar Navigation */
.settings-sidebar {
  width: 240px;
  background: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  height: fit-content;
  flex-shrink: 0;
}

.sidebar-section {
  margin-bottom: var(--spacing-xl);
}

.sidebar-section:last-child {
  margin-bottom: 0;
}

.sidebar-section-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: var(--spacing-md);
}

.sidebar-nav {
  list-style: none;
  margin: 0;
  padding: 0;
}

.sidebar-nav li {
  margin-bottom: var(--spacing-xs);
}

.nav-item {
  display: block;
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background: none;
  border: none;
  border-radius: var(--radius-md);
  color: var(--color-text);
  font-weight: 500;
  text-align: left;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.nav-item:hover {
  background: var(--color-gray-100);
  color: var(--color-text);
}

.nav-item.active {
  background: var(--color-primary-100);
  color: var(--color-primary-700);
}

.theme-dark .nav-item:hover {
  background: var(--color-gray-700);
}

.theme-dark .nav-item.active {
  background: var(--color-primary-900);
  color: var(--color-primary-300);
}

/* Content Area */
.settings-content {
  flex: 1;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
  min-height: 0;
  overflow-y: auto;
}

.content-section {
  max-width: 600px;
}

.content-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: var(--spacing-sm);
}

.content-description {
  font-size: 1rem;
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-xl);
  line-height: 1.5;
}

.action-button {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  background: var(--color-primary-500);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  font-weight: 500;
  cursor: pointer;
  transition: background-color var(--transition-fast);
}

.action-button:hover {
  background: var(--color-primary-600);
}

/* Form Elements */
.form-group {
  margin-bottom: var(--spacing-lg);
}

.form-label {
  display: block;
  font-weight: 500;
  color: var(--color-text);
  margin-bottom: var(--spacing-sm);
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