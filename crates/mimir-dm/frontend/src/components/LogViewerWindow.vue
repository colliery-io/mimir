<template>
  <div :class="['log-viewer-window', currentTheme]">
    <!-- Header -->
    <div class="log-viewer-header">
      <div class="header-info">
        <h1 class="window-title">
          Log Viewer
          <span v-if="currentFileName" class="file-name">- {{ currentFileName }}</span>
        </h1>
        <div class="file-info">
          <span v-if="logContent.total_lines" class="line-count">
            {{ logContent.total_lines }} lines
          </span>
          <span v-if="currentFileName && lastUpdated" class="last-updated">
            Updated: {{ formatTime(lastUpdated) }}
          </span>
        </div>
      </div>
      
      <div class="header-controls">
        <button 
          @click="toggleAutoScroll" 
          :class="['control-button', { active: autoScroll }]"
          title="Auto-scroll to bottom"
        >
          📜 Auto-scroll
        </button>
        <button 
          @click="toggleLiveMode" 
          :class="['control-button', { active: liveMode }]"
          title="Live updates"
        >
          {{ liveMode ? '⏸️' : '▶️' }} Live
        </button>
        <button 
          @click="refreshLogs" 
          class="control-button"
          title="Refresh"
        >
          🔄 Refresh
        </button>
      </div>
    </div>
    
    <!-- Search and Filters -->
    <div class="log-controls">
      <div class="search-bar">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search logs..."
          class="search-input"
          @input="onSearchInput"
        />
        <button v-if="searchQuery" @click="clearSearch" class="clear-search">
          ✕
        </button>
      </div>
      
      <div v-if="!isChatLog" class="log-level-filters">
        <button
          v-for="level in logLevels"
          :key="level"
          @click="toggleLogLevel(level)"
          :class="['level-button', level.toLowerCase(), { active: activeLevels.has(level) }]"
        >
          {{ level }}
        </button>
      </div>
    </div>
    
    <!-- Loading state -->
    <div v-if="loading" class="loading-container">
      <div class="spinner"></div>
      <p>Loading log content...</p>
    </div>
    
    <!-- Error state -->
    <div v-else-if="error" class="error-container">
      <p class="error-message">{{ error }}</p>
      <button @click="refreshLogs" class="retry-button">Try Again</button>
    </div>
    
    <!-- Log content -->
    <div v-else class="log-content-container">
      <div 
        ref="logContentRef" 
        class="log-content"
        @scroll="onScroll"
      >
        <div v-if="filteredLines.length === 0" class="no-content">
          <p v-if="searchQuery">No lines match your search criteria</p>
          <p v-else-if="currentFileName">No content to display</p>
          <p v-else>Select a log file to view</p>
        </div>
        
        <div v-else class="log-lines">
          <div
            v-for="(line, index) in filteredLines"
            :key="`${line.lineNumber}-${index}`"
            :class="['log-line', getLogLevelClass(line.content)]"
          >
            <span class="line-number">{{ line.lineNumber }}</span>
            <span class="line-content" v-html="highlightSearch(line.content)"></span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useThemeStore } from '@/stores/theme'

interface LogContent {
  lines: string[]
  total_lines: number
  position: number
}

interface LogContentResponse {
  success: boolean
  data?: LogContent
  error?: string
}

interface LogTailResponse {
  success: boolean
  data?: {
    new_lines: string[]
    new_position: number
  }
  error?: string
}

interface LogLine {
  content: string
  lineNumber: number
}

// Component state
const currentFileName = ref<string>('')
const logContent = ref<LogContent>({ lines: [], total_lines: 0, position: 0 })
const loading = ref(true)
const error = ref<string | null>(null)
const searchQuery = ref('')
const autoScroll = ref(true)
const liveMode = ref(true)
const lastUpdated = ref<Date | null>(null)
const logContentRef = ref<HTMLElement | null>(null)

// Log levels and filtering
const logLevels = ['ERROR', 'WARN', 'INFO', 'DEBUG', 'TRACE']
const activeLevels = ref(new Set(['ERROR', 'WARN', 'INFO']))

// Polling for live updates
let pollInterval: number | null = null

// Theme support
const themeStore = useThemeStore()
const currentTheme = computed(() => `theme-${themeStore.currentTheme}`)

// Detect if this is a chat log (ends with .log and is not mimir.log)
const isChatLog = computed(() => {
  return currentFileName.value.endsWith('.log') && !currentFileName.value.startsWith('mimir.log')
})

// Get current window and parse filename from label
onMounted(async () => {
  // Initialize theme
  await themeStore.loadThemes()
  themeStore.applyTheme()
  await themeStore.initThemeSync()
  
  const webview = getCurrentWebviewWindow()
  const label = webview.label
  
  // Extract filename from URL query parameter
  const urlParams = new URLSearchParams(window.location.search)
  const fileName = urlParams.get('file')
  
  if (fileName) {
    currentFileName.value = decodeURIComponent(fileName)
    await loadLogContent()
    
    if (liveMode.value) {
      startPolling()
    }
  } else {
    error.value = 'No log file specified'
    loading.value = false
  }
})

