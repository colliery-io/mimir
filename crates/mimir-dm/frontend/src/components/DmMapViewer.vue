<template>
  <div class="dm-map-viewer">
    <!-- Toolbar -->
    <div class="viewer-toolbar">
      <div class="toolbar-group">
        <span class="toolbar-label">Zoom:</span>
        <button class="toolbar-btn" @click="zoomOut" :disabled="!mapImageUrl">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M4 10a.75.75 0 01.75-.75h10.5a.75.75 0 010 1.5H4.75A.75.75 0 014 10z" clip-rule="evenodd" />
          </svg>
        </button>
        <span class="zoom-level">{{ Math.round(zoom * 100) }}%</span>
        <button class="toolbar-btn" @click="zoomIn" :disabled="!mapImageUrl">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path d="M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z" />
          </svg>
        </button>
        <button class="toolbar-btn" @click="resetView" :disabled="!mapImageUrl" title="Reset view">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M15.312 11.424a5.5 5.5 0 01-9.201 2.466l-.312-.311h2.433a.75.75 0 000-1.5H3.989a.75.75 0 00-.75.75v4.242a.75.75 0 001.5 0v-2.43l.31.31a7 7 0 0011.712-3.138.75.75 0 00-1.449-.39zm1.23-3.723a.75.75 0 00.219-.53V2.929a.75.75 0 00-1.5 0V5.36l-.31-.31A7 7 0 003.239 8.188a.75.75 0 101.448.389A5.5 5.5 0 0113.89 6.11l.311.31h-2.432a.75.75 0 000 1.5h4.243a.75.75 0 00.53-.219z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>

      <div class="toolbar-group">
        <button
          class="toolbar-btn sync-btn"
          :class="{ active: autoSync }"
          @click="toggleAutoSync"
          title="Auto-sync viewport to player display"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path d="M12.232 4.232a2.5 2.5 0 013.536 3.536l-1.225 1.224a.75.75 0 001.061 1.06l1.224-1.224a4 4 0 00-5.656-5.656l-3 3a4 4 0 00.225 5.865.75.75 0 00.977-1.138 2.5 2.5 0 01-.142-3.667l3-3z" />
            <path d="M11.603 7.963a.75.75 0 00-.977 1.138 2.5 2.5 0 01.142 3.667l-3 3a2.5 2.5 0 01-3.536-3.536l1.225-1.224a.75.75 0 00-1.061-1.06l-1.224 1.224a4 4 0 105.656 5.656l3-3a4 4 0 00-.225-5.865z" />
          </svg>
          <span>{{ autoSync ? 'Synced' : 'Sync' }}</span>
        </button>
        <button
          v-if="!autoSync"
          class="toolbar-btn push-btn"
          @click="pushViewport"
          :disabled="!mapImageUrl || !isDisplayOpen"
          title="Push current view to player display"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" />
          </svg>
          <span>Push View</span>
        </button>
      </div>
    </div>

    <!-- Map Viewport -->
    <div
      class="map-viewport"
      ref="viewport"
      @mousedown="startPan"
      @mousemove="onPan"
      @mouseup="endPan"
      @mouseleave="endPan"
      @wheel.prevent="onWheel"
    >
      <div v-if="loading" class="loading-state">
        Loading map...
      </div>

      <div v-else-if="!mapImageUrl" class="empty-state">
        <p>No map selected</p>
        <p class="empty-hint">Select a map from the sidebar to view and control it</p>
      </div>

      <div
        v-else
        class="map-container"
        :style="mapContainerStyle"
      >
        <img
          :src="mapImageUrl"
          :alt="mapName"
          class="map-image"
          @load="onImageLoad"
          ref="mapImage"
          draggable="false"
        />

        <!-- Grid Overlay -->
        <svg
          v-if="showGrid && gridType !== 'none' && imageLoaded"
          class="grid-overlay"
          :viewBox="`0 0 ${mapWidth} ${mapHeight}`"
          :style="{ width: mapWidth + 'px', height: mapHeight + 'px' }"
        >
          <defs>
            <pattern
              v-if="gridType === 'square'"
              id="dmGridPattern"
              :width="effectiveGridSize"
              :height="effectiveGridSize"
              patternUnits="userSpaceOnUse"
              :x="effectiveGridOffsetX"
              :y="effectiveGridOffsetY"
            >
              <rect
                :width="effectiveGridSize"
                :height="effectiveGridSize"
                fill="none"
                stroke="rgba(255, 255, 255, 0.4)"
                stroke-width="1"
              />
            </pattern>
            <pattern
              v-if="gridType === 'hex'"
              id="dmGridPattern"
              :width="effectiveGridSize * 1.5"
              :height="effectiveGridSize * 1.732"
              patternUnits="userSpaceOnUse"
              :x="effectiveGridOffsetX"
              :y="effectiveGridOffsetY"
            >
              <polygon
                :points="hexPoints"
                fill="none"
                stroke="rgba(255, 255, 255, 0.4)"
                stroke-width="1"
              />
            </pattern>
          </defs>
          <rect width="100%" height="100%" fill="url(#dmGridPattern)" />
        </svg>
      </div>
    </div>

    <!-- Status Bar -->
    <div class="status-bar">
      <span v-if="mapName">{{ mapName }}</span>
      <span v-if="mapWidth && mapHeight" class="dim">{{ mapWidth }}x{{ mapHeight }}</span>
      <span class="dim">Pan: {{ Math.round(panX) }}, {{ Math.round(panY) }}</span>
      <span v-if="isDisplayOpen" class="status-indicator connected">Display Connected</span>
      <span v-else class="status-indicator disconnected">Display Disconnected</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { usePlayerDisplay } from '@/composables/usePlayerDisplay'

