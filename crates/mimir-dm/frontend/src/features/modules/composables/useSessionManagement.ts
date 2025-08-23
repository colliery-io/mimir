import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Session } from '@/types'

export function useSessionManagement(moduleId: number | null) {
  const sessions = ref<Session[]>([])
  const sessionBoardConfig = ref<any>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Load sessions for the module
  async function loadSessions() {
    if (!moduleId) return
    
    isLoading.value = true
    error.value = null
    
    try {
      const response = await invoke<{ data: Session[] }>('list_module_sessions', {
        request: {
          module_id: moduleId
        }
      })
      sessions.value = response.data || []
    } catch (e) {
      error.value = `Failed to load sessions: ${e}`
      sessions.value = []
    } finally {
      isLoading.value = false
    }
  }

  // Load session board configuration
  async function loadSessionBoardConfig() {
    try {
      const response = await invoke<{ data: any }>('get_session_board_config', {})
      sessionBoardConfig.value = response.data
    } catch (e) {
      error.value = `Failed to load session board config: ${e}`
    }
  }

  // Create a new session
  async function createSession(sessionData: Partial<Session>) {
    if (!moduleId) return null
    
    try {
      const response = await invoke<{ data: Session }>('create_session', {
        request: {
          module_id: moduleId,
          ...sessionData
        }
      })
      
      if (response.data) {
        sessions.value.push(response.data)
        return response.data
      }
      return null
    } catch (e) {
      error.value = `Failed to create session: ${e}`
      throw e
    }
  }

  // Update session
  async function updateSession(sessionId: string | number, updates: Partial<Session>) {
    try {
      const response = await invoke<{ data: Session }>('update_session', {
        sessionId,
        ...updates
      })
      
      if (response.data) {
        const index = sessions.value.findIndex(s => String(s.id) === String(sessionId))
        if (index !== -1) {
          sessions.value[index] = response.data
        }
        return response.data
      }
      return null
    } catch (e) {
      error.value = `Failed to update session: ${e}`
      throw e
    }
  }

  // Delete session
  async function deleteSession(sessionId: string | number) {
    try {
      await invoke('delete_session', { sessionId })
      sessions.value = sessions.value.filter(s => String(s.id) !== String(sessionId))
    } catch (e) {
      error.value = `Failed to delete session: ${e}`
      throw e
    }
  }

  // Transition session to next phase
  async function transitionSession(sessionId: string | number, targetPhase: string) {
    try {
      const response = await invoke<{ data: Session }>('transition_session', {
        sessionId,
        targetPhase
      })
      
      if (response.data) {
        const index = sessions.value.findIndex(s => String(s.id) === String(sessionId))
        if (index !== -1) {
          sessions.value[index] = response.data
        }
        
        // Update module session count if transitioning to completed
        if (targetPhase === 'completed' && moduleId) {
          try {
            await invoke('increment_module_sessions', {
              moduleId
            })
          } catch (e) {
            // Non-critical error
          }
        }
        
        return response.data
      }
      return null
    } catch (e) {
      error.value = `Failed to transition session: ${e}`
      throw e
    }
  }

  // Computed properties
  const activeSessions = computed(() => 
    sessions.value.filter(s => s.status === 'active')
  )

  const plannedSessions = computed(() =>
    sessions.value.filter(s => s.status === 'planned')
  )

  const completedSessions = computed(() =>
    sessions.value.filter(s => s.status === 'completed')
  )

  const totalSessions = computed(() => sessions.value.length)

  const hasActiveSessions = computed(() => activeSessions.value.length > 0)

  return {
    sessions,
    sessionBoardConfig,
    isLoading,
    error,
    loadSessions,
    loadSessionBoardConfig,
    createSession,
    updateSession,
    deleteSession,
    transitionSession,
    activeSessions,
    plannedSessions,
    completedSessions,
    totalSessions,
    hasActiveSessions
  }
}