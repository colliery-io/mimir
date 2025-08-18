<template>
  <div id="rules-app" :class="`theme-${currentTheme}`">
    <div class="rules-container">
      <!-- Header -->
      <header class="rules-header">
        <div class="header-content">
          <h1 class="app-title">Reference Library</h1>
          <div class="header-actions">
            <!-- Add Book Button -->
            <button @click="handleAddBook" class="add-book-btn">
              + Add Book
            </button>
          </div>
        </div>
      </header>

      <!-- Main Content - Three Panel Layout -->
      <div class="rules-body">
        <!-- Panel 1: Book List -->
        <aside class="books-panel">
          <div class="panel-header">
            <h3 class="panel-title">My Books</h3>
          </div>
          <div class="books-list">
            <div 
              v-for="book in libraryBooks" 
              :key="book.id"
              class="book-item"
              @click="selectedBook = book"
              :class="{ active: selectedBook?.id === book.id }"
            >
              <div class="book-info-wrapper">
                <span class="book-name">{{ book.name }}</span>
                <span class="book-meta">
                  <span v-if="book.id === 'test-book'" class="dev-badge">DEV</span>
                  {{ book.image_count }} images
                </span>
              </div>
              <button 
                @click.stop="removeBook(book)" 
                class="remove-book-btn"
                title="Remove book"
              >
                ×
              </button>
            </div>
            <div v-if="libraryBooks.length === 0" class="no-books">
              No books added yet
            </div>
          </div>
        </aside>

        <!-- Panel 2: Table of Contents -->
        <nav class="toc-panel" v-if="selectedBook && bookContent">
          <div class="panel-header">
            <h3 class="panel-title">Contents</h3>
          </div>
          <div class="toc-list">
            <div v-if="bookContent.data && Array.isArray(bookContent.data)">
              <div v-for="(section, index) in bookContent.data" :key="index">
                <div 
                  @click="selectedSection = index"
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
                    @click.stop="jumpToEntry(index, entry.id)"
                  >
                    {{ entry.name }}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </nav>

        <!-- Panel 3: Content Viewer -->
        <main class="content-panel">
          <div v-if="!selectedBook" class="welcome-message">
            <h2>Welcome to the Reference Library</h2>
            <p>Add books to your library to start reading.</p>
            <p>Click "Add Book" to import book archives (tar.gz files).</p>
          </div>
          
          <div v-else-if="isLoading" class="loading-container">
            <p>Loading book content...</p>
          </div>
          
          <div v-else-if="bookContent && bookContent.data" class="book-content">
            <div v-if="bookContent.data[selectedSection]" class="content-wrapper">
              <div v-html="renderSection(bookContent.data[selectedSection])"></div>
            </div>
          </div>
          
          <div v-else class="error-message">
            <h2>Failed to load book</h2>
            <p>The book content could not be loaded.</p>
          </div>
        </main>
      </div>
    </div>
    
    <!-- Cross-reference Tooltip -->
    <div 
      v-if="tooltipVisible" 
      class="cross-ref-tooltip"
      :style="{ left: tooltipPosition.x + 'px', top: tooltipPosition.y + 'px' }"
    >
      <div v-html="tooltipContent"></div>
    </div>
    
    <!-- Cross-reference Modal -->
    <div v-if="modalVisible" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>{{ modalTitle }}</h2>
          <button class="modal-close" @click="closeModal">×</button>
        </div>
        <div class="modal-body">
          <div v-if="modalContent" v-html="renderModalContent(modalContent)"></div>
          <div v-else class="loading-message">Loading reference data...</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { useThemeStore } from './stores/theme'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

// Types
interface BookInfo {
  id: string             // Book ID (e.g., "phb", "dmg")
  name: string           // Display name
  size_bytes: number
  image_count: number
}

// Initialize theme store
const themeStore = useThemeStore()
const currentTheme = computed(() => themeStore.currentTheme)

// State
const searchQuery = ref('')
const isLoading = ref(false)

// Book Library
const libraryBooks = ref<BookInfo[]>([])
const selectedBook = ref<BookInfo | null>(null)
const bookContent = ref<any>(null)
const selectedSection = ref<number>(0)

// Cross-reference state
const tooltipVisible = ref(false)
const tooltipContent = ref('')
const tooltipPosition = ref({ x: 0, y: 0 })
const modalVisible = ref(false)
const modalContent = ref<any>(null)
const modalTitle = ref('')
let hoverTimeout: number | null = null

// Load library books
const loadLibraryBooks = async () => {
  try {
    const response = await invoke<{ success: boolean; data: BookInfo[] }>('list_library_books')
    if (response.success) {
      libraryBooks.value = response.data
    }
  } catch (error) {
    console.error('Failed to load library books:', error)
  }
}

// Load book content
const loadBookContent = async (book: BookInfo) => {
  try {
    console.log('Loading book content for:', book.id, book)
    isLoading.value = true
    const response = await invoke<{ success: boolean; data: any; message?: string }>('get_book_content', {
      bookId: book.id
    })
    console.log('Book content response:', response)
    if (response.success) {
      console.log('Book content loaded successfully:', response.data)
      bookContent.value = response.data
      selectedSection.value = 0
    } else {
      console.error('Failed to load book content:', response.message)
      console.error('Full response:', response)
      bookContent.value = null
    }
  } catch (error) {
    console.error('Error loading book content:', error)
    bookContent.value = null
  } finally {
    isLoading.value = false
  }
}

// Watch for book selection changes
watch(selectedBook, (newBook) => {
  if (newBook) {
    loadBookContent(newBook)
  } else {
    bookContent.value = null
  }
})

// Handle Add Book button click
const handleAddBook = async () => {
  console.log('Add Book button clicked')
  try {
    console.log('Opening file dialog...')
    // Open file dialog first
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Book Archive',
        extensions: ['tar.gz', 'gz']
      }],
      title: 'Select a book archive to add to your library'
    })
    
    console.log('File selected:', selected)
    
    if (selected) {
      // Handle both string and array returns
      const filePath = Array.isArray(selected) ? selected[0] : selected
      
      console.log('Calling backend to upload archive:', filePath)
      // Call backend to upload and extract the archive
      const response = await invoke<{ success: boolean; data?: BookInfo; message?: string }>('upload_book_archive', {
        archivePath: filePath
      })
      
      console.log('Backend response:', response)
      
      if (response.success && response.data) {
        console.log('Book added successfully:', response.data)
        // Reload books list
        await loadLibraryBooks()
      } else {
        console.error('Failed to add book:', response.message)
        alert(`Failed to add book: ${response.message}`)
      }
    } else {
      console.log('No file selected')
    }
  } catch (error) {
    console.error('Error in handleAddBook:', error)
    alert(`Error adding book to library: ${error}`)
  }
}

