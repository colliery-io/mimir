// Composable for managing book navigation and table of contents

import type { BookSection, SubEntry } from '../../types/book'

export function useBookNavigation() {
  
  // Get sub-entries from a section for TOC display
  function getSubEntries(section: BookSection): SubEntry[] {
    const entries: SubEntry[] = []
    
    function processEntries(items: any[], level: number = 0) {
      if (!items) return
      
      items.forEach((entry, index) => {
        if (typeof entry === 'object' && entry !== null) {
          // Add named entries to the sub-entries list
          if (entry.name && (entry.type === 'section' || entry.type === 'entries')) {
            entries.push({
              id: entry.id || `entry-${index}`,
              name: entry.name,
              level
            })
          }
          
          // Recursively process nested entries
          if (entry.entries && Array.isArray(entry.entries)) {
            processEntries(entry.entries, level + 1)
          }
        }
      })
    }
    
    if (section.entries && Array.isArray(section.entries)) {
      processEntries(section.entries)
    }
    
    return entries
  }
  
  // Scroll to top of content area
  function scrollToTop() {
    const contentArea = document.querySelector('.content-panel')
    if (contentArea) {
      contentArea.scrollTop = 0
    }
  }
  
  // Scroll to a specific element by ID
  function scrollToElement(elementId: string, smooth: boolean = true) {
    const element = document.getElementById(elementId)
    if (element) {
      element.scrollIntoView({ 
        behavior: smooth ? 'smooth' : 'auto', 
        block: 'start' 
      })
    }
  }
  
  return {
    getSubEntries,
    scrollToTop,
    scrollToElement
  }
}