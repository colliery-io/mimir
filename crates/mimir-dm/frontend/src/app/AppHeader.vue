<template>
  <header class="app-header">
    <div class="header-content">
      <div class="header-left">
        <router-link to="/" class="skull-icon-link" title="Home">
          <img :src="skullIcon" alt="Mimir" class="skull-icon" />
        </router-link>
        <div class="header-divider"></div>
        <CampaignSelector />
      </div>

      <nav class="header-nav">
        <router-link to="/players" class="nav-link" title="Manage Players">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
            <circle cx="9" cy="7" r="4"/>
            <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
            <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
          </svg>
          <span>Players</span>
        </router-link>
        <router-link to="/characters" class="nav-link" title="Manage Characters">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
            <circle cx="12" cy="7" r="4"/>
          </svg>
          <span>Characters</span>
        </router-link>
        <button @click="handleOpenRules" class="nav-link" title="Open Reference Library">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
            <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
          </svg>
          <span>Reference</span>
        </button>
      </nav>

      <div class="header-right">
        <button @click="handleOpenChat" class="chat-button" title="Open Chat (new window)">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
          </svg>
          <span>Chat</span>
        </button>
        <router-link to="/settings" class="settings-icon" title="Settings">
          <img :src="gearIcon" alt="Settings" class="gear-icon" />
        </router-link>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useThemeStore } from '../stores/theme'
import { invoke } from '@tauri-apps/api/core'
import CampaignSelector from '../features/campaigns/components/CampaignSelector.vue'
import { openSourcesReference } from '../shared/utils/windows'
// Gear icons
import lightGear from '../assets/images/themes/light/gear.png'
import darkGear from '../assets/images/themes/dark/gear.png'
import hyperGear from '../assets/images/themes/hyper/gear.png'
// Skull icons
import lightMimir from '../assets/images/themes/light/mimir.png'
import darkMimir from '../assets/images/themes/dark/mimir.png'
import hyperMimir from '../assets/images/themes/hyper/mimir.png'

const themeStore = useThemeStore()

// Handle opening the rules reference window
const handleOpenRules = async () => {
  try {
    await openSourcesReference()
  } catch (error) {
  }
}

// Handle opening the chat window
const handleOpenChat = async () => {
  try {
    await invoke('open_chat_window')
  } catch (error) {
    console.error('Failed to open chat window:', error)
  }
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
  /* Removed decorative glow effect for theme consistency */
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

/* Header divider between logo and campaign selector */
.header-divider {
  width: 1px;
  height: 24px;
  background-color: var(--color-border);
  opacity: 0.6;
}

/* Center navigation */
.header-nav {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.nav-link {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background-color: transparent;
  color: var(--color-text-secondary);
  border: none;
  font-size: 0.875rem;
  font-weight: 500;
  text-decoration: none;
  transition: all var(--transition-fast);
  cursor: pointer;
}

.nav-link:hover {
  background-color: var(--color-surface-variant);
  color: var(--color-text);
}

.nav-link:active {
  transform: scale(0.98);
}

.nav-link svg {
  opacity: 0.7;
  transition: opacity var(--transition-fast);
}

.nav-link:hover svg {
  opacity: 1;
}

/* Active nav link state */
.nav-link.router-link-active {
  background-color: var(--color-surface-variant);
  color: var(--color-text);
}

.nav-link.router-link-active svg {
  opacity: 1;
}

.chat-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background-color: var(--color-primary-500);
  color: white;
  border: none;
  font-size: 0.875rem;
  font-weight: 500;
  transition: all var(--transition-fast);
  cursor: pointer;
  box-shadow: var(--shadow-sm);
}

.chat-button:hover {
  background-color: var(--color-primary-600);
  box-shadow: var(--shadow);
  transform: translateY(-1px);
}

.chat-button:active {
  transform: translateY(0) scale(0.98);
  box-shadow: var(--shadow-sm);
}

.chat-button svg {
  opacity: 0.9;
}

</style>