// Remove book from library
const removeBook = async (book: BookInfo) => {
  if (!confirm(`Remove "${book.name}" from your library?`)) {
    return
  }
  
  try {
    const response = await invoke<{ success: boolean; message?: string }>('remove_book_from_library', {
      bookId: book.id
    })
    
    if (response.success) {
      console.log('Book removed successfully')
      // Clear selection if it was the removed book
      if (selectedBook.value?.id === book.id) {
        selectedBook.value = null
        bookContent.value = null
      }
      // Reload books list
      await loadLibraryBooks()
    } else {
      console.error('Failed to remove book:', response.message)
      alert(`Failed to remove book: ${response.message}`)
    }
  } catch (error) {
    console.error('Error removing book:', error)
    alert('Error removing book from library')
  }
}

// Get section name for table of contents
const getSectionName = (section: any): string => {
  if (typeof section === 'string') return section
  if (section.name) return section.name
  if (section.type === 'section' && section.entries && section.entries[0]) {
    if (typeof section.entries[0] === 'string') {
      return section.entries[0].substring(0, 50) + '...'
    }
  }
  return 'Untitled Section'
}

// Get sub-entries for nested TOC display (like 5etools)
const getSubEntries = (section: any): any[] => {
  const subEntries: any[] = []
  if (!section.entries) return subEntries
  
  section.entries.forEach((entry: any, index: number) => {
    if (entry && typeof entry === 'object') {
      if (entry.type === 'entries' && entry.name) {
        subEntries.push({ 
          name: entry.name, 
          id: `entry-${index}`,
          level: 1
        })
      } else if (entry.type === 'section' && entry.name) {
        subEntries.push({ 
          name: entry.name, 
          id: `section-${index}`,
          level: 0
        })
      }
    }
  })
  
  return subEntries
}

// Jump to a specific entry in the content
const jumpToEntry = (sectionIndex: number, entryId: string) => {
  // First, switch to the correct section
  selectedSection.value = sectionIndex
  
  // Wait for the DOM to update, then scroll to the element
  nextTick(() => {
    const element = document.getElementById(entryId)
    if (element) {
      element.scrollIntoView({ behavior: 'smooth', block: 'start' })
    }
  })
}

