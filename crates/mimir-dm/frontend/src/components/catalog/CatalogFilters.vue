<template>
  <Panel title="Filters" variant="default">
    <div class="filters-content">
      <!-- Category Selector Dropdown -->
      <div class="filter-section category-selector">
        <h4>Category</h4>
        <select v-model="selectedCategory" class="category-dropdown">
          <option v-for="cat in catalogCategories" :key="cat" :value="cat">
            {{ cat }}
          </option>
        </select>
      </div>
      
      <!-- Dynamic filters based on category -->
      <div v-if="selectedCategory === 'Spells'" class="filter-group">
        <div class="filter-section">
          <h4>Level</h4>
          <div class="filter-options">
            <label v-for="level in spellLevels" :key="level" class="filter-checkbox">
              <input type="checkbox" :value="level" />
              <span>{{ level === 0 ? 'Cantrip' : `Level ${level}` }}</span>
            </label>
          </div>
        </div>
        
        <div class="filter-section">
          <h4>School</h4>
          <div class="filter-options">
            <label v-for="school in spellSchools" :key="school" class="filter-checkbox">
              <input type="checkbox" :value="school" />
              <span>{{ school }}</span>
            </label>
          </div>
        </div>
        
        <div class="filter-section">
          <h4>Properties</h4>
          <div class="filter-options">
            <label class="filter-checkbox">
              <input type="checkbox" />
              <span>Ritual</span>
            </label>
            <label class="filter-checkbox">
              <input type="checkbox" />
              <span>Concentration</span>
            </label>
          </div>
        </div>
      </div>
      
      <div v-else-if="selectedCategory === 'Items'" class="filter-group">
        <div class="filter-section">
          <h4>Type</h4>
          <div class="filter-options">
            <label v-for="type in itemTypes" :key="type" class="filter-checkbox">
              <input type="checkbox" :value="type" />
              <span>{{ type }}</span>
            </label>
          </div>
        </div>
        
        <div class="filter-section">
          <h4>Rarity</h4>
          <div class="filter-options">
            <label v-for="rarity in rarities" :key="rarity" class="filter-checkbox">
              <input type="checkbox" :value="rarity" />
              <span>{{ rarity }}</span>
            </label>
          </div>
        </div>
        
        <div class="filter-section">
          <h4>Properties</h4>
          <div class="filter-options">
            <label class="filter-checkbox">
              <input type="checkbox" />
              <span>Requires Attunement</span>
            </label>
          </div>
        </div>
      </div>
      
      <div v-else-if="selectedCategory === 'Creatures'" class="filter-group">
        <div class="filter-section">
          <h4>CR Range</h4>
          <div class="filter-range">
            <input type="number" min="0" max="30" placeholder="Min" class="range-input" />
            <span>to</span>
            <input type="number" min="0" max="30" placeholder="Max" class="range-input" />
          </div>
        </div>
        
        <div class="filter-section">
          <h4>Type</h4>
          <div class="filter-options">
            <label v-for="type in creatureTypes" :key="type" class="filter-checkbox">
              <input type="checkbox" :value="type" />
              <span>{{ type }}</span>
            </label>
          </div>
        </div>
        
        <div class="filter-section">
          <h4>Size</h4>
          <div class="filter-options">
            <label v-for="size in creatureSizes" :key="size" class="filter-checkbox">
              <input type="checkbox" :value="size" />
              <span>{{ size }}</span>
            </label>
          </div>
        </div>
      </div>
      
      <div v-else class="filter-group">
        <p class="no-filters">No filters available for {{ selectedCategory }}</p>
      </div>
      
      <!-- Clear filters button -->
      <div class="filter-actions">
        <button class="clear-filters-btn">Clear All Filters</button>
      </div>
    </div>
  </Panel>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import Panel from '../layout/Panel.vue'

interface Emits {
  (e: 'categoryChange', category: string): void
}

const emit = defineEmits<Emits>()

const selectedCategory = ref('Spells')

// Catalog categories
const catalogCategories = [
  'Spells',
  'Items',
  'Creatures',
  'Races',
  'Classes',
  'Feats',
  'Backgrounds',
  'Conditions'
]

