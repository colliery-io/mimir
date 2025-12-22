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
          class="toolbar-btn add-token-btn"
          @click="showQuickAddModal = true"
          :disabled="!mapImageUrl"
          title="Quick add token (monster)"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path d="M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z" />
          </svg>
          <span>Add Token</span>
        </button>
      </div>

      <!-- Fog of War Controls -->
      <div class="toolbar-group fog-controls">
        <button
          class="toolbar-btn fog-toggle-btn"
          :class="{ active: fogEnabled }"
          @click="toggleFog"
          :disabled="!mapImageUrl"
          title="Toggle fog of war"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path d="M1 8.25a7.25 7.25 0 1014.5 0A7.25 7.25 0 001 8.25zm7.25 5.75a5.75 5.75 0 100-11.5 5.75 5.75 0 000 11.5z" />
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm0-2a6 6 0 100-12 6 6 0 000 12z" clip-rule="evenodd" opacity="0.5" />
          </svg>
          <span>{{ fogEnabled ? 'Fog On' : 'Fog Off' }}</span>
        </button>
        <button
          v-if="fogEnabled"
          class="toolbar-btn fog-reveal-btn"
          :class="{ active: fogToolActive }"
          @click="toggleFogTool"
          :disabled="!mapImageUrl"
          title="Reveal fog tool - drag to reveal areas"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path d="M10 12.5a2.5 2.5 0 100-5 2.5 2.5 0 000 5z" />
            <path fill-rule="evenodd" d="M.664 10.59a1.651 1.651 0 010-1.186A10.004 10.004 0 0110 3c4.257 0 7.893 2.66 9.336 6.41.147.381.146.804 0 1.186A10.004 10.004 0 0110 17c-4.257 0-7.893-2.66-9.336-6.41zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd" />
          </svg>
          <span>Reveal</span>
        </button>
        <button
          v-if="fogEnabled && revealedAreas.length > 0"
          class="toolbar-btn fog-reset-btn"
          @click="resetFog"
          :disabled="!mapImageUrl"
          title="Reset fog - cover entire map"
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M15.312 11.424a5.5 5.5 0 01-9.201 2.466l-.312-.311h2.433a.75.75 0 000-1.5H3.989a.75.75 0 00-.75.75v4.242a.75.75 0 001.5 0v-2.43l.31.31a7 7 0 0011.712-3.138.75.75 0 00-1.449-.39z" clip-rule="evenodd" />
          </svg>
          <span>Reset</span>
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
      :class="{ 'fog-tool-active': fogToolActive }"
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

        <!-- Fog of War Overlay (DM view - semi-transparent) -->
        <svg
          v-if="fogEnabled && imageLoaded"
          class="fog-overlay dm-fog"
          :viewBox="`0 0 ${mapWidth} ${mapHeight}`"
          :style="{ width: mapWidth + 'px', height: mapHeight + 'px' }"
        >
          <defs>
            <mask id="dmFogMask">
              <!-- White = visible, Black = hidden -->
              <rect width="100%" height="100%" fill="white" />
              <!-- Cut out revealed areas -->
              <rect
                v-for="area in revealedAreas"
                :key="area.id"
                :x="area.x"
                :y="area.y"
                :width="area.width"
                :height="area.height"
                fill="black"
              />
              <!-- Show current reveal preview -->
              <rect
                v-if="fogRevealStart && fogRevealCurrent"
                :x="Math.min(fogRevealStart.x, fogRevealCurrent.x)"
                :y="Math.min(fogRevealStart.y, fogRevealCurrent.y)"
                :width="Math.abs(fogRevealCurrent.x - fogRevealStart.x)"
                :height="Math.abs(fogRevealCurrent.y - fogRevealStart.y)"
                fill="black"
              />
            </mask>
          </defs>
          <!-- Semi-transparent fog for DM view -->
          <rect
            width="100%"
            height="100%"
            fill="rgba(0, 0, 0, 0.5)"
            mask="url(#dmFogMask)"
          />
        </svg>

        <!-- Fog Reveal Preview Rectangle -->
        <svg
          v-if="fogToolActive && fogRevealStart && fogRevealCurrent && imageLoaded"
          class="fog-reveal-preview"
          :viewBox="`0 0 ${mapWidth} ${mapHeight}`"
          :style="{ width: mapWidth + 'px', height: mapHeight + 'px' }"
        >
          <rect
            :x="Math.min(fogRevealStart.x, fogRevealCurrent.x)"
            :y="Math.min(fogRevealStart.y, fogRevealCurrent.y)"
            :width="Math.abs(fogRevealCurrent.x - fogRevealStart.x)"
            :height="Math.abs(fogRevealCurrent.y - fogRevealStart.y)"
            fill="rgba(255, 255, 0, 0.2)"
            stroke="rgba(255, 255, 0, 0.8)"
            stroke-width="2"
            stroke-dasharray="5,5"
          />
        </svg>

        <!-- Light Source Layer -->
        <LightSourceRenderer
          v-if="imageLoaded && lightSources.length > 0"
          :lights="lightSources"
          :tokens="tokens"
          :grid-size-px="effectiveGridSize"
          :map-width="mapWidth"
          :map-height="mapHeight"
          :show-inactive="true"
          :show-bright-border="true"
          :show-center-dot="true"
          :show-labels="false"
        />

        <!-- Token Layer -->
        <TokenRenderer
          v-if="imageLoaded && tokens.length > 0"
          ref="tokenRendererRef"
          :tokens="tokens"
          :grid-size-px="effectiveGridSize"
          :base-scale="1"
          :show-hidden="true"
          :selected-token-id="selectedTokenId"
          :dragging-token-id="draggingTokenId"
          :drag-offset="dragOffset"
          :interactive="true"
          @token-click="handleTokenClick"
          @token-context="handleTokenContext"
          @token-drag-start="handleTokenDragStart"
        />
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

    <!-- Token Context Menu -->
    <div
      v-if="contextMenu.visible"
      class="context-menu"
      :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
      @click.stop
    >
      <button @click="toggleSelectedTokenVisibility">
        {{ contextMenu.token?.visible_to_players ? 'Hide from Players' : 'Show to Players' }}
        <span class="shortcut">H</span>
      </button>
      <button class="danger" @click="deleteSelectedToken">
        Delete Token
        <span class="shortcut">Del</span>
      </button>
    </div>

    <!-- Click outside to close context menu -->
    <div
      v-if="contextMenu.visible"
      class="context-menu-backdrop"
      @click="closeContextMenu"
    ></div>

    <!-- Quick Add Token Modal -->
    <QuickAddTokenModal
      v-if="mapId"
      :visible="showQuickAddModal"
      :map-id="mapId"
      :grid-size-px="effectiveGridSize"
      @close="showQuickAddModal = false"
      @add-token="handleQuickAddToken"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { emit } from '@tauri-apps/api/event'
