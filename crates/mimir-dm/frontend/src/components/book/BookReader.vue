<template>
  <div id="book-reader" :class="`theme-${currentTheme}`">
    <!-- Mode Switcher -->
    <div class="mode-switcher-bar">
      <div class="mode-switcher">
        <button 
          :class="['mode-button', { active: currentMode === 'reading' }]"
          @click="currentMode = 'reading'"
        >
          Reading
        </button>
        <button 
          :class="['mode-button', { active: currentMode === 'catalog' }]"
          @click="currentMode = 'catalog'"
        >
          Catalog
        </button>
      </div>
    </div>
    
    <!-- Different layouts for different modes -->
    <ThreePanelLayout v-if="currentMode === 'reading'">
      <template #left>
        <BookLibrary
          :library-books="libraryBooks"
          :selected-book="selectedBook"
          :is-loading-library="isLoadingLibrary"
          :is-development="isDevelopment"
          :mode="currentMode"
          @select="selectBook"
          @updateSources="selectedSources = $event"
          @add="addBook"
          @remove="removeBook"
        />
      </template>
      
      <template #center>
        <BookTableOfContents
          v-if="selectedBook && bookContent?.data"
          :sections="bookContent.data"
          :selected-section="selectedSection"
          @select="selectedSection = $event"
          @jump="jumpToEntry"
        />
        <Panel v-else title="Contents" variant="default">
          <div class="empty-toc">
            <p>Select a book to view contents</p>
          </div>
        </Panel>
      </template>
      
      <template #right>
        <BookContentViewer
          :selected-book="selectedBook"
          :content="currentSection"
          :is-loading="isLoading"
          :error="error"
        />
      </template>
    </ThreePanelLayout>
    
    <!-- Two-panel layout for catalog mode -->
    <TwoPanelLayout v-else>
      <template #left>
        <BookLibrary
          :library-books="libraryBooks"
          :selected-book="selectedBook"
          :is-loading-library="isLoadingLibrary"
          :is-development="isDevelopment"
          :mode="currentMode"
          @select="selectBook"
          @updateSources="selectedSources = $event"
          @add="addBook"
          @remove="removeBook"
        />
      </template>
      
      <template #right>
        <CatalogPanel :selected-category="selectedCatalogCategory" :selected-sources="selectedSources" />
      </template>
    </TwoPanelLayout>
    
    <!-- Cross-reference tooltip -->
    <div 
      v-if="tooltipVisible"
      class="cross-ref-tooltip"
      :style="{ left: `${tooltipPosition.x}px`, top: `${tooltipPosition.y}px` }"
      v-html="tooltipContent"
    />
    
    <!-- Cross-reference modal -->
    <div v-if="modalContent.visible" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>{{ modalContent.title }}</h2>
          <button class="modal-close" @click="closeModal">×</button>
        </div>
        <div class="modal-body" v-html="modalContent.content"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useThemeStore } from '../../stores/theme'
import { useBookLibrary } from '../../composables/book/useBookLibrary'
import { useBookContent } from '../../composables/book/useBookContent'
import { useBookNavigation } from '../../composables/book/useBookNavigation'
import { useCrossReferences } from '../../composables/references/useCrossReferences'

// Components
import ThreePanelLayout from '../layout/ThreePanelLayout.vue'
import TwoPanelLayout from '../layout/TwoPanelLayout.vue'
import Panel from '../layout/Panel.vue'
import BookLibrary from './BookLibrary.vue'
import BookTableOfContents from './BookTableOfContents.vue'
import BookContentViewer from './BookContentViewer.vue'
import CatalogPanel from '../catalog/CatalogPanel.vue'

// Theme
const themeStore = useThemeStore()
const currentTheme = computed(() => themeStore.currentTheme)

// Mode state (reading vs catalog)
type AppMode = 'reading' | 'catalog'
const currentMode = ref<AppMode>('reading')

// Catalog state
const selectedCatalogCategory = ref('Spells')
const selectedSources = ref<string[]>([])

