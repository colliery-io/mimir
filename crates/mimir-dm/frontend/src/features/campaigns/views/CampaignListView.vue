<template>
  <MainLayout>
    <div class="campaign-list-view">
      <div class="header">
        <h1 class="page-title">Campaigns</h1>
        <router-link to="/campaigns/new" class="btn-primary">
          New Campaign
        </router-link>
      </div>

      <div v-if="campaignStore.loading" class="loading">
        Loading campaigns...
      </div>

      <div v-else-if="campaignStore.error" class="error-message">
        {{ campaignStore.error }}
      </div>

      <div v-else-if="campaignStore.campaigns.length === 0" class="empty-state">
        <p>No campaigns found.</p>
        <router-link to="/campaigns/new" class="btn-primary">
          Create your first campaign
        </router-link>
      </div>

      <div v-else class="campaign-grid">
        <div
          v-for="campaign in campaignStore.campaigns"
          :key="campaign.id"
          class="campaign-card"
          @click="selectCampaign(campaign.id)"
        >
          <h3 class="campaign-name">{{ campaign.name }}</h3>
          <div class="campaign-meta">
            <span class="campaign-status" :class="`status-${campaign.status}`">
              {{ campaign.status }}
            </span>
            <span class="campaign-date">
              Created {{ formatDate(campaign.created_at) }}
            </span>
          </div>
          <p class="campaign-path">{{ campaign.directory_path }}</p>
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import MainLayout from '../../../shared/components/layout/MainLayout.vue'
import { useCampaignStore } from '../../../stores/campaigns'

const router = useRouter()
const campaignStore = useCampaignStore()

onMounted(async () => {
  await campaignStore.fetchCampaigns()
})

const selectCampaign = (id: number) => {
  router.push(`/campaigns/${id}`)
}

const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  return date.toLocaleDateString()
}
</script>

<style scoped>
.campaign-list-view {
  @apply space-y-6;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.page-title {
  font-size: 2rem;
  font-weight: 700;
  color: var(--color-text);
}

.loading,
.empty-state {
  text-align: center;
  padding: var(--spacing-xl) 0;
  color: var(--color-text-secondary);
}

.empty-state {
  @apply space-y-4;
}

.campaign-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: var(--spacing-lg);
}

.campaign-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  cursor: pointer;
  transition: all var(--transition-base);
}

.campaign-card:hover {
  border-color: var(--color-primary-500);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.campaign-name {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: var(--spacing-sm);
}

.campaign-meta {
  display: flex;
  gap: var(--spacing-md);
  align-items: center;
  margin-bottom: var(--spacing-sm);
}

.campaign-status {
  font-size: 0.875rem;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
  font-weight: 500;
}

.status-planning {
  background-color: var(--color-surface-variant);
  color: var(--color-status-planning);
  border: 1px solid var(--color-border);
}

.status-active {
  background-color: var(--color-surface-variant);
  color: var(--color-status-active);
  border: 1px solid var(--color-status-active);
}

.status-completed {
  background-color: var(--color-surface-variant);
  color: var(--color-status-completed);
  border: 1px solid var(--color-border);
}

.campaign-date {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.campaign-path {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.btn-primary {
  padding: var(--spacing-sm) var(--spacing-lg);
  background-color: var(--color-primary-500);
  color: var(--color-background);
  border-radius: var(--radius-md);
  text-decoration: none;
  font-weight: 500;
  transition: all var(--transition-fast);
  display: inline-block;
}

.btn-primary:hover {
  background-color: var(--color-primary-600);
  transform: translateY(-1px);
}

.error-message {
  padding: var(--spacing-md);
  background-color: var(--color-error) / 0.1;
  border: 1px solid var(--color-error) / 0.2;
  border-radius: var(--radius-md);
  color: var(--color-error);
}

/* Theme-specific overrides no longer needed - using theme variables */
</style>