import { usePlayerDisplay } from '@/composables/usePlayerDisplay'
import { useTokens } from '@/composables/useTokens'
import { useFog, type FogRevealedArea } from '@/composables/useFog'
import { useLightSources, type LightSourceSummary } from '@/composables/useLightSources'
import TokenRenderer from '@/components/tokens/TokenRenderer.vue'
import QuickAddTokenModal from '@/components/tokens/QuickAddTokenModal.vue'
import LightSourceRenderer from '@/components/lighting/LightSourceRenderer.vue'
import type { Token, CreateTokenRequest } from '@/types/api'

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

// Token state - will be initialized when mapId is available
const tokens = ref<Token[]>([])
const selectedTokenId = ref<number | null>(null)

// Token drag state
const draggingTokenId = ref<number | null>(null)
const dragOffset = ref<{ x: number; y: number } | null>(null)
const dragStartPos = ref<{ x: number; y: number; tokenX: number; tokenY: number } | null>(null)
const tokenRendererRef = ref<InstanceType<typeof TokenRenderer> | null>(null)

// Token context menu state
const contextMenu = ref<{
  visible: boolean
  x: number
  y: number
  token: Token | null
}>({
  visible: false,
  x: 0,
  y: 0,
  token: null
})

// Quick add modal state
const showQuickAddModal = ref(false)

// Fog of war state
const fogEnabled = ref(false)
const revealedAreas = ref<FogRevealedArea[]>([])
const fogToolActive = ref(false)
const fogRevealStart = ref<{ x: number; y: number } | null>(null)
const fogRevealCurrent = ref<{ x: number; y: number } | null>(null)