// Process all 5etools formatting tags
const processFormatting = (text: string): string => {
    if (!text) return ''
    
    // Process formatting tags like {@b bold}, {@i italic}, etc.
    let processed = text
    
    // Bold text {@b text} or {@bold text}
    processed = processed.replace(/{@(?:b|bold)\s+([^}]+)}/g, '<strong>$1</strong>')
    
    // Italic text {@i text} or {@italic text}
    processed = processed.replace(/{@(?:i|italic)\s+([^}]+)}/g, '<em>$1</em>')
    
    // Strike-through {@s text} or {@strike text}
    processed = processed.replace(/{@(?:s|strike)\s+([^}]+)}/g, '<s>$1</s>')
    
    // Underline {@u text} or {@underline text}
    processed = processed.replace(/{@(?:u|underline)\s+([^}]+)}/g, '<u>$1</u>')
    
    // Code {@code text}
    processed = processed.replace(/{@code\s+([^}]+)}/g, '<code>$1</code>')
    
    // Note {@note text}
    processed = processed.replace(/{@note\s+([^}]+)}/g, '<span class="note">$1</span>')
    
    // Dice rolls {@dice 1d20+5}
    processed = processed.replace(/{@dice\s+([^}]+)}/g, '<span class="dice-roll">$1</span>')
    
    // Damage rolls {@damage 2d6}
    processed = processed.replace(/{@damage\s+([^}]+)}/g, '<span class="damage-roll">$1</span>')
    
    // d20 rolls {@d20 15}
    processed = processed.replace(/{@d20\s+(\d+)}/g, '<span class="d20-check">d20 ≥ $1</span>')
    
    // DC checks {@dc 15}
    processed = processed.replace(/{@dc\s+(\d+)}/g, '<span class="dc-check">DC $1</span>')
    
    // Scaled damage {@scaledamage 2d6|1-9|1d6}
    processed = processed.replace(/{@scaledamage\s+([^|}\s]+)(?:\|([^|}]+))?(?:\|([^}]+))?}/g, 
      (match, baseDamage) => `<span class="scaled-value">${baseDamage}</span>`)
    
    // Scaled dice {@scaledice 1d6|5-9|1d6}
    processed = processed.replace(/{@scaledice\s+([^|}\s]+)(?:\|([^|}]+))?(?:\|([^}]+))?}/g, 
      (match, baseDice) => `<span class="scaled-value">${baseDice}</span>`)
    
    // Skill checks {@skill Athletics} 
    processed = processed.replace(/{@skill\s+([^}]+)}/g, '<span class="skill-check">$1</span>')
    
    // Actions {@action Attack}
    processed = processed.replace(/{@action\s+([^}]+)}/g, '<span class="action-name">$1</span>')
    
    // Conditions {@condition poisoned}
    processed = processed.replace(/{@condition\s+([^}]+)}/g, (match, condition) => {
      return `<span class="condition" title="${condition}">${condition}</span>`
    })
    
    // Status {@status prone}
    processed = processed.replace(/{@status\s+([^}]+)}/g, (match, status) => {
      return `<span class="status" title="${status}">${status}</span>`
    })
    
    // Spells {@spell fireball} or {@spell fireball|phb|PHB}
    processed = processed.replace(/{@spell\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, spell, source, display) => {
      const spellName = display || spell
      const title = source ? `${spell} (${source})` : spell
      const dataAttrs = `data-ref-type="spell" data-ref-name="${spell}" data-ref-source="${source || ''}"` 
      return `<span class="spell-ref cross-ref-link" title="${title}" ${dataAttrs}>${spellName}</span>`
    })
    
    // Items {@item longsword} or {@item longsword|phb|PHB}
    processed = processed.replace(/{@item\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, item, source, display) => {
      const itemName = display || item
      const title = source ? `${item} (${source})` : item
      const dataAttrs = `data-ref-type="item" data-ref-name="${item}" data-ref-source="${source || ''}"` 
      return `<span class="item-ref cross-ref-link" title="${title}" ${dataAttrs}>${itemName}</span>`
    })
    
    // Creatures {@creature goblin} or {@creature goblin|mm|MM}
    processed = processed.replace(/{@creature\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, creature, source, display) => {
      const creatureName = display || creature
      const title = source ? `${creature} (${source})` : creature
      const dataAttrs = `data-ref-type="creature" data-ref-name="${creature}" data-ref-source="${source || ''}"` 
      return `<span class="creature-ref cross-ref-link" title="${title}" ${dataAttrs}>${creatureName}</span>`
    })
    
    // Races {@race dragonborn} or {@race dragonborn|phb}
    processed = processed.replace(/{@race\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, race, source, display) => {
      const raceName = display || race
      const title = source ? `${race} (${source})` : race
      const dataAttrs = `data-ref-type="race" data-ref-name="${race}" data-ref-source="${source || ''}"` 
      return `<span class="race-ref cross-ref-link" title="${title}" ${dataAttrs}>${raceName}</span>`
    })
    
    // Classes {@class fighter} or {@class fighter|phb}
    processed = processed.replace(/{@class\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, className, source, subclass, display) => {
      const displayName = display || className
      const title = subclass ? `${className} (${subclass})` : className
      const dataAttrs = `data-ref-type="class" data-ref-name="${className}" data-ref-source="${source || ''}" data-ref-subclass="${subclass || ''}"` 
      return `<span class="class-ref cross-ref-link" title="${title}" ${dataAttrs}>${displayName}</span>`
    })
    
    // Backgrounds {@background soldier} or {@background soldier|phb}
    processed = processed.replace(/{@background\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, background, source, display) => {
      const backgroundName = display || background
      const title = source ? `${background} (${source})` : background
      const dataAttrs = `data-ref-type="background" data-ref-name="${background}" data-ref-source="${source || ''}"` 
      return `<span class="background-ref cross-ref-link" title="${title}" ${dataAttrs}>${backgroundName}</span>`
    })
    
    // Feats {@feat alert} or {@feat alert|phb}
    processed = processed.replace(/{@feat\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, feat, source, display) => {
      const featName = display || feat
      const title = source ? `${feat} (${source})` : feat
      const dataAttrs = `data-ref-type="feat" data-ref-name="${feat}" data-ref-source="${source || ''}"` 
      return `<span class="feat-ref cross-ref-link" title="${title}" ${dataAttrs}>${featName}</span>`
    })
    
    // Books {@book Player's Handbook|PHB}
    processed = processed.replace(/{@book\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}|]+))?(?:\|([^}]+))?}/g, 
      (match, bookName, source, page, display) => {
        const displayText = display || bookName
        const title = page ? `${bookName}, page ${page}` : bookName
        return `<span class="book-ref" title="${title}">${displayText}</span>`
    })
    
    // Links {@link text|url}
    processed = processed.replace(/{@link\s+([^}|]+)\|([^}]+)}/g, '<a href="$2" target="_blank">$1</a>')
    
    // Chance/percentage {@chance 25} becomes "25% chance"
    processed = processed.replace(/{@chance\s+(\d+)(?:\|([^}]+))?}/g, (match, percent, display) => {
      return `<span class="chance">${percent}%${display ? ` ${display}` : ' chance'}</span>`
    })
    
    // Hit bonus {@hit +5} or {@hit 5}
    processed = processed.replace(/{@hit\s+([+-]?\d+)}/g, (match, bonus) => {
      const formattedBonus = bonus.startsWith('+') || bonus.startsWith('-') ? bonus : `+${bonus}`
      return `<span class="hit-bonus">${formattedBonus}</span>`
    })
    
    // Attack type {@atk mw} for "Melee Weapon Attack", {@atk rw} for "Ranged Weapon Attack", etc.
    processed = processed.replace(/{@atk\s+([^}]+)}/g, (match, type) => {
      const attackTypes: Record<string, string> = {
        'mw': 'Melee Weapon Attack',
        'rw': 'Ranged Weapon Attack',
        'ms': 'Melee Spell Attack',
        'rs': 'Ranged Spell Attack',
        'mw,rw': 'Melee or Ranged Weapon Attack',
        'ms,rs': 'Melee or Ranged Spell Attack'
      }
      return `<em>${attackTypes[type] || type}:</em>`
    })
    
    // Hit indicator {@h} for "Hit:"
    processed = processed.replace(/{@h}/g, '<em>Hit:</em>')
    
    // Recharge {@recharge} or {@recharge 5} 
    processed = processed.replace(/{@recharge\s*(\d+)?}/g, (match, num) => {
      return num ? `<span class="recharge">Recharge ${num}-6</span>` : '<span class="recharge">Recharge</span>'
    })
    
    return processed
}

