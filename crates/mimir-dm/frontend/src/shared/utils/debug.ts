import { invoke as originalInvoke } from '@tauri-apps/api/core'

// Debug flag - set to true to enable IPC logging
const DEBUG_IPC = import.meta.env.DEV && typeof window !== 'undefined' && window.location.search.includes('debug=true')

// Wrapper for invoke to add logging
export async function debugInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (DEBUG_IPC) {
    console.group(`🔹 IPC: ${cmd}`)
    console.log('Arguments:', args)
    console.time('Duration')
  }
  
  try {
    const result = await originalInvoke<T>(cmd, args)
    
    if (DEBUG_IPC) {
      console.log('✅ Success:', result)
      console.timeEnd('Duration')
      console.groupEnd()
    }
    
    return result
  } catch (error) {
    if (DEBUG_IPC) {
      console.error('❌ Error:', error)
      console.timeEnd('Duration')
      console.groupEnd()
    }
    throw error
  }
}

// Helper to enable debug mode
export function enableDebugMode() {
  // Add debug=true to URL
  const url = new URL(window.location.href)
  url.searchParams.set('debug', 'true')
  window.history.pushState({}, '', url)
  
  console.log('🔧 Debug mode enabled - reload to see IPC logs')
  console.log('To disable: remove ?debug=true from URL')
}

// Log all document-related issues
export function debugDocument(action: string, data: any) {
  if (DEBUG_IPC) {
    console.group(`📄 Document Debug: ${action}`)
    console.log('Data:', data)
    console.trace('Call stack')
    console.groupEnd()
  }
}