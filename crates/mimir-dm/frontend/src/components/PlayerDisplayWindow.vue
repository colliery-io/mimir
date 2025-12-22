<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import TokenRenderer from '@/components/tokens/TokenRenderer.vue'
import type { Token } from '@/types/api'

// Types for map display
interface MapState {
  mapId: number | null
  imageUrl: string | null
  gridType: 'square' | 'hex' | 'none'
  gridSizePx: number | null
  gridOffsetX: number
  gridOffsetY: number
  viewportX: number
  viewportY: number
  zoom: number
  isBlackout: boolean
}

// Reactive state
const mapState = ref<MapState>({
  mapId: null,
  imageUrl: null,
  gridType: 'none',
  gridSizePx: null,
  gridOffsetX: 0,
  gridOffsetY: 0,
  viewportX: 0,
  viewportY: 0,
  zoom: 1,
  isBlackout: false
})

const isLoading = ref(false)
const errorMessage = ref<string | null>(null)
const imageRef = ref<HTMLImageElement | null>(null)
const tokens = ref<Token[]>([])

// No viewport transforms - map always fits to screen
// The DM can pan/zoom their view independently

// Grid overlay types
interface SquareGridPattern {
  type: 'square'
  patternSize: number
  offsetX: number
  offsetY: number
}

interface HexGridPattern {
  type: 'hex'
  width: number
  height: number
  offsetX: number
  offsetY: number
}

type GridPattern = SquareGridPattern | HexGridPattern | null

// Helper to get hex points for SVG polygon
function getHexPoints(size: number): string {
  const w = size
  const h = size * Math.sqrt(3) / 2
  const points = [
    [w * 0.5, 0],
    [w, h * 0.5],
    [w, h * 1.5],
    [w * 0.5, h * 2],
    [0, h * 1.5],
    [0, h * 0.5]
  ]
  return points.map(p => p.join(',')).join(' ')
}

// Grid overlay SVG pattern
const gridPattern = computed<GridPattern>(() => {
  if (mapState.value.gridType === 'none' || !mapState.value.gridSizePx) {
    return null
  }

  const size = mapState.value.gridSizePx
  const offsetX = mapState.value.gridOffsetX
  const offsetY = mapState.value.gridOffsetY

  if (mapState.value.gridType === 'square') {
    return {
      type: 'square' as const,
      patternSize: size,
      offsetX,
      offsetY
    }
  } else if (mapState.value.gridType === 'hex') {
    // Hex grid calculations (pointy-top hexes)
    const hexWidth = size
    const hexHeight = size * Math.sqrt(3) / 2
    return {
      type: 'hex' as const,
      width: hexWidth,
      height: hexHeight,
      offsetX,
      offsetY
    }
  }

  return null
})

// Type-narrowed computed properties for template use
const isSquareGrid = computed(() => gridPattern.value?.type === 'square')
const isHexGrid = computed(() => gridPattern.value?.type === 'hex')
const squarePattern = computed(() => gridPattern.value?.type === 'square' ? gridPattern.value : null)
const hexPattern = computed(() => gridPattern.value?.type === 'hex' ? gridPattern.value : null)

// Event listeners for IPC from main window
let unlistenMapUpdate: UnlistenFn | null = null
let unlistenViewportUpdate: UnlistenFn | null = null
let unlistenBlackout: UnlistenFn | null = null
let unlistenTokensUpdate: UnlistenFn | null = null

