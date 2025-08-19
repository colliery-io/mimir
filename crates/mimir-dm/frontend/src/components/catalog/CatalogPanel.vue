<template>
  <div class="catalog-panel">
    <div class="catalog-header">
      <h2>{{ selectedCategory }}</h2>
      <p>{{ getCategoryDescription(selectedCategory) }}</p>
    </div>
    
    <div class="catalog-content">
      <div class="catalog-search">
        <input 
          v-model="searchQuery"
          type="text"
          :placeholder="`Search ${selectedCategory.toLowerCase()}...`"
          class="search-input"
        />
      </div>
      
      <div class="catalog-results">
        <p class="placeholder-text">
          {{ selectedCategory }} catalog coming soon...
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  selectedCategory?: string
  selectedSources?: string[]
}

const props = withDefaults(defineProps<Props>(), {
  selectedCategory: 'Spells',
  selectedSources: () => []
})

const searchQuery = ref('')

function getCategoryDescription(category: string): string {
  const descriptions: Record<string, string> = {
    'Spells': 'Browse all available spells and cantrips',
    'Items': 'Explore equipment, magic items, and treasures',
    'Creatures': 'View monsters and NPCs with stat blocks',
    'Races': 'Character races and subraces',
    'Classes': 'Character classes and subclasses',
    'Feats': 'Character feats and abilities',
    'Backgrounds': 'Character backgrounds and origins',
    'Conditions': 'Status effects and conditions'
  }
  return descriptions[category] || `Browse ${category.toLowerCase()}`
}
</script>

<style scoped>
.catalog-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: var(--spacing-lg, 16px);
  background: var(--color-background, #0d0d0d);
}

.catalog-header {
  margin-bottom: var(--spacing-lg, 16px);
}

.catalog-header h2 {
  color: var(--color-text, #e0e0e0);
  margin: 0 0 var(--spacing-sm, 8px) 0;
}

.catalog-header p {
  color: var(--color-text-secondary, #999);
  margin: 0;
}

.catalog-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md, 12px);
}

.catalog-search {
  margin-bottom: var(--spacing-md, 12px);
}

.search-input {
  width: 100%;
  padding: var(--spacing-sm, 8px) var(--spacing-md, 12px);
  background: var(--color-surface, #1a1a1a);
  border: 1px solid var(--color-border, #333);
  border-radius: 4px;
  color: var(--color-text, #e0e0e0);
  font-size: 0.9rem;
}

.search-input:focus {
  outline: none;
  border-color: var(--color-primary, #4a9eff);
}

.catalog-results {
  flex: 1;
  background: var(--color-surface, #1a1a1a);
  border: 1px solid var(--color-border, #333);
  border-radius: 4px;
  padding: var(--spacing-lg, 16px);
  overflow-y: auto;
}

.placeholder-text {
  color: var(--color-text-secondary, #999);
  text-align: center;
  font-style: italic;
}
</style>