onUnmounted(() => {
  stopPolling()
})

// Watch live mode to start/stop polling
watch(liveMode, (newValue) => {
  if (newValue) {
    startPolling()
  } else {
    stopPolling()
  }
})

// Load log content from backend
const loadLogContent = async () => {
  if (!currentFileName.value) return
  
  try {
    loading.value = true
    error.value = null
    
    const response = await invoke<LogContentResponse>('read_log_file', {
      fileName: currentFileName.value,
      offset: 0,
      limit: 10000 // Load up to 10k lines initially
    })
    
    if (response.success && response.data) {
      logContent.value = response.data
      lastUpdated.value = new Date()
      
      if (autoScroll.value) {
        await nextTick()
        scrollToBottom()
      }
    } else {
      error.value = response.error || 'Failed to load log content'
    }
  } catch (err) {
    console.error('Failed to load log content:', err)
    error.value = err instanceof Error ? err.message : 'Unknown error occurred'
  } finally {
    loading.value = false
  }
}

// Start polling for new log content
const startPolling = () => {
  if (pollInterval) return
  
  pollInterval = window.setInterval(async () => {
    await pollForUpdates()
  }, 1000) // Poll every second
}

// Stop polling
const stopPolling = () => {
  if (pollInterval) {
    clearInterval(pollInterval)
    pollInterval = null
  }
}

// Poll for new log content
const pollForUpdates = async () => {
  if (!currentFileName.value || loading.value) return
  
  try {
    const response = await invoke<LogTailResponse>('tail_log_file', {
      fileName: currentFileName.value,
      lastPosition: logContent.value.position
    })
    
    if (response.success && response.data && response.data.new_lines.length > 0) {
      // Append new lines to existing content
      logContent.value.lines.push(...response.data.new_lines)
      logContent.value.total_lines += response.data.new_lines.length
      logContent.value.position = response.data.new_position
      lastUpdated.value = new Date()
      
      if (autoScroll.value) {
        await nextTick()
        scrollToBottom()
      }
    }
  } catch (err) {
    console.error('Failed to poll for updates:', err)
  }
}

// Refresh logs manually
const refreshLogs = () => {
  loadLogContent()
}

// Toggle auto-scroll
const toggleAutoScroll = () => {
  autoScroll.value = !autoScroll.value
  if (autoScroll.value) {
    nextTick(() => scrollToBottom())
  }
}

// Toggle live mode
const toggleLiveMode = () => {
  liveMode.value = !liveMode.value
}

// Scroll to bottom of log content
const scrollToBottom = () => {
  if (logContentRef.value) {
    logContentRef.value.scrollTop = logContentRef.value.scrollHeight
  }
}

// Handle scroll events
const onScroll = () => {
  if (!logContentRef.value) return
  
  const { scrollTop, scrollHeight, clientHeight } = logContentRef.value
  const isAtBottom = scrollTop + clientHeight >= scrollHeight - 10 // 10px tolerance
  
  // Disable auto-scroll if user scrolls up manually
  if (!isAtBottom && autoScroll.value) {
    autoScroll.value = false
  }
}

// Process lines with line numbers
const processedLines = computed<LogLine[]>(() => {
  return logContent.value.lines.map((line, index) => ({
    content: line,
    lineNumber: index + 1
  }))
})

// Filter lines based on search and log levels
const filteredLines = computed<LogLine[]>(() => {
  let lines = processedLines.value
  
  // Filter by log levels (only for non-chat logs)
  if (!isChatLog.value && activeLevels.value.size < logLevels.length) {
    lines = lines.filter(line => {
      return Array.from(activeLevels.value).some(level => 
        line.content.includes(`[${level}]`) || 
        line.content.includes(level.toUpperCase())
      )
    })
  }
  
  // Filter by search query
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    lines = lines.filter(line => 
      line.content.toLowerCase().includes(query)
    )
  }
  
  return lines
})

// Toggle log level filter
const toggleLogLevel = (level: string) => {
  if (activeLevels.value.has(level)) {
    activeLevels.value.delete(level)
  } else {
    activeLevels.value.add(level)
  }
  // Create new Set to trigger reactivity
  activeLevels.value = new Set(activeLevels.value)
}

// Clear search
const clearSearch = () => {
  searchQuery.value = ''
}

// Handle search input
const onSearchInput = () => {
  // Search is reactive through computed property
}

