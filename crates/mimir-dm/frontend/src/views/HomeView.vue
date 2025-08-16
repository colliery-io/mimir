<template>
  <MainLayout>
    <div class="home-view">
      <section class="hero">
        <div class="hero-image-container">
          <img 
            :src="skullImage" 
            alt="Mimir - Mystical Skull" 
            class="hero-image"
          />
        </div>
        <h1 class="hero-title">Mimir DM : Campaign Assistant</h1>
      </section>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import MainLayout from '../components/layout/MainLayout.vue'
import { useThemeStore } from '../stores/theme'
import lightMimir from '../assets/images/themes/light/mimir.png'
import darkMimir from '../assets/images/themes/dark/mimir.png'
import hyperMimir from '../assets/images/themes/hyper/mimir.png'

const themeStore = useThemeStore()

// Dynamically select skull image based on current theme
const skullImage = computed(() => {
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
.home-view {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - 72px); /* Full height minus header */
}

.hero {
  text-align: center;
}

.hero-image-container {
  display: flex;
  justify-content: center;
  align-items: center;
  animation: float 6s ease-in-out infinite;
}

.hero-image {
  width: 400px;
  height: 400px;
  object-fit: contain;
  /* Removed decorative glow effect for theme consistency */
  display: block;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-20px);
  }
}

.hero-title {
  font-size: 2.5rem;
  font-weight: 700;
  color: var(--color-text);
  margin-top: var(--spacing-xl);
  letter-spacing: 0.05em;
  text-align: center;
}
</style>