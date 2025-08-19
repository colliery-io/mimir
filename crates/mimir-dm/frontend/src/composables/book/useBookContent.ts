// Composable for managing book content loading and rendering

import { ref, watch } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { BookInfo, BookContent, BookSection } from '../../types/book'

export function useBookContent() {
  const bookContent: Ref<any | null> = ref(null)
  const selectedSection = ref(0)
  const isLoading = ref(false)
  const error: Ref<string | null> = ref(null)

  // Load book content from backend
  async function loadBookContent(book: BookInfo) {
    try {
      console.log('Loading book content for:', book.id, book)
      isLoading.value = true
      error.value = null
      
      const response = await invoke<BookContent>('get_book_content', {
        bookId: book.id
      })
      
      console.log('Book content response:', response)
      
      if (response.success && response.data) {
        console.log('Book content loaded successfully:', response.data)
        bookContent.value = response.data
        selectedSection.value = 0
      } else {
        console.error('Failed to load book content:', response.message)
        bookContent.value = null
        error.value = response.message || 'Failed to load book content'
      }
    } catch (err) {
      console.error('Error loading book content:', err)
      bookContent.value = null
      error.value = 'An error occurred while loading the book'
    } finally {
      isLoading.value = false
    }
  }

  // Get section name from a section object
  function getSectionName(section: BookSection): string {
    if (!section) return 'Untitled Section'
    if (section.name) return section.name
    if (section.type === 'section' && section.entries && section.entries.length > 0) {
      const firstEntry = section.entries[0]
      if (typeof firstEntry === 'string') return firstEntry.substring(0, 50)
      if (firstEntry && typeof firstEntry === 'object' && firstEntry.name) return firstEntry.name
    }
    return 'Untitled Section'
  }

  // Jump to a specific entry within a section
  function jumpToEntry(sectionIndex: number, entryId: string) {
    selectedSection.value = sectionIndex
    // Wait for DOM update then scroll to the entry
    setTimeout(() => {
      const element = document.getElementById(entryId)
      if (element) {
        element.scrollIntoView({ behavior: 'smooth', block: 'start' })
      }
    }, 100)
  }

  // Get the current selected section
  function getCurrentSection(): BookSection | null {
    if (!bookContent.value?.data || !Array.isArray(bookContent.value.data)) {
      return null
    }
    return bookContent.value.data[selectedSection.value] || null
  }

  // Clear content
  function clearContent() {
    bookContent.value = null
    selectedSection.value = 0
    error.value = null
  }

  return {
    bookContent,
    selectedSection,
    isLoading,
    error,
    loadBookContent,
    getSectionName,
    jumpToEntry,
    getCurrentSection,
    clearContent
  }
}