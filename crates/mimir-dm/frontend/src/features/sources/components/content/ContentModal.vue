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
  
  // Check if clicked element is a clickable reference
  if (target.classList.contains('clickable')) {
    event.preventDefault()
    event.stopPropagation()
    
    // Get reference info from data attributes
    const type = target.getAttribute('data-ref-type') || ''
    const name = target.getAttribute('data-ref-name') || target.textContent || ''
    const source = target.getAttribute('data-ref-source') || undefined
    
    console.log('Reference clicked:', { type, name, source, target })
    
    if (name && type) {
      emit('reference-click', { type, name, source })
    }
  }
}
</script>

<!-- Component styles have been moved to centralized CSS files -->