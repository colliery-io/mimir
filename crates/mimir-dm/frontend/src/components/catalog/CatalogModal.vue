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
  
  // Check if clicked element is a reference span
  if (target.classList.contains('creature-ref') || 
      target.classList.contains('item-ref') || 
      target.classList.contains('spell-ref')) {
    
    event.preventDefault()
    event.stopPropagation()
    
    // Extract reference type and name
    let type = ''
    if (target.classList.contains('creature-ref')) type = 'creature'
    else if (target.classList.contains('item-ref')) type = 'item'
    else if (target.classList.contains('spell-ref')) type = 'spell'
    
    // Try multiple ways to get the name and source
    const name = target.getAttribute('data-ref-name') || 
                 target.getAttribute('data-name') || 
                 target.textContent || ''
    const source = target.getAttribute('data-ref-source') || 
                   target.getAttribute('data-source') || 
                   undefined
    
    console.log('Reference clicked:', { type, name, source, target })
    
    if (name && type) {
      emit('reference-click', { type, name, source })
    }
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.modal-content {
  background: var(--color-surface, #1a1a1a);
  border: 1px solid var(--color-border, #333);
  border-radius: 8px;
  max-width: 800px;
  max-height: 80vh;
  width: 90%;
  display: flex;
  flex-direction: column;
  animation: slideUp 0.3s;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.modal-header {
  padding: var(--spacing-lg, 16px);
  border-bottom: 1px solid var(--color-border, #333);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header h3 {
  margin: 0;
  color: var(--color-text, #e0e0e0);
  font-size: 1.25rem;
}

.modal-close {
  background: none;
  border: none;
  color: var(--color-text-secondary, #999);
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
}

.modal-close:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.1));
  color: var(--color-text, #e0e0e0);
}

.modal-body {
  padding: var(--spacing-lg, 16px);
  overflow-y: auto;
  flex: 1;
}

/* Enhanced spell details styles */
:deep(.spell-details) {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

:deep(.spell-details.enhanced) {
  gap: 1.75rem;
}

:deep(.spell-header-section) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 0.75rem;
  border-bottom: 2px solid var(--color-primary, #4a9eff);
}

:deep(.spell-level-school) {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--color-primary, #4a9eff);
  text-transform: capitalize;
}

:deep(.spell-tags) {
  display: flex;
  gap: 0.5rem;
}

:deep(.spell-tag) {
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 0.85rem;
  font-weight: 500;
}

:deep(.spell-tag.ritual) {
  background: rgba(168, 199, 255, 0.2);
  color: #a8c7ff;
  border: 1px solid rgba(168, 199, 255, 0.3);
}

:deep(.spell-tag.concentration) {
  background: rgba(74, 158, 255, 0.2);
  color: var(--color-primary, #4a9eff);
  border: 1px solid rgba(74, 158, 255, 0.3);
}

:deep(.spell-tag.healing) {
  background: rgba(108, 207, 127, 0.15);
  color: #6bcf7f;
  border-color: rgba(108, 207, 127, 0.3);
}

:deep(.spell-tag.summoning) {
  background: rgba(162, 155, 254, 0.15);
  color: #a29bfe;
  border-color: rgba(162, 155, 254, 0.3);
}

:deep(.spell-tag.light) {
  background: rgba(255, 218, 121, 0.15);
  color: #ffda79;
  border-color: rgba(255, 218, 121, 0.3);
}

:deep(.spell-tag.scaling) {
  background: rgba(255, 159, 67, 0.15);
  color: #ff9f43;
  border-color: rgba(255, 159, 67, 0.3);
}

:deep(.spell-properties-grid) {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.02);
  border-radius: 6px;
  border: 1px solid var(--color-border-light, #222);
}

:deep(.property-item) {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

:deep(.property-item.full-width) {
  grid-column: 1 / -1;
}

:deep(.property-label) {
  font-size: 0.85rem;
  color: var(--color-text-secondary, #999);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

:deep(.property-value) {
  color: var(--color-text, #e0e0e0);
  font-size: 0.95rem;
}

/* Combat Mechanics Section */
:deep(.spell-combat-section) {
  margin: 1.5rem 0;
}

:deep(.spell-combat-section h4) {
  color: var(--color-accent, #6bcf7f);
  font-size: 1rem;
  font-weight: 600;
  margin: 0 0 1rem 0;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--color-border-light, #333);
}

:deep(.combat-mechanics-grid) {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

:deep(.combat-item) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: rgba(255, 255, 255, 0.02);
  border-radius: 4px;
  border: 1px solid var(--color-border-light, #222);
}

:deep(.combat-label) {
  font-weight: 600;
  color: var(--color-text-secondary, #aaa);
  font-size: 0.875rem;
  min-width: 60px;
}

:deep(.combat-value) {
  font-weight: 500;
  color: var(--color-text, #e0e0e0);
}

/* Damage type badges */
:deep(.damage-type) {
  display: inline-block;
  padding: 3px 8px;
  margin: 0 3px 3px 0;
  border-radius: 4px;
  font-size: 0.8rem;
  font-weight: 500;
  border: 1px solid;
}

:deep(.damage-type.fire) {
  background: rgba(255, 107, 107, 0.15);
  color: #ff6b6b;
  border-color: rgba(255, 107, 107, 0.3);
}

:deep(.damage-type.cold) {
  background: rgba(116, 185, 255, 0.15);
  color: #74b9ff;
  border-color: rgba(116, 185, 255, 0.3);
}

:deep(.damage-type.acid) {
  background: rgba(108, 207, 127, 0.15);
  color: #6bcf7f;
  border-color: rgba(108, 207, 127, 0.3);
}

/* Condition badges */
:deep(.condition-badge) {
  display: inline-block;
  padding: 3px 8px;
  margin: 0 3px 3px 0;
  border-radius: 4px;
  font-size: 0.8rem;
  font-weight: 500;
  background: rgba(255, 159, 67, 0.15);
  color: #ff9f43;
  border: 1px solid rgba(255, 159, 67, 0.3);
}

/* Cantrip scaling styles */
:deep(.spell-scaling-section) {
  margin: 1.5rem 0;
}

:deep(.spell-scaling-section h4) {
  color: var(--color-accent, #6bcf7f);
  font-size: 1rem;
  font-weight: 600;
  margin: 0 0 1rem 0;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--color-border-light, #333);
}

:deep(.cantrip-scaling) {
  padding: 1rem;
  background: rgba(255, 255, 255, 0.02);
  border-radius: 6px;
  border: 1px solid var(--color-border-light, #222);
}

:deep(.scaling-label) {
  font-weight: 600;
  color: var(--color-text-secondary, #aaa);
  margin-right: 0.5rem;
}

:deep(.scaling-progression) {
  font-family: 'Courier New', monospace;
  color: var(--color-text, #e0e0e0);
}

:deep(.scaling-dice) {
  color: #ff6b6b;
  font-weight: 600;
}

:deep(.scaling-level) {
  color: var(--color-text-secondary, #aaa);
  font-size: 0.875rem;
}

:deep(.spell-description-section),
:deep(.spell-higher-level-section) {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

:deep(.spell-description-section h4),
:deep(.spell-higher-level-section h4) {
  color: var(--color-text, #e0e0e0);
  font-size: 1rem;
  font-weight: 600;
  margin: 0;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--color-border-light, #333);
}

:deep(.description-text) {
  color: var(--color-text-secondary, #ccc);
  line-height: 1.6;
}

:deep(.description-text p) {
  margin: 0 0 0.75rem 0;
}

:deep(.description-text p:last-child) {
  margin-bottom: 0;
}

:deep(.spell-footer) {
  padding-top: 1rem;
  border-top: 1px solid var(--color-border-light, #222);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

:deep(.source-info) {
  color: var(--color-text-dim, #888);
  font-size: 0.875rem;
  font-style: italic;
}

/* All D&D content styles (insets, formatting tags, etc.) are now in dnd-content.css */
/* Modal-specific styles only below */

:deep(.class-header-info) {
  margin-bottom: 1rem;
}

:deep(.class-properties) {
  margin: 1rem 0;
}

:deep(.parent-class-info) {
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--color-border-light, #333);
}

:deep(.parent-class-info h4) {
  color: var(--color-text-secondary, #999);
  margin-bottom: 0.5rem;
}

:deep(.subclass-section) {
  margin-top: 1rem;
  padding: 0.75rem;
  background: rgba(255, 255, 255, 0.02);
  border-radius: 4px;
  border-left: 3px solid var(--color-primary-dim, #3a7fbf);
}

:deep(.subclass-section h4) {
  color: var(--color-primary, #4a9eff);
  margin-top: 0;
  margin-bottom: 0.5rem;
  font-size: 0.95rem;
}

:deep(.feature-list),
:deep(.spell-list) {
  margin: 0.5rem 0;
  padding-left: 1.5rem;
}

:deep(.feature-list li) {
  color: var(--color-text, #e0e0e0);
  margin-bottom: 0.25rem;
}

:deep(.spell-item) {
  color: var(--color-spell, #a8c7ff);
  font-style: italic;
  margin-bottom: 0.25rem;
}

/* Spell specific styles */
:deep(.spell-level) {
  color: var(--color-primary, #4a9eff);
  font-weight: 600;
}

:deep(.spell-school) {
  color: var(--color-text-secondary, #999);
  font-style: italic;
}

/* Monster specific styles */
:deep(.creature-stats) {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 0.5rem;
  margin: 1rem 0;
  padding: 0.75rem;
  background: rgba(255, 255, 255, 0.02);
  border-radius: 4px;
}

:deep(.stat) {
  text-align: center;
}

:deep(.stat-label) {
  color: var(--color-text-secondary, #999);
  font-size: 0.75rem;
  text-transform: uppercase;
}

:deep(.stat-value) {
  color: var(--color-text, #e0e0e0);
  font-weight: 600;
  font-size: 1.1rem;
}

:deep(.stat-modifier) {
  color: var(--color-text-dim, #666);
  font-size: 0.875rem;
}

/* Enhanced equipment/item styles */
:deep(.item-details) {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

:deep(.item-details.enhanced) {
  gap: 1.75rem;
}

:deep(.item-header-section) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 0.75rem;
  border-bottom: 2px solid var(--color-primary, #4a9eff);
}

:deep(.item-type-rarity) {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--color-primary, #4a9eff);
  text-transform: capitalize;
}

:deep(.item-tags) {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

:deep(.item-tag) {
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 0.85rem;
  font-weight: 500;
  border: 1px solid;
}

:deep(.item-tag.attunement) {
  background: rgba(168, 199, 255, 0.2);
  color: #a8c7ff;
  border-color: rgba(168, 199, 255, 0.3);
}

:deep(.item-tag.tier) {
  background: rgba(162, 155, 254, 0.15);
  color: #a29bfe;
  border-color: rgba(162, 155, 254, 0.3);
}

:deep(.item-tag.misc) {
  background: rgba(255, 159, 67, 0.15);
  color: #ff9f43;
  border-color: rgba(255, 159, 67, 0.3);
}

:deep(.item-properties-grid) {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.02);
  border-radius: 6px;
  border: 1px solid var(--color-border-light, #222);
}

/* Combat mechanics section for items/weapons */
:deep(.item-combat-section) {
  margin: 1.5rem 0;
}

:deep(.item-combat-section h4) {
  color: var(--color-accent, #6bcf7f);
  font-size: 1rem;
  font-weight: 600;
  margin: 0 0 1rem 0;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--color-border-light, #333);
}

:deep(.combat-properties-grid) {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1rem;
}

:deep(.combat-item) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: rgba(255, 255, 255, 0.02);
  border-radius: 4px;
  border: 1px solid var(--color-border-light, #222);
}

:deep(.combat-label) {
  font-weight: 600;
  color: var(--color-text-secondary, #aaa);
  font-size: 0.875rem;
  min-width: 60px;
}

:deep(.combat-value) {
  font-weight: 500;
  color: var(--color-text, #e0e0e0);
}

/* Magic item properties section */
:deep(.item-magic-section) {
  margin: 1.5rem 0;
}

:deep(.item-magic-section h4) {
  color: var(--color-magic, #a29bfe);
  font-size: 1rem;
  font-weight: 600;
  margin: 0 0 1rem 0;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--color-border-light, #333);
}

:deep(.magic-properties-grid) {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1rem;
}

:deep(.magic-item) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: rgba(162, 155, 254, 0.05);
  border-radius: 4px;
  border: 1px solid rgba(162, 155, 254, 0.2);
}

:deep(.magic-item.full-width) {
  grid-column: 1 / -1;
}

:deep(.magic-label) {
  font-weight: 600;
  color: var(--color-magic, #a29bfe);
  font-size: 0.875rem;
}

:deep(.magic-value) {
  font-weight: 500;
  color: var(--color-text, #e0e0e0);
}

/* Container and transport section */
:deep(.item-container-section) {
  margin: 1.5rem 0;
}

:deep(.item-container-section h4) {
  color: var(--color-container, #fdcb6e);
  font-size: 1rem;
  font-weight: 600;
  margin: 0 0 1rem 0;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--color-border-light, #333);
}

:deep(.container-properties-grid) {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

:deep(.container-item) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: rgba(253, 203, 110, 0.05);
  border-radius: 4px;
  border: 1px solid rgba(253, 203, 110, 0.2);
}

:deep(.container-item.full-width) {
  grid-column: 1 / -1;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.5rem;
}

:deep(.container-label) {
  font-weight: 600;
  color: var(--color-container, #fdcb6e);
  font-size: 0.875rem;
}

:deep(.container-value) {
  font-weight: 500;
  color: var(--color-text, #e0e0e0);
}

:deep(.pack-contents-list) {
  margin: 0;
  padding-left: 1.5rem;
  color: var(--color-text-secondary, #ccc);
}

:deep(.pack-contents-list li) {
  margin-bottom: 0.25rem;
}

/* Light source section */
:deep(.item-light-section) {
  margin: 1.5rem 0;
}

:deep(.item-light-section h4) {
  color: var(--color-light, #ffda79);
  font-size: 1rem;
  font-weight: 600;
  margin: 0 0 1rem 0;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--color-border-light, #333);
}

:deep(.light-properties-grid) {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
}

:deep(.light-item) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: rgba(255, 218, 121, 0.05);
  border-radius: 4px;
  border: 1px solid rgba(255, 218, 121, 0.2);
}

:deep(.light-label) {
  font-weight: 600;
  color: var(--color-light, #ffda79);
  font-size: 0.875rem;
}

:deep(.light-value) {
  font-weight: 500;
  color: var(--color-text, #e0e0e0);
}

/* Enhanced damage type badges for equipment */
:deep(.damage-dice) {
  color: #ff6b6b;
  font-weight: 600;
  font-family: 'Courier New', monospace;
  margin-right: 0.5rem;
}

:deep(.damage-type.p) {
  background: rgba(189, 195, 199, 0.15);
  color: #bdc3c7;
  border: 1px solid rgba(189, 195, 199, 0.3);
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.8rem;
  text-transform: capitalize;
}

:deep(.damage-type.b) {
  background: rgba(155, 89, 182, 0.15);
  color: #9b59b6;
  border: 1px solid rgba(155, 89, 182, 0.3);
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.8rem;
  text-transform: capitalize;
}

:deep(.damage-type.s) {
  background: rgba(231, 76, 60, 0.15);
  color: #e74c3c;
  border: 1px solid rgba(231, 76, 60, 0.3);
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.8rem;
  text-transform: capitalize;
}

/* Weapon property styling */
:deep(.weapon-property) {
  display: inline-block;
  padding: 3px 8px;
  margin: 0 3px 3px 0;
  border-radius: 4px;
  font-size: 0.8rem;
  font-weight: 500;
  background: rgba(116, 185, 255, 0.1);
  color: #74b9ff;
  border: 1px solid rgba(116, 185, 255, 0.2);
}

:deep(.weapon-property.finesse) {
  background: rgba(108, 207, 127, 0.1);
  color: #6bcf7f;
  border-color: rgba(108, 207, 127, 0.2);
}

:deep(.weapon-property.heavy) {
  background: rgba(231, 76, 60, 0.1);
  color: #e74c3c;
  border-color: rgba(231, 76, 60, 0.2);
}

:deep(.weapon-property.light) {
  background: rgba(255, 218, 121, 0.1);
  color: #ffda79;
  border-color: rgba(255, 218, 121, 0.2);
}

/* Reference link styles */
:deep(.creature-ref),
:deep(.item-ref),
:deep(.spell-ref) {
  color: var(--color-primary, #4a9eff);
  cursor: pointer;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
  transition: all 0.2s;
}

:deep(.creature-ref:hover),
:deep(.item-ref:hover),
:deep(.spell-ref:hover) {
  color: var(--color-primary-bright, #6ab7ff);
  text-decoration-style: solid;
  background: rgba(74, 158, 255, 0.1);
  padding: 0 2px;
  border-radius: 2px;
}

:deep(.creature-ref) {
  color: #ff9f43;
}

:deep(.creature-ref:hover) {
  color: #ffb366;
  background: rgba(255, 159, 67, 0.1);
}

:deep(.item-ref) {
  color: #6bcf7f;
}

:deep(.item-ref:hover) {
  color: #8ed99f;
  background: rgba(108, 207, 127, 0.1);
}

:deep(.spell-ref) {
  color: #a8c7ff;
}

:deep(.spell-ref:hover) {
  color: #c5d9ff;
  background: rgba(168, 199, 255, 0.1);
}
</style>