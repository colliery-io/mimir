<template>
  <div class="play-mode-layout">
    <!-- Play Mode Header -->
    <header class="play-header">
      <div class="header-left">
        <button class="back-button" @click="navigateBack">
          <span class="back-icon">&larr;</span>
          Back to Module
        </button>
      </div>

      <div class="header-center">
        <h1 class="module-name">{{ module?.name || 'Loading...' }}</h1>
        <div class="play-mode-badge">PLAY MODE</div>
      </div>

      <div class="header-right">
        <button class="end-session-button" @click="handleEndSession">
          End Session
        </button>
      </div>
    </header>

    <!-- Main Play Area -->
    <div class="play-content">
      <!-- Collapsible Sidebar -->
      <aside class="play-sidebar" :class="{ collapsed: sidebarCollapsed }">
        <button class="sidebar-toggle" @click="toggleSidebar">
          {{ sidebarCollapsed ? '&raquo;' : '&laquo;' }}
        </button>

        <div class="sidebar-content" v-show="!sidebarCollapsed">
          <div class="sidebar-section">
            <h3>Quick Access</h3>
            <p class="placeholder-text">Monster cards and NPC info will appear here</p>
          </div>

          <div class="sidebar-section">
            <h3>Encounters</h3>
            <p class="placeholder-text">Encounter groups will be listed here</p>
          </div>
        </div>
      </aside>

      <!-- Main Content Area -->
      <main class="play-main">
        <div class="content-panel session-notes-panel">
          <h2>Session Notes</h2>
          <p class="placeholder-text">
            Session notes and quick reference content will appear here.
            This area will integrate with the session document editor.
          </p>
        </div>

        <div class="content-panel card-display-panel">
          <h2>Active Content</h2>
          <p class="placeholder-text">
            Monster stat blocks, NPC cards, and other reference material
            will be displayed here during play.
          </p>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import type { Module } from '@/types'

const route = useRoute()
const router = useRouter()

const moduleId = computed(() => parseInt(route.params.id as string))
const module = ref<Module | null>(null)
const sidebarCollapsed = ref(false)

// Load module data
async function loadModule() {
  try {
    const response = await invoke<{ data: Module }>('get_module', {
      id: moduleId.value
    })
    module.value = response.data
  } catch (error) {
    console.error('Failed to load module:', error)
  }
}

// Navigation
function navigateBack() {
  router.push({ name: 'module-board', params: { id: moduleId.value } })
}

function handleEndSession() {
  // For now, just navigate back
  // Later this could prompt to save session notes, update session status, etc.
  if (confirm('End this play session and return to module prep?')) {
    navigateBack()
  }
}

// Sidebar
function toggleSidebar() {
  sidebarCollapsed.value = !sidebarCollapsed.value
}

onMounted(() => {
  loadModule()
})
</script>

<style scoped>
.play-mode-layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--color-base-200);
}

/* Header Styles */
.play-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1.5rem;
  background: var(--color-base-300);
  border-bottom: 2px solid var(--color-accent, #e67e22);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.header-left,
.header-right {
  flex: 1;
}

.header-right {
  display: flex;
  justify-content: flex-end;
}

.header-center {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.25rem;
}

.module-name {
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0;
  color: var(--color-text);
}

.play-mode-badge {
  font-size: 0.7rem;
  font-weight: 700;
  letter-spacing: 0.1em;
  padding: 0.2rem 0.6rem;
  background: var(--color-accent, #e67e22);
  color: white;
  border-radius: 0.25rem;
}

.back-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: transparent;
  border: 1px solid var(--color-border);
  border-radius: 0.375rem;
  color: var(--color-text);
  cursor: pointer;
  transition: all 0.2s;
}

.back-button:hover {
  background: var(--color-surface);
  border-color: var(--color-primary);
}

.back-icon {
  font-size: 1.1rem;
}

.end-session-button {
  padding: 0.5rem 1rem;
  background: var(--color-error, #dc3545);
  color: white;
  border: none;
  border-radius: 0.375rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.end-session-button:hover {
  background: var(--color-error-dark, #c82333);
}

/* Content Area */
.play-content {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* Sidebar Styles */
.play-sidebar {
  width: 280px;
  background: var(--color-surface);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  transition: width 0.3s ease;
  position: relative;
}

.play-sidebar.collapsed {
  width: 40px;
}

.sidebar-toggle {
  position: absolute;
  right: -12px;
  top: 1rem;
  width: 24px;
  height: 24px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
  font-size: 0.8rem;
  color: var(--color-text-muted);
}

.sidebar-toggle:hover {
  background: var(--color-base-200);
  border-color: var(--color-primary);
}

.sidebar-content {
  padding: 1rem;
  overflow-y: auto;
  flex: 1;
}

.sidebar-section {
  margin-bottom: 1.5rem;
}

.sidebar-section h3 {
  font-size: 0.875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-text-muted);
  margin-bottom: 0.75rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--color-border);
}

/* Main Content */
.play-main {
  flex: 1;
  padding: 1.5rem;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.content-panel {
  background: var(--color-surface);
  border-radius: 0.5rem;
  padding: 1.5rem;
  border: 1px solid var(--color-border);
}

.content-panel h2 {
  font-size: 1.1rem;
  font-weight: 600;
  margin: 0 0 1rem 0;
  color: var(--color-text);
}

.session-notes-panel {
  flex: 1;
  min-height: 200px;
}

.card-display-panel {
  flex: 2;
  min-height: 300px;
}

.placeholder-text {
  color: var(--color-text-muted);
  font-style: italic;
}
</style>
