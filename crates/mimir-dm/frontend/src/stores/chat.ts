import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useSharedContextStore } from './sharedContext'
import { DEFAULT_SYSTEM_PROMPT } from '@/constants/defaultSystemPrompt'

export interface ChatMessage {
  id: string
  role: 'user' | 'assistant' | 'system' | 'tool'
  content: string
  timestamp: number
  tokenUsage?: {
    prompt: number
    completion: number
    total: number
  }
  isIntermediate?: boolean
  iteration?: number
  toolName?: string
  toolCalls?: string[]
  success?: boolean
}

export interface SystemMessageConfig {
  baseInstructions?: string
  contextEnabled?: boolean
  tools?: string[]
  customInstructions?: string
  temperature?: number
  maxTokens?: number
}

export interface ModelInfo {
  model: string
  contextLength: number
  defaultMaxTokens: number
  architecture: string
}

export interface ChatResponseWithUsage {
  content: string
  prompt_tokens: number
  completion_tokens: number
  total_tokens: number
}

export interface ActionDescription {
  title: string
  description: string
  changes: string[]
}

export interface ToolConfirmationRequest {
  id: string
  tool_name: string
  action: ActionDescription
}

export interface IntermediateMessage {
  role: string
  content: string
  tool_calls: string[]
  iteration: number
  session_id?: string
}

export interface ToolResultMessage {
  tool_name: string
  result: string
  success: boolean
  iteration: number
  session_id?: string
}

export interface PendingConfirmation {
  request: ToolConfirmationRequest
  status: 'pending' | 'confirmed' | 'rejected'
  messageId?: string
}

export interface ChatSession {
  id: string
  title: string
  created_at: number
  updated_at: number
  messages: ChatMessage[]
}

export interface ChatSessionMetadata {
  id: string
  title: string
  created_at: number
  updated_at: number
  message_count: number
  preview: string
}

export interface TodoItem {
  content: string
  status: 'pending' | 'in_progress' | 'completed'
  activeForm: string
}