// Light source state
const lightSources = ref<LightSourceSummary[]>([])

// Load tokens when map changes
async function loadTokens(mapId: number) {
  try {
    const response = await invoke<{ success: boolean; data?: Token[] }>('list_tokens', { mapId })
    if (response.success && response.data) {
      tokens.value = response.data
      // Send visible tokens to player display
      sendTokensToDisplay()
    }
  } catch (e) {
    console.error('Failed to load tokens:', e)
    tokens.value = []
  }
}

// Send visible tokens to player display via IPC
async function sendTokensToDisplay() {
  if (!isDisplayOpen.value || !props.mapId) return

  const visibleTokens = tokens.value.filter(t => t.visible_to_players)
  try {
    await emit('player-display:tokens-update', {
      mapId: props.mapId,
      tokens: visibleTokens
    })
  } catch (e) {
    console.error('Failed to send tokens to display:', e)
  }
}

// Load fog state
async function loadFogState(mapId: number) {
  try {
    const response = await invoke<{ success: boolean; data?: { fog_enabled: boolean; revealed_areas: FogRevealedArea[] } }>('get_fog_state', { mapId })
    if (response.success && response.data) {
      fogEnabled.value = response.data.fog_enabled
      revealedAreas.value = response.data.revealed_areas
      // Send fog state to player display
      sendFogToDisplay()
    }
  } catch (e) {
    console.error('Failed to load fog state:', e)
  }
}

// Load light sources for the map
async function loadLightSources(mapId: number) {
  try {
    const response = await invoke<{ success: boolean; data?: LightSourceSummary[] }>('list_light_sources', { mapId })
    if (response.success && response.data) {
      lightSources.value = response.data
      // Send light sources to player display
      sendLightSourcesToDisplay()
    }
  } catch (e) {
    console.error('Failed to load light sources:', e)
    lightSources.value = []
  }
}

// Send light sources to player display
async function sendLightSourcesToDisplay() {
  if (!isDisplayOpen.value || !props.mapId) return

  // Only send active light sources to player display
  const activeLights = lightSources.value.filter(l => l.is_active)
  try {
    await emit('player-display:light-sources-update', {
      mapId: props.mapId,
      lightSources: activeLights
    })
  } catch (e) {
    console.error('Failed to send light sources to display:', e)
  }
}

// Toggle fog on/off
async function toggleFog() {
  if (!props.mapId) return

  try {
    const response = await invoke<{ success: boolean; data?: boolean }>('toggle_fog', { mapId: props.mapId })
    if (response.success && response.data !== undefined) {
      fogEnabled.value = response.data
      sendFogToDisplay()
    }
  } catch (e) {
    console.error('Failed to toggle fog:', e)
  }
}

// Toggle fog tool mode
function toggleFogTool() {
  fogToolActive.value = !fogToolActive.value
  if (!fogToolActive.value) {
    fogRevealStart.value = null
    fogRevealCurrent.value = null
  }
}

// Send fog state to player display
async function sendFogToDisplay() {
  if (!isDisplayOpen.value || !props.mapId) return

  try {
    await emit('player-display:fog-update', {
      mapId: props.mapId,
      fogEnabled: fogEnabled.value,
      revealedAreas: revealedAreas.value
    })
  } catch (e) {
    console.error('Failed to send fog to display:', e)
  }
}

// Reveal a rectangular area
async function revealRect(x: number, y: number, width: number, height: number) {
  if (!props.mapId) return

  try {
    const request = { map_id: props.mapId, x, y, width, height }
    const response = await invoke<{ success: boolean; data?: FogRevealedArea }>('reveal_rect', { request })
    if (response.success && response.data) {
      revealedAreas.value.push(response.data)
      sendFogToDisplay()
    }
  } catch (e) {
    console.error('Failed to reveal rect:', e)
  }
}

// Reset fog (clear all revealed areas)
async function resetFog() {
  if (!props.mapId) return

  try {
    const response = await invoke<{ success: boolean }>('reset_fog', { mapId: props.mapId })
    if (response.success) {
      revealedAreas.value = []
      sendFogToDisplay()
    }
  } catch (e) {
    console.error('Failed to reset fog:', e)
  }
}

