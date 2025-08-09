// Mock implementations for Tauri commands
import { mockIPC } from '@tauri-apps/api/mocks'

interface DocumentRequest {
  filePath?: string
  file_path?: string
  templateId?: string
  campaignId?: number
}

// Mock data
const mockCampaign = {
  id: 1,
  name: "Test Campaign",
  status: "concept",
  directory_path: "/tmp/test-campaign",
  created_at: "2024-01-01T00:00:00"
}

const mockDocument = {
  id: 1,
  campaign_id: 1,
  module_id: null,
  session_id: null,
  template_id: "campaign-pitch",
  document_type: "campaign_pitch",
  title: "Campaign Pitch",
  file_path: "/tmp/test-campaign/campaign-pitch.md",
  created_at: "2024-01-01T00:00:00",
  updated_at: "2024-01-01T00:00:00",
  completed_at: null
}

const mockBoardConfig = {
  board_type: "campaign",
  stages: [
    {
      key: "concept",
      display_name: "Concept",
      description: "Initial campaign planning and pitch development",
      required_documents: ["campaign-pitch"],
      optional_documents: [],
      completion_message: "Great! Your campaign pitch is complete.",
      transition_prompt: "Ready to move to Session Zero?",
      help_text: null
    },
    {
      key: "session_zero",
      display_name: "Session Zero",
      description: "Player onboarding and character creation",
      required_documents: ["starting-scenario", "world-primer", "character-guidelines", "table-expectations"],
      optional_documents: ["safety-tools", "house-rules"],
      completion_message: null,
      transition_prompt: null,
      help_text: null
    }
  ],
  transitions: []
}

export function setupMocks() {
  console.log('🔧 Setting up Tauri mocks for debugging')
  
  mockIPC((cmd, args) => {
    console.log(`📨 IPC Call: ${cmd}`, args)
    const request = args as DocumentRequest | undefined
    
    switch (cmd) {
      case 'get_campaign':
        return { success: true, data: mockCampaign }
      
      case 'get_board_configuration':
        return { success: true, data: mockBoardConfig }
      
      case 'get_campaign_documents':
        console.log('📄 Returning mock documents')
        return { success: true, data: [mockDocument] }
      
      case 'read_document_file':
        console.log('📖 Reading document file:', request?.filePath || request?.file_path)
        return { 
          success: true, 
          data: "# Campaign Pitch\n\nThis is mock content for testing.\n\n## The Hook\n\nYour amazing campaign idea goes here!" 
        }
      
      case 'save_document_file':
        console.log('💾 Saving document file:', request?.filePath || request?.file_path)
        return { success: true, data: null }
      
      case 'create_document_from_template':
        console.log('🆕 Creating document from template:', request?.templateId)
        return { 
          success: true, 
          data: {
            ...mockDocument,
            id: Date.now(),
            template_id: request?.templateId || 'unknown',
            title: (request?.templateId || 'unknown').replace('-', ' ').replace(/\b\w/g, (l: string) => l.toUpperCase())
          }
        }
      
      case 'check_campaign_stage_completion':
        return { 
          success: true, 
          data: {
            board_type: "campaign",
            current_stage: "concept",
            total_required_documents: 1,
            completed_required_documents: 0,
            total_optional_documents: 0,
            completed_optional_documents: 0,
            missing_required_documents: ["campaign-pitch"],
            is_stage_complete: false,
            can_progress: false,
            next_stage: "session_zero",
            stage_metadata: mockBoardConfig.stages[0]
          }
        }
      
      default:
        console.warn(`⚠️ Unmocked command: ${cmd}`)
        return { success: false, error: `Command ${cmd} not mocked` }
    }
  })
  
  console.log('✅ Mocks ready - all IPC calls will be intercepted')
}

export function clearMocks() {
  // @ts-ignore
  window.__TAURI_IPC__ = undefined
  console.log('🧹 Mocks cleared')
}