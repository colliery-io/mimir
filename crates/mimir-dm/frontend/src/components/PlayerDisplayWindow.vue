<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, emit, type UnlistenFn } from '@tauri-apps/api/event'
import TokenRenderer from '@/components/tokens/TokenRenderer.vue'
import LightSourceRenderer from '@/components/lighting/LightSourceRenderer.vue'
import type { Token } from '@/types/api'
import type { LightSourceSummary } from '@/composables/useLightSources'
import { useVisionCalculation, type AmbientLight } from '@/composables/useVisionCalculation'

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
  ambientLight: AmbientLight
  mapWidth: number
  mapHeight: number
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
  isBlackout: false,
  ambientLight: 'bright',
  mapWidth: 0,
  mapHeight: 0
})

const isLoading = ref(false)
const errorMessage = ref<string | null>(null)
const imageRef = ref<HTMLImageElement | null>(null)
const tokens = ref<Token[]>([])

// Track actual display scale and image dimensions
const displayScale = ref(1)
const imageNaturalWidth = ref(0)
const imageNaturalHeight = ref(0)

// Fog of war state (vision-based)
interface VisionCircle {
  tokenId: number
  x: number
  y: number
  radiusPx: number
}

const fogEnabled = ref(false)
const visionCircles = ref<VisionCircle[]>([])

// Light source state
const lightSources = ref<LightSourceSummary[]>([])

// Vision calculation
const ambientLightRef = computed(() => mapState.value.ambientLight)
const gridSizePxRef = computed(() => mapState.value.gridSizePx || 70)
const mapWidthRef = computed(() => mapState.value.mapWidth || imageRef.value?.naturalWidth || 0)
const mapHeightRef = computed(() => mapState.value.mapHeight || imageRef.value?.naturalHeight || 0)

const {
  visibilityCircles,
  needsVisionOverlay,
  lightZones
} = useVisionCalculation({
  tokens,
  lightSources,
  ambientLight: ambientLightRef,
  gridSizePx: gridSizePxRef,
  mapWidth: mapWidthRef,
  mapHeight: mapHeightRef
})

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
let unlistenFogUpdate: UnlistenFn | null = null
let unlistenLightSourcesUpdate: UnlistenFn | null = null

onMounted(async () => {
  console.log('PlayerDisplayWindow: Setting up event listeners')

  // Listen for map updates from main window
  unlistenMapUpdate = await listen<{
    mapId: number
    gridType: string
    gridSizePx: number | null
    gridOffsetX: number
    gridOffsetY: number
    ambientLight?: string
    mapWidth?: number
    mapHeight?: number
  }>('player-display:map-update', async (event) => {
    console.log('PlayerDisplayWindow: Received map-update event:', event.payload)
    const data = event.payload
    mapState.value.mapId = data.mapId
    mapState.value.gridType = data.gridType as 'square' | 'hex' | 'none'
    mapState.value.gridSizePx = data.gridSizePx
    mapState.value.gridOffsetX = data.gridOffsetX
    mapState.value.gridOffsetY = data.gridOffsetY
    // Handle ambient light if provided
    if (data.ambientLight) {
      mapState.value.ambientLight = data.ambientLight as AmbientLight
    }
    // Handle map dimensions if provided
    if (data.mapWidth) {
      mapState.value.mapWidth = data.mapWidth
    }
    if (data.mapHeight) {
      mapState.value.mapHeight = data.mapHeight
    }

    // Load the map image
    await loadMapImage(data.mapId)

    // Request current state from DM window now that we're ready
    console.log('PlayerDisplayWindow: Requesting state for map', data.mapId)
    await emit('player-display:request-state', { mapId: data.mapId })
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
    // Accept tokens if they're for the current map OR if we don't have a map yet (initial load)
    if (mapState.value.mapId === null || event.payload.mapId === mapState.value.mapId) {
      tokens.value = event.payload.tokens
    }
  })

  // Listen for fog of war updates (vision-based)
  unlistenFogUpdate = await listen<{
    mapId: number
    fogEnabled: boolean
    visionCircles: VisionCircle[]
  }>('player-display:fog-update', (event) => {
    console.log('PlayerDisplayWindow: Received fog-update event:', event.payload.fogEnabled, event.payload.visionCircles?.length || 0, 'vision circles')
    // Accept fog if it's for the current map OR if we don't have a map yet (initial load)
    if (mapState.value.mapId === null || event.payload.mapId === mapState.value.mapId) {
      fogEnabled.value = event.payload.fogEnabled
      visionCircles.value = event.payload.visionCircles || []
    }
  })

  // Listen for light source updates
  unlistenLightSourcesUpdate = await listen<{
    mapId: number
    lightSources: LightSourceSummary[]
  }>('player-display:light-sources-update', (event) => {
    console.log('PlayerDisplayWindow: Received light-sources-update event:', event.payload.lightSources.length, 'lights')
    // Accept lights if they're for the current map OR if we don't have a map yet (initial load)
    if (mapState.value.mapId === null || event.payload.mapId === mapState.value.mapId) {
      lightSources.value = event.payload.lightSources
    }
  })

  // Handle keyboard shortcuts
  window.addEventListener('keydown', handleKeydown)

  // Handle window resize to recalculate scale
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  unlistenMapUpdate?.()
  unlistenViewportUpdate?.()
  unlistenBlackout?.()
  unlistenTokensUpdate?.()
  unlistenFogUpdate?.()
  unlistenLightSourcesUpdate?.()
  window.removeEventListener('keydown', handleKeydown)
  window.removeEventListener('resize', handleResize)
})

