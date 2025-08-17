// Re-export all existing types from their current locations
export * from './api'
// Campaign is already exported from api.ts, so we skip campaign.ts to avoid conflicts

// Additional shared types that are currently duplicated across components

// Document types (currently duplicated in DocumentSidebar components)
export interface Document {
  id: number
  campaign_id: number
  module_id: number | null
  session_id: number | null
  template_id: string
  document_type: string
  title: string
  file_path: string
  created_at: string
  updated_at: string
  completed_at: string | null
}

export interface DocumentTemplate {
  templateId: string
  title: string
  required: boolean
  stage: string
  instance?: Document
}

// Board configuration types (used across board views)
export interface BoardStage {
  key: string
  display_name: string
  templates?: DocumentTemplate[]
  required_documents?: string[]
  optional_documents?: string[]
  no_completion_required_documents?: string[]
}

export interface BoardConfig {
  stages: BoardStage[]
  documents: DocumentTemplate[]
  transitions?: Record<string, string[]>
}

// Theme constants (these are actually static in the app)
export const THEMES = {
  LIGHT: 'light',
  DARK: 'dark',
  HYPER: 'hyper'
} as const

export type Theme = typeof THEMES[keyof typeof THEMES]

// Entity type for polymorphic components
export type EntityType = 'campaign' | 'module' | 'session'

// Stage progress data structure
export interface StageDocuments {
  documents: DocumentTemplate[]
  completed: number
  total: number
  percentage: number
}

// Common board entity interface for abstraction
export interface BoardEntity {
  id: number
  name: string
  status: string
  created_at: string
}

// Icon paths map for themes
export interface ThemeIcons {
  edit: string
  locked: string
  gear?: string
  mimir?: string
  new?: string
}

export type IconMap = Record<Theme, ThemeIcons>