<template>
  <div id="rules-app" :class="`theme-${currentTheme}`">
    <!-- Name Input Dialog -->
    <div v-if="showNameInput" class="dialog-overlay" @click.self="handleNameDialogCancel">
      <div class="dialog-content">
        <h3>Name Your Book</h3>
        <p>Enter a custom name to distinguish this book (e.g., "PHB 2014" vs "PHB 2024")</p>
        <input 
          v-model="nameInputValue" 
          @keyup.enter="handleNameDialogSubmit"
          @keyup.escape="handleNameDialogCancel"
          type="text" 
          class="name-input"
          autofocus
        />
        <div class="dialog-actions">
          <button @click="handleNameDialogCancel" class="btn-cancel">Cancel</button>
          <button @click="handleNameDialogSubmit" class="btn-submit" :disabled="!nameInputValue.trim()">Add Book</button>
        </div>
      </div>
    </div>
    
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
            <button @click="handleAddImages" class="add-images-btn" v-if="selectedBook" title="Add images to selected book">
              + Images
            </button>
          </div>
          <div class="books-list">
            <div 
              v-for="book in libraryBooks" 
              :key="book.folder_name"
              class="book-item"
              @click="selectedBook = book"
              :class="{ active: selectedBook?.folder_name === book.folder_name }"
            >
              <div class="book-info-wrapper">
                <span class="book-name">{{ book.name }}</span>
                <span class="book-meta">{{ book.image_count }} images</span>
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
            <p>Click "Add Book" to import 5etools JSON files.</p>
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
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useThemeStore } from './stores/theme'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