// Throttle helper for smooth updates
function throttle<T extends (...args: any[]) => void>(fn: T, limit: number): T {
  let lastCall = 0
  let pendingCall: number | null = null

  return ((...args: any[]) => {
    const now = Date.now()
    const remaining = limit - (now - lastCall)

    if (remaining <= 0) {
      if (pendingCall) {
        cancelAnimationFrame(pendingCall)
        pendingCall = null
      }
      lastCall = now
      fn(...args)
    } else if (!pendingCall) {
      pendingCall = requestAnimationFrame(() => {
        lastCall = Date.now()
        pendingCall = null
        fn(...args)
      })
    }
  }) as T
}

interface Props {
  mapId: number | null
  gridType?: string
  gridSizePx?: number | null
  gridOffsetX?: number
  gridOffsetY?: number
  showGrid?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  gridType: 'none',
  gridSizePx: null,
  gridOffsetX: 0,
  gridOffsetY: 0,
  showGrid: true
})

const { isDisplayOpen, updateViewport } = usePlayerDisplay()

// Computed grid values (with defaults for null)
const effectiveGridSize = computed(() => props.gridSizePx ?? 70)
const effectiveGridOffsetX = computed(() => props.gridOffsetX ?? 0)
const effectiveGridOffsetY = computed(() => props.gridOffsetY ?? 0)

// Map state
const loading = ref(false)
const mapImageUrl = ref<string | null>(null)
const mapName = ref('')
const mapWidth = ref(0)
const mapHeight = ref(0)
const imageLoaded = ref(false)

// View state
const panX = ref(0)
const panY = ref(0)
const zoom = ref(1)
const autoSync = ref(true)

// Pan/zoom interaction state
const isPanning = ref(false)
const isZooming = ref(false)
const lastMouseX = ref(0)
const lastMouseY = ref(0)
let zoomTimeout: number | null = null

// Refs
const viewport = ref<HTMLElement | null>(null)
const mapImage = ref<HTMLImageElement | null>(null)

