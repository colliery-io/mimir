import { invoke } from '@tauri-apps/api/core'
import type { Document } from './DocumentService'

export interface ModuleData {
  name: string
  description?: string
  campaign_id: number
  module_type?: string
  status?: string
}

export interface Module {
  id: number
  name: string
  description?: string
  campaign_id: number
  module_id?: number // Some responses include this
  module_type: string
  status: string
  session_count: number
  created_at: string
  updated_at: string
  // Additional fields for compatibility - set defaults where needed
  module_number: number
  expected_sessions: number
  actual_sessions: number
  sessions_planned: number
  sessions_completed: number
  started_at: string | null
  completed_at: string | null
  documents?: Document[]
  sessions?: Session[]
}

export interface Session {
  id: number // Changed from string | number to match components
  module_id: number
  name: string
  date?: string
  players?: string[]
  status: 'planned' | 'active' | 'completed' | string
  session_number: number // Made required with default
  scheduled_date?: string
  actual_date?: string
  notes?: string
  created_at: string
  updated_at?: string
  completed_at?: string | null
}

class ModuleServiceClass {
  private cache = new Map<string, Module | Module[]>()
  
  // Core CRUD operations
  async get(id: number): Promise<Module> {
    const cacheKey = `module-${id}`
    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey) as Module
    }
    
    try {
      const response = await invoke<{ data: Module } | { success: boolean; data: Module }>('get_module', { 
        id 
      })
      
      const module = 'success' in response ? response.data : response.data
      // Ensure required fields have defaults
      const normalizedModule = {
        ...module,
        module_number: module.module_number ?? 1,
        expected_sessions: module.expected_sessions ?? 0,
        actual_sessions: module.actual_sessions ?? 0,
        sessions_planned: module.sessions_planned ?? 0,
        sessions_completed: module.sessions_completed ?? 0,
        started_at: module.started_at ?? null,
        completed_at: module.completed_at ?? null
      }
      this.cache.set(cacheKey, normalizedModule)
      return normalizedModule
    } catch (error) {
      throw new Error(`Failed to get module: ${error}`)
    }
  }
  
  async list(campaignId: number): Promise<Module[]> {
    const cacheKey = `modules-campaign-${campaignId}`
    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey) as Module[]
    }
    
    try {
      const response = await invoke<{ data: Module[] }>('list_campaign_modules', {
        campaignId
      })
      
      const modules = (response.data || []).map((module, index) => ({
        ...module,
        module_number: module.module_number ?? index + 1,
        expected_sessions: module.expected_sessions ?? 0,
        actual_sessions: module.actual_sessions ?? 0,
        sessions_planned: module.sessions_planned ?? 0,
        sessions_completed: module.sessions_completed ?? 0,
        started_at: module.started_at ?? null,
        completed_at: module.completed_at ?? null
      }))
      this.cache.set(cacheKey, modules)
      return modules
    } catch (error) {
      throw new Error(`Failed to list modules: ${error}`)
    }
  }
  
  async create(data: ModuleData): Promise<Module> {
    try {
      const response = await invoke<{ data: Module } | Module>('create_module', {
        ...data
      })
      
      // Clear campaign cache since we added a new module
      this.clearCampaignCache(data.campaign_id)
      
      const module = 'data' in response ? response.data : response
      // Ensure required fields have defaults
      return {
        ...module,
        module_number: module.module_number ?? 1,
        expected_sessions: module.expected_sessions ?? 0,
        actual_sessions: module.actual_sessions ?? 0,
        sessions_planned: module.sessions_planned ?? 0,
        sessions_completed: module.sessions_completed ?? 0,
        started_at: module.started_at ?? null,
        completed_at: module.completed_at ?? null
      }
    } catch (error) {
      throw new Error(`Failed to create module: ${error}`)
    }
  }
  
  async update(id: number, data: Partial<ModuleData>): Promise<Module> {
    try {
      const response = await invoke<{ data: Module }>('update_module', {
        id,
        ...data
      })
      
      // Clear caches
      this.cache.delete(`module-${id}`)
      if (data.campaign_id) {
        this.clearCampaignCache(data.campaign_id)
      }
      
      // Ensure required fields have defaults
      return {
        ...response.data,
        module_number: response.data.module_number ?? 1,
        expected_sessions: response.data.expected_sessions ?? 0,
        actual_sessions: response.data.actual_sessions ?? 0,
        sessions_planned: response.data.sessions_planned ?? 0,
        sessions_completed: response.data.sessions_completed ?? 0,
        started_at: response.data.started_at ?? null,
        completed_at: response.data.completed_at ?? null
      }
    } catch (error) {
      throw new Error(`Failed to update module: ${error}`)
    }
  }
  
  async delete(id: number): Promise<void> {
    try {
      await invoke('delete_module', { id })
      
      // Clear all caches since we don't know the campaign
      this.clearCache()
    } catch (error) {
      throw new Error(`Failed to delete module: ${error}`)
    }
  }
  
  // Status/Stage management
  async updateStatus(id: number, status: string): Promise<Module> {
    try {
      const response = await invoke<{ data: Module }>('update_module_status', {
        moduleId: id,
        status
      })
      
      // Clear module cache
      this.cache.delete(`module-${id}`)
      
      // Ensure required fields have defaults
      return {
        ...response.data,
        module_number: response.data.module_number ?? 1,
        expected_sessions: response.data.expected_sessions ?? 0,
        actual_sessions: response.data.actual_sessions ?? 0,
        sessions_planned: response.data.sessions_planned ?? 0,
        sessions_completed: response.data.sessions_completed ?? 0,
        started_at: response.data.started_at ?? null,
        completed_at: response.data.completed_at ?? null
      }
    } catch (error) {
      throw new Error(`Failed to update module status: ${error}`)
    }
  }
  
  async transitionStage(id: number, newStage: string): Promise<Module> {
    try {
      const response = await invoke<{ data: Module }>('transition_module_stage', {
        moduleId: id,
        newStage
      })
      
      // Clear module cache
      this.cache.delete(`module-${id}`)
      
      // Ensure required fields have defaults
      return {
        ...response.data,
        module_number: response.data.module_number ?? 1,
        expected_sessions: response.data.expected_sessions ?? 0,
        actual_sessions: response.data.actual_sessions ?? 0,
        sessions_planned: response.data.sessions_planned ?? 0,
        sessions_completed: response.data.sessions_completed ?? 0,
        started_at: response.data.started_at ?? null,
        completed_at: response.data.completed_at ?? null
      }
    } catch (error) {
      throw new Error(`Failed to transition module stage: ${error}`)
    }
  }
  
  // Document management
  async initializeDocuments(id: number): Promise<string[]> {
    try {
      const response = await invoke<{ data: string[] }>('initialize_module_documents', {
        moduleId: id
      })
      
      return response.data || []
    } catch (error) {
      throw new Error(`Failed to initialize module documents: ${error}`)
    }
  }
  
  async getDocuments(id: number): Promise<Document[]> {
    try {
      const response = await invoke<{ data: Document[] }>('get_module_documents', {
        request: {
          module_id: id
        }
      })
      
      return response.data || []
    } catch (error) {
      throw new Error(`Failed to get module documents: ${error}`)
    }
  }
  
  // Session management
  async listSessions(id: number): Promise<Session[]> {
    try {
      const response = await invoke<{ data: Session[] }>('list_module_sessions', {
        request: {
          module_id: id
        }
      })
      
      // Ensure required fields have defaults
      return (response.data || []).map((session, index) => ({
        ...session,
        session_number: session.session_number ?? index + 1,
        created_at: session.created_at ?? new Date().toISOString()
      }))
    } catch (error) {
      throw new Error(`Failed to list module sessions: ${error}`)
    }
  }
  
  async incrementSessionCount(id: number): Promise<void> {
    try {
      await invoke('increment_module_sessions', {
        moduleId: id
      })
      
      // Clear module cache to get updated count
      this.cache.delete(`module-${id}`)
    } catch (error) {
      // Non-critical error, don't throw
      console.warn(`Failed to increment session count: ${error}`)
    }
  }
  
  // Helper methods
  private clearCampaignCache(campaignId: number) {
    this.cache.delete(`modules-campaign-${campaignId}`)
  }
  
  clearCache() {
    this.cache.clear()
  }
  
  // Utility method to check if a module can transition
  async canTransition(id: number, boardConfig: any): Promise<boolean> {
    try {
      const module = await this.get(id)
      const documents = await this.getDocuments(id)
      
      if (!boardConfig?.stages) return false
      
      const currentStage = boardConfig.stages.find((s: any) => s.key === module.status)
      if (!currentStage) return false
      
      // Check if required documents are complete
      const requiredDocs = currentStage.required_documents || []
      const noCompletionRequired = currentStage.no_completion_required_documents || []
      const completionRequiredDocs = requiredDocs.filter((docId: string) => 
        !noCompletionRequired.includes(docId)
      )
      
      const completedDocs = documents.filter(doc => 
        doc.template_id && 
        completionRequiredDocs.includes(doc.template_id) && 
        doc.completed_at
      )
      
      return completedDocs.length === completionRequiredDocs.length && 
             completionRequiredDocs.length > 0
    } catch (error) {
      return false
    }
  }
}

export const ModuleService = new ModuleServiceClass()