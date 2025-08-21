// Composable for managing the book library

import { ref } from 'vue'
import type { Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import type { BookInfo } from '../../../types/book'

export function useBookLibrary() {
  const libraryBooks: Ref<BookInfo[]> = ref([])
  const selectedBook: Ref<BookInfo | null> = ref(null)
  const isLoadingLibrary = ref(false)

  // Check for development mode
  const isDevelopment = import.meta.env.DEV
  
  // Install dev test book if in dev mode
  async function installDevTestBook() {
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
  }

  // Load library books from backend
  async function loadLibraryBooks() {
    try {
      isLoadingLibrary.value = true
      const response = await invoke<{ success: boolean; data: BookInfo[]; message?: string }>('list_library_books')
      
      if (response.success && response.data) {
        libraryBooks.value = response.data
        
        // Auto-select first book if none selected
        if (!selectedBook.value && response.data.length > 0) {
          selectedBook.value = response.data[0]
        }
      } else {
        console.error('Failed to load library books:', response.message)
        libraryBooks.value = []
      }
    } catch (error) {
      console.error('Failed to load library books:', error)
      libraryBooks.value = []
    } finally {
      isLoadingLibrary.value = false
    }
  }

  // Add a new book to the library
  async function addBook(): Promise<boolean> {
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
          return true
        } else {
          console.error('Failed to add book:', response.message)
          alert(`Failed to add book: ${response.message}`)
          return false
        }
      } else {
        console.log('No file selected')
        return false
      }
    } catch (error) {
      console.error('Failed to add book:', error)
      alert('Failed to add book. Please try again.')
      return false
    }
  }

  // Remove a book from the library
  async function removeBook(book: BookInfo): Promise<boolean> {
    if (!confirm(`Are you sure you want to remove "${book.name}" from your library?`)) {
      return false
    }

    try {
      const response = await invoke<{ success: boolean; message?: string }>('remove_book_from_library', {
        bookId: book.id
      })
      
      if (response.success) {
        // Reload the library
        await loadLibraryBooks()
        
        // Clear selection if this was the selected book
        if (selectedBook.value?.id === book.id) {
          selectedBook.value = libraryBooks.value[0] || null
        }
        
        return true
      } else {
        console.error('Failed to remove book:', response.message)
        alert(`Failed to remove book: ${response.message}`)
        return false
      }
    } catch (error) {
      console.error('Failed to remove book:', error)
      alert('Failed to remove book. Please try again.')
      return false
    }
  }

  // Select a book
  function selectBook(book: BookInfo | null) {
    selectedBook.value = book
  }

  return {
    libraryBooks,
    selectedBook,
    isLoadingLibrary,
    isDevelopment,
    loadLibraryBooks,
    addBook,
    removeBook,
    selectBook,
    installDevTestBook
  }
}