// Book library management
const {
  libraryBooks,
  selectedBook,
  isLoadingLibrary,
  isDevelopment,
  loadLibraryBooks,
  addBook,
  removeBook,
  selectBook,
  installDevTestBook
} = useBookLibrary()

// Book content management
const {
  bookContent,
  selectedSection,
  isLoading,
  error,
  loadBookContent,
  jumpToEntry: jumpToEntryBase,
  getCurrentSection
} = useBookContent()

// Navigation
const { scrollToElement } = useBookNavigation()

// Cross-references
const {
  tooltipContent,
  tooltipVisible,
  tooltipPosition,
  modalContent,
  handleCrossRefHover,
  handleCrossRefClick,
  hideTooltip,
  closeModal
} = useCrossReferences()

// Current section content
const currentSection = computed(() => getCurrentSection())

// Jump to entry with scroll
function jumpToEntry(sectionIndex: number, entryId: string) {
  jumpToEntryBase(sectionIndex, entryId)
}

// Watch for book selection changes
watch(selectedBook, (newBook) => {
  if (newBook) {
    loadBookContent(newBook)
  }
})

// Setup cross-reference event handlers
function setupCrossRefHandlers() {
  // Remove old listeners
  document.removeEventListener('mouseover', handleCrossRefHover as any)
  document.removeEventListener('mouseout', hideTooltip)
  document.removeEventListener('click', handleCrossRefClick as any)
  
  // Add new listeners
  document.addEventListener('mouseover', handleCrossRefHover as any)
  document.addEventListener('mouseout', (e) => {
    const target = e.target as HTMLElement
    if (target.classList?.contains('cross-ref-link')) {
      hideTooltip()
    }
  })
  document.addEventListener('click', handleCrossRefClick as any)
}

// Load initial data
onMounted(async () => {
  // Initialize theme - exactly as in original BookApp.vue
  themeStore.applyTheme()
  await themeStore.initThemeSync()
  
  // Install dev test book if in dev mode
  await installDevTestBook()
  
  // Load library books
  await loadLibraryBooks()
  
  // Setup cross-reference handlers
  setupCrossRefHandlers()
})

// Re-setup handlers when content changes
watch([bookContent, selectedSection], () => {
  nextTick(() => {
    setupCrossRefHandlers()
  })
})
</script>

<style>
/* Global theme styles */
#book-reader {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  background: var(--color-background, #0d0d0d);
  color: var(--color-text, #e0e0e0);
}