// Computed styles - use translate3d/scale3d for GPU compositing
const isInteracting = computed(() => isPanning.value || isZooming.value)
const mapContainerStyle = computed(() => ({
  // Use 3D transforms to force GPU layer compositing
  transform: `translate3d(${panX.value}px, ${panY.value}px, 0) scale3d(${zoom.value}, ${zoom.value}, 1)`,
  transformOrigin: 'center center',
  transition: isInteracting.value ? 'none' : 'transform 0.1s ease-out',
  willChange: 'transform',
  backfaceVisibility: 'hidden' as const
}))

// Hex grid points calculation
const hexPoints = computed(() => {
  const size = effectiveGridSize.value
  const h = size * 0.866
  return `${size * 0.5},0 ${size},${h * 0.5} ${size},${h * 1.5} ${size * 0.5},${h * 2} 0,${h * 1.5} 0,${h * 0.5}`
})

// Load map image when mapId changes
watch(() => props.mapId, async (newId) => {
  if (newId) {
    await loadMapImage(newId)
  } else {
    mapImageUrl.value = null
    mapName.value = ''
    mapWidth.value = 0
    mapHeight.value = 0
    imageLoaded.value = false
  }
}, { immediate: true })

async function loadMapImage(id: number) {
  console.log('DmMapViewer: Loading map with id:', id)
  loading.value = true
  imageLoaded.value = false

  try {
    // Get map details
    const mapResponse = await invoke<{ success: boolean; data?: any }>('get_map', { id })
    console.log('DmMapViewer: get_map response:', mapResponse)
    if (mapResponse.success && mapResponse.data) {
      mapName.value = mapResponse.data.name
      mapWidth.value = mapResponse.data.width_px
      mapHeight.value = mapResponse.data.height_px
    }

    // Get map image
    const imageResponse = await invoke<{ success: boolean; data?: string }>('serve_map_image', { id })
    console.log('DmMapViewer: serve_map_image response success:', imageResponse.success, 'has data:', !!imageResponse.data)
    if (imageResponse.success && imageResponse.data) {
      mapImageUrl.value = imageResponse.data
    }
  } catch (e) {
    console.error('DmMapViewer: Failed to load map:', e)
  } finally {
    loading.value = false
  }
}

function onImageLoad() {
  imageLoaded.value = true
  resetView()
}

// Zoom controls
function zoomIn() {
  zoom.value = Math.min(zoom.value * 1.25, 5)
  syncViewportIfNeeded()
}

function zoomOut() {
  zoom.value = Math.max(zoom.value / 1.25, 0.1)
  syncViewportIfNeeded()
}

function onWheel(event: WheelEvent) {
  const delta = event.deltaY > 0 ? 0.9 : 1.1
  const newZoom = Math.max(0.1, Math.min(5, zoom.value * delta))

  // Mark as zooming for smooth updates
  isZooming.value = true
  if (zoomTimeout) clearTimeout(zoomTimeout)
  zoomTimeout = window.setTimeout(() => {
    isZooming.value = false
  }, 150)

  // Zoom toward mouse position
  if (viewport.value) {
    const rect = viewport.value.getBoundingClientRect()
    const mouseX = event.clientX - rect.left - rect.width / 2
    const mouseY = event.clientY - rect.top - rect.height / 2

    const zoomRatio = newZoom / zoom.value
    panX.value = mouseX - (mouseX - panX.value) * zoomRatio
    panY.value = mouseY - (mouseY - panY.value) * zoomRatio
  }

  zoom.value = newZoom
  throttledSync()
}

function resetView() {
  panX.value = 0
  panY.value = 0
  zoom.value = 1
  syncViewportIfNeeded()
}

// Pan controls
function startPan(event: MouseEvent) {
  if (event.button !== 0) return // Only left click
  isPanning.value = true
  lastMouseX.value = event.clientX
  lastMouseY.value = event.clientY
}

function onPan(event: MouseEvent) {
  if (!isPanning.value) return

  const deltaX = event.clientX - lastMouseX.value
  const deltaY = event.clientY - lastMouseY.value

  panX.value += deltaX
  panY.value += deltaY

  lastMouseX.value = event.clientX
  lastMouseY.value = event.clientY

  // Use throttled sync during panning for smoothness
  throttledSync()
}

