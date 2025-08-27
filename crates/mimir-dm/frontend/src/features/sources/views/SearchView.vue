<template>
  <div class="catalog-panel">
    <!-- Header -->
    <div class="catalog-header">
      <div class="header-top">
        <ContentTypeSelector v-model="selectedCategory" />
        
        <div class="search-bar">
          <input 
            type="text" 
            v-model="searchQuery" 
            @input="debouncedSearch"
            placeholder="Search..."
            class="search-input"
          >
        </div>
        
        <div class="results-count" v-if="searchPerformed">
          {{ resultCount }} results
        </div>
      </div>
      
      <SearchFilters 
        v-if="showFilters"
        :category="selectedCategory"
        :filters="filters"
        @update="filters = $event; performSearch()"
      />
    </div>
    
    <!-- Content -->
    <div class="catalog-content">
      <div class="table-container">
        <SearchResults
          :category="selectedCategory"
          :results="results"
          :search-performed="searchPerformed"
          :sort-column="sortColumn"
          :sort-direction="sortDirection"
          :monster-filters="filters.monsters"
          :available-sources="classSources"
          @select-spell="selectSpell"
          @select-item="selectItem"
          @select-monster="selectMonster"
          @select-class="selectClass"
          @select-feat="selectFeat"
          @select-race="selectRace"
          @sort="handleSort"
          @update-monster-filters="updateMonsterFilters"
        />
      </div>
    </div>
    
    <!-- Modal Stack -->
    <BaseModal
      v-for="(modal, index) in modalStack"
      :key="`modal-${index}`"
      :visible="modal.visible"
      :title="modal.title"
      :content="modal.content"
      :z-index="1000 + index * 10"
      @close="() => closeModal(index)"
      @reference-click="handleReferenceClick"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useSearch } from '../composables/useSearch'
import BaseModal from '@/components/shared/BaseModal.vue'
import ContentTypeSelector from '../components/search/ContentTypeSelector.vue'
import SearchFilters from '../components/search/SearchFilters.vue'
import SearchResults from '../components/search/SearchResults.vue'

interface Props {
  selectedSources: string[]
  selectedCategory: string
}

const props = defineProps<Props>()

const {
  selectedCategory,
  searchQuery,
  searchPerformed,
  sortColumn,
  sortDirection,
  results,
  filters,
  modalStack,
  resultCount,
  classSources,
  performSearch,
  debouncedSearch,
  handleSort,
  updateMonsterFilters,
  selectSpell,
  selectItem,
  selectMonster,
  selectClass,
  selectFeat,
  selectRace,
  closeModal,
  handleReferenceClick,
  initialize
} = useSearch(props.selectedCategory, props.selectedSources)

const showFilters = computed(() => {
  return ['Spells', 'Equipment', 'Magic Items'].includes(selectedCategory.value)
})

onMounted(() => {
  initialize()
})
</script>

<style scoped>
.catalog-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--color-background, #0d0d0d);
}

.catalog-header {
  padding: var(--spacing-md, 12px) var(--spacing-lg, 16px);
  background: var(--color-surface, #1a1a1a);
  border-bottom: 1px solid var(--color-border, #333);
}

.header-top {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg, 16px);
  margin-bottom: var(--spacing-sm, 8px);
}

.search-bar {
  flex: 1;
  max-width: 400px;
}

.search-input {
  width: 100%;
  padding: var(--spacing-xs, 4px) var(--spacing-sm, 8px);
  background: var(--color-background, #0d0d0d);
  border: 1px solid var(--color-border, #333);
  border-radius: 4px;
  color: var(--color-text, #e0e0e0);
  font-size: 0.9rem;
}

.search-input:focus {
  outline: none;
  border-color: var(--color-primary, #4a9eff);
}

.results-count {
  color: var(--color-text-secondary, #999);
  font-size: 0.9rem;
}

.catalog-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.table-container {
  flex: 1;
  overflow: auto;
  padding: 0;
}
</style>