/* Panel overrides */
.empty-toc {
  padding: var(--spacing-lg, 16px);
  text-align: center;
  color: var(--color-text-secondary, #999);
}

/* Cross-reference links - base styles */
.cross-ref-link {
  cursor: pointer;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
  transition: all 0.2s;
}

.cross-ref-link:hover {
  text-decoration-style: solid;
}

/* Tooltip styles */
.cross-ref-tooltip {
  position: fixed;
  z-index: 10000;
  background: var(--color-surface, #2a2a2a);
  color: var(--color-text, #e0e0e0);
  border: 1px solid var(--color-border, #404040);
  border-radius: 6px;
  padding: 8px 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
  max-width: 300px;
  font-size: 0.9em;
  pointer-events: none;
  line-height: 1.4;
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 20000;
  padding: var(--spacing-xl, 24px);
}

.modal-content {
  background: var(--color-surface, #1a1a1a);
  border: 1px solid var(--color-border, #333);
  border-radius: 8px;
  max-width: 800px;
  max-height: 80vh;
  width: 100%;
  display: flex;
  flex-direction: column;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.8);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg, 16px);
  border-bottom: 1px solid var(--color-border, #333);
}

.modal-header h2 {
  margin: 0;
  color: var(--color-text, #e0e0e0);
}

.modal-close {
  background: transparent;
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
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-lg, 16px);
}

/* Table styles */
.table-wrapper {
  margin: var(--spacing-md, 12px) 0;
  overflow-x: auto;
}

.content-table {
  width: 100%;
  border-collapse: collapse;
  margin: var(--spacing-sm, 8px) 0;
  background: var(--color-background, #0d0d0d);
  border: 1px solid var(--color-border, #333);
}

.content-table caption {
  padding: var(--spacing-sm, 8px);
  font-weight: 600;
  text-align: left;
  color: var(--color-text, #e0e0e0);
  border-bottom: 2px solid var(--color-border, #333);
}

.content-table th,
.content-table td {
  padding: var(--spacing-sm, 8px);
  text-align: left;
  border: 1px solid var(--color-border, #333);
}

.content-table th {
  background: var(--color-surface, #1a1a1a);
  font-weight: 600;
  color: var(--color-text, #e0e0e0);
}

.content-table tr:hover {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
}
</style>

<style>
/* Global styles for book content (non-scoped) - FROM ORIGINAL BookApp.vue */
.book-content {
  padding: 10px 15px;
}

/* Game Mechanics - Subtle, theme-aware formatting */
.dice-roll {
  display: inline-block;
  padding: 1px 4px;
  background-color: rgba(239, 68, 68, 0.1);
  color: var(--color-text);
  border-radius: var(--radius-sm);
  font-weight: 600;
  font-family: 'Courier New', monospace;
}

.damage-roll {
  display: inline-block;
  padding: 1px 4px;
  background-color: rgba(239, 68, 68, 0.1);
  color: var(--color-text);
  border-radius: var(--radius-sm);
  font-weight: 600;
  font-family: 'Courier New', monospace;
}

.d20-check {
  display: inline-block;
  padding: 1px 4px;
  background-color: rgba(239, 68, 68, 0.1);
  color: var(--color-text);
  border-radius: var(--radius-sm);
  font-weight: 600;
  font-family: 'Courier New', monospace;
}

.dc-check {
  display: inline-block;
  padding: 1px 5px;
  background-color: rgba(236, 72, 153, 0.1);
  color: var(--color-text);
  border-radius: var(--radius-sm);
  font-weight: 600;
  text-transform: uppercase;
  font-size: 0.9em;
}

.scaled-value {
  display: inline-block;
  padding: 1px 4px;
  background-color: rgba(16, 185, 129, 0.1);
  color: var(--color-text);
  border-radius: var(--radius-sm);
  font-weight: 600;
  font-family: 'Courier New', monospace;
}

.skill-check {
  display: inline-block;
  padding: 0 3px;
  color: var(--color-primary-600);
  font-weight: 600;
  text-transform: capitalize;
}

.action-name {
  display: inline-block;
  padding: 0 3px;
  color: var(--color-success);
  font-weight: 700;
  text-transform: uppercase;
  font-size: 0.9em;
  letter-spacing: 0.3px;
}

.condition {
  display: inline-block;
  padding: 0 3px;
  color: var(--color-warning);
  font-weight: 600;
  font-style: italic;
}

.status {
  display: inline-block;
  padding: 0 3px;
  color: var(--color-warning);
  font-weight: 600;
  font-style: italic;
}

.chance {
  display: inline-block;
  padding: 1px 4px;
  background-color: var(--color-surface);
  color: var(--color-primary-600);
  border-radius: var(--radius-sm);
  font-weight: 600;
  font-size: 0.95em;
}

.hit-bonus {
  display: inline-block;
  padding: 1px 4px;
  background-color: rgba(34, 197, 94, 0.1);
  color: var(--color-text);
  border-radius: var(--radius-sm);
  font-weight: 600;
  font-family: 'Courier New', monospace;
}

.recharge {
  display: inline-block;
  padding: 1px 4px;
  background-color: var(--color-info-light);
  color: var(--color-info);
  border-radius: var(--radius-sm);
  font-weight: 600;
  font-size: 0.9em;
}

/* Cross-reference styles - from original BookApp.vue */
.book-content .spell-ref {
  color: var(--color-primary-600);
  font-style: italic;
  cursor: help;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
}

.book-content .spell-ref:hover {
  color: var(--color-primary-700);
  text-decoration-color: var(--color-primary-500);
}

.book-content .item-ref {
  color: var(--color-success);
  cursor: help;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
}

.book-content .item-ref:hover {
  color: var(--color-success-dark);
  text-decoration-color: var(--color-success);
}

.book-content .creature-ref {
  color: var(--color-error);
  font-weight: 500;
  cursor: help;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
  text-decoration-color: var(--color-error-bg);
}

.book-content .creature-ref:hover {
  color: var(--color-error-dark);
  text-decoration-color: var(--color-error);
}

.book-content .book-ref {
  color: var(--color-primary-500);
  font-style: italic;
  cursor: help;
}

/* Content wrapper */
.content-wrapper {
  max-width: 850px;
  margin: 0;
  line-height: 1.6;
  font-size: 14px;
  font-family: var(--font-family-base);
}

/* Headers */
.section-content h1, .content-wrapper h1 {
  font-size: 1.7em;
  font-weight: bold;
  margin: 0.67em 0;
  padding: 0;
  border: none;
  font-family: var(--font-family-base);
  color: var(--color-primary-500);
  border-bottom: 2px solid var(--color-primary-500);
  padding-bottom: 0.3em;
}

.section-content h2, .content-wrapper h2 {
  font-size: 1.5em;
  font-weight: bold;
  margin: 0.83em 0;
  padding: 0;
  border: none;
  font-family: var(--font-family-base);
  color: var(--color-primary-600);
  border-bottom: 1px solid var(--color-border);
  padding-bottom: 0.2em;
}

.section-content h3, .content-wrapper h3 {
  font-size: 1.17em;
  font-weight: bold;
  margin: 1em 0;
  padding: 0;
  font-family: var(--font-family-base);
  color: var(--color-primary-400);
  border-left: 3px solid var(--color-primary-500);
  padding-left: 0.5em;
}

.section-content h4, .content-wrapper h4 {
  font-size: 1em;
  font-weight: bold;
  margin: 1.33em 0 0.5em 0;
  padding: 0;
  text-transform: none;
  letter-spacing: normal;
  font-family: var(--font-family-base);
  color: var(--color-text);
  font-style: italic;
}

/* Paragraphs */
.section-content p, .content-wrapper p, .entries p {
  margin: 0 0 5px;
  text-align: left;
  color: var(--color-text);
}

/* Basic inset-readaloud styles */
.inset-readaloud {
  background-color: var(--color-surface);
  border-left: 4px solid var(--color-primary);
  padding: var(--spacing-md);
  margin: var(--spacing-lg) 0;
  border-radius: var(--radius-md);
}

.inset-readaloud p {
  font-style: italic;
  color: var(--color-text-secondary);
}

.inset-readaloud p:first-child {
  margin-top: 0 !important;
}

.inset-readaloud p:last-child {
  margin-bottom: 0 !important;
}

/* Read-aloud boxes - EXACT CSS from original BookApp.vue */
.book-content .inset-readaloud {
  background: var(--gradient-chromatic-subtle, linear-gradient(135deg, rgba(147, 51, 234, 0.15), rgba(59, 130, 246, 0.15), rgba(6, 182, 212, 0.15), rgba(16, 185, 129, 0.15)));
  border: 2px solid var(--color-chromatic-border, rgba(147, 51, 234, 0.4));
  border-radius: var(--radius-lg, 0.5rem);
  padding: var(--spacing-lg, 1.5rem);
  margin: var(--spacing-lg, 1.5rem) 0;
  font-size: inherit;
  line-height: 1.6;
  position: relative;
  overflow: hidden;
}

.book-content .inset-readaloud::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
  animation: shimmer 3s infinite;
}

.book-content .inset-readaloud p {
  text-align: left;
  font-style: italic;
  color: var(--color-text);
  margin: 8px 0;
  position: relative;
  z-index: 1;
}

/* Regular inset boxes - modern card style */
.book-content .inset {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--spacing-md);
  margin: var(--spacing-md) 0;
  font-size: inherit;
  line-height: inherit;
  box-shadow: var(--shadow-sm);
  transition: all 0.3s ease;
}

.book-content .inset:hover {
  box-shadow: var(--shadow-md);
  border-color: var(--color-primary-300);
}

.book-content .inset h4 {
  margin-top: 0;
  margin-bottom: var(--spacing-sm);
  font-weight: bold;
  color: var(--color-primary-500);
}

.book-content .inset p {
  margin: 5px 0;
  color: var(--color-text);
}

@keyframes shimmer {
  to {
    left: 100%;
  }
}

/* Lists */
.book-content ul, .book-content .content-list {
  margin: 10px 0;
  padding-left: 20px;
}

.book-content li {
  margin: 3px 0;
  color: var(--color-text);
}

/* Section and entries */
.section {
  margin-bottom: 0 !important;
}

.entries {
  margin: 5px 0 !important;
}

.entries > h4 {
  margin: 10px 0 5px 0 !important;
}

.content-list {
  padding-left: 40px !important;
  margin: 5px 0 !important;
}

.content-list li {
  margin-bottom: 0 !important;
  line-height: 1.42857143 !important;
}

/* Additional reference styles */
.book-content .race-ref,
.book-content .class-ref,
.book-content .background-ref,
.book-content .feat-ref {
  color: var(--color-primary-500);
  font-style: italic;
  cursor: help;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
}

.book-content .race-ref:hover,
.book-content .class-ref:hover,
.book-content .background-ref:hover,
.book-content .feat-ref:hover {
  color: var(--color-primary-600);
  text-decoration-style: solid;
}

/* Creature stat block styling */
.creature-details .creature-type {
  font-style: italic;
  margin-bottom: 10px;
  color: var(--color-text-secondary);
}

.creature-details .creature-stats {
  border-top: 1px solid var(--color-border);
  border-bottom: 1px solid var(--color-border);
  padding: 10px 0;
  margin: 10px 0;
}

.creature-details .creature-stats p {
  margin: 5px 0;
}

.creature-details .ability-scores {
  margin: 15px 0;
}

.creature-details .ability-scores table {
  width: 100%;
  text-align: center;
  border-collapse: collapse;
}

.creature-details .ability-scores th {
  font-weight: bold;
  padding: 5px;
  border-bottom: 1px solid var(--color-border);
}

.creature-details .ability-scores td {
  padding: 5px;
}

.creature-details .creature-traits,
.creature-details .creature-actions {
  margin-top: 20px;
  border-top: 2px solid var(--color-primary);
  padding-top: 10px;
}

.creature-details .creature-traits h4,
.creature-details .creature-actions h4 {
  color: var(--color-primary);
  margin-bottom: 10px;
  font-size: 1.1em;
}

.creature-details .trait,
.creature-details .action {
  margin-bottom: 10px;
  line-height: 1.4;
}

.reference-details {
  line-height: 1.6;
}

.reference-details p {
  margin: var(--spacing-sm) 0;
}

.reference-details .description {
  margin-top: var(--spacing-md);
  padding-top: var(--spacing-md);
  border-top: 1px solid var(--color-border);
}

.loading-message {
  text-align: center;
  color: var(--color-text-secondary);
  padding: var(--spacing-xl);
}

/* Notes */
.book-content .note {
  font-style: italic;
  color: var(--color-text-secondary);
}

/* Mode Switcher */
.mode-switcher-bar {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 48px;
  background: var(--color-surface, #1a1a1a);
  border-bottom: 1px solid var(--color-border, #333);
  flex-shrink: 0;
}

.mode-switcher {
  display: flex;
  gap: 0;
  background: var(--color-background, #0d0d0d);
  border-radius: 6px;
  padding: 2px;
}

.mode-button {
  padding: 6px 16px;
  background: transparent;
  border: none;
  color: var(--color-text-secondary, #999);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  border-radius: 4px;
  transition: all 0.2s;
}

.mode-button:hover {
  color: var(--color-text, #e0e0e0);
}

.mode-button.active {
  background: var(--color-primary, #4a9eff);
  color: var(--color-background, #0d0d0d);
}
</style>