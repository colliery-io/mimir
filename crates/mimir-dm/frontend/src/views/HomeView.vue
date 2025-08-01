<template>
  <MainLayout>
    <div class="home-view">
      <section class="hero">
        <h1 class="hero-title">Welcome to Mimir</h1>
        <p class="hero-subtitle">
          Your intelligent D&D campaign management assistant
        </p>
      </section>
      
      <section class="quick-actions">
        <h2 class="section-title">Quick Actions</h2>
        <div class="action-grid">
          <router-link to="/campaigns/new" class="action-card">
            <h3 class="action-title">Create Campaign</h3>
            <p class="action-description">Start a new campaign from scratch</p>
          </router-link>
          
          <router-link to="/campaigns" class="action-card">
            <h3 class="action-title">Manage Campaigns</h3>
            <p class="action-description">View and edit your existing campaigns</p>
          </router-link>
          
          <router-link to="/templates" class="action-card">
            <h3 class="action-title">Browse Templates</h3>
            <p class="action-description">Explore campaign and session templates</p>
          </router-link>
        </div>
      </section>
      
      <section class="recent-activity" v-if="recentCampaigns.length > 0">
        <h2 class="section-title">Recent Campaigns</h2>
        <div class="campaign-list">
          <div
            v-for="campaign in recentCampaigns"
            :key="campaign.id"
            class="campaign-item"
            @click="navigateToCampaign(campaign.id)"
          >
            <h4 class="campaign-name">{{ campaign.name }}</h4>
            <span class="campaign-status" :class="`status-${campaign.status}`">
              {{ campaign.status }}
            </span>
          </div>
        </div>
      </section>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import MainLayout from '../components/layout/MainLayout.vue'
import { useCampaignStore } from '../stores/campaigns'

const router = useRouter()
const campaignStore = useCampaignStore()

// Get the 5 most recent campaigns
const recentCampaigns = computed(() => {
  return campaignStore.campaigns.slice(0, 5)
})

const navigateToCampaign = (id: number) => {
  router.push(`/campaigns/${id}`)
}

onMounted(async () => {
  // Load campaigns when the home page is mounted
  await campaignStore.fetchCampaigns()
})
</script>

<style scoped>
.home-view {
  @apply space-y-12;
}

.hero {
  text-align: center;
  padding: var(--spacing-xl) 0;
}

.hero-title {
  font-size: 3rem;
  font-weight: 800;
  color: var(--color-text);
  margin-bottom: var(--spacing-md);
}

.hero-subtitle {
  font-size: 1.25rem;
  color: var(--color-text-secondary);
}

.section-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: var(--spacing-lg);
}

.quick-actions {
  @apply space-y-6;
}

.action-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--spacing-lg);
}

.action-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  text-decoration: none;
  transition: all var(--transition-base);
  cursor: pointer;
}

.action-card:hover {
  border-color: var(--color-primary-500);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.action-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: var(--spacing-sm);
}

.action-description {
  color: var(--color-text-secondary);
  font-size: 0.875rem;
}

.recent-activity {
  @apply space-y-4;
}

.campaign-list {
  @apply space-y-2;
}

.campaign-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md);
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.campaign-item:hover {
  background-color: var(--color-surface-variant);
  border-color: var(--color-border-hover);
}

.campaign-name {
  font-weight: 500;
  color: var(--color-text);
}

.campaign-status {
  font-size: 0.875rem;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
  font-weight: 500;
}

.status-planning {
  background-color: var(--color-primary-100);
  color: var(--color-primary-700);
}

.status-active {
  background-color: #10b98133;
  color: #15803d;
}

.status-paused {
  background-color: #f59e0b33;
  color: #92400e;
}

.status-completed {
  background-color: #6b728033;
  color: var(--color-text-secondary);
}

.theme-dark .status-planning {
  background-color: var(--color-primary-900);
  color: var(--color-primary-300);
}

.theme-dark .status-active {
  background-color: #10b98133;
  color: #34d399;
}

.theme-dark .status-paused {
  background-color: #f59e0b33;
  color: #fbbf24;
}
</style>