onMounted(async () => {
  console.log('PlayerDisplayWindow: Setting up event listeners')

  // Listen for map updates from main window
  unlistenMapUpdate = await listen<{
    mapId: number
    gridType: string
    gridSizePx: number | null
    gridOffsetX: number
    gridOffsetY: number
  }>('player-display:map-update', async (event) => {
    console.log('PlayerDisplayWindow: Received map-update event:', event.payload)
    const data = event.payload
    mapState.value.mapId = data.mapId
    mapState.value.gridType = data.gridType as 'square' | 'hex' | 'none'
    mapState.value.gridSizePx = data.gridSizePx
    mapState.value.gridOffsetX = data.gridOffsetX
    mapState.value.gridOffsetY = data.gridOffsetY

    // Load the map image
    await loadMapImage(data.mapId)
  })

  // Listen for viewport updates (pan/zoom)
  unlistenViewportUpdate = await listen<{
    x: number
    y: number
    zoom: number
  }>('player-display:viewport-update', (event) => {
    mapState.value.viewportX = event.payload.x
    mapState.value.viewportY = event.payload.y
    mapState.value.zoom = event.payload.zoom
  })

  // Listen for blackout toggle
  unlistenBlackout = await listen<{ isBlackout: boolean }>('player-display:blackout', (event) => {
    mapState.value.isBlackout = event.payload.isBlackout
  })

  // Listen for token updates
  unlistenTokensUpdate = await listen<{
    mapId: number
    tokens: Token[]
  }>('player-display:tokens-update', (event) => {
    console.log('PlayerDisplayWindow: Received tokens-update event:', event.payload.tokens.length, 'tokens')
    // Only update tokens if they're for the current map
    if (event.payload.mapId === mapState.value.mapId) {
      tokens.value = event.payload.tokens
    }
  })

  // Handle keyboard shortcuts
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  unlistenMapUpdate?.()
  unlistenViewportUpdate?.()
  unlistenBlackout?.()
  unlistenTokensUpdate?.()
  window.removeEventListener('keydown', handleKeydown)
})

// Load map image from backend
async function loadMapImage(mapId: number) {
  isLoading.value = true
  errorMessage.value = null
  tokens.value = [] // Clear tokens when loading a new map

  try {
    const response = await invoke<{ success: boolean; data?: string; error?: string }>(
      'serve_map_image',
      { id: mapId }
    )

    if (response.success && response.data) {
      mapState.value.imageUrl = response.data
    } else {
      errorMessage.value = response.error || 'Failed to load map image'
    }
  } catch (err) {
    errorMessage.value = `Error loading map: ${err}`
  } finally {
    isLoading.value = false
  }
}

// Keyboard shortcuts
function handleKeydown(event: KeyboardEvent) {
  // F11 to toggle fullscreen
  if (event.key === 'F11') {
    event.preventDefault()
    invoke('toggle_player_display_fullscreen')
  }
  // Escape to exit blackout or close window
  if (event.key === 'Escape') {
    if (mapState.value.isBlackout) {
      // Just visual feedback, main window controls blackout
    }
  }
}
</script>

