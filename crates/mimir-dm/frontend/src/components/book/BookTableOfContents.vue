<template>
  <Panel title="Contents" variant="default">
    <div class="toc-list">
      <div v-if="sections && sections.length > 0">
        <div v-for="(section, index) in sections" :key="index">
          <div 
            @click="$emit('select', index)"
            :class="['toc-item', { active: selectedSection === index }]"
          >
            <span class="toc-name">{{ getSectionName(section) }}</span>
          </div>
          
          <!-- Show sub-entries if available -->
          <div v-if="section.entries && Array.isArray(section.entries)" class="toc-sub-entries">
            <div 
              v-for="(entry, subIndex) in getSubEntries(section)" 
              :key="`${index}-${subIndex}`"
              class="toc-sub-item"
              :style="{ paddingLeft: `${30 + (entry.level * 15)}px` }"
              @click.stop="$emit('jump', index, entry.id)"
            >
              {{ entry.name }}
            </div>
          </div>
        </div>
      </div>
      <div v-else class="empty-toc">
        No content available
      </div>
    </div>
  </Panel>
</template>

<script setup lang="ts">
import Panel from '../layout/Panel.vue'
import { useBookNavigation } from '../../composables/book/useBookNavigation'
import { useBookContent } from '../../composables/book/useBookContent'
import type { BookSection } from '../../types/book'

interface Props {
  sections: BookSection[]
  selectedSection: number
}

interface Emits {
  (e: 'select', index: number): void
  (e: 'jump', sectionIndex: number, entryId: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const { getSubEntries } = useBookNavigation()
const { getSectionName } = useBookContent()
</script>

<style scoped>
.toc-list {
  height: 100%;
  overflow-y: auto;
}

.empty-toc {
  padding: var(--spacing-lg, 16px);
  text-align: center;
  color: var(--color-text-secondary, #999);
}

.toc-item {
  padding: var(--spacing-sm, 8px) var(--spacing-md, 12px);
  cursor: pointer;
  border-left: 3px solid transparent;
  transition: all 0.2s;
}

.toc-item:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
}

.toc-item.active {
  background: var(--color-primary-alpha, rgba(74, 158, 255, 0.1));
  border-left-color: var(--color-primary, #4a9eff);
}

.toc-name {
  color: var(--color-text, #e0e0e0);
  font-size: 0.9rem;
  font-weight: 500;
}

.toc-sub-entries {
  background: var(--color-background-alpha, rgba(0, 0, 0, 0.2));
}

.toc-sub-item {
  padding: var(--spacing-xs, 4px) var(--spacing-md, 12px);
  cursor: pointer;
  color: var(--color-text-secondary, #999);
  font-size: 0.85rem;
  transition: all 0.2s;
}

.toc-sub-item:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
  color: var(--color-text, #e0e0e0);
}
</style>