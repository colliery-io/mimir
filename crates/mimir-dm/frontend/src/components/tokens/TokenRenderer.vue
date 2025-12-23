<template>
  <div class="token-layer" :style="layerStyle">
    <div
      v-for="token in displayTokens"
      :key="token.id"
      class="token"
      :class="{
        'token-hidden': !token.visible_to_players && showHidden,
        'token-selected': selectedTokenId === token.id,
        'token-dragging': draggingTokenId === token.id
      }"
      :style="getTokenStyle(token)"
      :title="token.name"
      @mousedown.stop="handleMouseDown($event, token)"
      @click.stop="handleClick($event, token)"
      @contextmenu.prevent="$emit('token-context', $event, token)"
    >
      <span class="token-label">{{ getTokenLabel(token) }}</span>
      <span
        v-if="!token.visible_to_players && showHidden"
        class="visibility-badge"
        title="Hidden from players"
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M3.28 2.22a.75.75 0 00-1.06 1.06l14.5 14.5a.75.75 0 101.06-1.06l-1.745-1.745a10.029 10.029 0 003.3-4.38 1.651 1.651 0 000-1.185A10.004 10.004 0 009.999 3a9.956 9.956 0 00-4.744 1.194L3.28 2.22zM7.752 6.69l1.092 1.092a2.5 2.5 0 013.374 3.373l1.091 1.092a4 4 0 00-5.557-5.557z" clip-rule="evenodd" />
          <path d="M10.748 13.93l2.523 2.523a9.987 9.987 0 01-3.27.547c-4.258 0-7.894-2.66-9.337-6.41a1.651 1.651 0 010-1.186A10.007 10.007 0 012.839 6.02L6.07 9.252a4 4 0 004.678 4.678z" />
        </svg>
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { Token, TokenSize, TokenType } from '@/types/api'
import { TOKEN_SIZE_GRID_SQUARES, TOKEN_TYPE_COLORS } from '@/types/api'

interface Props {
  tokens: Token[]
  gridSizePx: number
  baseScale?: number
  showHidden?: boolean
  selectedTokenId?: number | null
  draggingTokenId?: number | null
  dragOffset?: { x: number; y: number } | null
  interactive?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  baseScale: 1,
  showHidden: true,
  selectedTokenId: null,
  draggingTokenId: null,
  dragOffset: null,
  interactive: true
})

const emit = defineEmits<{
  'token-click': [token: Token]
  'token-context': [event: MouseEvent, token: Token]
  'token-drag-start': [event: MouseEvent, token: Token]
}>()

// Track if we've moved enough to consider it a drag
const hasMoved = ref(false)
const mouseDownPos = ref<{ x: number; y: number } | null>(null)

function handleMouseDown(event: MouseEvent, token: Token) {
  if (event.button !== 0 || !props.interactive) return

  hasMoved.value = false
  mouseDownPos.value = { x: event.clientX, y: event.clientY }

  // Emit drag start - parent will handle actual dragging
  emit('token-drag-start', event, token)
}

function handleClick(event: MouseEvent, token: Token) {
  // Only emit click if we haven't dragged
  if (!hasMoved.value) {
    emit('token-click', token)
  }
  hasMoved.value = false
  mouseDownPos.value = null
}

// Expose hasMoved so parent can set it
defineExpose({ setHasMoved: (val: boolean) => { hasMoved.value = val } })

// Filter tokens - if showHidden is false, only show visible tokens
const displayTokens = computed(() => {
  if (props.showHidden) {
    return props.tokens
  }
  return props.tokens.filter(t => t.visible_to_players)
})

// Layer style (sized to match the map)
const layerStyle = computed(() => ({
  position: 'absolute' as const,
  top: 0,
  left: 0,
  width: '100%',
  height: '100%',
  pointerEvents: props.interactive ? 'auto' as const : 'none' as const
}))

// Scale tokens to 85% of grid size so they fit within grid cells
const TOKEN_SCALE = 0.85

// Get token display style
function getTokenStyle(token: Token) {
  const gridSquares = TOKEN_SIZE_GRID_SQUARES[token.size as TokenSize] || 1
  const tokenSizePx = gridSquares * props.gridSizePx * props.baseScale * TOKEN_SCALE
  const color = token.color || TOKEN_TYPE_COLORS[token.token_type as TokenType] || '#666666'

  // Position at token center, offset by half the token size
  let left = (token.x * props.baseScale) - (tokenSizePx / 2)
  let top = (token.y * props.baseScale) - (tokenSizePx / 2)

  // Apply drag offset if this token is being dragged
  if (props.draggingTokenId === token.id && props.dragOffset) {
    left += props.dragOffset.x
    top += props.dragOffset.y
  }

  return {
    left: left + 'px',
    top: top + 'px',
    width: tokenSizePx + 'px',
    height: tokenSizePx + 'px',
    backgroundColor: color,
    borderColor: color,
    fontSize: Math.max(tokenSizePx * 0.24, 16) + 'px'
  }
}

// Get token label - full name
function getTokenLabel(token: Token): string {
  return token.name
}
</script>

<style scoped>
.token-layer {
  pointer-events: none;
}

.token {
  position: absolute;
  border-radius: 50%;
  border: 3px solid;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.4);
  transition: transform 0.1s, box-shadow 0.1s;
  pointer-events: auto;
  cursor: pointer;
}

.token:hover {
  transform: scale(1.1);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.5);
  z-index: 10;
}

.token-hidden {
  opacity: 0.5;
}

.token-selected {
  box-shadow: 0 0 0 3px white, 0 0 0 6px var(--color-primary-500, #3b82f6);
  z-index: 20;
}

.token-dragging {
  opacity: 0.8;
  transform: scale(1.1);
  z-index: 100;
  cursor: grabbing;
  transition: none;
}

.token-label {
  font-weight: 600;
  font-size: 0.65em;
  color: white;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.7);
  user-select: none;
  text-align: center;
  line-height: 1.1;
  max-width: 90%;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  word-break: break-word;
}

.visibility-badge {
  position: absolute;
  top: -4px;
  right: -4px;
  width: 16px;
  height: 16px;
  background: rgba(0, 0, 0, 0.7);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.visibility-badge svg {
  width: 10px;
  height: 10px;
  fill: #fbbf24;
}
</style>
