<template>
  <div v-if="visible" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h2 class="modal-title">Manage Campaigns</h2>
        <button @click="closeModal" class="close-button">×</button>
      </div>
      
      <div class="modal-tabs">
        <button 
          :class="['tab-button', { active: activeTab === 'active' }]"
          @click="activeTab = 'active'"
        >
          Active Campaigns
        </button>
        <button 
          :class="['tab-button', { active: activeTab === 'archived' }]"
          @click="activeTab = 'archived'"
        >
          Archived Campaigns
        </button>
      </div>
      
      <div class="modal-body">
        <!-- Active Campaigns Tab -->
        <div v-if="activeTab === 'active'">
          <div v-if="isLoading" class="loading-message">
            Loading campaigns...
          </div>
          
          <div v-else-if="activeCampaigns.length === 0" class="empty-state">
            <p>No active campaigns</p>
            <p class="empty-subtitle">Create a new campaign to get started</p>
          </div>
          
          <div v-else class="campaign-list">
            <div v-for="campaign in activeCampaigns" :key="campaign.id" class="campaign-item">
              <div class="campaign-info">
                <div class="campaign-name">{{ campaign.name }}</div>
                <div class="campaign-meta">
                  <span class="campaign-status">{{ formatStatus(campaign.status) }}</span>
                  <span class="campaign-activity">
                    Last activity: {{ formatDate(campaign.last_activity_at) }}
                  </span>
                </div>
              </div>
              <button 
                @click="handleArchiveCampaign(campaign)" 
                class="archive-button"
                title="Archive campaign"
              >
                Archive
              </button>
            </div>
          </div>
        </div>

        <!-- Archived Campaigns Tab -->
        <div v-if="activeTab === 'archived'">
          <div v-if="isLoading" class="loading-message">
            Loading archived campaigns...
          </div>
          
          <div v-else-if="archivedCampaigns.length === 0" class="empty-state">
            <p>No archived campaigns</p>
            <p class="empty-subtitle">Archived campaigns will appear here</p>
          </div>
          
          <div v-else class="campaign-list">
            <div v-for="campaign in archivedCampaigns" :key="campaign.id" class="campaign-item archived">
              <div class="campaign-info">
                <div class="campaign-name">{{ campaign.name }}</div>
                <div class="campaign-meta">
                  <span class="campaign-status">{{ formatStatus(campaign.status) }}</span>
                  <span class="campaign-activity">
                    Archived: {{ formatDate(campaign.archived_at!) }}
                  </span>
                </div>
              </div>
              <div class="campaign-actions">
                <button 
                  @click="handleUnarchiveCampaign(campaign)" 
                  class="unarchive-button"
                  title="Restore campaign"
                >
                  Restore
                </button>
                <button 
                  @click="handleDeleteCampaign(campaign)" 
                  class="delete-button"
                  title="Delete campaign permanently"
                >
                  Delete
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Delete Confirmation Modal -->
  <div v-if="showDeleteModal" class="modal-overlay">
    <div class="modal-content delete-modal" @click.stop>
      <div class="modal-header">
        <h2 class="modal-title">Delete Campaign</h2>
      </div>
      <div class="modal-body">
        <p>Are you sure you want to permanently delete "<strong>{{ campaignToDelete?.name }}</strong>"?</p>
        <p class="warning-text">This action cannot be undone.</p>
        
        <div v-if="deleteError" class="error-message">
          {{ deleteError }}
        </div>
        
        <div class="delete-options">
          <label class="checkbox-label">
            <input 
              type="checkbox" 
              v-model="deleteFiles"
            />
            Also delete all campaign files and directories
          </label>
        </div>
      </div>
      <div class="modal-footer">
        <button @click="confirmDelete" class="delete-confirm-button">
          Delete Campaign
        </button>
        <button @click="cancelDelete" class="cancel-button">
          Cancel
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useCampaignStore } from '@/stores/campaigns'
import type { Campaign } from '@/types/api'

