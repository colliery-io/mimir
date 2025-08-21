<template>
  <div class="multi-select-filter">
    <button 
      @click="toggleDropdown" 
      class="filter-button"
      :class="{ active: isOpen || hasActiveFilters }"
    >
      {{ label }}
      <span v-if="activeCount > 0" class="active-count">{{ activeCount }}</span>
      <svg class="chevron" :class="{ rotated: isOpen }" width="12" height="12" viewBox="0 0 12 12">
        <path d="M3 4.5L6 7.5L9 4.5" stroke="currentColor" stroke-width="1.5" fill="none"/>
      </svg>
    </button>
    
    <div v-if="isOpen" class="dropdown-content">
      <div class="filter-search" v-if="options.length > 10">
        <input 
          type="text" 
          v-model="searchTerm" 
          :placeholder="`Search ${label.toLowerCase()}...`"
          class="filter-search-input"
        >
      </div>
      
      <div class="filter-options">
        <label 
          v-for="option in filteredOptions" 
          :key="option"
          class="filter-option"
        >
          <input 
            type="checkbox" 
            :checked="modelValue.includes(option)"
            @change="toggleOption(option)"
          >
          <span>{{ option }}</span>
        </label>
      </div>
      
      <div class="filter-actions">
        <button @click="clearAll" class="clear-button">Clear All</button>
        <button @click="selectAll" class="select-button">Select All</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

interface Props {
  label: string
  options: string[]
  modelValue: string[]
}

const props = defineProps<Props>()
const emit = defineEmits<{
  'update:modelValue': [value: string[]]
}>()

const isOpen = ref(false)
const searchTerm = ref('')

const filteredOptions = computed(() => {
  if (!searchTerm.value) return props.options
  const term = searchTerm.value.toLowerCase()
  return props.options.filter(opt => opt.toLowerCase().includes(term))
})

const hasActiveFilters = computed(() => props.modelValue.length > 0)
const activeCount = computed(() => props.modelValue.length)

function toggleDropdown() {
  isOpen.value = !isOpen.value
}

function toggleOption(option: string) {
  const current = [...props.modelValue]
  const index = current.indexOf(option)
  
  if (index > -1) {
    current.splice(index, 1)
  } else {
    current.push(option)
  }
  
  emit('update:modelValue', current)
}

function clearAll() {
  emit('update:modelValue', [])
}

function selectAll() {
  emit('update:modelValue', [...filteredOptions.value])
}

// Close dropdown when clicking outside
function handleClickOutside(event: MouseEvent) {
  const target = event.target as HTMLElement
  if (!target.closest('.multi-select-filter')) {
    isOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.multi-select-filter {
  position: relative;
  display: inline-block;
}

.filter-button {
  padding: 0.5rem 1rem;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
}

.filter-button:hover {
  border-color: var(--color-primary-500);
}

.filter-button.active {
  border-color: var(--color-primary-500);
  background: var(--color-primary-100);
}

.theme-dark .filter-button.active {
  background: var(--color-primary-900);
}

.active-count {
  padding: 0.125rem 0.375rem;
  background: var(--color-primary-500);
  color: white;
  border-radius: var(--radius-sm);
  font-size: 0.75rem;
  font-weight: 600;
}

.chevron {
  transition: transform 0.2s;
}

.chevron.rotated {
  transform: rotate(180deg);
}

.dropdown-content {
  position: absolute;
  top: calc(100% + 0.25rem);
  left: 0;
  min-width: 200px;
  max-width: 300px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  z-index: 100;
}

.filter-search {
  padding: 0.5rem;
  border-bottom: 1px solid var(--color-border);
}

.filter-search-input {
  width: 100%;
  padding: 0.375rem 0.5rem;
  background: var(--color-background);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  color: var(--color-text);
}

.filter-options {
  max-height: 300px;
  overflow-y: auto;
  padding: 0.5rem;
}

.filter-option {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem;
  cursor: pointer;
  border-radius: var(--radius-sm);
}

.filter-option:hover {
  background: var(--color-surface-hover);
}

.filter-option input[type="checkbox"] {
  cursor: pointer;
}

.filter-actions {
  padding: 0.5rem;
  border-top: 1px solid var(--color-border);
  display: flex;
  gap: 0.5rem;
}

.clear-button,
.select-button {
  flex: 1;
  padding: 0.375rem 0.5rem;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background: var(--color-surface);
  color: var(--color-text);
  cursor: pointer;
  font-size: 0.875rem;
}

.clear-button:hover,
.select-button:hover {
  background: var(--color-surface-hover);
}
</style>