// Render a section with proper HTML
const renderSection = (section: any): string => {
  if (!section) return ''
  
  // Recursive renderer for entries
  const renderEntry = (entry: any, index: number = 0, depth: number = 0): string => {
    if (typeof entry === 'string') {
      return `<p>${processFormatting(entry)}</p>`
    }
    
    if (entry.type === 'section') {
      // Top-level sections (like "Introduction") are h1
      // Sub-sections are h2 or h3 based on depth
      const headerLevel = depth === 0 ? 'h1' : (depth === 1 ? 'h2' : 'h3')
      const currentId = `section-${index}`
      return `
        <div class="section" id="${currentId}">
          <${headerLevel}>${processFormatting(entry.name || '')}</${headerLevel}>
          ${entry.entries ? entry.entries.map((e: any, i: number) => renderEntry(e, i, depth + 1)).join('') : ''}
        </div>
      `
    }
    
    if (entry.type === 'entries') {
      // entries with names at depth 0 become h2 (like "Worlds of Adventure")
      // deeper entries become h3 or h4
      const currentId = `entry-${index}`
      let headerTag = ''
      if (entry.name) {
        const headerLevel = depth === 0 ? 'h2' : (depth === 1 ? 'h3' : 'h4')
        headerTag = `<${headerLevel}>${processFormatting(entry.name)}</${headerLevel}>`
      }
      return `
        <div class="entries" id="${currentId}">
          ${headerTag}
          ${entry.entries ? entry.entries.map((e: any, i: number) => renderEntry(e, i, depth + 1)).join('') : ''}
        </div>
      `
    }
    
    if (entry.type === 'insetReadaloud') {
      return `
        <div class="inset-readaloud">
          ${entry.entries ? entry.entries.map((e: any, i: number) => renderEntry(e, i, depth + 1)).join('') : ''}
        </div>
      `
    }
    
    if (entry.type === 'inset') {
      return `
        <div class="inset">
          ${entry.name ? `<h4>${processFormatting(entry.name)}</h4>` : ''}
          ${entry.entries ? entry.entries.map((e: any, i: number) => renderEntry(e, i, depth + 1)).join('') : ''}
        </div>
      `
    }
    
    if (entry.type === 'list') {
      return `
        <ul class="content-list">
          ${entry.items ? entry.items.map((item: any, i: number) => `<li>${typeof item === 'string' ? processFormatting(item) : renderEntry(item, i, depth + 1)}</li>`).join('') : ''}
        </ul>
      `
    }
    
    if (entry.type === 'table') {
      // Simple table rendering
      return `<div class="table-wrapper">Table content here</div>`
    }
    
    if (entry.type === 'image') {
      // Handle images - show actual image if available
      const imagePath = entry.href?.path || ''
      const imageName = imagePath.split('/').pop() || 'image'
      
      // Create a unique ID for this image element
      const imageId = `img-${Math.random().toString(36).substr(2, 9)}`
      
      // Load the image asynchronously after render
      setTimeout(async () => {
        const imgElement = document.getElementById(imageId)
        if (imgElement && selectedBook.value) {
          try {
            const response = await invoke<{ success: boolean; data?: string; message?: string }>('serve_book_image', {
              bookId: selectedBook.value.id,
              imagePath: imagePath
            })
            
            if (response.success && response.data) {
              imgElement.innerHTML = `<img src="${response.data}" alt="${entry.title || imageName}" style="max-width: 100%; height: auto; display: block; margin: 0 auto;" />`
              // Remove the placeholder min-height once image loads
              imgElement.style.minHeight = ''
            }
          } catch (error) {
            console.error('Failed to load image:', error)
          }
        }
      }, 0)
      
      return `
        <div class="image-container">
          <div id="${imageId}" class="image-wrapper">
            <div class="image-placeholder" style="max-width: 100%; height: auto; min-height: 100px;">
              <p>Loading image: ${imageName}</p>
            </div>
          </div>
          ${entry.title ? `<p class="image-caption">${entry.title}</p>` : ''}
        </div>
      `
    }
    
    // Default: render as JSON for unknown types
    return `<pre>${JSON.stringify(entry, null, 2)}</pre>`
  }
  
  // Start rendering
  if (section.entries && Array.isArray(section.entries)) {
    return `
      <div class="section-content">
        ${section.name ? `<h1>${section.name}</h1>` : ''}
        ${section.entries.map((e: any, i: number) => renderEntry(e, i, 0)).join('')}
      </div>
    `
  }
  
  return renderEntry(section, 0, 0)
}

// Cache for reference lookups
const referenceCache = new Map<string, any>()

// Lookup reference data from backend
const lookupReference = async (refType: string, refName: string, refSource?: string): Promise<any> => {
  const cacheKey = `${refType}:${refName}:${refSource || ''}`
  
  // Check cache first
  if (referenceCache.has(cacheKey)) {
    return referenceCache.get(cacheKey)
  }
  
  try {
    const response = await invoke<{ success: boolean; data?: any; message?: string }>('lookup_reference', {
      refType,
      refName,
      refSource
    })
    
    if (response.success && response.data) {
      // Cache the result
      referenceCache.set(cacheKey, response.data)
      return response.data
    }
  } catch (error) {
    console.error('Failed to lookup reference:', error)
  }
  
  return null
}

// Handle cross-reference hover
const handleCrossRefHover = async (event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (!target.classList.contains('cross-ref-link')) return
  
  // Clear any existing timeout
  if (hoverTimeout) {
    clearTimeout(hoverTimeout)
  }
  
  // Set timeout for showing tooltip
  hoverTimeout = window.setTimeout(async () => {
    const refType = target.dataset.refType || ''
    const refName = target.dataset.refName || ''
    const refSource = target.dataset.refSource
    
    // Get tooltip content from backend
    const refData = await lookupReference(refType, refName, refSource)
    
    if (refData && refData.preview) {
      tooltipContent.value = refData.preview
    } else {
      tooltipContent.value = `${refType}: ${refName}<br/><em>Click for details</em>`
    }
    
    // Position tooltip near the element
    const rect = target.getBoundingClientRect()
    tooltipPosition.value = {
      x: rect.left + window.scrollX,
      y: rect.bottom + window.scrollY + 5
    }
    
    tooltipVisible.value = true
  }, 300) // 300ms delay before showing tooltip
}

// Handle mouse leave from cross-reference
const handleCrossRefLeave = () => {
  if (hoverTimeout) {
    clearTimeout(hoverTimeout)
    hoverTimeout = null
  }
  tooltipVisible.value = false
}

// Handle cross-reference clicks
const handleCrossRefClick = async (event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (!target.classList.contains('cross-ref-link')) return
  
  event.preventDefault()
  event.stopPropagation()
  
  // Hide tooltip when clicking
  tooltipVisible.value = false
  
  const refType = target.dataset.refType || ''
  const refName = target.dataset.refName || ''
  const refSource = target.dataset.refSource
  
  console.log('Cross-reference clicked:', { refType, refName, refSource })
  
  // Set modal title
  modalTitle.value = `${refName}`
  
  // Show loading state
  modalContent.value = null
  modalTitle.value = `${refName} (Loading...)`
  modalVisible.value = true
  
  // Load reference content
  const refData = await lookupReference(refType, refName, refSource)
  if (refData) {
    modalContent.value = {
      ...refData.data,
      ref_type: refType  // Store the type for rendering
    }
    modalTitle.value = refData.name
  } else {
    modalTitle.value = refName
    modalContent.value = {
      type: refType,
      name: refName,
      entries: [`No data found for ${refType}: ${refName}`]
    }
  }
}

