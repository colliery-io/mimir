<template>
  <div v-if="visible" class="modal-overlay" @click="close" :style="{ zIndex }">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>{{ title }}</h3>
        <button class="modal-close" @click="close">×</button>
      </div>
      <div class="modal-body dnd-content" v-html="content" @click="handleContentClick"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  visible: boolean
  title: string
  content: string
  zIndex?: number
}

const props = withDefaults(defineProps<Props>(), {
  zIndex: 1000
})

const emit = defineEmits<{
  close: []
  'reference-click': [event: { type: string; name: string; source?: string }]
}>()

function close() {
  emit('close')
}

function handleContentClick(event: MouseEvent) {
  const target = event.target as HTMLElement
  
  // Check for cross-reference links by class or data attribute
  // textFormatting.ts creates links with class="cross-ref-link {type}-ref" and data-ref-type
  if (target.classList.contains('cross-ref-link') ||
      target.classList.contains('reference-link') ||
      target.classList.contains('creature-ref') ||
      target.classList.contains('item-ref') ||
      target.classList.contains('spell-ref') ||
      target.classList.contains('condition-ref') ||
      target.classList.contains('race-ref') ||
      target.classList.contains('class-ref') ||
      target.classList.contains('feat-ref') ||
      target.classList.contains('background-ref') ||
      target.classList.contains('action-ref') ||
      target.classList.contains('feature-ref') ||
      target.classList.contains('clickable') ||
      (target.tagName === 'A' && target.hasAttribute('data-ref-type'))) {
    
    event.preventDefault()
    event.stopPropagation()
    
    // Extract reference type and name
    let type = target.getAttribute('data-ref-type') || ''
    
    // Fallback to class-based type detection if no data-ref-type
    if (!type) {
      if (target.classList.contains('creature-ref')) type = 'creature'
      else if (target.classList.contains('item-ref')) type = 'item'
      else if (target.classList.contains('spell-ref')) type = 'spell'
    }
    
    // Try multiple ways to get the name and source
    const name = target.getAttribute('data-ref-name') || 
                 target.getAttribute('data-name') || 
                 target.textContent || ''
    const source = target.getAttribute('data-ref-source') || 
                   target.getAttribute('data-source') || 
                   undefined
                   
    if (name && type) {
      emit('reference-click', { type, name, source })
    }
  }
}
</script>

<!-- Component styles have been moved to centralized CSS files -->