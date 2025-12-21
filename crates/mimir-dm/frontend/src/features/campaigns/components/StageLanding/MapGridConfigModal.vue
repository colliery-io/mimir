<template>
  <Teleport to="body">
    <div v-if="visible" class="modal-overlay" @click.self="handleClose">
      <div class="modal-content" :class="{ 'expanded': gridType !== 'none' }">
        <div class="modal-header">
          <h2>Configure Grid - {{ map.name }}</h2>
          <button class="close-btn" @click="handleClose">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div class="modal-body">
          <!-- Grid Type Selection -->
          <div class="form-group">
            <label>Grid Type</label>
            <div class="radio-group">
              <label class="radio-option">
                <input type="radio" v-model="gridType" value="none" />
                <span>No Grid</span>
              </label>
              <label class="radio-option">
                <input type="radio" v-model="gridType" value="square" />
                <span>Square</span>
              </label>
              <label class="radio-option">
                <input type="radio" v-model="gridType" value="hex" />
                <span>Hexagonal</span>
              </label>
            </div>
          </div>

          <!-- Visual Grid Editor (when grid enabled) -->
          <div v-if="gridType !== 'none'" class="grid-editor-section">
            <!-- Controls Row -->
            <div class="controls-row">
              <div class="form-group compact">
                <label for="grid-size">Cell Size (px)</label>
                <div class="size-input-group">
                  <button class="size-btn" @click="adjustGridSize(-5)">-</button>
                  <input
                    id="grid-size"
                    v-model.number="gridSizePx"
                    type="number"
                    class="form-input size-input"
                    min="10"
                    max="500"
                  />
                  <button class="size-btn" @click="adjustGridSize(5)">+</button>
                </div>
              </div>
              <div class="form-group compact">
                <label>Offset</label>
                <div class="offset-display">
                  X: {{ gridOffsetX }}, Y: {{ gridOffsetY }}
                </div>
              </div>
              <button class="reset-btn" @click="resetOffset" title="Reset offset to 0,0">
                Reset Offset
              </button>
            </div>

            <!-- Map Preview with Grid Overlay -->
            <div class="preview-container" ref="previewContainer">
              <div
                class="preview-wrapper"
                :style="previewWrapperStyle"
                @mousedown="startDrag"
                @mousemove="onDrag"
                @mouseup="endDrag"
                @mouseleave="endDrag"
              >
                <!-- Map Image -->
                <img
                  v-if="mapImageUrl"
                  :src="mapImageUrl"
                  :alt="map.name"
                  class="preview-image"
                  @load="onImageLoad"
                  ref="previewImage"
                />
                <div v-else class="loading-preview">
                  Loading map preview...
                </div>

                <!-- Grid Overlay SVG -->
                <svg
                  v-if="mapImageUrl && imageLoaded"
                  class="grid-overlay"
                  :viewBox="`0 0 ${scaledWidth} ${scaledHeight}`"
                  :style="{ width: scaledWidth + 'px', height: scaledHeight + 'px' }"
                >
                  <defs>
                    <!-- Square Grid Pattern -->
                    <pattern
                      v-if="gridType === 'square'"
                      id="gridPattern"
                      :width="scaledGridSize"
                      :height="scaledGridSize"
                      patternUnits="userSpaceOnUse"
                      :x="scaledOffsetX"
                      :y="scaledOffsetY"
                    >
                      <rect
                        :width="scaledGridSize"
                        :height="scaledGridSize"
                        fill="none"
                        stroke="rgba(255, 255, 255, 0.6)"
                        stroke-width="1"
                      />
                    </pattern>

                    <!-- Hex Grid Pattern -->
                    <pattern
                      v-if="gridType === 'hex'"
                      id="gridPattern"
                      :width="scaledGridSize * 1.5"
                      :height="scaledGridSize * 1.732"
                      patternUnits="userSpaceOnUse"
                      :x="scaledOffsetX"
                      :y="scaledOffsetY"
                    >
                      <polygon
                        :points="hexPoints"
                        fill="none"
                        stroke="rgba(255, 255, 255, 0.6)"
                        stroke-width="1"
                      />
                      <polygon
                        :points="hexPointsOffset"
                        fill="none"
                        stroke="rgba(255, 255, 255, 0.6)"
                        stroke-width="1"
                      />
                    </pattern>
                  </defs>
                  <rect width="100%" height="100%" fill="url(#gridPattern)" />

                  <!-- Origin Marker -->
                  <circle
                    :cx="scaledOffsetX"
                    :cy="scaledOffsetY"
                    r="6"
                    fill="var(--color-primary-500)"
                    stroke="white"
                    stroke-width="2"
                  />
                </svg>

                <!-- Drag indicator -->
                <div v-if="isDragging" class="drag-indicator">
                  Drag to set grid origin
                </div>
              </div>
            </div>

            <p class="help-text">
              Click and drag on the map to set the grid origin point. Use the cell size controls to adjust grid spacing.
            </p>
          </div>

          <!-- Simple offset inputs as fallback -->
          <div v-if="gridType !== 'none'" class="manual-offset-section">
            <details>
              <summary>Manual Offset Input</summary>
              <div class="form-row">
                <div class="form-group">
                  <label for="offset-x">X Offset</label>
                  <input
                    id="offset-x"
                    v-model.number="gridOffsetX"
                    type="number"
                    class="form-input"
                    placeholder="0"
                  />
                </div>
                <div class="form-group">
                  <label for="offset-y">Y Offset</label>
                  <input
                    id="offset-y"
                    v-model.number="gridOffsetY"
                    type="number"
                    class="form-input"
                    placeholder="0"
                  />
                </div>
              </div>
            </details>
          </div>
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" @click="handleClose" :disabled="saving">
            Cancel
          </button>
          <button
            class="btn-primary"
            @click="handleSave"
            :disabled="saving"
          >
            {{ saving ? 'Saving...' : 'Save Grid Settings' }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Map {
  id: number
  name: string
  width_px: number
  height_px: number
  grid_type: string
  grid_size_px: number | null
  grid_offset_x: number
  grid_offset_y: number
}

const props = defineProps<{
  visible: boolean
  map: Map
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

// Form state
const gridType = ref(props.map.grid_type || 'none')
const gridSizePx = ref(props.map.grid_size_px || 70)
const gridOffsetX = ref(props.map.grid_offset_x || 0)
const gridOffsetY = ref(props.map.grid_offset_y || 0)
const saving = ref(false)

// Preview state
const mapImageUrl = ref<string | null>(null)
const imageLoaded = ref(false)
const previewContainer = ref<HTMLElement | null>(null)
const previewImage = ref<HTMLImageElement | null>(null)
const isDragging = ref(false)
const dragStartX = ref(0)
const dragStartY = ref(0)

// Calculate preview dimensions (fit within container)
const maxPreviewWidth = 700
const maxPreviewHeight = 400

const scale = computed(() => {
  if (!props.map.width_px || !props.map.height_px) return 1
  const scaleX = maxPreviewWidth / props.map.width_px
  const scaleY = maxPreviewHeight / props.map.height_px
  return Math.min(scaleX, scaleY, 1) // Don't scale up
})

const scaledWidth = computed(() => Math.round(props.map.width_px * scale.value))
const scaledHeight = computed(() => Math.round(props.map.height_px * scale.value))
const scaledGridSize = computed(() => gridSizePx.value * scale.value)
const scaledOffsetX = computed(() => gridOffsetX.value * scale.value)
const scaledOffsetY = computed(() => gridOffsetY.value * scale.value)

const previewWrapperStyle = computed(() => ({
  width: scaledWidth.value + 'px',
  height: scaledHeight.value + 'px',
  cursor: isDragging.value ? 'grabbing' : 'crosshair'
}))

// Hex grid calculations
const hexPoints = computed(() => {
  const size = scaledGridSize.value
  const h = size * 0.866 // sqrt(3)/2
  return `${size * 0.5},0 ${size},${h * 0.5} ${size},${h * 1.5} ${size * 0.5},${h * 2} 0,${h * 1.5} 0,${h * 0.5}`
})

const hexPointsOffset = computed(() => {
  const size = scaledGridSize.value
  const h = size * 0.866
  const offsetX = size * 0.75
  const offsetY = h
  return `${offsetX + size * 0.5},${offsetY} ${offsetX + size},${offsetY + h * 0.5} ${offsetX + size},${offsetY + h * 1.5} ${offsetX + size * 0.5},${offsetY + h * 2} ${offsetX},${offsetY + h * 1.5} ${offsetX},${offsetY + h * 0.5}`
})

// Update form when map changes
watch(() => props.map, (newMap) => {
  gridType.value = newMap.grid_type || 'none'
  gridSizePx.value = newMap.grid_size_px || 70
  gridOffsetX.value = newMap.grid_offset_x || 0
  gridOffsetY.value = newMap.grid_offset_y || 0
  imageLoaded.value = false
}, { immediate: true })

// Load map image when modal becomes visible
watch(() => props.visible, async (visible) => {
  if (visible && props.map.id) {
    await loadMapImage()
  }
})

async function loadMapImage() {
  try {
    const response = await invoke<{ success: boolean; data?: string }>('serve_map_image', {
      id: props.map.id
    })
    if (response.success && response.data) {
      mapImageUrl.value = response.data
    }
  } catch (e) {
    console.error('Failed to load map image:', e)
  }
}

function onImageLoad() {
  imageLoaded.value = true
}

function adjustGridSize(delta: number) {
  const newSize = gridSizePx.value + delta
  if (newSize >= 10 && newSize <= 500) {
    gridSizePx.value = newSize
  }
}

function resetOffset() {
  gridOffsetX.value = 0
  gridOffsetY.value = 0
}

// Drag handling for setting grid origin
function startDrag(event: MouseEvent) {
  if (!previewContainer.value) return
  isDragging.value = true
  updateOffsetFromEvent(event)
}

function onDrag(event: MouseEvent) {
  if (!isDragging.value) return
  updateOffsetFromEvent(event)
}

function endDrag() {
  isDragging.value = false
}

function updateOffsetFromEvent(event: MouseEvent) {
  const target = event.currentTarget as HTMLElement
  const rect = target.getBoundingClientRect()
  const x = event.clientX - rect.left
  const y = event.clientY - rect.top

  // Convert from scaled coordinates to actual coordinates
  gridOffsetX.value = Math.round(x / scale.value)
  gridOffsetY.value = Math.round(y / scale.value)

  // Clamp to valid range
  gridOffsetX.value = Math.max(0, Math.min(gridOffsetX.value, props.map.width_px))
  gridOffsetY.value = Math.max(0, Math.min(gridOffsetY.value, props.map.height_px))
}

async function handleSave() {
  saving.value = true

  try {
    const response = await invoke<{ success: boolean; error?: string }>('update_map_grid', {
      id: props.map.id,
      gridType: gridType.value,
      gridSizePx: gridType.value !== 'none' ? gridSizePx.value : null,
      offsetX: gridOffsetX.value,
      offsetY: gridOffsetY.value
    })

    if (response.success) {
      emit('saved')
    } else {
      alert(`Failed to save grid settings: ${response.error}`)
    }
  } catch (e) {
    console.error('Failed to save grid settings:', e)
    alert('Failed to save grid settings')
  } finally {
    saving.value = false
  }
}

function handleClose() {
  if (!saving.value) {
    emit('close')
  }
}

// Load image on mount if already visible
onMounted(() => {
  if (props.visible && props.map.id) {
    loadMapImage()
  }
})
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--color-surface);
  border-radius: var(--radius-lg);
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.2);
  width: 100%;
  max-width: 450px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition: max-width 0.2s ease;
}