// Render modal content based on type
const renderModalContent = (content: any): string => {
  if (!content) return ''
  
  let html = '<div class="reference-details">'
  
  const refType = content.ref_type || content.type || 'unknown'
  
  if (refType === 'spell' || content.level !== undefined) {
    const level = content.level || 0
    const school = content.school || 'Unknown'
    const castingTime = content.time?.[0] ? 
      `${content.time[0].number} ${content.time[0].unit}` : '1 action'
    const range = formatRange(content.range)
    const components = formatComponents(content.components)
    const duration = formatDuration(content.duration)
    const description = content.entries?.map((e: any) => processFormatting(e)).join('<br/><br/>') || ''
    
    html += `
      <div class="spell-details">
        <p><strong>Level:</strong> ${level === 0 ? 'Cantrip' : level}</p>
        <p><strong>School:</strong> ${getSchoolName(school)}</p>
        <p><strong>Casting Time:</strong> ${castingTime}</p>
        <p><strong>Range:</strong> ${range}</p>
        <p><strong>Components:</strong> ${components}</p>
        <p><strong>Duration:</strong> ${duration}</p>
        <div class="description">${description}</div>
      </div>
    `
  } else if (refType === 'item') {
    const itemType = content.type || 'Item'
    const rarity = content.rarity || ''
    const value = content.value || ''
    const weight = content.weight || ''
    const description = content.entries?.map((e: any) => processFormatting(e)).join('<br/><br/>') || ''
    
    html += `
      <div class="item-details">
        <p><strong>Type:</strong> ${itemType}</p>
        ${rarity ? `<p><strong>Rarity:</strong> ${rarity}</p>` : ''}
        ${value ? `<p><strong>Value:</strong> ${value} gp</p>` : ''}
        ${weight ? `<p><strong>Weight:</strong> ${weight} lb</p>` : ''}
        <div class="description">${description}</div>
      </div>
    `
  } else if (refType === 'creature' || refType === 'monster') {
    const size = content.size || 'Medium'
    const type = content.type || 'creature'
    const alignment = formatAlignment(content.alignment)
    const ac = formatAC(content.ac)
    const hp = formatHP(content.hp)
    const speed = formatSpeed(content.speed)
    const cr = content.cr || '0'
    
    // Ability scores
    const str = content.str || 10
    const dex = content.dex || 10
    const con = content.con || 10
    const int = content.int || 10
    const wis = content.wis || 10
    const cha = content.cha || 10
    
    // Calculate modifiers
    const getModifier = (score: number) => {
      const mod = Math.floor((score - 10) / 2)
      return mod >= 0 ? `+${mod}` : `${mod}`
    }
    
    html += `
      <div class="creature-details">
        <div class="creature-type">${getSizeName(size)} ${type}${alignment ? `, ${alignment}` : ''}</div>
        
        <div class="creature-stats">
          <p><strong>Armor Class:</strong> ${ac}</p>
          <p><strong>Hit Points:</strong> ${hp}</p>
          <p><strong>Speed:</strong> ${speed}</p>
        </div>
        
        <div class="ability-scores">
          <table>
            <tr>
              <th>STR</th>
              <th>DEX</th>
              <th>CON</th>
              <th>INT</th>
              <th>WIS</th>
              <th>CHA</th>
            </tr>
            <tr>
              <td>${str} (${getModifier(str)})</td>
              <td>${dex} (${getModifier(dex)})</td>
              <td>${con} (${getModifier(con)})</td>
              <td>${int} (${getModifier(int)})</td>
              <td>${wis} (${getModifier(wis)})</td>
              <td>${cha} (${getModifier(cha)})</td>
            </tr>
          </table>
        </div>
        
        ${content.skill ? `<p><strong>Skills:</strong> ${formatSkills(content.skill)}</p>` : ''}
        ${content.senses ? `<p><strong>Senses:</strong> ${content.senses.join(', ')}</p>` : ''}
        ${content.languages ? `<p><strong>Languages:</strong> ${content.languages.join(', ')}</p>` : ''}
        <p><strong>Challenge:</strong> ${cr}</p>
        
        ${content.trait && content.trait.length > 0 ? `
          <div class="creature-traits">
            <h4>Traits</h4>
            ${content.trait.map((t: any) => `
              <div class="trait">
                <strong>${t.name}.</strong> ${t.entries ? t.entries.map((e: any) => processFormatting(e)).join(' ') : ''}
              </div>
            `).join('')}
          </div>
        ` : ''}
        
        ${content.action && content.action.length > 0 ? `
          <div class="creature-actions">
            <h4>Actions</h4>
            ${content.action.map((a: any) => `
              <div class="action">
                <strong>${a.name}.</strong> ${a.entries ? a.entries.map((e: any) => processFormatting(e)).join(' ') : ''}
              </div>
            `).join('')}
          </div>
        ` : ''}
      </div>
    `
  } else if (refType === 'class') {
    const hitDice = content.hd?.faces || 8
    const primaryAbility = content.primaryAbility || 'Varies'
    const description = content.entries?.map((e: any) => processFormatting(e)).join('<br/><br/>') || 
                       content.fluff?.[0]?.entries?.map((e: any) => processFormatting(e)).join('<br/><br/>') || ''
    
    html += `
      <div class="class-details">
        <p><strong>Hit Die:</strong> d${hitDice}</p>
        <p><strong>Primary Ability:</strong> ${primaryAbility}</p>
        <div class="description">${description}</div>
      </div>
    `
  } else if (refType === 'race') {
    const size = getSizeName(content.size?.[0] || 'M')
    const speed = content.speed || 30
    const description = content.entries?.map((e: any) => processFormatting(e)).join('<br/><br/>') || ''
    
    // Format ability score improvements
    let abilityScores = ''
    if (content.ability && content.ability.length > 0) {
      const abilities = content.ability[0]
      const scores = []
      if (abilities.str) scores.push(`Strength +${abilities.str}`)
      if (abilities.dex) scores.push(`Dexterity +${abilities.dex}`)
      if (abilities.con) scores.push(`Constitution +${abilities.con}`)
      if (abilities.int) scores.push(`Intelligence +${abilities.int}`)
      if (abilities.wis) scores.push(`Wisdom +${abilities.wis}`)
      if (abilities.cha) scores.push(`Charisma +${abilities.cha}`)
      abilityScores = scores.join(', ')
    }
    
    html += `
      <div class="race-details">
        <p><strong>Size:</strong> ${size}</p>
        <p><strong>Speed:</strong> ${speed} ft.</p>
        ${abilityScores ? `<p><strong>Ability Score Increase:</strong> ${abilityScores}</p>` : ''}
        <div class="description">${description}</div>
      </div>
    `
  } else if (refType === 'background') {
    const description = content.entries?.map((e: any) => processFormatting(e)).join('<br/><br/>') || ''
    
    // Format skill proficiencies
    let skills = ''
    if (content.skillProficiencies && content.skillProficiencies.length > 0) {
      const skillList = Object.keys(content.skillProficiencies[0])
        .map(skill => skill.charAt(0).toUpperCase() + skill.slice(1))
        .join(', ')
      skills = skillList
    }
    
    html += `
      <div class="background-details">
        ${skills ? `<p><strong>Skill Proficiencies:</strong> ${skills}</p>` : ''}
        <div class="description">${description}</div>
      </div>
    `
  } else if (refType === 'feat') {
    const description = content.entries?.map((e: any) => processFormatting(e)).join('<br/><br/>') || ''
    const prerequisite = content.prerequisite?.map((p: any) => {
      if (typeof p === 'string') return p
      if (p.level) return `${p.level}th level`
      if (p.race) return `${p.race.name || p.race} race`
      if (p.ability) {
        const abilities = []
        for (const [key, value] of Object.entries(p.ability)) {
          abilities.push(`${key.toUpperCase()} ${value}`)
        }
        return abilities.join(' or ')
      }
      return JSON.stringify(p)
    }).join(', ')
    
    html += `
      <div class="feat-details">
        ${prerequisite ? `<p><strong>Prerequisite:</strong> ${prerequisite}</p>` : ''}
        <div class="description">${description}</div>
      </div>
    `
  } else {
    // Generic fallback
    const description = content.entries?.map((e: any) => processFormatting(e)).join('<br/><br/>') || 
                       (content.description ? processFormatting(content.description) : '') || 
                       JSON.stringify(content, null, 2)
    
    html += `
      <div class="generic-details">
        <div class="description">${description}</div>
      </div>
    `
  }
  
  html += '</div>'
  return html
}