// Get CSS class for log level
const getLogLevelClass = (content: string): string => {
  if (content.includes('[ERROR]') || content.includes('ERROR')) return 'log-error'
  if (content.includes('[WARN]') || content.includes('WARN')) return 'log-warn'  
  if (content.includes('[INFO]') || content.includes('INFO')) return 'log-info'
  if (content.includes('[DEBUG]') || content.includes('DEBUG')) return 'log-debug'
  if (content.includes('[TRACE]') || content.includes('TRACE')) return 'log-trace'
  return 'log-default'
}

// Highlight search terms in content
const highlightSearch = (content: string): string => {
  if (!searchQuery.value.trim()) return content
  
  const query = searchQuery.value.trim()
  const regex = new RegExp(`(${query})`, 'gi')
  return content.replace(regex, '<strong>$1</strong>')
}

// Format time for display
const formatTime = (date: Date): string => {
  return date.toLocaleTimeString()
}
</script>

<style scoped>
.log-viewer-window {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--color-surface);
  color: var(--color-text);
  font-family: ui-monospace, 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
}

/* Header */
.log-viewer-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
  background: var(--color-surface-variant);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}

.header-info {
  flex: 1;
}

.window-title {
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0 0 var(--spacing-xs) 0;
}

.file-name {
  font-weight: 400;
  color: var(--color-primary-600);
}

.file-info {
  display: flex;
  gap: var(--spacing-md);
  font-size: 0.75rem;
  color: var(--color-text-secondary);
}

.header-controls {
  display: flex;
  gap: var(--spacing-sm);
}

.control-button {
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-size: 0.75rem;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.control-button:hover {
  background: var(--color-gray-100);
  border-color: var(--color-border-hover);
}

.control-button.active {
  background: var(--color-primary-100);
  border-color: var(--color-primary-300);
  color: var(--color-primary-700);
}

.theme-dark .control-button:hover {
  background: var(--color-gray-700);
}

.theme-dark .control-button.active {
  background: var(--color-primary-900);
  color: var(--color-primary-300);
}

/* Controls */
.log-controls {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}

.search-bar {
  position: relative;
  flex: 1;
  max-width: 300px;
}

.search-input {
  width: 100%;
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  transition: border-color var(--transition-fast);
}

.search-input:focus {
  outline: none;
  border-color: var(--color-primary-500);
}

.clear-search {
  position: absolute;
  right: var(--spacing-xs);
  top: 50%;
  transform: translateY(-50%);
  background: none;
  border: none;
  color: var(--color-text-secondary);
  cursor: pointer;
  padding: 0;
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.log-level-filters {
  display: flex;
  gap: var(--spacing-xs);
}

.level-button {
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-size: 0.75rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.level-button.error {
  border-color: #dc2626;
  color: #dc2626;
}

.level-button.warn {
  border-color: #f59e0b;
  color: #f59e0b;
}

.level-button.info {
  border-color: #3b82f6;
  color: #3b82f6;
}

.level-button.debug {
  border-color: #10b981;
  color: #10b981;
}

.level-button.trace {
  border-color: #8b5cf6;
  color: #8b5cf6;
}

.level-button.active {
  font-weight: 700;
  border-width: 2px;
}

/* Content */
.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  gap: var(--spacing-md);
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--color-border);
  border-top: 3px solid var(--color-primary-500);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  gap: var(--spacing-md);
}

.error-message {
  color: var(--color-error, #dc2626);
}

.retry-button {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-primary-500);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
}

.log-content-container {
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.log-content {
  height: 100%;
  overflow-y: auto;
  padding: var(--spacing-sm);
  background: var(--color-surface);
}

.no-content {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--color-text-secondary);
  font-style: italic;
}

.log-lines {
  display: flex;
  flex-direction: column;
}

.log-line {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  font-size: 0.875rem;
  line-height: 1.4;
  border-radius: var(--radius-sm);
  margin-bottom: 1px;
}

.log-line:hover {
  background: var(--color-surface-variant);
}

.line-number {
  flex-shrink: 0;
  width: 50px;
  text-align: right;
  color: var(--color-text-secondary);
  font-size: 0.75rem;
  padding-top: 1px;
}

.line-content {
  flex: 1;
  word-break: break-word;
  white-space: pre-wrap;
}

/* Log level colors */
.log-error {
  border-left: 3px solid #dc2626;
  background: rgba(220, 38, 38, 0.05);
}

.log-warn {
  border-left: 3px solid #f59e0b;
  background: rgba(245, 158, 11, 0.05);
}

.log-info {
  border-left: 3px solid #3b82f6;
  background: rgba(59, 130, 246, 0.05);
}

.log-debug {
  border-left: 3px solid #10b981;
  background: rgba(16, 185, 129, 0.05);
}

.log-trace {
  border-left: 3px solid #8b5cf6;
  background: rgba(139, 92, 246, 0.05);
}

/* Search highlighting */
.line-content :deep(strong) {
  font-weight: 700;
  color: var(--color-primary-600);
}

.theme-dark .line-content :deep(strong) {
  color: var(--color-primary-400);
}
</style>