// Handle token click
function handleTokenClick(token: Token) {
  selectedTokenId.value = token.id === selectedTokenId.value ? null : token.id
}

// Handle token context menu
function handleTokenContext(event: MouseEvent, token: Token) {
  selectedTokenId.value = token.id
  contextMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    token
  }
}

// Close context menu
function closeContextMenu() {
  contextMenu.value.visible = false
}

// Toggle visibility of selected token
async function toggleSelectedTokenVisibility() {
  const token = contextMenu.value.token || tokens.value.find(t => t.id === selectedTokenId.value)
  if (!token) return

  try {
    const response = await invoke<{ success: boolean; data?: Token; error?: string }>('toggle_token_visibility', {
      id: token.id
    })

    if (response.success && response.data) {
      // Update local token
      const index = tokens.value.findIndex(t => t.id === token.id)
      if (index !== -1) {
        tokens.value[index] = response.data
      }
      // Sync to player display
      sendTokensToDisplay()
    }
  } catch (e) {
    console.error('Failed to toggle token visibility:', e)
  }

  closeContextMenu()
}

// Delete selected token
async function deleteSelectedToken() {
  const token = contextMenu.value.token || tokens.value.find(t => t.id === selectedTokenId.value)
  if (!token) return

  try {
    const response = await invoke<{ success: boolean; error?: string }>('delete_token', {
      id: token.id
    })

    if (response.success) {
      // Remove from local state
      tokens.value = tokens.value.filter(t => t.id !== token.id)
      if (selectedTokenId.value === token.id) {
        selectedTokenId.value = null
      }
      // Sync to player display
      sendTokensToDisplay()
    }
  } catch (e) {
    console.error('Failed to delete token:', e)
  }

  closeContextMenu()
}

// Handle quick-add token
async function handleQuickAddToken(request: CreateTokenRequest) {
  if (!props.mapId) return

  // Calculate center position in map coordinates
  // The viewport center is at (0, 0) in pan coordinates
  // We need to convert that to map pixel coordinates
  const viewportRect = viewport.value?.getBoundingClientRect()
  if (!viewportRect) return

  const viewportCenterX = viewportRect.width / 2
  const viewportCenterY = viewportRect.height / 2

  // Convert viewport center to map coordinates
  // Account for current pan and zoom
  const mapX = (viewportCenterX - panX.value) / zoom.value
  const mapY = (viewportCenterY - panY.value) / zoom.value

  // Snap to grid center
  const { x: snappedX, y: snappedY } = snapToGrid(mapX, mapY)

  // Create the token with the calculated position
  try {
    const response = await invoke<{ success: boolean; data?: Token; error?: string }>('create_token', {
      request: {
        ...request,
        map_id: props.mapId,
        x: snappedX,
        y: snappedY
      }
    })

    if (response.success && response.data) {
      // Add to local tokens
      tokens.value.push(response.data)
      // Select the new token
      selectedTokenId.value = response.data.id
      // Sync to player display
      sendTokensToDisplay()
    } else {
      console.error('Failed to create token:', response.error)
    }
  } catch (e) {
    console.error('Failed to create token:', e)
  }
}

// Handle token drag start
function handleTokenDragStart(event: MouseEvent, token: Token) {
  draggingTokenId.value = token.id
  selectedTokenId.value = token.id
  dragOffset.value = { x: 0, y: 0 }
  dragStartPos.value = {
    x: event.clientX,
    y: event.clientY,
    tokenX: token.x,
    tokenY: token.y
  }

  // Add document-level listeners for drag
  document.addEventListener('mousemove', handleTokenDrag)
  document.addEventListener('mouseup', handleTokenDragEnd)
}

// Handle token drag movement
function handleTokenDrag(event: MouseEvent) {
  if (!draggingTokenId.value || !dragStartPos.value) return

  const deltaX = (event.clientX - dragStartPos.value.x) / zoom.value
  const deltaY = (event.clientY - dragStartPos.value.y) / zoom.value

  // Mark as moved if we've dragged more than 5px
  if (Math.abs(deltaX) > 5 || Math.abs(deltaY) > 5) {
    tokenRendererRef.value?.setHasMoved(true)
  }

  dragOffset.value = { x: deltaX, y: deltaY }

  // Send live position update to player display for visible tokens
  const token = tokens.value.find(t => t.id === draggingTokenId.value)
  if (token?.visible_to_players) {
    sendTokensToDisplayWithDragOffset()
  }
}

