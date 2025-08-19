import { defineStore } from 'pinia'
import { ref, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { emit, listen } from '@tauri-apps/api/event'
import type { Theme } from '../types/api'
import type { UnlistenFn } from '@tauri-apps/api/event'

export const useThemeStore = defineStore('theme', () => {
  const themes = ref<Theme[]>([])
  const currentTheme = ref<string>('light')
  let unlistenThemeChange: UnlistenFn | null = null
  
  // Load available themes from backend
  const loadThemes = async () => {
    try {
      const response = await invoke<{ success: boolean; data: Theme[] }>('get_themes')
      if (response.success) {
        themes.value = response.data
      }
    } catch (error) {
      console.error('Failed to load themes:', error)
    }
  }
  
  // Get saved theme preference from localStorage
  const getSavedTheme = (): string => {
    return localStorage.getItem('theme') || 'light'
  }
  
  // Save theme preference to localStorage
  const saveTheme = (theme: string) => {
    localStorage.setItem('theme', theme)
  }
  
  // Apply theme to the application
  const applyTheme = () => {
    const savedTheme = getSavedTheme()
    currentTheme.value = savedTheme
  }
  
  // Change theme and broadcast to other windows
  const setTheme = async (theme: string, broadcast = true) => {
    console.log(`[ThemeStore] Setting theme to: ${theme}, broadcast: ${broadcast}`)
    currentTheme.value = theme
    saveTheme(theme)
    
    // Broadcast theme change to all windows
    if (broadcast) {
      try {
        console.log(`[ThemeStore] Broadcasting theme change: ${theme}`)
        await emit('theme-changed', { theme })
        console.log(`[ThemeStore] Theme change broadcast successful`)
      } catch (error) {
        console.error('Failed to broadcast theme change:', error)
      }
    }
  }
  
  // Initialize cross-window theme synchronization
  const initThemeSync = async () => {
    console.log('[ThemeStore] Initializing theme sync...')
    // Listen for theme changes from other windows
    unlistenThemeChange = await listen<{ theme: string }>('theme-changed', (event) => {
      console.log(`[ThemeStore] Received theme change event:`, event.payload)
      // Update theme without broadcasting to avoid infinite loop
      setTheme(event.payload.theme, false)
    })
    console.log('[ThemeStore] Theme sync initialized')
  }
  
  // Clean up event listener
  const cleanup = () => {
    if (unlistenThemeChange) {
      unlistenThemeChange()
      unlistenThemeChange = null
    }
  }
  
  // Auto-cleanup when store is disposed
  onUnmounted(() => {
    cleanup()
  })
  
  return {
    themes,
    currentTheme,
    loadThemes,
    applyTheme,
    setTheme,
    initThemeSync,
    cleanup
  }
})