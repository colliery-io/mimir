<template>
  <div class="print-button-container">
    <button
      v-if="templates.length <= 1"
      @click="handleSingleTemplate"
      class="print-button"
      :class="{ 'is-loading': isGenerating }"
      :disabled="disabled || isGenerating"
      :title="buttonTitle"
    >
      <span v-if="isGenerating" class="spinner-small"></span>
      <span v-else class="print-icon">{{ icon }}</span>
      <span v-if="showLabel">{{ label }}</span>
    </button>

    <div v-else class="print-dropdown" ref="dropdownRef">
      <button
        @click="toggleDropdown"
        class="print-button dropdown-trigger"
        :class="{ 'is-loading': isGenerating }"
        :disabled="disabled || isGenerating"
        :title="buttonTitle"
      >
        <span v-if="isGenerating" class="spinner-small"></span>
        <span v-else class="print-icon">{{ icon }}</span>
        <span v-if="showLabel">{{ label }}</span>
        <span class="dropdown-arrow">▼</span>
      </button>

      <div v-if="dropdownOpen" class="dropdown-menu">
        <button
          v-for="template in templates"
          :key="template.id"
          @click="handleSelectTemplate(template)"
          class="dropdown-item"
        >
          {{ template.name }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

export interface PrintTemplate {
  id: string
  name: string
  category?: string
}

interface Props {
  templates?: PrintTemplate[]
  label?: string
  icon?: string
  disabled?: boolean
  showLabel?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  templates: () => [],
  label: 'Print',
  icon: 'P',
  disabled: false,
  showLabel: true
})

const emit = defineEmits<{
  print: [templateId: string]
}>()

const isGenerating = ref(false)
const dropdownOpen = ref(false)
const dropdownRef = ref<HTMLElement | null>(null)

const buttonTitle = computed(() => {
  if (props.disabled) return 'Print not available'
  if (isGenerating.value) return 'Generating PDF...'
  if (props.templates.length === 0) return props.label
  if (props.templates.length === 1) return `Print as ${props.templates[0].name}`
  return 'Select print template'
})

function toggleDropdown() {
  dropdownOpen.value = !dropdownOpen.value
}

function handleSingleTemplate() {
  const templateId = props.templates.length > 0 ? props.templates[0].id : 'default'
  emit('print', templateId)
}

function handleSelectTemplate(template: PrintTemplate) {
  dropdownOpen.value = false
  emit('print', template.id)
}

function handleClickOutside(event: MouseEvent) {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    dropdownOpen.value = false
  }
}

function setGenerating(generating: boolean) {
  isGenerating.value = generating
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})

defineExpose({
  setGenerating
})
</script>

<style scoped>
.print-button-container {
  position: relative;
  display: inline-block;
}

.print-button {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
  background: var(--color-surface-variant);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-xs) var(--spacing-sm);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.print-button:hover:not(:disabled) {
  background: var(--color-gray-200);
  border-color: var(--color-border-hover);
}

.theme-dark .print-button:hover:not(:disabled) {
  background: var(--color-gray-700);
}

.print-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.print-button.is-loading {
  pointer-events: none;
}

.print-icon {
  font-size: 1rem;
  line-height: 1;
}

.spinner-small {
  width: 14px;
  height: 14px;
  border: 2px solid var(--color-border);
  border-top-color: var(--color-primary-500);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.dropdown-trigger {
  padding-right: var(--spacing-md);
}

.dropdown-arrow {
  font-size: 0.625rem;
  margin-left: var(--spacing-xs);
  opacity: 0.7;
}

.print-dropdown {
  position: relative;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  right: 0;
  min-width: 180px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  z-index: 100;
  margin-top: var(--spacing-xs);
  overflow: hidden;
}

.dropdown-item {
  display: block;
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background: none;
  border: none;
  text-align: left;
  font-size: 0.875rem;
  color: var(--color-text);
  cursor: pointer;
  transition: background-color var(--transition-fast);
}

.dropdown-item:hover {
  background: var(--color-gray-100);
}

.theme-dark .dropdown-item:hover {
  background: var(--color-gray-800);
}

.dropdown-item + .dropdown-item {
  border-top: 1px solid var(--color-border);
}
</style>