interface Props {
  visible: boolean
}

interface Emits {
  (e: 'close'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const campaignStore = useCampaignStore()
const activeTab = ref<'active' | 'archived'>('active')
const showDeleteModal = ref(false)
const campaignToDelete = ref<Campaign | null>(null)
const deleteFiles = ref(false)
const deleteError = ref<string | null>(null)

const activeCampaigns = computed(() => campaignStore.campaigns)
const archivedCampaigns = computed(() => campaignStore.archivedCampaigns)
const isLoading = computed(() => campaignStore.loading)

// Load campaigns when modal becomes visible
watch(() => props.visible, async (newVisible) => {
  if (newVisible) {
    await loadCampaigns()
  }
})

// Load campaigns when switching tabs
watch(activeTab, async (newTab) => {
  if (props.visible) {
    await loadCampaigns()
  }
})

async function loadCampaigns() {
  if (activeTab.value === 'active') {
    await campaignStore.fetchCampaigns()
  } else {
    await campaignStore.fetchArchivedCampaigns()
  }
}

async function handleArchiveCampaign(campaign: Campaign) {
  const success = await campaignStore.archiveCampaign(campaign.id)
  if (success) {
    // Refresh both lists
    await campaignStore.fetchCampaigns()
    await campaignStore.fetchArchivedCampaigns()
  } else {
    // Show error in a non-system way - could be improved with toast notifications
    console.error(`Failed to archive campaign: ${campaignStore.error}`)
  }
}

async function handleUnarchiveCampaign(campaign: Campaign) {
  const success = await campaignStore.unarchiveCampaign(campaign.id)
  if (success) {
    // Refresh both lists
    await campaignStore.fetchCampaigns()
    await campaignStore.fetchArchivedCampaigns()
  } else {
    // Show error in a non-system way - could be improved with toast notifications
    console.error(`Failed to restore campaign: ${campaignStore.error}`)
  }
}

function handleDeleteCampaign(campaign: Campaign) {
  campaignToDelete.value = campaign
  deleteFiles.value = false
  showDeleteModal.value = true
}

async function confirmDelete() {
  if (!campaignToDelete.value) return

  deleteError.value = null
  const success = await campaignStore.deleteCampaign(campaignToDelete.value.id, deleteFiles.value)
  if (success) {
    showDeleteModal.value = false
    campaignToDelete.value = null
    deleteFiles.value = false
    // Refresh archived list
    await campaignStore.fetchArchivedCampaigns()
  } else {
    deleteError.value = campaignStore.error || 'Failed to delete campaign'
  }
}

function cancelDelete() {
  showDeleteModal.value = false
  campaignToDelete.value = null
  deleteFiles.value = false
  deleteError.value = null
}

function closeModal() {
  emit('close')
}

function handleOverlayClick() {
  closeModal()
}

function formatStatus(status: string): string {
  return status.replace('_', ' ').split(' ')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString()
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--color-overlay);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  width: 90%;
  max-width: 700px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.delete-modal {
  max-width: 500px;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--color-border);
}

.modal-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text);
  margin: 0;
}

.close-button {
  background: none;
  border: none;
  font-size: 1.5rem;
  color: var(--color-text-secondary);
  cursor: pointer;
  padding: 4px;
  line-height: 1;
  transition: color var(--transition-fast);
}

.close-button:hover {
  color: var(--color-text);
}

.modal-tabs {
  display: flex;
  border-bottom: 1px solid var(--color-border);
}

.tab-button {
  flex: 1;
  padding: var(--spacing-md) var(--spacing-lg);
  background: none;
  border: none;
  font-weight: 500;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast);
  border-bottom: 2px solid transparent;
}

.tab-button:hover {
  color: var(--color-text);
  background: var(--color-surface-hover);
}

.tab-button.active {
  color: var(--color-primary);
  border-bottom-color: var(--color-primary);
  background: var(--color-surface-hover);
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-lg);
  min-height: 300px;
}

