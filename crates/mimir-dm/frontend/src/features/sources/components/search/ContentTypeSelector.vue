<template>
  <div class="category-selector">
    <label for="category-select">Category:</label>
    <select 
      id="category-select" 
      :value="modelValue" 
      @change="$emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
      class="category-select"
    >
      <option value="Spells">Spells</option>
      <option value="Equipment">Equipment</option>
      <option value="Monsters">Monsters/Bestiary</option>
      <option value="Magic Items">Magic Items</option>
      <option value="Classes">Classes</option>
      <option value="Races">Races</option>
      <option value="Backgrounds">Backgrounds</option>
      <option value="Feats">Feats</option>
    </select>
  </div>
</template>

<script setup lang="ts">
interface Props {
  modelValue: string
}

defineProps<Props>()

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>

<style scoped>
.category-selector {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 8px);
}

.category-selector label {
  color: var(--color-text-secondary, #999);
  font-size: 0.9rem;
  font-weight: 500;
}

.category-select {
  padding: var(--spacing-xs, 4px) var(--spacing-sm, 8px);
  background: var(--color-background, #0d0d0d);
  border: 1px solid var(--color-border, #333);
  border-radius: 4px;
  color: var(--color-text, #e0e0e0);
  font-size: 0.9rem;
  cursor: pointer;
}

.category-select:focus {
  outline: none;
  border-color: var(--color-primary, #4a9eff);
}
</style>