// Watch for category changes
watch(selectedCategory, (newCategory) => {
  emit('categoryChange', newCategory)
})

// Filter options
const spellLevels = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
const spellSchools = [
  'Abjuration',
  'Conjuration', 
  'Divination',
  'Enchantment',
  'Evocation',
  'Illusion',
  'Necromancy',
  'Transmutation'
]

const itemTypes = [
  'Weapon',
  'Armor',
  'Potion',
  'Ring',
  'Rod',
  'Scroll',
  'Staff',
  'Wand',
  'Wondrous'
]

const rarities = [
  'Common',
  'Uncommon',
  'Rare',
  'Very Rare',
  'Legendary',
  'Artifact'
]

const creatureTypes = [
  'Aberration',
  'Beast',
  'Celestial',
  'Construct',
  'Dragon',
  'Elemental',
  'Fey',
  'Fiend',
  'Giant',
  'Humanoid',
  'Monstrosity',
  'Ooze',
  'Plant',
  'Undead'
]

const creatureSizes = [
  'Tiny',
  'Small',
  'Medium',
  'Large',
  'Huge',
  'Gargantuan'
]
</script>

<style scoped>
.filters-content {
  padding: var(--spacing-md, 12px);
  height: calc(100% - 20px);
  overflow-y: auto;
  overflow-x: hidden;
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm, 8px);
}

.filter-section {
  border-bottom: 1px solid var(--color-border, #333);
  padding-bottom: var(--spacing-sm, 8px);
  margin-bottom: var(--spacing-sm, 8px);
}

.filter-section:last-of-type {
  border-bottom: none;
}

.category-selector {
  background: var(--color-surface-alpha, rgba(255, 255, 255, 0.02));
  padding: var(--spacing-sm, 8px);
  border-radius: 4px;
  margin-bottom: var(--spacing-md, 12px);
  border-bottom: 2px solid var(--color-primary-alpha, rgba(74, 158, 255, 0.2));
}

.category-dropdown {
  width: 100%;
  padding: var(--spacing-sm, 8px);
  background: var(--color-surface, #1a1a1a);
  border: 1px solid var(--color-border, #333);
  border-radius: 4px;
  color: var(--color-text, #e0e0e0);
  font-size: 0.9rem;
  cursor: pointer;
}

.category-dropdown:focus {
  outline: none;
  border-color: var(--color-primary, #4a9eff);
}

.filter-section h4 {
  margin: 0 0 var(--spacing-xs, 4px) 0;
  color: var(--color-text, #e0e0e0);
  font-size: 0.85rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.filter-options {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs, 4px);
}

.filter-checkbox {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs, 4px);
  cursor: pointer;
  padding: 2px 0;
  color: var(--color-text-secondary, #999);
  font-size: 0.85rem;
  transition: color 0.2s;
}

.filter-checkbox:hover {
  color: var(--color-text, #e0e0e0);
}

.filter-checkbox input[type="checkbox"] {
  margin: 0;
  cursor: pointer;
}

.filter-range {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 8px);
}

.range-input {
  width: 60px;
  padding: var(--spacing-xs, 4px);
  background: var(--color-surface, #1a1a1a);
  border: 1px solid var(--color-border, #333);
  border-radius: 3px;
  color: var(--color-text, #e0e0e0);
  font-size: 0.85rem;
}

.range-input:focus {
  outline: none;
  border-color: var(--color-primary, #4a9eff);
}

.no-filters {
  color: var(--color-text-secondary, #999);
  font-style: italic;
  text-align: center;
  padding: var(--spacing-lg, 16px);
}

.filter-actions {
  margin-top: var(--spacing-sm, 8px);
  padding-top: var(--spacing-sm, 8px);
  border-top: 1px solid var(--color-border, #333);
}

.clear-filters-btn {
  width: 100%;
  padding: var(--spacing-sm, 8px);
  background: var(--color-surface, #1a1a1a);
  border: 1px solid var(--color-border, #333);
  border-radius: 4px;
  color: var(--color-text-secondary, #999);
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.2s;
}

.clear-filters-btn:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
  color: var(--color-text, #e0e0e0);
  border-color: var(--color-primary, #4a9eff);
}
</style>