<template>
  <div class="player-display" :class="{ blackout: mapState.isBlackout }">
    <!-- Blackout overlay -->
    <div v-if="mapState.isBlackout" class="blackout-overlay">
      <div class="blackout-text">Display Paused</div>
    </div>

    <!-- Map display area -->
    <div v-else class="map-viewport">
      <!-- Loading state -->
      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner"></div>
        <div class="loading-text">Loading map...</div>
      </div>

      <!-- Error state -->
      <div v-else-if="errorMessage" class="error-state">
        <div class="error-icon">!</div>
        <div class="error-text">{{ errorMessage }}</div>
      </div>

      <!-- No map selected -->
      <div v-else-if="!mapState.imageUrl" class="empty-state">
        <div class="empty-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-24 h-24">
            <path fill-rule="evenodd" d="M8.161 2.58a1.875 1.875 0 011.678 0l4.993 2.498c.106.052.23.052.336 0l3.869-1.935A1.875 1.875 0 0121.75 4.82v12.485c0 .71-.401 1.36-1.037 1.677l-4.875 2.437a1.875 1.875 0 01-1.676 0l-4.994-2.497a.375.375 0 00-.336 0l-3.868 1.935A1.875 1.875 0 012.25 19.18V6.695c0-.71.401-1.36 1.036-1.677l4.875-2.437zM9 6a.75.75 0 01.75.75V15a.75.75 0 01-1.5 0V6.75A.75.75 0 019 6zm6.75 3a.75.75 0 00-1.5 0v8.25a.75.75 0 001.5 0V9z" clip-rule="evenodd" />
          </svg>
        </div>
        <div class="empty-text">Waiting for map selection...</div>
        <div class="empty-hint">Select a map from the DM window to display</div>
      </div>

      <!-- Map with grid overlay - always fits to screen -->
      <div v-else class="map-container">
        <img
          ref="imageRef"
          :src="mapState.imageUrl"
          alt="Battle Map"
          class="map-image"
          draggable="false"
        />

        <!-- Grid overlay -->
        <svg
          v-if="gridPattern"
          class="grid-overlay"
          :style="{
            width: imageRef?.naturalWidth + 'px',
            height: imageRef?.naturalHeight + 'px'
          }"
        >
          <defs>
            <!-- Square grid pattern -->
            <pattern
              v-if="squarePattern"
              id="grid-pattern"
              :width="squarePattern.patternSize"
              :height="squarePattern.patternSize"
              patternUnits="userSpaceOnUse"
              :patternTransform="`translate(${squarePattern.offsetX}, ${squarePattern.offsetY})`"
            >
              <path
                :d="`M ${squarePattern.patternSize} 0 L 0 0 0 ${squarePattern.patternSize}`"
                fill="none"
                stroke="rgba(255, 255, 255, 0.3)"
                stroke-width="1"
              />
            </pattern>

            <!-- Hex grid pattern (pointy-top) -->
            <pattern
              v-if="hexPattern"
              id="grid-pattern"
              :width="hexPattern.width * 1.5"
              :height="hexPattern.height * 2"
              patternUnits="userSpaceOnUse"
              :patternTransform="`translate(${hexPattern.offsetX}, ${hexPattern.offsetY})`"
            >
              <!-- Hex paths would go here -->
              <polygon
                :points="getHexPoints(hexPattern.width)"
                fill="none"
                stroke="rgba(255, 255, 255, 0.3)"
                stroke-width="1"
              />
            </pattern>
          </defs>
          <rect width="100%" height="100%" fill="url(#grid-pattern)" />
        </svg>

        <!-- Token Layer (only visible tokens) -->
        <TokenRenderer
          v-if="tokens.length > 0 && mapState.gridSizePx"
          :tokens="tokens"
          :grid-size-px="mapState.gridSizePx"
          :base-scale="1"
          :show-hidden="false"
          :interactive="false"
        />
      </div>
    </div>

    <!-- Minimal status bar (only visible on hover in fullscreen) -->
    <!-- Minimal status bar - hidden by default, shows on hover -->
    <div class="status-bar">
      <span v-if="mapState.mapId">Map loaded</span>
    </div>
  </div>
</template>

<style scoped>
.player-display {
  position: fixed;
  inset: 0;
  background: #000;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  cursor: none;
}

.player-display:not(.blackout):hover {
  cursor: default;
}

/* Blackout mode */
.blackout-overlay {
  position: absolute;
  inset: 0;
  background: #000;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.blackout-text {
  color: #333;
  font-size: 1.5rem;
  font-family: system-ui, sans-serif;
}

/* Map viewport */
.map-viewport {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
}

.map-container {
  position: relative;
  transition: transform 0.1s ease-out;
}

.map-image {
  max-width: 100vw;
  max-height: 100vh;
  object-fit: contain;
  display: block;
  user-select: none;
  -webkit-user-drag: none;
}

.grid-overlay {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: none;
}

/* Loading state */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  color: #666;
}

.loading-spinner {
  width: 48px;
  height: 48px;
  border: 3px solid #333;
  border-top-color: #666;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.loading-text {
  font-size: 1rem;
  font-family: system-ui, sans-serif;
}

/* Error state */
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  color: #cc4444;
}

.error-icon {
  width: 48px;
  height: 48px;
  border: 3px solid currentColor;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  font-weight: bold;
}

.error-text {
  font-size: 1rem;
  font-family: system-ui, sans-serif;
  max-width: 400px;
  text-align: center;
}

/* Empty state */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  color: #444;
}

.empty-icon {
  opacity: 0.5;
}

.empty-icon svg {
  width: 96px;
  height: 96px;
}

.empty-text {
  font-size: 1.25rem;
  font-family: system-ui, sans-serif;
}

.empty-hint {
  font-size: 0.875rem;
  opacity: 0.7;
}

/* Status bar */
.status-bar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 0.5rem 1rem;
  background: rgba(0, 0, 0, 0.8);
  color: #666;
  font-size: 0.75rem;
  font-family: system-ui, sans-serif;
  display: flex;
  gap: 1rem;
  opacity: 0;
  transition: opacity 0.3s;
}

.player-display:hover .status-bar {
  opacity: 1;
}
</style>
