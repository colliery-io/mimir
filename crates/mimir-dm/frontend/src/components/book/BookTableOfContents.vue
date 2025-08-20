<template>
  <Panel title="Contents" variant="default">
    <div class="toc-list">
      <div v-if="sections && sections.length > 0">
        <div v-for="(section, index) in sections" :key="index">
          <div 
            @click="$emit('select', index)"
            :class="['toc-item', { active: selectedSection === index, 'has-children': hasSubEntries(section) }]"
          >
            <!-- Chevron for collapsible sections -->
            <span 
              v-if="hasSubEntries(section)"
              class="toc-chevron"
              :class="{ expanded: expandedSections.has(index) }"
              @click.stop="toggleSection(index)"
            >
              ▶
            </span>
            <span 
              v-else
              class="toc-chevron-spacer"
            ></span>
            <span class="toc-name">{{ getSectionName(section) }}</span>
          </div>
          
          <!-- Show sub-entries if expanded -->
          <div v-if="hasSubEntries(section) && expandedSections.has(index)" class="toc-sub-entries">
            <template v-for="(entry, subIndex) in getSubEntries(section)" :key="`${index}-${subIndex}`">
              <div 
                class="toc-sub-item"
                :class="{ 'has-children': entry.children && entry.children.length > 0 }"
                :style="{ paddingLeft: `${30 + (entry.level * 15)}px` }"
                @click.stop="$emit('jump', index, entry.id)"
              >
                <!-- Chevron for collapsible sub-sections -->
                <span 
                  v-if="entry.children && entry.children.length > 0"
                  class="toc-chevron"
                  :class="{ expanded: expandedSections.has(`${index}-${subIndex}`) }"
                  @click.stop="toggleSection(`${index}-${subIndex}`)"
                >
                  ▶
                </span>
                <span 
                  v-else
                  class="toc-chevron-spacer"
                ></span>
                <span class="toc-sub-name">{{ entry.name }}</span>
              </div>
              
              <!-- Nested sub-entries -->
              <div v-if="entry.children && entry.children.length > 0 && expandedSections.has(`${index}-${subIndex}`)" class="toc-nested-entries">
                <div 
                  v-for="(child, childIndex) in entry.children"
                  :key="`${index}-${subIndex}-${childIndex}`"
                  class="toc-nested-item"
                  :style="{ paddingLeft: `${45 + ((entry.level + 1) * 15)}px` }"
                  @click.stop="$emit('jump', index, child.id)"
                >
                  {{ child.name }}
                </div>
              </div>
            </template>
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
import { ref } from 'vue'
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

// Track which sections are expanded (using string keys to support nested sections)
const expandedSections = ref<Set<string | number>>(new Set())

// Toggle section expansion
function toggleSection(index: string | number) {
  if (expandedSections.value.has(index)) {
    expandedSections.value.delete(index)
  } else {
    expandedSections.value.add(index)
  }
  // Trigger reactivity
  expandedSections.value = new Set(expandedSections.value)
}

// Check if section has sub-entries
function hasSubEntries(section: BookSection): boolean {
  return !!(section.entries && Array.isArray(section.entries) && getSubEntries(section).length > 0)
}
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
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.toc-item:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
}

.toc-item.active {
  background: var(--color-primary-alpha, rgba(74, 158, 255, 0.1));
  border-left-color: var(--color-primary, #4a9eff);
}

.toc-chevron {
  color: var(--color-text-secondary, #999);
  font-size: 0.7rem;
  width: 12px;
  height: 12px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: transform 0.2s;
  flex-shrink: 0;
}

.toc-chevron:hover {
  color: var(--color-text, #e0e0e0);
}

.toc-chevron.expanded {
  transform: rotate(90deg);
}

.toc-chevron-spacer {
  width: 12px;
  height: 12px;
  display: inline-block;
  flex-shrink: 0;
}

.toc-name {
  color: var(--color-text, #e0e0e0);
  font-size: 0.9rem;
  font-weight: 500;
  flex: 1;
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
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.toc-sub-item:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
  color: var(--color-text, #e0e0e0);
}

.toc-sub-name {
  flex: 1;
}

.toc-nested-entries {
  background: var(--color-background-alpha, rgba(0, 0, 0, 0.15));
}

.toc-nested-item {
  padding: var(--spacing-xs, 4px) var(--spacing-md, 12px);
  cursor: pointer;
  color: var(--color-text-dim, #777);
  font-size: 0.8rem;
  transition: all 0.2s;
}

.toc-nested-item:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
  color: var(--color-text-secondary, #999);
}
</style>