export const useChatStore = defineStore('chat', () => {
  // State
  const messages = ref<ChatMessage[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const modelInfo = ref<ModelInfo | null>(null)
  const totalTokensUsed = ref(0)
  const maxResponseTokens = ref(16384)
  const pendingConfirmations = ref<Map<string, PendingConfirmation>>(new Map())
  
  // Session state
  const currentSessionId = ref<string | null>(null)
  const sessions = ref<ChatSessionMetadata[]>([])
  const sessionsLoading = ref(false)
  
  // Todo state
  const todos = ref<TodoItem[]>([])
  const todosVisible = ref(false)
  
  // System message configuration
  const systemConfig = ref<SystemMessageConfig>({
    baseInstructions: DEFAULT_SYSTEM_PROMPT,
    contextEnabled: true,
    tools: [],
    customInstructions: '',
    temperature: 0.3,  // Lower temperature for more deterministic tool calling
    maxTokens: 16384   // Increased to allow room for thinking blocks and tool calls
  })
  
  // Computed
  const conversationTokens = computed(() => {
    return messages.value.reduce((total, msg) => {
      return total + (msg.tokenUsage?.total || 0)
    }, 0)
  })
  
  const contextUsagePercentage = computed(() => {
    if (!modelInfo.value) return 0
    return (conversationTokens.value / modelInfo.value.contextLength) * 100
  })
  
  const lastMessage = computed(() => {
    return messages.value[messages.value.length - 1] || null
  })
  
  // Todo computed properties
  const todoProgress = computed(() => {
    const total = todos.value.length
    if (total === 0) return { completed: 0, total: 0, percentage: 0 }
    
    const completed = todos.value.filter(t => t.status === 'completed').length
    const percentage = Math.round((completed / total) * 100)
    return { completed, total, percentage }
  })
  
  const currentTodo = computed(() => {
    return todos.value.find(t => t.status === 'in_progress')
  })
  
  const hasTodos = computed(() => {
    return todos.value.length > 0
  })
  
  // Actions
  const initialize = async () => {
    try {
      // Get model info
      const info = await invoke<ModelInfo>('get_model_context_info')
      modelInfo.value = info
      maxResponseTokens.value = info.defaultMaxTokens
      
      // Load system configuration
      loadSystemConfig()
      
      // Load available sessions
      await loadSessions()
      
      // Try to load most recent session or create new one
      if (sessions.value.length > 0) {
        // Load the most recent session
        await loadSession(sessions.value[0].id)
      } else {
        // Create a new session
        await createNewSession()
      }
      
      // Set up event listener for tool confirmation requests
      await listen<ToolConfirmationRequest>('tool-confirmation-request', (event) => {
        console.log('Received confirmation request:', event.payload)
        const request = event.payload
        
        // Add to pending confirmations
        pendingConfirmations.value.set(request.id, {
          request,
          status: 'pending',
          messageId: `confirm_${Date.now()}`
        })
        
        // Add a system message to show the confirmation UI
        messages.value.push({
          id: `confirm_${Date.now()}`,
          role: 'system',
          content: `TOOL_CONFIRMATION:${request.id}`,
          timestamp: Date.now()
        })
      })
      
      // Set up event listener for todo updates
      await listen<{session_id: string, todos: TodoItem[]}>('todos-updated', (event) => {
        console.log('Received todos update:', event.payload)
        const { session_id, todos: newTodos } = event.payload
        
        // Only update if this is for the current session
        if (currentSessionId.value === session_id) {
          updateTodos(newTodos)
          console.log(`Updated ${newTodos.length} todos for current session`)
        }
      })
      
      // Set up event listener for intermediate LLM messages
      await listen<IntermediateMessage>('llm-intermediate-message', (event) => {
        console.log('Received intermediate LLM message:', event.payload)
        const intermediateMsg = event.payload
        
        // Only process if this is for the current session
        if (!intermediateMsg.session_id || currentSessionId.value === intermediateMsg.session_id) {
          const message: ChatMessage = {
            id: `intermediate_${Date.now()}_${Math.random()}`,
            role: 'assistant',
            content: intermediateMsg.content,
            timestamp: Date.now(),
            isIntermediate: true,
            iteration: intermediateMsg.iteration,
            toolCalls: intermediateMsg.tool_calls
          }
          messages.value.push(message)
          console.log(`Added intermediate message (iteration ${intermediateMsg.iteration})`)
        }
      })
      
      // Set up event listener for tool result messages
      await listen<ToolResultMessage>('tool-result-message', (event) => {
        console.log('Received tool result message:', event.payload)
        const toolResult = event.payload
        
        // Only process if this is for the current session
        if (!toolResult.session_id || currentSessionId.value === toolResult.session_id) {
          const message: ChatMessage = {
            id: `tool_${Date.now()}_${Math.random()}`,
            role: 'tool',
            content: toolResult.result,
            timestamp: Date.now(),
            toolName: toolResult.tool_name,
            success: toolResult.success,
            iteration: toolResult.iteration
          }
          messages.value.push(message)
          console.log(`Added tool result message: ${toolResult.tool_name}`)
        }
      })
      
      // Set up event listener for task state changes
      await listen<{task_content: string, old_status: string, new_status: string, session_id: string}>('task-state-changed', (event) => {
        console.log('Received task state change event:', event.payload)
        const stateChange = event.payload
        
        // Only process if this is for the current session
        if (currentSessionId.value === stateChange.session_id) {
          // Create appropriate message based on state change
          let content = ''
          let success = true
          
          if (stateChange.old_status === 'new' && stateChange.new_status === 'pending') {
            content = `Added task: ${stateChange.task_content}`
          } else if (stateChange.new_status === 'in_progress') {
            content = `Started task: ${stateChange.task_content}`
          } else if (stateChange.new_status === 'completed') {
            content = `Completed task: ${stateChange.task_content}`
          } else {
            content = `Task "${stateChange.task_content}" changed from ${stateChange.old_status} to ${stateChange.new_status}`
          }
          
          const message: ChatMessage = {
            id: `task_${Date.now()}_${Math.random()}`,
            role: 'tool',
            content,
            timestamp: Date.now(),
            toolName: 'todo_write',
            success
          }
          messages.value.push(message)
          console.log(`Added task state change message: ${stateChange.task_content} (${stateChange.old_status} → ${stateChange.new_status})`)
        }
      })
    } catch (err) {
      console.error('Failed to initialize chat:', err)
      error.value = String(err)
    }
  }
  
  // Note: Thinking blocks are now preserved in conversation context for better continuity
  
  // Build system message from configuration and context
  const buildSystemMessage = (): ChatMessage => {
    const contextStore = useSharedContextStore()
    const parts: string[] = []
    
    // Base instructions
    if (systemConfig.value.baseInstructions) {
      parts.push(systemConfig.value.baseInstructions)
    }
    
    // Add current context if enabled - send the full raw context as JSON
    if (systemConfig.value.contextEnabled) {
      const fullContext = {
        campaign: contextStore.campaign,
        module: contextStore.module,
        session: contextStore.session,
        reference: contextStore.reference,
        windows: Array.from(contextStore.windows.values()),
        recentActions: contextStore.recentActions.slice(0, 5), // Last 5 actions
        contextUsage: contextStore.contextUsage
      }
      
      parts.push('Current Application Context:')
      parts.push('```json')
      parts.push(JSON.stringify(fullContext, null, 2))
      parts.push('```')
    }
    
    // Add tools information if any
    if (systemConfig.value.tools && systemConfig.value.tools.length > 0) {
      parts.push(`Available tools: ${systemConfig.value.tools.join(', ')}`)
    }
    
    // Add custom instructions
    if (systemConfig.value.customInstructions) {
      parts.push(systemConfig.value.customInstructions)
    }
    
    return {
      id: 'system',
      role: 'system',
      content: parts.join('\n\n'),
      timestamp: Date.now()
    }
  }
  
  const sendMessage = async (content: string): Promise<void> => {
    if (!content.trim() || isLoading.value) return
    
    error.value = null
    isLoading.value = true
    
    // Add user message
    const userMessage: ChatMessage = {
      id: `msg_${Date.now()}`,
      role: 'user',
      content: content.trim(),
      timestamp: Date.now()
    }
    messages.value.push(userMessage)
    
    try {
      // Build system message with current context
      const systemMessage = buildSystemMessage()
      
      // Prepare messages for API (keep full content including thinking blocks)
      const conversationMessages = messages.value.map(msg => ({
        role: msg.role,
        content: msg.content
      }))
      
      // Combine system message with conversation (system message always first)
      const apiMessages = [
        { role: systemMessage.role, content: systemMessage.content },
        ...conversationMessages
      ]
      
      // Send to backend
      const response = await invoke<ChatResponseWithUsage>('send_chat_message', {
        messages: apiMessages,
        maxTokens: systemConfig.value.maxTokens || maxResponseTokens.value,
        temperature: systemConfig.value.temperature,
        enableTools: true,  // Enable tools for testing
        sessionId: currentSessionId.value
      })
      
      // Add assistant response
      const assistantMessage: ChatMessage = {
        id: `msg_${Date.now()}_assistant`,
        role: 'assistant',
        content: response.content, // Keep raw content for display (thinking blocks will be rendered as collapsible)
        timestamp: Date.now(),
        tokenUsage: {
          prompt: response.prompt_tokens,
          completion: response.completion_tokens,
          total: response.total_tokens
        }
      }
      
      messages.value.push(assistantMessage)
      
      // Note: The token counts from the API reflect what was actually sent/received
      // (without thinking blocks in the history). This is accurate for billing
      // and context window tracking since thinking blocks are stripped from the API calls.
      
      // Update total tokens
      totalTokensUsed.value += response.total_tokens
      
      // Refresh todos after any assistant message (simple and reliable)
      if (currentSessionId.value) {
        console.log('Refreshing todos after message for session:', currentSessionId.value)
        await loadTodosForSession(currentSessionId.value)
      } else {
        console.warn('No current session ID available for todo refresh')
      }
      
      // Auto-save current session
      await saveCurrentSession()
      
    } catch (err) {
      console.error('Failed to send message:', err)
      error.value = String(err)
      
      // Remove the user message if the request failed
      const idx = messages.value.findIndex(m => m.id === userMessage.id)
      if (idx !== -1) {
        messages.value.splice(idx, 1)
      }
    } finally {
      isLoading.value = false
    }
  }
  
  const clearHistory = async () => {
    if (currentSessionId.value) {
      // Create a new session to replace the current one
      await createNewSession()
    }
  }
  
  const deleteMessage = async (messageId: string) => {
    const idx = messages.value.findIndex(m => m.id === messageId)
    if (idx !== -1) {
      const msg = messages.value[idx]
      if (msg.tokenUsage) {
        totalTokensUsed.value -= msg.tokenUsage.total
      }
      messages.value.splice(idx, 1)
      await saveCurrentSession()
    }
  }
  
  const setMaxResponseTokens = (tokens: number) => {
    maxResponseTokens.value = Math.min(
      tokens,
      modelInfo.value?.defaultMaxTokens || 2048
    )
    systemConfig.value.maxTokens = maxResponseTokens.value
  }
  
  // System configuration methods
  const updateSystemConfig = (config: Partial<SystemMessageConfig>) => {
    systemConfig.value = { ...systemConfig.value, ...config }
    saveSystemConfig()
  }
  
  const toggleContext = () => {
    systemConfig.value.contextEnabled = !systemConfig.value.contextEnabled
    saveSystemConfig()
  }
  
  const setSystemInstructions = (instructions: string) => {
    systemConfig.value.baseInstructions = instructions
    saveSystemConfig()
  }
  
  const setCustomInstructions = (instructions: string) => {
    systemConfig.value.customInstructions = instructions
    saveSystemConfig()
  }
  
  const resetToDefaultPrompt = () => {
    systemConfig.value.baseInstructions = DEFAULT_SYSTEM_PROMPT
    saveSystemConfig()
  }
  
  
  const saveSystemConfig = () => {
    localStorage.setItem('chat_system_config', JSON.stringify(systemConfig.value))
  }
  
  const loadSystemConfig = () => {
    const saved = localStorage.getItem('chat_system_config')
    if (saved) {
      try {
        systemConfig.value = JSON.parse(saved)
      } catch (e) {
        console.error('Failed to load system config:', e)
      }
    }
  }
  
  
  // Tool confirmation methods
  const confirmToolAction = async (confirmationId: string) => {
    try {
      await invoke('confirm_tool_action', {
        confirmationId,
        confirmed: true
      })
      
      // Update status
      const confirmation = pendingConfirmations.value.get(confirmationId)
      if (confirmation) {
        confirmation.status = 'confirmed'
      }
    } catch (error) {
      console.error('Failed to confirm action:', error)
      throw error
    }
  }
  
  const rejectToolAction = async (confirmationId: string) => {
    try {
      await invoke('confirm_tool_action', {
        confirmationId,
        confirmed: false
      })
      
      // Update status
      const confirmation = pendingConfirmations.value.get(confirmationId)
      if (confirmation) {
        confirmation.status = 'rejected'
      }
    } catch (error) {
      console.error('Failed to reject action:', error)
      throw error
    }
  }
  
  const getConfirmationForMessage = (messageContent: string) => {
    // Check if this is a confirmation message
    if (messageContent.startsWith('TOOL_CONFIRMATION:')) {
      const confirmationId = messageContent.split(':')[1]
      return pendingConfirmations.value.get(confirmationId)
    }
    return null
  }
  
  // Session management methods
  const loadSessions = async () => {
    try {
      sessionsLoading.value = true
      const sessionList = await invoke<ChatSessionMetadata[]>('list_chat_sessions')
      sessions.value = sessionList
    } catch (err) {
      console.error('Failed to load sessions:', err)
      error.value = String(err)
    } finally {
      sessionsLoading.value = false
    }
  }
  
  const loadSession = async (sessionId: string) => {
    try {
      const session = await invoke<ChatSession | null>('load_chat_session', { sessionId })
      if (session) {
        currentSessionId.value = session.id
        messages.value = session.messages
        // Recalculate total tokens from messages
        totalTokensUsed.value = messages.value.reduce((total, msg) => {
          return total + (msg.tokenUsage?.total || 0)
        }, 0)
        // Clear todos from UI cache and load todos for this session
        todos.value = []
        await loadTodosForSession(session.id)
      }
    } catch (err) {
      console.error('Failed to load session:', err)
      error.value = String(err)
    }
  }
  
  const saveCurrentSession = async () => {
    if (!currentSessionId.value) return
    
    try {
      const session: ChatSession = {
        id: currentSessionId.value,
        title: '', // Will be auto-generated by backend
        created_at: 0, // Will be set by backend
        updated_at: Date.now(),
        messages: messages.value
      }
      await invoke('save_chat_session', { session })
      // Reload sessions to get updated metadata
      await loadSessions()
    } catch (err) {
      console.error('Failed to save session:', err)
      // Don't show error to user for auto-save failures
    }
  }
  
  const createNewSession = async () => {
    try {
      const newSession = await invoke<ChatSession>('create_chat_session')
      currentSessionId.value = newSession.id
      messages.value = []
      totalTokensUsed.value = 0
      error.value = null
      // Clear todos from UI cache and load todos for new session (should be empty)
      todos.value = []
      await loadTodosForSession(newSession.id)
      await loadSessions()
    } catch (err) {
      console.error('Failed to create session:', err)
      error.value = String(err)
    }
  }
  
  const deleteSession = async (sessionId: string) => {
    try {
      const deleted = await invoke<boolean>('delete_chat_session', { sessionId })
      if (deleted) {
        await loadSessions()
        // If we deleted the current session, create a new one
        if (currentSessionId.value === sessionId) {
          await createNewSession()
        }
      }
    } catch (err) {
      console.error('Failed to delete session:', err)
      error.value = String(err)
    }
  }
  
  const switchToSession = async (sessionId: string) => {
    if (currentSessionId.value !== sessionId) {
      await loadSession(sessionId)
    }
  }
  
  // Todo management methods
  const updateTodos = (newTodos: TodoItem[]) => {
    todos.value = newTodos
  }
  
  const toggleTodosVisibility = () => {
    todosVisible.value = !todosVisible.value
  }
  
  const clearTodos = () => {
    todos.value = []
  }
  
  const loadTodosForSession = async (sessionId: string) => {
    try {
      console.log(`Loading todos for session: ${sessionId}`)
      const response = await invoke<{success: boolean, data?: TodoItem[], error?: string}>('get_todos', { sessionId })
      console.log('Todo API response:', response)
      if (response.success && response.data) {
        todos.value = response.data
        console.log(`Successfully loaded ${response.data.length} todos for session ${sessionId}:`, response.data)
      } else {
        console.error('Todo API returned error:', response.error || 'Unknown error')
      }
    } catch (err) {
      console.error('Failed to load todos:', err)
    }
  }
  
  // Parse todos from assistant messages that contain "Todos have been modified successfully"
  const extractTodosFromMessage = async (content: string) => {
    // Look for assistant messages that indicate todo updates
    if (content.includes("Todos have been modified successfully")) {
      // This indicates todos were updated via the tool, refresh them
      if (currentSessionId.value) {
        console.log('Detected todo update in message, refreshing todos...')
        await loadTodosForSession(currentSessionId.value)
        return true
      }
    }
    return false
  }
  
  return {
    // State
    messages,
    isLoading,
    error,
    modelInfo,
    totalTokensUsed,
    maxResponseTokens,
    systemConfig,
    pendingConfirmations,
    
    // Session state
    currentSessionId,
    sessions,
    sessionsLoading,
    
    // Todo state
    todos,
    todosVisible,
    
    // Computed
    conversationTokens,
    contextUsagePercentage,
    lastMessage,
    todoProgress,
    currentTodo,
    hasTodos,
    
    // Actions
    initialize,
    sendMessage,
    clearHistory,
    deleteMessage,
    setMaxResponseTokens,
    
    // System config methods
    updateSystemConfig,
    toggleContext,
    setSystemInstructions,
    setCustomInstructions,
    resetToDefaultPrompt,
    buildSystemMessage,
    
    // Tool confirmation methods
    confirmToolAction,
    rejectToolAction,
    getConfirmationForMessage,
    
    // Session management methods
    loadSessions,
    loadSession,
    saveCurrentSession,
    createNewSession,
    deleteSession,
    switchToSession,
    
    // Todo management methods
    updateTodos,
    toggleTodosVisibility,
    clearTodos,
    loadTodosForSession,
    extractTodosFromMessage
  }
})