.loading-message {
  text-align: center;
  color: var(--color-text-secondary);
  padding: var(--spacing-xl) 0;
}

.empty-state {
  text-align: center;
  color: var(--color-text-secondary);
  padding: var(--spacing-xl) 0;
}

.empty-subtitle {
  font-size: 0.875rem;
  margin-top: var(--spacing-sm);
}

.campaign-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.campaign-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md);
  background: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
}

.campaign-item:hover {
  background: var(--color-gray-100);
}

.campaign-item.archived {
  opacity: 0.8;
}

.theme-dark .campaign-item:hover {
  background: var(--color-gray-800);
}

.campaign-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.campaign-name {
  font-weight: 500;
  color: var(--color-text);
  font-size: 1rem;
}

.campaign-meta {
  display: flex;
  flex-direction: column;
  gap: 2px;
  font-size: 0.75rem;
  color: var(--color-text-secondary);
}

.campaign-status {
  font-weight: 500;
  color: var(--color-primary);
}

.campaign-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.archive-button {
  background: var(--color-warning-100);
  color: var(--color-warning-700);
  border: 1px solid var(--color-warning-200);
  border-radius: var(--radius-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.archive-button:hover {
  background: var(--color-warning-200);
  color: var(--color-warning-800);
}

.unarchive-button {
  background: var(--color-success-100);
  color: var(--color-success-700);
  border: 1px solid var(--color-success-200);
  border-radius: var(--radius-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.unarchive-button:hover {
  background: var(--color-success-200);
  color: var(--color-success-800);
}

.delete-button {
  background: var(--color-error-100);
  color: var(--color-error-600);
  border: 1px solid var(--color-error-200);
  border-radius: var(--radius-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.delete-button:hover {
  background: var(--color-error-200);
  color: var(--color-error-700);
}

.theme-dark .archive-button {
  background: var(--color-warning-900);
  color: var(--color-warning-400);
  border-color: var(--color-warning-800);
}

.theme-dark .archive-button:hover {
  background: var(--color-warning-800);
  color: var(--color-warning-300);
}

.theme-dark .unarchive-button {
  background: var(--color-success-900);
  color: var(--color-success-400);
  border-color: var(--color-success-800);
}

.theme-dark .unarchive-button:hover {
  background: var(--color-success-800);
  color: var(--color-success-300);
}

.theme-dark .delete-button {
  background: var(--color-error-900);
  color: var(--color-error-400);
  border-color: var(--color-error-800);
}

.theme-dark .delete-button:hover {
  background: var(--color-error-800);
  color: var(--color-error-300);
}

.warning-text {
  color: var(--color-error-600);
  font-weight: 500;
  margin-top: var(--spacing-sm);
}

.error-message {
  background: var(--color-error-100);
  color: var(--color-error-700);
  border: 1px solid var(--color-error-200);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-md);
  margin: var(--spacing-md) 0;
  font-size: 0.875rem;
}

.theme-dark .error-message {
  background: var(--color-error-900);
  color: var(--color-error-300);
  border-color: var(--color-error-800);
}

.delete-options {
  margin: var(--spacing-lg) 0;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  cursor: pointer;
  color: var(--color-text);
}

.checkbox-label input[type="checkbox"] {
  cursor: pointer;
  margin: 0;
}

.modal-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--spacing-md);
  padding: var(--spacing-lg);
  border-top: 1px solid var(--color-border);
}

.delete-confirm-button {
  background: var(--color-error-500);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  font-weight: 500;
  cursor: pointer;
  transition: background-color var(--transition-fast);
}

.delete-confirm-button:hover {
  background: var(--color-error-600);
}

.cancel-button {
  background: var(--color-surface-variant);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.cancel-button:hover {
  background: var(--color-gray-200);
  border-color: var(--color-border-hover);
}

.theme-dark .cancel-button:hover {
  background: var(--color-gray-700);
}
</style>