.modal-content.expanded {
  max-width: 800px;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--color-border);
}

.modal-header h2 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text);
}

.close-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--color-text-muted);
  cursor: pointer;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}

.close-btn:hover {
  background: var(--color-base-200);
  color: var(--color-text);
}

.close-btn svg {
  width: 20px;
  height: 20px;
}

.modal-body {
  padding: var(--spacing-lg);
  overflow-y: auto;
}

.form-group {
  margin-bottom: var(--spacing-md);
}

.form-group.compact {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text);
  margin-bottom: var(--spacing-xs);
}

.radio-group {
  display: flex;
  gap: var(--spacing-md);
}

.radio-option {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  cursor: pointer;
  font-size: 0.875rem;
  color: var(--color-text);
}

.radio-option input {
  cursor: pointer;
}

.form-input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  font-size: 0.875rem;
  background: var(--color-background);
  color: var(--color-text);
  transition: all var(--transition-fast);
}

.form-input:focus {
  outline: none;
  border-color: var(--color-primary-500);
  box-shadow: 0 0 0 2px var(--color-primary-100);
}

/* Grid Editor Section */
.grid-editor-section {
  margin-top: var(--spacing-md);
  padding-top: var(--spacing-md);
  border-top: 1px solid var(--color-border);
}

.controls-row {
  display: flex;
  align-items: flex-end;
  gap: var(--spacing-lg);
  margin-bottom: var(--spacing-md);
  flex-wrap: wrap;
}