// Helper functions for formatting
const getSchoolName = (school: string): string => {
  const schools: Record<string, string> = {
    'A': 'Abjuration',
    'C': 'Conjuration',
    'D': 'Divination',
    'E': 'Enchantment',
    'V': 'Evocation',
    'I': 'Illusion',
    'N': 'Necromancy',
    'T': 'Transmutation'
  }
  return schools[school] || school
}

const getSizeName = (size: string): string => {
  const sizes: Record<string, string> = {
    'T': 'Tiny',
    'S': 'Small',
    'M': 'Medium',
    'L': 'Large',
    'H': 'Huge',
    'G': 'Gargantuan'
  }
  return sizes[size] || size
}

const formatRange = (range: any): string => {
  if (!range) return 'Varies'
  if (range.type === 'point') {
    if (range.distance) {
      return `${range.distance.amount} ${range.distance.type}`
    }
  }
  return range.type || 'Varies'
}

const formatComponents = (components: any): string => {
  if (!components) return 'None'
  const parts = []
  if (components.v) parts.push('V')
  if (components.s) parts.push('S')
  if (components.m) parts.push(`M (${typeof components.m === 'string' ? components.m : 'materials'})`)
  return parts.join(', ') || 'None'
}

const formatDuration = (duration: any): string => {
  if (!duration || !duration[0]) return 'Instantaneous'
  const d = duration[0]
  if (d.type === 'instant') return 'Instantaneous'
  if (d.type === 'timed') {
    return `${d.duration.amount} ${d.duration.type}${d.duration.amount > 1 ? 's' : ''}`
  }
  return d.type || 'Varies'
}

const formatAC = (ac: any): string => {
  if (!ac) return '10'
  if (typeof ac === 'number') return ac.toString()
  if (Array.isArray(ac) && ac[0]) {
    if (typeof ac[0] === 'number') return ac[0].toString()
    if (ac[0].ac) return ac[0].ac.toString()
  }
  return '10'
}

const formatHP = (hp: any): string => {
  if (!hp) return '1'
  if (hp.average) return `${hp.average} (${hp.formula || ''})`
  return '1'
}

const formatSpeed = (speed: any): string => {
  if (!speed) return '30 ft.'
  const speeds = []
  if (speed.walk) speeds.push(`${speed.walk} ft.`)
  if (speed.fly) speeds.push(`fly ${speed.fly} ft.`)
  if (speed.swim) speeds.push(`swim ${speed.swim} ft.`)
  if (speed.climb) speeds.push(`climb ${speed.climb} ft.`)
  if (speed.burrow) speeds.push(`burrow ${speed.burrow} ft.`)
  return speeds.length > 0 ? speeds.join(', ') : '30 ft.'
}

const formatAlignment = (alignment: any): string => {
  if (!alignment) return ''
  if (typeof alignment === 'string') return alignment
  if (Array.isArray(alignment)) {
    const alignmentMap: Record<string, string> = {
      'L': 'lawful',
      'N': 'neutral',
      'C': 'chaotic',
      'G': 'good',
      'E': 'evil',
      'U': 'unaligned',
      'A': 'any alignment'
    }
    return alignment.map(a => alignmentMap[a] || a).join(' ')
  }
  return ''
}

const formatSkills = (skills: any): string => {
  if (!skills) return ''
  if (typeof skills === 'object') {
    return Object.entries(skills).map(([skill, bonus]) => `${skill} ${bonus}`).join(', ')
  }
  return ''
}

