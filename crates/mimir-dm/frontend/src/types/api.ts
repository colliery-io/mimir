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
  directory_path: string
  created_at: string
  session_zero_date?: string
  first_session_date?: string
  last_activity_at: string
  archived_at?: string
}

export interface NewCampaign {
  name: string
  description?: string
  directory_location: string
}

// Module types
export interface Module {
  id: number
  campaign_id: number
  name: string
  module_number: number
  description?: string
  status: string
  sessions_planned: number
  sessions_completed: number
  expected_sessions?: number
  actual_sessions?: number
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

// Character types moved to ./character.ts
// (Placeholder removed - now using full character management system)