// Handle token drag end
async function handleTokenDragEnd(event: MouseEvent) {
  document.removeEventListener('mousemove', handleTokenDrag)
  document.removeEventListener('mouseup', handleTokenDragEnd)

  if (!draggingTokenId.value || !dragStartPos.value || !dragOffset.value) {
    draggingTokenId.value = null
    dragOffset.value = null
    dragStartPos.value = null
    return
  }

  const token = tokens.value.find(t => t.id === draggingTokenId.value)
  if (!token) {
    draggingTokenId.value = null
    dragOffset.value = null
    dragStartPos.value = null
    return
  }

  // Calculate new position with grid snapping
  const rawX = dragStartPos.value.tokenX + dragOffset.value.x
  const rawY = dragStartPos.value.tokenY + dragOffset.value.y
  const { x: snappedX, y: snappedY } = snapToGrid(rawX, rawY)

  // Only update if position changed
  if (snappedX !== token.x || snappedY !== token.y) {
    try {
      const response = await invoke<{ success: boolean; error?: string }>('update_token', {
        id: token.id,
        request: { x: snappedX, y: snappedY }
      })

      if (response.success) {
        // Update local token position
        token.x = snappedX
        token.y = snappedY
        // Sync to player display
        sendTokensToDisplay()
      } else {
        console.error('Failed to update token position:', response.error)
      }
    } catch (e) {
      console.error('Failed to update token position:', e)
    }
  }

  // Clear drag state
  draggingTokenId.value = null
  dragOffset.value = null
  dragStartPos.value = null
}

// Send tokens with live drag offset for smooth player display updates
async function sendTokensToDisplayWithDragOffset() {
  if (!isDisplayOpen.value || !props.mapId) return

  const visibleTokens = tokens.value
    .filter(t => t.visible_to_players)
    .map(t => {
      if (t.id === draggingTokenId.value && dragStartPos.value && dragOffset.value) {
        // Apply drag offset to the dragging token
        return {
          ...t,
          x: dragStartPos.value.tokenX + dragOffset.value.x,
          y: dragStartPos.value.tokenY + dragOffset.value.y
        }
      }
      return t
    })

  try {
    await emit('player-display:tokens-update', {
      mapId: props.mapId,
      tokens: visibleTokens
    })
  } catch (e) {
    console.error('Failed to send tokens to display:', e)
  }
}

// Snap position to grid center
function snapToGrid(x: number, y: number): { x: number; y: number } {
  const gridSize = effectiveGridSize.value
  const offsetX = effectiveGridOffsetX.value
  const offsetY = effectiveGridOffsetY.value

  // Snap to nearest grid cell center
  const gridX = Math.round((x - offsetX) / gridSize) * gridSize + offsetX + gridSize / 2
  const gridY = Math.round((y - offsetY) / gridSize) * gridSize + offsetY + gridSize / 2

  return { x: gridX, y: gridY }
}

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

// Load map image and tokens when mapId changes
watch(() => props.mapId, async (newId) => {
  if (newId) {
    await loadMapImage(newId)
    await loadTokens(newId)
    await loadFogState(newId)
    await loadLightSources(newId)
  } else {
    mapImageUrl.value = null
    mapName.value = ''
    mapWidth.value = 0
    mapHeight.value = 0
    imageLoaded.value = false
    tokens.value = []
    fogEnabled.value = false
    revealedAreas.value = []
    lightSources.value = []
  }
}, { immediate: true })

// Send tokens, fog, and lights to display when display opens
watch(isDisplayOpen, (open) => {
  if (open && props.mapId) {
    sendTokensToDisplay()
    sendFogToDisplay()
    sendLightSourcesToDisplay()
  }
})

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

// Convert screen coordinates to map coordinates
function screenToMapCoords(clientX: number, clientY: number): { x: number; y: number } {
  const rect = viewport.value?.getBoundingClientRect()
  if (!rect) return { x: 0, y: 0 }

  const screenX = clientX - rect.left - rect.width / 2
  const screenY = clientY - rect.top - rect.height / 2

  const mapX = (screenX - panX.value) / zoom.value
  const mapY = (screenY - panY.value) / zoom.value

  return { x: mapX, y: mapY }
}

