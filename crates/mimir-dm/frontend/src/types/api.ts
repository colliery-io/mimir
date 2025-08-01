// API Response types
export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
}

// Theme types
export interface Theme {
  id: string
  name: string
  description: string
}

// Campaign types
export interface Campaign {
  id: number
  name: string
  status: string
  created_at: string
}

export interface NewCampaign {
  name: string
}

// Module types
export interface Module {
  id: number
  campaign_id: number
  name: string
  description?: string
  status: string
  sessions_planned: number
  sessions_completed: number
  created_at: string
}

// Session types
export interface Session {
  id: number
  module_id: number
  session_number: number
  name: string
  status: string
  scheduled_date?: string
  actual_date?: string
  notes?: string
  created_at: string
}

// Workflow card types
export interface WorkflowCard {
  id: number
  board_type: 'campaign' | 'module' | 'session'
  board_id: number
  title: string
  description?: string
  status: string
  position: number
  due_date?: string
  created_at: string
  updated_at: string
}

// Template types
export interface Template {
  document_id: string
  version_number: number
  document_content: string
  document_type?: string
  document_level?: string
  purpose?: string
  is_active: boolean
}

export interface RenderTemplateRequest {
  template_id: string
  variables: Record<string, any>
}

// Character types (for future implementation)
export interface Character {
  id: number
  name: string
  race: string
  class: string
  level: number
  campaign_id?: number
  created_at: string
}