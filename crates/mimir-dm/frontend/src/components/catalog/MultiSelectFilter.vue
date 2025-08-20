<template>
  <div class="multi-select-filter" ref="dropdownRef">
    <button class="filter-button" @click="toggleDropdown" :class="{ active: isOpen || hasSelections }">
      <span class="filter-label">{{ label }}</span>
      <span class="filter-count" v-if="selectedCount > 0">({{ selectedCount }})</span>
      <span class="dropdown-arrow">{{ isOpen ? '▲' : '▼' }}</span>
    </button>
    
    <div v-if="isOpen" class="dropdown-menu">
      <div class="dropdown-header">
        <button class="select-all-btn" @click="selectAll">All</button>
        <button class="clear-btn" @click="clearAll">Clear</button>
      </div>
      
      <div class="dropdown-options">
        <label v-for="option in options" :key="option.value" class="option-item">
          <input 
            type="checkbox" 
            :value="option.value"
            :checked="isSelected(option.value)"
            @change="toggleOption(option.value)"
          />
          <span>{{ option.label }}</span>
        </label>
      </div>
      
      <div class="dropdown-footer">
        <button class="apply-btn" @click="applyFilter">Apply</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

interface Option {
  value: string
  label: string
}

interface Props {
  label: string
  options: Option[]
  modelValue: string[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:modelValue': [value: string[]]
  'change': [value: string[]]
}>()

const isOpen = ref(false)
const dropdownRef = ref<HTMLElement>()
const selectedValues = ref<Set<string>>(new Set(props.modelValue))

const selectedCount = computed(() => selectedValues.value.size)
const hasSelections = computed(() => selectedCount.value > 0)

function isSelected(value: string): boolean {
  return selectedValues.value.has(value)
}

function toggleDropdown() {
  isOpen.value = !isOpen.value
}

function toggleOption(value: string) {
  if (selectedValues.value.has(value)) {
    selectedValues.value.delete(value)
  } else {
    selectedValues.value.add(value)
  }
}

function selectAll() {
  selectedValues.value = new Set(props.options.map(o => o.value))
}

function clearAll() {
  selectedValues.value.clear()
}

function applyFilter() {
  const values = Array.from(selectedValues.value)
  emit('update:modelValue', values)
  emit('change', values)
  isOpen.value = false
}

// Click outside to close
function handleClickOutside(event: MouseEvent) {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
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
  width: 100%;
}

.filter-button {
  width: 100%;
  padding: 2px 6px;
  background: var(--color-background, #0d0d0d);
  border: 1px solid var(--color-border, #333);
  border-radius: 3px;
  color: var(--color-text-secondary, #999);
  font-size: 0.875rem;
  font-weight: normal;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 4px;
  transition: all 0.2s;
}

.filter-button:hover {
  border-color: var(--color-primary, #4a9eff);
  color: var(--color-text, #e0e0e0);
}

.filter-button.active {
  border-color: var(--color-primary, #4a9eff);
  color: var(--color-primary, #4a9eff);
}

.filter-label {
  flex: 1;
  text-align: left;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.filter-count {
  color: var(--color-primary, #4a9eff);
  font-weight: 600;
}

.dropdown-arrow {
  font-size: 0.75rem;
  opacity: 0.6;
}

.dropdown-menu {
  position: absolute;
  top: calc(100% + 2px);
  left: 0;
  right: 0;
  min-width: 150px;
  max-width: 250px;
  background: var(--color-surface, #1a1a1a);
  border: 1px solid var(--color-border, #333);
  border-radius: 4px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
  z-index: 1000;
  animation: slideDown 0.2s ease-out;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-4px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.dropdown-header {
  padding: 6px;
  border-bottom: 1px solid var(--color-border, #333);
  display: flex;
  gap: 4px;
}

.select-all-btn,
.clear-btn {
  flex: 1;
  padding: 4px 8px;
  background: var(--color-background, #0d0d0d);
  border: 1px solid var(--color-border, #333);
  border-radius: 3px;
  color: var(--color-text-secondary, #999);
  font-size: 0.75rem;
  cursor: pointer;
  transition: all 0.2s;
}

.select-all-btn:hover,
.clear-btn:hover {
  background: var(--color-primary, #4a9eff);
  color: white;
  border-color: var(--color-primary, #4a9eff);
}

.dropdown-options {
  max-height: 200px;
  overflow-y: auto;
  padding: 4px;
}

.option-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 6px;
  cursor: pointer;
  border-radius: 3px;
  transition: background 0.1s;
}

.option-item:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
}

.option-item input[type="checkbox"] {
  margin: 0;
  cursor: pointer;
}

.option-item span {
  color: var(--color-text, #e0e0e0);
  font-size: 0.875rem;
}

.dropdown-footer {
  padding: 6px;
  border-top: 1px solid var(--color-border, #333);
}

.apply-btn {
  width: 100%;
  padding: 6px;
  background: var(--color-primary, #4a9eff);
  border: none;
  border-radius: 3px;
  color: white;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.apply-btn:hover {
  background: var(--color-primary-hover, #5aafff);
}

/* Scrollbar styling */
.dropdown-options::-webkit-scrollbar {
  width: 6px;
}

.dropdown-options::-webkit-scrollbar-track {
  background: var(--color-background, #0d0d0d);
}

.dropdown-options::-webkit-scrollbar-thumb {
  background: var(--color-border, #333);
  border-radius: 3px;
}

.dropdown-options::-webkit-scrollbar-thumb:hover {
  background: var(--color-primary, #4a9eff);
}
</style>