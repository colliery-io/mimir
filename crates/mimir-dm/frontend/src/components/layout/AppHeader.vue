<template>
  <header class="app-header">
    <div class="header-content">
      <div class="header-left">
        <router-link to="/" class="skull-icon-link" title="Home">
          <img :src="skullIcon" alt="Mimir" class="skull-icon" />
        </router-link>
        <CampaignSelector />
        <button @click="createNewCampaign" class="new-campaign-button" title="Create New Campaign">
          <img :src="newIcon" alt="New Campaign" class="new-icon" />
        </button>
      </div>
      
      <div class="header-center">
        <!-- Spacer to push elements to sides -->
      </div>
      
      <div class="header-right">
        <router-link to="/settings" class="settings-icon" title="Settings">
          <img :src="gearIcon" alt="Settings" class="gear-icon" />
        </router-link>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useThemeStore } from '../../stores/theme'
import CampaignSelector from '../campaign/CampaignSelector.vue'
// Gear icons
import lightGear from '../../assets/images/themes/light/gear.png'
import darkGear from '../../assets/images/themes/dark/gear.png'
import hyperGear from '../../assets/images/themes/hyper/gear.png'
// Skull icons
import lightMimir from '../../assets/images/themes/light/mimir.png'
import darkMimir from '../../assets/images/themes/dark/mimir.png'
import hyperMimir from '../../assets/images/themes/hyper/mimir.png'
// New campaign icons
import lightNew from '../../assets/images/themes/light/new.png'
import darkNew from '../../assets/images/themes/dark/new.png'
import hyperNew from '../../assets/images/themes/hyper/new.png'

const route = useRoute()
const router = useRouter()
const themeStore = useThemeStore()

// Navigate to new campaign page
const createNewCampaign = () => {
  router.push('/campaigns/new')
}

// Dynamically select gear icon based on current theme
const gearIcon = computed(() => {
  switch (themeStore.currentTheme) {
    case 'dark':
      return darkGear
    case 'hyper':
      return hyperGear
    default:
      return lightGear
  }
})

// Dynamically select skull icon based on current theme
const skullIcon = computed(() => {
  switch (themeStore.currentTheme) {
    case 'dark':
      return darkMimir
    case 'hyper':
      return hyperMimir
    default:
      return lightMimir
  }
})

// Dynamically select new campaign icon based on current theme
const newIcon = computed(() => {
  switch (themeStore.currentTheme) {
    case 'dark':
      return darkNew
    case 'hyper':
      return hyperNew
    default:
      return lightNew
  }
})
</script>

<style scoped>
.app-header {
  background-color: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  box-shadow: var(--shadow-sm);
}

.header-content {
  max-width: 1280px;
  margin: 0 auto;
  padding: 0 30px;
  height: 72px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-left {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.skull-icon-link {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
  cursor: pointer;
}

.skull-icon-link:hover {
  background-color: var(--color-surface-variant);
}

.skull-icon {
  width: 44px;
  height: 44px;
  object-fit: contain;
  filter: drop-shadow(0 0 4px rgba(125, 211, 252, 0.2));
  transition: transform var(--transition-fast);
  /* Scale up to eat negative space in the image */
  transform: scale(1.2);
}

.skull-icon-link:hover .skull-icon {
  transform: scale(1.35);
}

.header-center {
  flex: 1;
  /* Empty spacer to push left and right elements to sides */
}

.header-right {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.settings-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  border-radius: var(--radius-md);
  color: var(--color-text-secondary);
  transition: all var(--transition-fast);
  cursor: pointer;
}

.settings-icon:hover {
  color: var(--color-text);
  background-color: var(--color-surface-variant);
}

.gear-icon {
  width: 44px;
  height: 44px;
  transition: transform var(--transition-fast);
  /* Scale up to eat negative space in the image */
  transform: scale(1.2);
}

.settings-icon:hover .gear-icon {
  transform: rotate(45deg) scale(1.25);
}

.new-campaign-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  border: none;
  border-radius: var(--radius-md);
  background-color: transparent;
  cursor: pointer;
  transition: all var(--transition-fast);
  padding: 0;
}

.new-campaign-button:hover {
  background-color: var(--color-surface-variant);
}

.new-icon {
  width: 44px;
  height: 44px;
  object-fit: contain;
  transition: transform var(--transition-fast);
  /* Scale up to eat negative space in the image */
  transform: scale(1.2);
}

.new-campaign-button:hover .new-icon {
  transform: scale(1.35);
}
</style>