function endPan() {
  if (isPanning.value) {
    isPanning.value = false
    // Final sync to ensure we capture the end position
    syncViewportIfNeeded()
  }
}

// Sync controls
function toggleAutoSync() {
  autoSync.value = !autoSync.value
  if (autoSync.value) {
    syncViewportIfNeeded()
  }
}

function syncViewportIfNeeded() {
  if (autoSync.value && isDisplayOpen.value) {
    pushViewport()
  }
}

// Throttled sync for smooth panning - only sync every 50ms during drag
const throttledSync = throttle(() => {
  syncViewportIfNeeded()
}, 50)

async function pushViewport() {
  if (!isDisplayOpen.value) return

  try {
    // Convert pan coordinates to normalized values
    // The player display expects x, y as offsets from center
    await updateViewport(panX.value, panY.value, zoom.value)
  } catch (e) {
    console.error('Failed to push viewport:', e)
  }
}

// Keyboard shortcuts
function handleKeydown(event: KeyboardEvent) {
  if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
    return
  }

  switch (event.key) {
    case '+':
    case '=':
      zoomIn()
      break
    case '-':
      zoomOut()
      break
    case '0':
      resetView()
      break
    case 'p':
    case 'P':
      if (!autoSync.value) {
        pushViewport()
      }
      break
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.dm-map-viewer {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-base-200);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.viewer-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}

.toolbar-group {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.toolbar-label {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  font-weight: 500;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs) var(--spacing-sm);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background: var(--color-background);
  color: var(--color-text);
  font-size: 0.75rem;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.toolbar-btn:hover:not(:disabled) {
  background: var(--color-base-200);
  border-color: var(--color-primary-500);
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toolbar-btn svg {
  width: 16px;
  height: 16px;
}

.sync-btn.active {
  background: var(--color-primary-100);
  border-color: var(--color-primary-500);
  color: var(--color-primary-700);
}

.push-btn {
  background: var(--color-primary-500);
  border-color: var(--color-primary-500);
  color: white;
}

.push-btn:hover:not(:disabled) {
  background: var(--color-primary-600);
  border-color: var(--color-primary-600);
}

.zoom-level {
  font-size: 0.75rem;
  font-family: monospace;
  min-width: 40px;
  text-align: center;
  color: var(--color-text);
}

.map-viewport {
  flex: 1;
  overflow: hidden;
  position: relative;
  cursor: grab;
  display: flex;
  align-items: center;
  justify-content: center;
}

.map-viewport:active {
  cursor: grabbing;
}

.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--color-text-muted);
  font-size: 0.875rem;
}

.empty-hint {
  font-size: 0.75rem;
  margin-top: var(--spacing-xs);
  opacity: 0.7;
}

.map-container {
  position: relative;
  /* transition and will-change handled dynamically in computed style */
  /* Force GPU layer for the container */
  transform-style: preserve-3d;
  perspective: 1000px;
}

.map-image {
  display: block;
  max-width: none;
  user-select: none;
  -webkit-user-drag: none;
  /* GPU optimizations for large images */
  will-change: transform;
  backface-visibility: hidden;
  image-rendering: auto;
  /* Prevent layout recalculations */
  contain: layout style paint;
}

.grid-overlay {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: none;
  /* GPU layer for grid */
  will-change: transform;
  backface-visibility: hidden;
}

.status-bar {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-xs) var(--spacing-md);
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
  font-size: 0.75rem;
  color: var(--color-text);
  flex-shrink: 0;
}

.status-bar .dim {
  color: var(--color-text-muted);
}

.status-indicator {
  margin-left: auto;
  padding: 2px 8px;
  border-radius: var(--radius-sm);
  font-size: 0.625rem;
  font-weight: 600;
  text-transform: uppercase;
}

.status-indicator.connected {
  background: var(--color-success-100);
  color: var(--color-success);
}

.status-indicator.disconnected {
  background: var(--color-base-200);
  color: var(--color-text-muted);
}
</style>
