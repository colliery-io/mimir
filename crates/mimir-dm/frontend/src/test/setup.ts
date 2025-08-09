import { vi } from 'vitest'
import { config } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'

// Setup Pinia
const pinia = createPinia()
setActivePinia(pinia)

// Add Pinia to Vue Test Utils global plugins
config.global.plugins = [pinia]

// Ensure window object exists for Tauri mocks
global.window = global.window || {}

// Setup default Tauri mock
if (!global.window.__TAURI_INTERNALS__) {
  global.window.__TAURI_INTERNALS__ = {
    invoke: vi.fn(() => Promise.resolve({ success: true, data: null }))
  }
}

// Mock import.meta.env
Object.defineProperty(import.meta, 'env', {
  value: {
    DEV: true,
    PROD: false
  }
})

// Mock router-link component globally
config.global.stubs = {
  'router-link': {
    template: '<a><slot /></a>'
  }
}

// Mock router globally
const mockRouter = {
  currentRoute: {
    value: {
      params: {}
    }
  },
  push: vi.fn(),
  replace: vi.fn()
}

// Add to window for components that might use it
;(global as any).mockRouter = mockRouter