.size-input-group {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.size-input {
  width: 80px;
  text-align: center;
}

.size-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background: var(--color-background);
  color: var(--color-text);
  cursor: pointer;
  font-size: 1.25rem;
  transition: all var(--transition-fast);
}

.size-btn:hover {
  background: var(--color-base-200);
  border-color: var(--color-primary-500);
}

.offset-display {
  font-size: 0.875rem;
  color: var(--color-text);
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-base-200);
  border-radius: var(--radius-md);
  font-family: monospace;
}

.reset-btn {
  padding: var(--spacing-xs) var(--spacing-md);
  font-size: 0.75rem;
  font-weight: 500;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  background: var(--color-background);
  color: var(--color-text-muted);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.reset-btn:hover {
  background: var(--color-base-200);
  color: var(--color-text);
}

/* Preview Container */
.preview-container {
  background: var(--color-base-200);
  border-radius: var(--radius-md);
  padding: var(--spacing-md);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.preview-wrapper {
  position: relative;
  user-select: none;
}

.preview-image {
  width: 100%;
  height: 100%;
  object-fit: contain;
  display: block;
}

.loading-preview {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 200px;
  color: var(--color-text-muted);
  font-size: 0.875rem;
}

.grid-overlay {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: none;
}

.drag-indicator {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: rgba(0, 0, 0, 0.7);
  color: white;
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  font-size: 0.875rem;
  pointer-events: none;
}

.help-text {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  margin-top: var(--spacing-sm);
  text-align: center;
}

/* Manual Offset Section */
.manual-offset-section {
  margin-top: var(--spacing-md);
}

.manual-offset-section details {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.manual-offset-section summary {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-base-200);
  cursor: pointer;
  font-size: 0.875rem;
  color: var(--color-text-muted);
}

.manual-offset-section summary:hover {
  background: var(--color-base-300);
}

.manual-offset-section .form-row {
  padding: var(--spacing-md);
}

.form-row {
  display: flex;
  gap: var(--spacing-md);
}

.form-row .form-group {
  flex: 1;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--color-border);
}

.btn-secondary,
.btn-primary {
  padding: var(--spacing-sm) var(--spacing-lg);
  font-size: 0.875rem;
  font-weight: 500;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.btn-secondary {
  border: 1px solid var(--color-border);
  background: var(--color-background);
  color: var(--color-text);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--color-surface);
}

.btn-primary {
  border: none;
  background: var(--color-primary-500);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--color-primary-600);
}

.btn-primary:disabled,
.btn-secondary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
