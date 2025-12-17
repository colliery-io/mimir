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
              <li>
                <button 
                  @click="activeSection = 'logs'"
                  :class="['nav-item', { active: activeSection === 'logs' }]"
                >
                  Logs
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
              <li>
                <button
                  @click="activeSection = 'provider-config'"
                  :class="['nav-item', { active: activeSection === 'provider-config' }]"
                >
                  Provider Configuration
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
              <li>
                <button
                  @click="activeSection = 'about'"
                  :class="['nav-item', { active: activeSection === 'about' }]"
                >
                  About
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
          
          <!-- Provider Configuration -->
          <div v-else-if="activeSection === 'provider-config'" class="content-section">
            <h2 class="content-title">Provider Configuration</h2>
            <p class="content-description">Configure which LLM provider to use for AI assistant requests</p>

            <div class="form-group">
              <label for="provider-type" class="form-label">Provider</label>
              <select
                id="provider-type"
                class="form-input"
                v-model="providerSettings.provider_type"
                @change="handleProviderTypeChange"
              >
                <option value="ollama">Ollama (Local)</option>
                <option value="groq">Groq (Cloud)</option>
              </select>
              <p class="input-help">
                Choose between local Ollama installation or cloud-based Groq service.
              </p>
            </div>

            <!-- Ollama-specific settings -->
            <div v-if="providerSettings.provider_type === 'ollama'" class="form-group">
              <label for="ollama-base-url" class="form-label">Ollama Base URL</label>
              <input
                id="ollama-base-url"
                type="url"
                class="form-input"
                v-model="providerSettings.ollama_config.base_url"
                placeholder="http://localhost:11434"
              />
              <p class="input-help">
                Enter the URL where Ollama is running. Default is <code>http://localhost:11434</code>.
              </p>
            </div>

            <!-- Groq-specific settings -->
            <div v-if="providerSettings.provider_type === 'groq'" class="form-group">
              <label for="groq-api-key" class="form-label">Groq API Key</label>
              <input
                id="groq-api-key"
                type="password"
                class="form-input"
                v-model="providerSettings.groq_config.api_key"
                placeholder="Enter your Groq API key"
              />
              <p class="input-help">
                Your Groq API key. You can get one at <a href="https://console.groq.com" target="_blank">console.groq.com</a>.
              </p>
            </div>

            <!-- Model Selection (shown for both providers) -->
            <div class="form-group">
              <label for="model-input" class="form-label">Model</label>
              <input
                id="model-input"
                type="text"
                class="form-input"
                :value="getCurrentModel() || ''"
                @input="setCurrentModel(($event.target as HTMLInputElement).value || undefined)"
                :placeholder="providerSettings.provider_type === 'ollama' ? 'gpt-oss:20b' : 'openai/gpt-oss-120b'"
              />
              <p class="input-help">
                <template v-if="providerSettings.provider_type === 'ollama'">
                  Enter the name of an Ollama model (e.g., <code>gpt-oss:20b</code>, <code>qwen3:8b</code>, <code>llama3.2</code>).
                  Leave empty to use the default.
                </template>
                <template v-else>
                  Enter a Groq model name (e.g., <code>openai/gpt-oss-120b</code>, <code>llama-3.3-70b-versatile</code>).
                  Leave empty to use the default.
                </template>
              </p>
            </div>

            <div class="form-actions">
              <button
                @click="saveProviderSettings"
                class="button action-button"
                :disabled="isSavingSettings"
              >
                {{ isSavingSettings ? 'Saving...' : 'Save Settings' }}
              </button>
              <p v-if="settingsSaveMessage" :class="['settings-message', settingsSaveMessageType]">
                {{ settingsSaveMessage }}
              </p>
            </div>
          </div>
          
          <!-- Theme -->
          <div v-else-if="activeSection === 'theme'" class="content-section">
            <h2 class="content-title">Theme</h2>
            <p class="content-description">Customize the application appearance</p>
            <div class="form-group">
              <ThemeSelector />
            </div>
          </div>
          
          <!-- Logs -->
          <div v-else-if="activeSection === 'logs'" class="content-section">
            <h2 class="content-title">Application Logs</h2>
            <p class="content-description">View and monitor application log files</p>
            <LogsSection />
          </div>

          <!-- About -->
          <div v-else-if="activeSection === 'about'" class="content-section">
            <h2 class="content-title">About Mimir</h2>
            <p class="content-description">Application information</p>
            <div class="about-info">
              <div class="info-row">
                <span class="info-label">Version</span>
                <span class="info-value">{{ appVersion || 'Loading...' }}</span>
              </div>
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
import { ref, watch, onMounted, reactive } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getVersion } from '@tauri-apps/api/app'
import MainLayout from '../shared/components/layout/MainLayout.vue'
import ThemeSelector from '../shared/components/ui/ThemeSelector.vue'
import SystemPromptEditor from '@/components/SystemPromptEditor.vue'
import BookManagementModal from '@/components/BookManagementModal.vue'
import CampaignManagementModal from '@/components/CampaignManagementModal.vue'
import LogsSection from '@/components/LogsSection.vue'
import { useChatStore } from '@/stores/chat'