// Types
interface BookInfo {
  name: string           // Display name
  folder_name: string    // Folder name on disk
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

// Dialog state
const showNameInput = ref(false)
const nameInputValue = ref('')
const nameInputResolve = ref<((value: string | null) => void) | null>(null)

// Simple name dialog function
const showNameDialog = (defaultName: string): Promise<string | null> => {
  return new Promise((resolve) => {
    nameInputValue.value = defaultName
    nameInputResolve.value = resolve
    showNameInput.value = true
  })
}

const handleNameDialogSubmit = () => {
  if (nameInputResolve.value && nameInputValue.value.trim()) {
    nameInputResolve.value(nameInputValue.value.trim())
    showNameInput.value = false
    nameInputResolve.value = null
    nameInputValue.value = ''
  }
}

const handleNameDialogCancel = () => {
  if (nameInputResolve.value) {
    nameInputResolve.value(null)
    showNameInput.value = false
    nameInputResolve.value = null
    nameInputValue.value = ''
  }
}

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
    isLoading.value = true
    const response = await invoke<{ success: boolean; data: any; message?: string }>('get_book_content', {
      folderName: book.folder_name
    })
    if (response.success) {
      bookContent.value = response.data
      selectedSection.value = 0
    } else {
      console.error('Failed to load book content:', response.message)
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
        name: 'JSON',
        extensions: ['json']
      }],
      title: 'Select a 5etools book file to add to your library'
    })
    
    console.log('File selected:', selected)
    
    if (selected) {
      // Handle both string and array returns
      const filePath = Array.isArray(selected) ? selected[0] : selected
      
      // Extract filename without extension for default name
      const defaultName = filePath.split('/').pop()?.replace('.json', '') || 'Book'
      
      // For now, use a simple Vue reactive dialog since prompt() doesn't work in Tauri
      // We'll create a proper dialog component later
      const bookName = await showNameDialog(defaultName)
      if (!bookName) {
        console.log('User cancelled name input')
        return
      }
      
      console.log('Calling backend with:', { bookName, filePath })
      // Call backend to copy the file with custom name
      const response = await invoke<{ success: boolean; data?: string; message?: string }>('add_book_to_library', {
        bookName: bookName,
        filePath: filePath
      })
      
      console.log('Backend response:', response)
      
      if (response.success) {
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

// Handle adding images to a book
const handleAddImages = async () => {
  if (!selectedBook.value) return
  
  try {
    console.log('Opening image file dialog...')
    // Open file dialog for images
    const selected = await open({
      multiple: true,
      filters: [{
        name: 'Images',
        extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif']
      }],
      title: `Select images to add to "${selectedBook.value.name}"`
    })
    
    console.log('Images selected:', selected)
    
    if (selected) {
      // Handle both string and array returns
      const imagePaths = Array.isArray(selected) ? selected : [selected]
      
      console.log('Calling backend to add images:', { 
        folderName: selectedBook.value.folder_name, 
        imagePaths 
      })
      
      // Call backend to copy the images
      const response = await invoke<{ success: boolean; data?: number; message?: string }>('add_book_images', {
        folderName: selectedBook.value.folder_name,
        imagePaths: imagePaths
      })
      
      console.log('Backend response:', response)
      
      if (response.success) {
        console.log(`Added ${response.data} images successfully`)
        alert(`Successfully added ${response.data} images to "${selectedBook.value.name}"`)
        // Reload books list to update image count
        await loadLibraryBooks()
      } else {
        console.error('Failed to add images:', response.message)
        alert(`Failed to add images: ${response.message}`)
      }
    }
  } catch (error) {
    console.error('Error in handleAddImages:', error)
    alert(`Error adding images: ${error}`)
  }
}

// Remove book from library
const removeBook = async (book: BookInfo) => {
  if (!confirm(`Remove "${book.name}" from your library?`)) {
    return
  }
  
  try {
    const response = await invoke<{ success: boolean; message?: string }>('remove_book_from_library', {
      folderName: book.folder_name
    })
    
    if (response.success) {
      console.log('Book removed successfully')
      // Clear selection if it was the removed book
      if (selectedBook.value?.folder_name === book.folder_name) {
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

// Render a section with proper HTML
const renderSection = (section: any): string => {
  if (!section) return ''
  
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
    
    // Dice rolls {@dice 1d20+5} or {@damage 2d6}
    processed = processed.replace(/{@(?:dice|damage)\s+([^}]+)}/g, '<span class="dice-roll">$1</span>')
    
    // DC checks {@dc 15}
    processed = processed.replace(/{@dc\s+(\d+)}/g, '<span class="dc-check">DC $1</span>')
    
    // Skill checks {@skill Athletics} 
    processed = processed.replace(/{@skill\s+([^}]+)}/g, '<span class="skill-check">$1</span>')
    
    // Actions {@action Attack}
    processed = processed.replace(/{@action\s+([^}]+)}/g, '<span class="action-name">$1</span>')
    
    // Conditions {@condition poisoned}
    processed = processed.replace(/{@condition\s+([^}]+)}/g, (match, condition) => {
      return `<span class="condition" title="${condition}">${condition}</span>`
    })
    
    // Spells {@spell fireball} or {@spell fireball|phb|PHB}
    processed = processed.replace(/{@spell\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, spell, source, display) => {
      const spellName = display || spell
      const title = source ? `${spell} (${source})` : spell
      return `<span class="spell-ref" title="${title}">${spellName}</span>`
    })
    
    // Items {@item longsword} or {@item longsword|phb|PHB}
    processed = processed.replace(/{@item\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, item, source, display) => {
      const itemName = display || item
      const title = source ? `${item} (${source})` : item
      return `<span class="item-ref" title="${title}">${itemName}</span>`
    })
    
    // Creatures {@creature goblin} or {@creature goblin|mm|MM}
    processed = processed.replace(/{@creature\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, creature, source, display) => {
      const creatureName = display || creature
      const title = source ? `${creature} (${source})` : creature
      return `<span class="creature-ref" title="${title}">${creatureName}</span>`
    })
    
    // Races {@race dragonborn} or {@race dragonborn|phb}
    processed = processed.replace(/{@race\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, race, source, display) => {
      const raceName = display || race
      const title = source ? `${race} (${source})` : race
      return `<span class="race-ref" title="${title}">${raceName}</span>`
    })
    
    // Classes {@class fighter} or {@class fighter|phb}
    processed = processed.replace(/{@class\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, className, source, subclass, display) => {
      const displayName = display || className
      const title = subclass ? `${className} (${subclass})` : className
      return `<span class="class-ref" title="${title}">${displayName}</span>`
    })
    
    // Backgrounds {@background soldier} or {@background soldier|phb}
    processed = processed.replace(/{@background\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, background, source, display) => {
      const backgroundName = display || background
      const title = source ? `${background} (${source})` : background
      return `<span class="background-ref" title="${title}">${backgroundName}</span>`
    })
    
    // Feats {@feat alert} or {@feat alert|phb}
    processed = processed.replace(/{@feat\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, (match, feat, source, display) => {
      const featName = display || feat
      const title = source ? `${feat} (${source})` : feat
      return `<span class="feat-ref" title="${title}">${featName}</span>`
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
    processed = processed.replace(/{@hit\s+([+-]?\d+)}/g, '<span class="hit-bonus">$1</span>')
    
    // Recharge {@recharge} or {@recharge 5} 
    processed = processed.replace(/{@recharge\s*(\d+)?}/g, (match, num) => {
      return num ? `<span class="recharge">(Recharge ${num}-6)</span>` : '<span class="recharge">(Recharge)</span>'
    })
    
    return processed
  }
  
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
              folderName: selectedBook.value.folder_name,
              imageName: imageName
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

// Apply theme and initialize synchronization on mount
onMounted(async () => {
  themeStore.applyTheme()
  await themeStore.initThemeSync()
  await loadLibraryBooks()
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

/* Add Images Button */
.add-images-btn {
  padding: var(--spacing-xs) var(--spacing-sm);
  background-color: var(--color-primary);
  color: var(--color-primary-text);
  border: none;
  border-radius: var(--radius-sm);
  font-size: var(--font-size-xs);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.add-images-btn:hover {
  background-color: var(--color-primary-dark);
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

.dice-roll {
  display: inline-block;
  padding: 2px 6px;
  background: linear-gradient(135deg, #fbbf24, #f59e0b);
  color: #451a03;
  border-radius: 4px;
  font-weight: 600;
  font-family: 'Courier New', monospace;
  cursor: help;
}

.dc-check {
  display: inline-block;
  padding: 2px 8px;
  background: linear-gradient(135deg, #ef4444, #dc2626);
  color: white;
  border-radius: 4px;
  font-weight: 600;
  font-size: 0.9em;
}

.skill-check {
  color: #0891b2;
  font-weight: 600;
  text-transform: capitalize;
}

.action-name {
  color: #16a34a;
  font-weight: 600;
  text-transform: uppercase;
  font-size: 0.9em;
  letter-spacing: 0.5px;
}

.condition {
  color: #ea580c;
  font-weight: 600;
  cursor: help;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
}

.spell-ref {
  color: #7c3aed;
  font-style: italic;
  cursor: help;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
}

.spell-ref:hover {
  color: #6d28d9;
  text-decoration-style: solid;
}

.item-ref {
  color: #059669;
  font-weight: 500;
  cursor: help;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
}

.item-ref:hover {
  color: #047857;
  text-decoration-style: solid;
}

.creature-ref {
  color: #dc2626;
  font-weight: 600;
  cursor: help;
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
}

.creature-ref:hover {
  color: #b91c1c;
  text-decoration-style: solid;
}

.book-ref {
  color: var(--color-primary);
  font-weight: 500;
  font-style: italic;
  cursor: help;
}

.chance {
  color: #8b5cf6;
  font-weight: 500;
}

.hit-bonus {
  display: inline-block;
  padding: 1px 4px;
  background-color: rgba(34, 197, 94, 0.1);
  color: #16a34a;
  border-radius: 3px;
  font-weight: 600;
  font-family: 'Courier New', monospace;
}

.recharge {
  display: inline-block;
  padding: 2px 6px;
  background-color: rgba(59, 130, 246, 0.1);
  color: var(--color-primary);
  border-radius: 3px;
  font-weight: 500;
  font-size: 0.9em;
}

/* Dialog Styles */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.dialog-content {
  background-color: var(--color-bg-primary);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
  max-width: 400px;
  width: 90%;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
}

.dialog-content h3 {
  margin: 0 0 var(--spacing-sm) 0;
  color: var(--color-text-primary);
}

.dialog-content p {
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-lg);
  font-size: 0.9rem;
}

.name-input {
  width: 100%;
  padding: var(--spacing-sm);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background-color: var(--color-bg-secondary);
  color: var(--color-text-primary);
  font-size: 1rem;
  margin-bottom: var(--spacing-lg);
}

.name-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
}

.btn-cancel, .btn-submit {
  padding: var(--spacing-sm) var(--spacing-lg);
  border: none;
  border-radius: var(--radius-sm);
  font-size: 0.9rem;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-cancel {
  background-color: var(--color-bg-secondary);
  color: var(--color-text-secondary);
}

.btn-cancel:hover {
  background-color: var(--color-bg-tertiary);
}

.btn-submit {
  background-color: var(--color-primary);
  color: white;
}

.btn-submit:hover:not(:disabled) {
  background-color: var(--color-primary-dark);
  transform: translateY(-1px);
}

.btn-submit:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>

<style>
/* Global styles for book content (non-scoped) */
.book-content {
  padding: 10px 15px;
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

/* Formatting tags */
.book-content .spell-ref,
.book-content .item-ref,
.book-content .creature-ref,
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