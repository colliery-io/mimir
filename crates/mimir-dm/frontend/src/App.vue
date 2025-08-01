<template>
  <div id="app" :class="[currentTheme]">
    <router-view />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useThemeStore } from './stores/theme'

const themeStore = useThemeStore()
const currentTheme = computed(() => `theme-${themeStore.currentTheme}`)

onMounted(async () => {
  // Load available themes from backend
  await themeStore.loadThemes()
  // Apply saved theme preference
  themeStore.applyTheme()
})
</script>

<style>
#app {
  min-height: 100vh;
  background-color: var(--color-background);
  color: var(--color-text);
  transition: background-color 0.3s ease, color 0.3s ease;
}
</style>