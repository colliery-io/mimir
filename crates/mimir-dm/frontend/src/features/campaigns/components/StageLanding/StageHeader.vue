<template>
  <div class="stage-header">
    <h2>{{ title }}</h2>
    <p class="stage-subtitle">{{ subtitle }}</p>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  subtitle: string
}>()
</script>
