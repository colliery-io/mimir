<template>
  <header class="app-header">
    <div class="header-content">
      <div class="header-left">
        <h1 class="app-title">Mimir</h1>
        <span class="app-subtitle">D&D Campaign Assistant</span>
      </div>
      
      <nav class="header-nav">
        <router-link
          v-for="item in navItems"
          :key="item.path"
          :to="item.path"
          class="nav-link"
          active-class="nav-link-active"
        >
          {{ item.label }}
        </router-link>
      </nav>
      
      <div class="header-right">
        <ThemeSelector />
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import ThemeSelector from '../common/ThemeSelector.vue'

interface NavItem {
  path: string
  label: string
}

const navItems: NavItem[] = [
  { path: '/', label: 'Home' },
  { path: '/campaigns', label: 'Campaigns' },
  { path: '/templates', label: 'Templates' },
  { path: '/settings', label: 'Settings' }
]
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
  padding: 0 var(--spacing-lg);
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-left {
  display: flex;
  align-items: baseline;
  gap: var(--spacing-sm);
}

.app-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-primary-600);
  margin: 0;
}

.app-subtitle {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.header-nav {
  display: flex;
  gap: var(--spacing-lg);
  align-items: center;
}

.nav-link {
  color: var(--color-text-secondary);
  text-decoration: none;
  font-weight: 500;
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.nav-link:hover {
  color: var(--color-text);
  background-color: var(--color-surface-variant);
}

.nav-link-active {
  color: var(--color-primary-600);
  background-color: var(--color-primary-50);
}

.theme-dark .nav-link-active {
  background-color: var(--color-primary-900);
}

.header-right {
  display: flex;
  align-items: center;
}
</style>