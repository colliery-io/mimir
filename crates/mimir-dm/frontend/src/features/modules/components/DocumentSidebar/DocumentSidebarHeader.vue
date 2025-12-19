<template>
  <div class="sidebar-header">
    <h3 v-if="['active', 'completed'].includes(moduleStage)">Module Documents</h3>
    <h3 v-else>Module Documents</h3>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  moduleStage: string
}>()
</script>

<style scoped>
.sidebar-header {
  padding: var(--spacing-md);
  border-bottom: 1px solid var(--color-border);
}

.sidebar-header h3 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text);
}
</style>
