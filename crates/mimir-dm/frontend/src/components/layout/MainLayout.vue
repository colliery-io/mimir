<template>
  <div class="main-layout">
    <AppHeader />
    <main class="main-content">
      <div class="content-wrapper">
        <slot />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import AppHeader from './AppHeader.vue'
</script>

<style scoped>
.main-layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.main-content {
  flex: 1;
  padding: var(--spacing-xl) 0;
}

.content-wrapper {
  max-width: 1280px;
  margin: 0 auto;
  padding: 0 var(--spacing-lg);
}
</style>