const chatStore = useChatStore()
const showBookManagementModal = ref(false)
const showCampaignManagementModal = ref(false)
const activeSection = ref('theme')
const appVersion = ref('')

// Provider settings state
interface OllamaConfig {
  base_url: string
  model?: string
}

interface GroqConfig {
  api_key: string
  model?: string
}

interface ProviderSettings {
  provider_type: 'ollama' | 'groq'
  ollama_config: OllamaConfig
  groq_config: GroqConfig
}

const providerSettings = reactive<ProviderSettings>({
  provider_type: 'ollama',
  ollama_config: {
    base_url: 'http://localhost:11434',
    model: undefined
  },
  groq_config: {
    api_key: '',
    model: undefined
  }
})


const isSavingSettings = ref(false)
const settingsSaveMessage = ref('')
const settingsSaveMessageType = ref<'success' | 'error'>('success')

// Load provider settings and app version on mount
onMounted(async () => {
  try {
    const settings = await invoke<ProviderSettings>('get_provider_settings')

    // Update reactive state
    providerSettings.provider_type = settings.provider_type

    if (settings.ollama_config) {
      providerSettings.ollama_config = settings.ollama_config
    }

    if (settings.groq_config) {
      providerSettings.groq_config = settings.groq_config
    }
  } catch (error) {
    console.error('Failed to load provider settings:', error)
  }

  try {
    appVersion.value = await getVersion()
  } catch (error) {
    console.error('Failed to get app version:', error)
    appVersion.value = 'Unknown'
  }
})

// Save provider settings
const saveProviderSettings = async () => {
  isSavingSettings.value = true
  settingsSaveMessage.value = ''

  try {
    // Build the settings object to send
    const settingsToSave: any = {
      provider_type: providerSettings.provider_type
    }

    if (providerSettings.provider_type === 'ollama') {
      settingsToSave.ollama_config = providerSettings.ollama_config
      settingsToSave.groq_config = null
    } else {
      settingsToSave.groq_config = providerSettings.groq_config
      settingsToSave.ollama_config = null
    }

    // Save settings
    await invoke('save_provider_settings', { settings: settingsToSave })

    // Reload LLM service to apply changes immediately
    await invoke('reload_llm_service')

    settingsSaveMessage.value = 'Settings saved and applied successfully!'
    settingsSaveMessageType.value = 'success'

    // Clear message after 5 seconds
    setTimeout(() => {
      settingsSaveMessage.value = ''
    }, 5000)
  } catch (error) {
    console.error('Failed to save provider settings:', error)
    settingsSaveMessage.value = `Failed to save settings: ${error}`
    settingsSaveMessageType.value = 'error'
  } finally {
    isSavingSettings.value = false
  }
}

// Get the current model for the active provider
const getCurrentModel = () => {
  if (providerSettings.provider_type === 'ollama') {
    return providerSettings.ollama_config.model
  } else {
    return providerSettings.groq_config.model
  }
}

// Set the current model for the active provider
const setCurrentModel = (model: string | undefined) => {
  if (providerSettings.provider_type === 'ollama') {
    providerSettings.ollama_config.model = model
  } else {
    providerSettings.groq_config.model = model
  }
}

// Handle provider type change
const handleProviderTypeChange = () => {
  // Initialize default values when switching providers
  if (providerSettings.provider_type === 'ollama' && !providerSettings.ollama_config.base_url) {
    providerSettings.ollama_config.base_url = 'http://localhost:11434'
  }
}

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
  max-width: 1200px;
  width: 100%;
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

.input-help {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  margin-top: var(--spacing-sm);
  line-height: 1.4;
}

.input-help code {
  background-color: var(--color-gray-100);
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-family: ui-monospace, 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
  font-size: 0.8125rem;
}

.theme-dark .input-help code {
  background-color: var(--color-gray-700);
}

.input-help a {
  color: var(--color-primary-500);
  text-decoration: underline;
}

.input-help a:hover {
  color: var(--color-primary-600);
}

.form-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  margin-top: var(--spacing-xl);
}

.settings-message {
  font-size: 0.875rem;
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  margin: 0;
}

.settings-message.success {
  background-color: var(--color-success-100);
  color: var(--color-success-700);
  border: 1px solid var(--color-success-300);
}

.settings-message.error {
  background-color: var(--color-error-100);
  color: var(--color-error-700);
  border: 1px solid var(--color-error-300);
}

.theme-dark .settings-message.success {
  background-color: var(--color-success-900);
  color: var(--color-success-300);
  border-color: var(--color-success-700);
}

.theme-dark .settings-message.error {
  background-color: var(--color-error-900);
  color: var(--color-error-300);
  border-color: var(--color-error-700);
}

button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

select.form-input {
  cursor: pointer;
}

/* About Section */
.about-info {
  background: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-lg);
}

.info-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-sm) 0;
}

.info-label {
  font-weight: 500;
  color: var(--color-text-secondary);
}

.info-value {
  font-family: ui-monospace, 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
  color: var(--color-text);
}
</style>