// Close modal
const closeModal = () => {
  modalVisible.value = false
  modalContent.value = null
}

// Apply theme and initialize synchronization on mount
onMounted(async () => {
  themeStore.applyTheme()
  await themeStore.initThemeSync()
  
  // Check if in dev mode and auto-install test book
  try {
    const isDevMode = await invoke<boolean>('is_dev_mode')
    console.log('Dev mode check:', isDevMode)
    if (isDevMode) {
      console.log('Dev mode detected, installing test book...')
      const response = await invoke<{ success: boolean; data?: string; message?: string }>('install_dev_test_book')
      console.log('Install test book response:', response)
      if (response.success) {
        console.log('Dev test book installed:', response.data)
      } else {
        console.error('Failed to install test book:', response.message)
      }
    }
  } catch (error) {
    console.error('Failed to check dev mode:', error)
  }
  
  await loadLibraryBooks()
  
  // Add global event handlers for cross-references
  document.addEventListener('click', handleCrossRefClick)
  document.addEventListener('mouseover', handleCrossRefHover)
  document.addEventListener('mouseout', handleCrossRefLeave)
})

// Clean up event listeners on unmount
onUnmounted(() => {
  document.removeEventListener('click', handleCrossRefClick)
  document.removeEventListener('mouseover', handleCrossRefHover)
  document.removeEventListener('mouseout', handleCrossRefLeave)
})
</script>

<style scoped>
#rules-app {
  height: 100vh;
  overflow: hidden;
  background-color: var(--color-background);
  color: var(--color-text);
}

.rules-container {
  height: 100%;
  display: flex;
  flex-direction: column;
}

/* Header */
.rules-header {
  background-color: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  box-shadow: var(--shadow-sm);
  flex-shrink: 0;
}

.header-content {
  padding: var(--spacing-md) var(--spacing-lg);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
}

.add-book-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--color-primary);
  color: var(--color-primary-text);
  border: none;
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.add-book-btn:hover {
  background-color: var(--color-primary-dark);
  transform: translateY(-1px);
  box-shadow: var(--shadow-md);
}

.add-book-btn:active {
  transform: translateY(0);
}

.app-title {
  font-size: var(--font-size-xl);
  font-weight: 600;
  color: var(--color-text);
  margin: 0;
}

/* Body Layout - Three Panels */
.rules-body {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* Panel 1: Books List */
.books-panel {
  width: 250px;
  background-color: var(--color-surface);
  border-right: 1px solid var(--color-border);
  overflow-y: auto;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
}

/* Panel 2: Table of Contents */
.toc-panel {
  width: 300px;
  background-color: var(--color-surface-variant);
  border-right: 1px solid var(--color-border);
  overflow-y: auto;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
}

/* Panel 3: Content */
.content-panel {
  flex: 1;
  background-color: var(--color-background);
  overflow-y: auto;
  padding: var(--spacing-lg);
}

.sidebar-section {
  padding: var(--spacing-lg);
}

.section-title {
  font-size: var(--font-size-sm);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-text-secondary);
  margin: 0 0 var(--spacing-md) 0;
}

/* Books List */
.books-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.book-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm);
  background-color: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.book-item:hover {
  background-color: var(--color-surface);
  border-color: var(--color-primary);
}

.book-item.active {
  background-color: var(--color-primary);
  border-color: var(--color-primary);
}

.book-item.active .book-name {
  color: var(--color-primary-text);
}

.book-name {
  flex: 1;
  font-size: var(--font-size-sm);
  color: var(--color-text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.remove-book-btn {
  width: 20px;
  height: 20px;
  padding: 0;
  background-color: transparent;
  border: none;
  color: var(--color-text-secondary);
  font-size: var(--font-size-lg);
  cursor: pointer;
  transition: all var(--transition-fast);
  opacity: 0.6;
}

.remove-book-btn:hover {
  color: var(--color-error);
  opacity: 1;
}

.book-item.active .remove-book-btn {
  color: var(--color-primary-text);
}

.no-books {
  padding: var(--spacing-md);
  text-align: center;
  color: var(--color-text-tertiary);
  font-size: var(--font-size-sm);
  font-style: italic;
}

/* Main Content */
.rules-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Search */
.search-container {
  padding: var(--spacing-lg);
  background-color: var(--color-background);
  border-bottom: 1px solid var(--color-border);
}

.search-input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  color: var(--color-text);
  font-size: var(--font-size-base);
  transition: all var(--transition-fast);
}

.search-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-alpha);
}

.search-input::placeholder {
  color: var(--color-text-tertiary);
}

/* Content Area */
.content-area {
  flex: 1;
  padding: var(--spacing-lg);
  overflow-y: auto;
}

.welcome-message {
  text-align: center;
  padding: var(--spacing-xl);
}

.welcome-message h2 {
  color: var(--color-text);
  margin-bottom: var(--spacing-md);
}

.welcome-message p {
  color: var(--color-text-secondary);
  font-size: var(--font-size-lg);
}

.results-container h2 {
  margin-bottom: var(--spacing-lg);
  color: var(--color-text);
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-xl);
  gap: var(--spacing-md);
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--color-border);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* Panel Headers */
.panel-header {
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--color-border);
  background-color: var(--color-surface);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.panel-title {
  font-size: var(--font-size-sm);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-text-secondary);
  margin: 0;
}

/* Book Item Updates */
.book-info-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.book-meta {
  font-size: var(--font-size-xs);
  color: var(--color-text-tertiary);
  margin-top: var(--spacing-xs);
}

.dev-badge {
  display: inline-block;
  padding: 1px 4px;
  background-color: #fbbf24;
  color: #451a03;
  border-radius: 3px;
  font-weight: 700;
  font-size: 0.7em;
  text-transform: uppercase;
  margin-right: 4px;
}

/* Table of Contents */
.toc-list {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-sm);
}

.toc-item {
  padding: 6px 12px;
  cursor: pointer;
  border-radius: 0;
  margin-bottom: 0;
  transition: background-color 0.2s;
  border-left: 3px solid transparent;
  font-size: 14px;
}

.toc-item:hover {
  background-color: rgba(0, 0, 0, 0.05);
}

.toc-item.active {
  background-color: rgba(0, 0, 0, 0.08);
  color: var(--color-text);
  border-left-color: var(--color-primary);
  font-weight: bold;
}