// Load map image from backend
async function loadMapImage(mapId: number) {
  isLoading.value = true
  errorMessage.value = null
  tokens.value = [] // Clear tokens when loading a new map
  lightSources.value = [] // Clear light sources when loading a new map

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

// Calculate the scale needed to fit the image in the viewport
function updateDisplayScale() {
  if (!imageRef.value) return

  const naturalWidth = imageRef.value.naturalWidth
  const naturalHeight = imageRef.value.naturalHeight

  if (naturalWidth === 0 || naturalHeight === 0) return

  // Store for other components
  imageNaturalWidth.value = naturalWidth
  imageNaturalHeight.value = naturalHeight

  // Get viewport dimensions
  const viewportWidth = window.innerWidth
  const viewportHeight = window.innerHeight

  // Calculate scale to fit (same as object-fit: contain logic)
  const scaleX = viewportWidth / naturalWidth
  const scaleY = viewportHeight / naturalHeight

  // Use the smaller scale to fit within viewport
  displayScale.value = Math.min(scaleX, scaleY)
  console.log('PlayerDisplayWindow: Updated display scale to', displayScale.value,
    `(natural: ${naturalWidth}x${naturalHeight}, viewport: ${viewportWidth}x${viewportHeight})`)
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

// Handle image load to calculate scale
function handleImageLoad() {
  console.log('PlayerDisplayWindow: Image loaded')
  updateDisplayScale()
}

// Handle window resize to recalculate scale
function handleResize() {
  updateDisplayScale()
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

      <!-- Map with grid overlay - scaled to fit screen -->
      <div
        v-else
        class="map-container"
        :style="{
          transform: `scale(${displayScale})`,
          transformOrigin: 'center center'
        }"
      >
        <img
          ref="imageRef"
          :src="mapState.imageUrl"
          alt="Battle Map"
          class="map-image"
          draggable="false"
          @load="handleImageLoad"
        />

        <!-- Grid overlay -->
        <svg
          v-if="gridPattern"
          class="grid-overlay"
          :style="{
            width: imageNaturalWidth + 'px',
            height: imageNaturalHeight + 'px'
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

        <!-- Light Source Layer (only active lights) -->
        <LightSourceRenderer
          v-if="lightSources.length > 0 && mapState.gridSizePx && imageNaturalWidth > 0"
          :lights="lightSources"
          :tokens="tokens"
          :grid-size-px="mapState.gridSizePx"
          :map-width="imageNaturalWidth"
          :map-height="imageNaturalHeight"
          :show-inactive="false"
          :show-bright-border="false"
          :show-center-dot="false"
          :show-labels="false"
        />

        <!-- Token Layer (only visible tokens, below fog overlay) -->
        <TokenRenderer
          v-if="tokens.length > 0 && mapState.gridSizePx"
          :key="`tokens-${mapState.mapId}-${mapState.gridSizePx}`"
          :tokens="tokens"
          :grid-size-px="mapState.gridSizePx"
          :base-scale="1"
          :show-hidden="false"
          :interactive="false"
        />

        <!-- Fog of War Overlay (Player view - vision-based) -->
        <svg
          v-if="fogEnabled && imageNaturalWidth > 0"
          class="fog-overlay"
          :style="{
            width: imageNaturalWidth + 'px',
            height: imageNaturalHeight + 'px'
          }"
          :viewBox="`0 0 ${imageNaturalWidth} ${imageNaturalHeight}`"
        >
          <defs>
            <!-- Blur filter for soft vision edges -->
            <filter id="playerVisionBlur" x="-50%" y="-50%" width="200%" height="200%">
              <feGaussianBlur in="SourceGraphic" stdDeviation="20" />
            </filter>
            <mask id="playerFogMask">
              <!-- White = visible (fog), Black = hidden (revealed) -->
              <rect width="100%" height="100%" fill="white" />
              <!-- Cut out vision circles from player tokens (with blur for soft edges) -->
              <g filter="url(#playerVisionBlur)">
                <circle
                  v-for="circle in visionCircles"
                  :key="'vision-' + circle.tokenId"
                  :cx="circle.x"
                  :cy="circle.y"
                  :r="circle.radiusPx"
                  fill="black"
                />
              </g>
            </mask>
          </defs>
          <!-- Fully opaque fog for player view -->
          <rect
            width="100%"
            height="100%"
            fill="#000000"
            mask="url(#playerFogMask)"
          />
        </svg>

        <!-- Vision/Lighting Overlay (darkness with vision cutouts) -->
        <svg
          v-if="needsVisionOverlay && imageNaturalWidth > 0"
          class="vision-overlay"
          :style="{
            width: imageNaturalWidth + 'px',
            height: imageNaturalHeight + 'px'
          }"
          :viewBox="`0 0 ${imageNaturalWidth} ${imageNaturalHeight}`"
        >
          <defs>
            <!-- Mask for darkness (white = show darkness, black = hide darkness) -->
            <mask id="darknessMask">
              <!-- White background = darkness everywhere by default -->
              <rect width="100%" height="100%" fill="white" />
              <!-- Cut out (hide darkness) in dim vision areas with gray -->
              <circle
                v-for="zone in lightZones"
                :key="`dark-light-dim-${zone.lightSourceId}`"
                :cx="zone.x"
                :cy="zone.y"
                :r="zone.dimRadiusPx"
                fill="#666"
              />
              <circle
                v-for="circle in visibilityCircles"
                :key="`dark-vision-dim-${circle.tokenId}`"
                :cx="circle.x"
                :cy="circle.y"
                :r="circle.dimRadiusPx"
                fill="#666"
              />
              <!-- Fully cut out (no darkness) in bright vision areas -->
              <circle
                v-for="zone in lightZones"
                :key="`dark-light-bright-${zone.lightSourceId}`"
                :cx="zone.x"
                :cy="zone.y"
                :r="zone.brightRadiusPx"
                fill="black"
              />
              <circle
                v-for="circle in visibilityCircles"
                :key="`dark-vision-bright-${circle.tokenId}`"
                :cx="circle.x"
                :cy="circle.y"
                :r="circle.brightRadiusPx"
                fill="black"
              />
            </mask>

            <!-- Radial gradients for soft vision edges -->
            <radialGradient
              v-for="circle in visibilityCircles"
              :key="`gradient-${circle.tokenId}`"
              :id="`visionGradient-${circle.tokenId}`"
            >
              <stop offset="70%" stop-color="black" />
              <stop offset="100%" stop-color="white" />
            </radialGradient>
          </defs>

          <!-- Main darkness overlay -->
          <rect
            width="100%"
            height="100%"
            :fill="mapState.ambientLight === 'darkness' ? 'rgba(0, 0, 0, 0.92)' : 'rgba(0, 0, 0, 0.75)'"
            mask="url(#darknessMask)"
          />
        </svg>

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
  /* Container is sized to natural image dimensions and scaled via transform */
}

.map-image {
  display: block;
  max-width: none;
  user-select: none;
  -webkit-user-drag: none;
  /* Image renders at natural size, container handles scaling via transform */
}

.grid-overlay {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: none;
}

.fog-overlay {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: none;
  z-index: 10;
}

.vision-overlay {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: none;
  z-index: 9; /* Below fog overlay */
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
