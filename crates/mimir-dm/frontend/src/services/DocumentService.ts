import { invoke } from '@tauri-apps/api/core'

export interface DocumentData {
  title: string
  content: string
  documentType: 'vision' | 'strategy' | 'initiative' | 'task' | 'adr'
  parentId?: string
  templateId?: string
  moduleId?: number
  campaignId?: number
}

export interface Document {
  id: string | number
  title: string
  content: string
  phase?: 'draft' | 'review' | 'published' | 'archived'
  documentType?: 'vision' | 'strategy' | 'initiative' | 'task' | 'adr'
  document_type?: string
  parent_id?: string
  parentId?: string
  template_id?: string
  templateId?: string
  created_at?: string
  createdAt?: Date
  updated_at?: string
  updatedAt?: Date
  completed_at?: string | null
  completedAt?: Date | null
  blocked_by?: string[]
  blockedBy?: string[]
  exit_criteria?: any[]
  exitCriteria?: any[]
  // Additional fields for compatibility
  campaign_id?: number
  module_id?: number | null
  session_id?: number | null
  file_path?: string
}

class DocumentServiceClass {
  private cache = new Map<string, Document[]>()
  
  async create(data: DocumentData): Promise<Document> {
    const response = await invoke<{ data: Document }>('create_document', {
      request: data
    })
    
    this.clearCache()
    return response.data
  }
  
  async update(id: string | number, content: string): Promise<Document> {
    const response = await invoke<{ data: Document }>('update_document', {
      request: {
        document_id: id,
        content
      }
    })
    
    this.clearCache()
    return response.data
  }
  
  async updateMetadata(id: string | number, metadata: Partial<Document>): Promise<Document> {
    const response = await invoke<{ data: Document }>('update_document_metadata', {
      request: {
        document_id: id,
        ...metadata
      }
    })
    
    this.clearCache()
    return response.data
  }
  
  async delete(id: string | number): Promise<void> {
    await invoke('delete_document', {
      request: { document_id: id }
    })
    
    this.clearCache()
  }
  
  async transition(id: string | number, phase: string): Promise<Document> {
    const response = await invoke<{ data: Document }>('transition_document_phase', {
      request: {
        document_id: id,
        phase
      }
    })
    
    this.clearCache()
    return response.data
  }
  
  async complete(id: string | number): Promise<Document> {
    const response = await invoke<{ data: Document }>('complete_document', {
      request: { document_id: id }
    })
    
    this.clearCache()
    return response.data
  }
  
  async uncomplete(id: string | number): Promise<Document> {
    const response = await invoke<{ data: Document }>('uncomplete_document', {
      request: { document_id: id }
    })
    
    this.clearCache()
    return response.data
  }
  
  async validateExitCriteria(id: string | number): Promise<boolean> {
    const response = await invoke<{ data: boolean }>('validate_exit_criteria', {
      request: { document_id: id }
    })
    
    return response.data
  }
  
  async list(moduleId?: number, campaignId?: number): Promise<Document[]> {
    const cacheKey = `${moduleId || ''}-${campaignId || ''}`
    
    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey)!
    }
    
    const response = await invoke<{ data: Document[] }>('list_documents', {
      request: {
        module_id: moduleId,
        campaign_id: campaignId
      }
    })
    
    const documents = response.data || []
    this.cache.set(cacheKey, documents)
    return documents
  }
  
  async getByType(moduleId: number | undefined, type: string): Promise<Document[]> {
    const all = await this.list(moduleId)
    return all.filter(d => 
      d.documentType === type || d.document_type === type
    )
  }
  
  async getByTemplate(moduleId: number, templateId: string): Promise<Document | undefined> {
    const all = await this.list(moduleId)
    return all.find(d => 
      d.templateId === templateId || d.template_id === templateId
    )
  }
  
  async batchUpdate(updates: Array<{ id: string | number; content: string }>): Promise<Document[]> {
    const results = await Promise.all(
      updates.map(({ id, content }) => this.update(id, content))
    )
    
    this.clearCache()
    return results
  }
  
  clearCache() {
    this.cache.clear()
  }
}

export const DocumentService = new DocumentServiceClass()