/* Sub-items in TOC (nested entries) */
.toc-sub-entries {
  background-color: rgba(0, 0, 0, 0.02);
}

.toc-sub-item {
  padding: 4px 12px 4px 30px;
  cursor: pointer;
  font-size: 13px;
  color: #666;
  transition: all 0.2s;
}

.toc-sub-item:hover {
  background-color: rgba(0, 0, 0, 0.05);
  color: #333;
}

.toc-name {
  font-size: var(--font-size-sm);
  line-height: 1.5;
}

/* Content Rendering */
.book-content {
  max-width: 800px;
  margin: 0 auto;
}

.content-wrapper {
  line-height: 1.6;
  color: var(--color-text);
}

.section-content h1 {
  font-size: var(--font-size-2xl);
  margin-bottom: var(--spacing-lg);
  color: var(--color-text);
}

.section-content h2 {
  font-size: var(--font-size-xl);
  margin-top: var(--spacing-xl);
  margin-bottom: var(--spacing-md);
  color: var(--color-text);
}

.section-content h3 {
  font-size: var(--font-size-lg);
  margin-top: var(--spacing-lg);
  margin-bottom: var(--spacing-md);
  color: var(--color-text);
}

.section-content p {
  margin-bottom: var(--spacing-md);
}

/* Inset Readaloud */
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

/* Image Placeholder */
.image-container {
  margin: var(--spacing-md) 0;
  display: block;
}

.image-placeholder {
  background-color: var(--color-surface);
  border: 2px dashed var(--color-border);
  border-radius: var(--radius-md);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--color-text-tertiary);
  padding: var(--spacing-md);
  min-height: 100px;
}

.image-note {
  font-size: var(--font-size-sm);
  margin-top: var(--spacing-sm);
}

.image-caption {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  margin-top: var(--spacing-sm);
  text-align: center;
  font-style: italic;
}

.image-wrapper {
  display: block;
  width: 100%;
}

.image-wrapper img {
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-md);
}


.inset-readaloud p:first-child {
  margin-top: 0 !important;
}

.inset-readaloud p:last-child {
  margin-bottom: 0 !important;
}

/* 5etools lists */
.content-list {
  padding-left: 40px !important;
  margin: 5px 0 !important;
}

.content-list li {
  margin-bottom: 0 !important;
  line-height: 1.42857143 !important;
}

/* 5etools section spacing */
.section {
  margin-bottom: 0 !important;
}

.entries {
  margin: 5px 0 !important;
}

.entries > h4 {
  margin: 10px 0 5px 0 !important;
}

/* Page numbers - subtle like 5etools */
.page-number {
  display: inline-block !important;
  padding: 0 4px !important;
  background: none !important;
  color: #777 !important;
  border-radius: 0 !important;
  font-weight: normal !important;
  font-size: 0.9em !important;
  box-shadow: none !important;
  margin-left: 5px !important;
}

/* 5etools Formatting Styles */
strong {
  font-weight: 700;
  color: var(--color-text-primary);
}

em {
  font-style: italic;
}

code {
  background-color: var(--color-surface);
  padding: 2px 6px;
  border-radius: 3px;
  font-family: 'Courier New', monospace;
  font-size: 0.95em;
}

.note {
  display: inline-block;
  padding: 2px 8px;
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: 3px;
  font-style: italic;
  font-size: 0.95em;
}

</style>

<style>
/* Global styles for book content (non-scoped) */
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

/* Cross-reference styles - Subtle and theme-aware */
.spell-ref {
  color: var(--color-primary-600);
  font-style: italic;
  cursor: help;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
  text-decoration-color: var(--color-primary-300);
}

.spell-ref:hover {
  color: var(--color-primary-700);
  text-decoration-color: var(--color-primary-500);
}

.item-ref {
  color: var(--color-success);
  cursor: help;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
  text-decoration-color: var(--color-success-light);
}

.item-ref:hover {
  color: var(--color-success-dark);
  text-decoration-color: var(--color-success);
}

.creature-ref {
  color: var(--color-danger);
  font-weight: 500;
  cursor: help;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
  text-decoration-color: var(--color-danger-light);
}

.creature-ref:hover {
  color: var(--color-danger-dark);
  text-decoration-color: var(--color-danger);
}

.book-ref {
  color: var(--color-primary-500);
  font-style: italic;
  cursor: help;
}

/* Tooltip styles */
.cross-ref-tooltip {
  position: absolute;
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
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 20000;
}

.modal-content {
  background: var(--color-background);
  border-radius: var(--radius-lg);
  max-width: 800px;
  max-height: 80vh;
  width: 90%;
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-xl);
}

.modal-header {
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header h2 {
  margin: 0;
  color: var(--color-primary);
  font-size: 1.5em;
}

.modal-close {
  background: none;
  border: none;
  font-size: 2em;
  color: var(--color-text-secondary);
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}

.modal-close:hover {
  background: var(--color-surface);
  color: var(--color-text);
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

.modal-body {
  padding: var(--spacing-lg);
  overflow-y: auto;
  flex: 1;
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

/* Read-aloud boxes - special chromatic styling */
.book-content .inset-readaloud {
  background: var(--gradient-chromatic-subtle);
  border: 2px solid var(--color-chromatic-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  margin: var(--spacing-lg) 0;
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

.book-content .spell-ref:hover,
.book-content .item-ref:hover,
.book-content .creature-ref:hover,
.book-content .race-ref:hover,
.book-content .class-ref:hover,
.book-content .background-ref:hover,
.book-content .feat-ref:hover {
  color: var(--color-primary-600);
  text-decoration-style: solid;
}

.book-content .book-ref {
  color: var(--color-primary);
  text-decoration: underline;
  cursor: help;
}

.book-content .dice-roll,
.book-content .dc-check,
.book-content .skill-check {
  font-weight: bold;
  color: var(--color-text);
}

.book-content .action-name {
  font-weight: bold;
  font-style: italic;
  color: var(--color-text);
}

.book-content .condition {
  color: var(--color-error);
  font-style: italic;
  cursor: help;
}

.book-content .note {
  font-style: italic;
  color: var(--color-text-secondary);
}
</style>