import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useSharedContextStore } from './sharedContext'

export interface ChatMessage {
  id: string
  role: 'user' | 'assistant' | 'system'
  content: string
  timestamp: number
  tokenUsage?: {
    prompt: number
    completion: number
    total: number
  }
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

export const useChatStore = defineStore('chat', () => {
  // State
  const messages = ref<ChatMessage[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const modelInfo = ref<ModelInfo | null>(null)
  const totalTokensUsed = ref(0)
  const maxResponseTokens = ref(2048)
  
  // System message configuration
  const systemConfig = ref<SystemMessageConfig>({
    baseInstructions: `You are a D&D 5e Dungeon Master assistant.

RESPONSE FORMAT RULES:
- Provide direct, helpful answers
- Do NOT add system status messages like "(System: ...)" 
- Do NOT add roleplay elements unless asked
- Do NOT use decorative formatting unless needed for clarity
- Focus on answering the actual question

CONTEXT USAGE:
- Use the provided JSON context to give relevant assistance
- The context shows current campaign, module, session, and user actions
- Reference specific context details when relevant`,
    contextEnabled: true,
    tools: [],
    customInstructions: '',
    temperature: 0.5,  // Lower temperature for better instruction following
    maxTokens: 2048
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
  
  // Actions
  const initialize = async () => {
    try {
      // Get model info
      const info = await invoke<ModelInfo>('get_model_context_info')
      modelInfo.value = info
      maxResponseTokens.value = info.defaultMaxTokens
      
      // Load system configuration
      loadSystemConfig()
      
      // Load saved messages from localStorage if any
      const saved = localStorage.getItem('chat_history')
      if (saved) {
        const parsed = JSON.parse(saved)
        messages.value = parsed.messages || []
        totalTokensUsed.value = parsed.totalTokens || 0
      }
    } catch (err) {
      console.error('Failed to initialize chat:', err)
      error.value = String(err)
    }
  }
  
  // Strip thinking blocks from content for API
  const stripThinkingBlocks = (content: string): string => {
    // Remove <thinking>, <think> blocks and their content
    return content.replace(/<think(?:ing)?>([\s\S]*?)<\/think(?:ing)?>/gi, '').trim()
  }
  
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
      
      // Prepare messages for API (strip thinking blocks from assistant messages)
      const conversationMessages = messages.value.map(msg => ({
        role: msg.role,
        content: msg.role === 'assistant' ? stripThinkingBlocks(msg.content) : msg.content
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
        enableTools: true  // Enable tools for testing
      })
      
      // Add assistant response
      const assistantMessage: ChatMessage = {
        id: `msg_${Date.now()}_assistant`,
        role: 'assistant',
        content: response.content,
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
      
      // Save to localStorage
      saveToLocalStorage()
      
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
  
  const clearHistory = () => {
    messages.value = []
    totalTokensUsed.value = 0
    error.value = null
    localStorage.removeItem('chat_history')
  }
  
  const deleteMessage = (messageId: string) => {
    const idx = messages.value.findIndex(m => m.id === messageId)
    if (idx !== -1) {
      const msg = messages.value[idx]
      if (msg.tokenUsage) {
        totalTokensUsed.value -= msg.tokenUsage.total
      }
      messages.value.splice(idx, 1)
      saveToLocalStorage()
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
  
  const saveToLocalStorage = () => {
    localStorage.setItem('chat_history', JSON.stringify({
      messages: messages.value,
      totalTokens: totalTokensUsed.value,
      savedAt: Date.now()
    }))
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
    
    // Computed
    conversationTokens,
    contextUsagePercentage,
    lastMessage,
    
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
    buildSystemMessage
  }
})