// Pan controls (also handles fog reveal when fog tool is active)
function startPan(event: MouseEvent) {
  if (event.button !== 0) return // Only left click

  // If fog tool is active, start fog reveal instead of pan
  if (fogToolActive.value && fogEnabled.value) {
    const coords = screenToMapCoords(event.clientX, event.clientY)
    fogRevealStart.value = coords
    fogRevealCurrent.value = coords
    return
  }

  isPanning.value = true
  lastMouseX.value = event.clientX
  lastMouseY.value = event.clientY
}

function onPan(event: MouseEvent) {
  // If fog tool is active and we're revealing
  if (fogToolActive.value && fogRevealStart.value) {
    const coords = screenToMapCoords(event.clientX, event.clientY)
    fogRevealCurrent.value = coords
    return
  }

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
  // If fog tool was active, complete the reveal
  if (fogToolActive.value && fogRevealStart.value && fogRevealCurrent.value) {
    const x = Math.min(fogRevealStart.value.x, fogRevealCurrent.value.x)
    const y = Math.min(fogRevealStart.value.y, fogRevealCurrent.value.y)
    const width = Math.abs(fogRevealCurrent.value.x - fogRevealStart.value.x)
    const height = Math.abs(fogRevealCurrent.value.y - fogRevealStart.value.y)

    // Only reveal if the area is larger than 10px
    if (width > 10 && height > 10) {
      revealRect(x, y, width, height)
    }

    fogRevealStart.value = null
    fogRevealCurrent.value = null
    return
  }

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
    case 'h':
    case 'H':
      // Toggle visibility of selected token
      if (selectedTokenId.value) {
        toggleSelectedTokenVisibility()
      }
      break
    case 'Delete':
    case 'Backspace':
      // Delete selected token
      if (selectedTokenId.value) {
        deleteSelectedToken()
      }
      break
    case 'Escape':
      // Close context menu and deselect
      closeContextMenu()
      selectedTokenId.value = null
      break
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
  // Clean up any lingering drag listeners
  document.removeEventListener('mousemove', handleTokenDrag)
  document.removeEventListener('mouseup', handleTokenDragEnd)
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

.add-token-btn {
  background: var(--color-success-100);
  border-color: var(--color-success);
  color: var(--color-success);
}

.add-token-btn:hover:not(:disabled) {
  background: var(--color-success-200);
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

/* Context Menu */
.context-menu-backdrop {
  position: fixed;
  inset: 0;
  z-index: 99;
}

.context-menu {
  position: fixed;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 100;
  min-width: 180px;
  padding: var(--spacing-xs) 0;
}

.context-menu button {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  border: none;
  background: none;
  color: var(--color-text);
  text-align: left;
  cursor: pointer;
  font-size: 0.875rem;
}

.context-menu button:hover {
  background: var(--color-base-200);
}

.context-menu button.danger {
  color: var(--color-error);
}

.context-menu button.danger:hover {
  background: var(--color-error-100);
}

.context-menu .shortcut {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  font-family: monospace;
  background: var(--color-base-200);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
}

/* Fog of War Controls */
.fog-controls {
  border-left: 1px solid var(--color-border);
  padding-left: var(--spacing-md);
  margin-left: var(--spacing-sm);
}

.fog-toggle-btn.active {
  background: var(--color-warning-100);
  border-color: var(--color-warning);
  color: var(--color-warning-700);
}

.fog-reveal-btn.active {
  background: var(--color-primary-100);
  border-color: var(--color-primary-500);
  color: var(--color-primary-700);
}

.fog-reset-btn {
  color: var(--color-error);
}

.fog-reset-btn:hover:not(:disabled) {
  background: var(--color-error-100);
  border-color: var(--color-error);
}

/* Fog Overlay */
.fog-overlay {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: none;
  will-change: transform;
  backface-visibility: hidden;
}

.fog-overlay.dm-fog {
  /* DM view - semi-transparent so DM can see hidden areas */
  opacity: 1;
}

.fog-reveal-preview {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: none;
  z-index: 10;
}

/* Cursor change when fog tool is active */
.map-viewport.fog-tool-active {
  cursor: crosshair;
}
</style>
