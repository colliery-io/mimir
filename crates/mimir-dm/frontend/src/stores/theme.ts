import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Theme } from '../types/api'

export const useThemeStore = defineStore('theme', () => {
  const themes = ref<Theme[]>([])
  const currentTheme = ref<string>('light')
  
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
  
  // Change theme
  const setTheme = (theme: string) => {
    currentTheme.value = theme
    saveTheme(theme)
  }
  
  return {
    themes,
    currentTheme,
    loadThemes,
    applyTheme,
    setTheme
  }
})