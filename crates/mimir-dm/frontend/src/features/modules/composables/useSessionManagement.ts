import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ModuleService, type Session } from '@/services/ModuleService'
import { useSharedContextStore } from '@/stores/sharedContext'

export function useSessionManagement(moduleId: number | null) {
  const sessions = ref<Session[]>([])
  const sessionBoardConfig = ref<any>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const contextStore = useSharedContextStore()

  // Load sessions for the module
  async function loadSessions() {
    if (!moduleId) return
    
    isLoading.value = true
    error.value = null
    
    try {
      const loadedSessions = await ModuleService.listSessions(moduleId)
      // Ensure all sessions have required fields
      sessions.value = loadedSessions.map((session, index) => ({
        ...session,
        session_number: session.session_number ?? index + 1,
        created_at: session.created_at ?? new Date().toISOString()
      }))
      
      // Update module context with session info
      if (sessions.value.length > 0) {
        const sessionInfo = sessions.value.map(s => ({
          id: s.id.toString(),
          name: `Session #${s.session_number}`,
          status: s.status
        }))
        
        await contextStore.updateModule({
          ...contextStore.module,
          sessions: sessionInfo
        })
        
        // Update session context with the most relevant session
        // Priority: ready > in_prep > next_week > complete
        const priorityStatuses = ['ready', 'in_prep', 'next_week', 'prep_needed', 'complete']
        let currentSession = null
        
        for (const status of priorityStatuses) {
          currentSession = sessions.value.find(s => s.status === status)
          if (currentSession) break
        }
        
        if (currentSession) {
          await contextStore.updateSession({
            id: currentSession.id.toString(),
            name: `Session #${currentSession.session_number}`,
            moduleId: moduleId.toString(),
            status: currentSession.status,
            notes: currentSession.notes || undefined
          })
        } else {
          // Clear session context if no sessions
          await contextStore.updateSession({})
        }
      }
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
      // First get the module to get campaign info
      const moduleResponse = await invoke<{ data: any }>('get_module', { 
        id: moduleId 
      })
      
      if (!moduleResponse.data) {
        throw new Error('Module not found')
      }
      
      const module = moduleResponse.data
      
      // Get campaign to get directory
      const campaignResponse = await invoke<{ data: any }>('get_campaign', { 
        id: module.campaign_id 
      })
      
      if (!campaignResponse.data) {
        throw new Error('Campaign not found')
      }
      
      const response = await invoke<{ data: Session }>('create_session', {
        request: {
          module_id: moduleId,
          campaign_id: module.campaign_id,
          campaign_directory: campaignResponse.data.directory_path || campaignResponse.data.name,
          module_number: module.module_number || 1
        }
      })
      
      if (response.data) {
        // Ensure required fields have defaults
        const session = {
          ...response.data,
          session_number: response.data.session_number ?? sessions.value.length + 1,
          created_at: response.data.created_at ?? new Date().toISOString()
        }
        sessions.value.push(session)
        return session
      }
      return null
    } catch (e) {
      console.error('Failed to create session:', e)
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
        const session = {
          ...response.data,
          session_number: response.data.session_number ?? 1,
          created_at: response.data.created_at ?? new Date().toISOString()
        }
        const index = sessions.value.findIndex(s => String(s.id) === String(sessionId))
        if (index !== -1) {
          sessions.value[index] = session
        }
        return session
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
      const response = await invoke<{ data: Session }>('transition_session_status', {
        request: {
          session_id: Number(sessionId),
          new_status: targetPhase
        }
      })
      
      if (response.data) {
        const session = {
          ...response.data,
          session_number: response.data.session_number ?? 1,
          created_at: response.data.created_at ?? new Date().toISOString()
        }
        
        // Update session context if this is the current session
        await contextStore.updateSession({
          id: session.id.toString(),
          name: `Session #${session.session_number}`,
          moduleId: moduleId?.toString(),
          status: session.status
        })
        
        const index = sessions.value.findIndex(s => String(s.id) === String(sessionId))
        if (index !== -1) {
          sessions.value[index] = session
        }
        
        // Update module session count if transitioning to completed
        if (targetPhase === 'completed' && moduleId) {
          try {
            await ModuleService.incrementSessionCount(moduleId)
          } catch (e) {
            // Non-critical error
          }
        }
        
        return session
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