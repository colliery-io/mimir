import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest'
import { mockIPC, clearMocks } from '@tauri-apps/api/mocks'
import { BoardConfigurationService } from '../boardConfigService'

describe('BoardConfigurationService', () => {
  let service: BoardConfigurationService
  
  const mockBoardConfig = {
    success: true,
    data: {
      board_type: 'campaign',
      stages: [
        {
          key: 'concept',
          display_name: 'Concept',
          description: 'Initial campaign planning',
          required_documents: ['campaign_pitch'],
          optional_documents: [],
          completion_message: 'Great! Your campaign pitch is complete.',
          transition_prompt: 'Ready to move to Session Zero?',
          help_text: 'The Concept stage is where you develop your initial idea.'
        },
        {
          key: 'session_zero',
          display_name: 'Session Zero',
          description: 'Preparing materials for session zero',
          required_documents: ['starting_scenario', 'world_primer', 'character_guidelines', 'table_expectations', 'character_integration'],
          optional_documents: ['safety_tools', 'house_rules'],
          completion_message: 'Excellent! Your Session Zero materials are ready.',
          transition_prompt: 'Remember to share these documents with your players.',
          help_text: 'Session Zero is a collaborative session.'
        },
        {
          key: 'integration',
          display_name: 'Integration',
          description: 'Integrating player feedback',
          required_documents: ['campaign_bible', 'major_npc_tracker'],
          optional_documents: ['player_secrets', 'faction_overview'],
          completion_message: 'Perfect! Your campaign is fully integrated.',
          transition_prompt: 'These documents will be your reference.',
          help_text: 'The Integration stage is where you weave everything together.'
        },
        {
          key: 'active',
          display_name: 'Active',
          description: 'Campaign is actively being played',
          required_documents: [],
          optional_documents: ['session_notes', 'player_handouts'],
          completion_message: null,
          transition_prompt: null,
          help_text: 'Your campaign is now active!'
        }
      ],
      transitions: [
        { from: 'concept', to: 'session_zero', allowed: true },
        { from: 'session_zero', to: 'integration', allowed: true },
        { from: 'integration', to: 'active', allowed: true },
        { from: 'concept', to: 'integration', allowed: false },
        { from: 'session_zero', to: 'concept', allowed: false }
      ]
    }
  }
  
  beforeEach(() => {
    service = new BoardConfigurationService()
    
    // Setup IPC mock
    mockIPC((cmd, args) => {
      if (cmd === 'get_board_configuration') {
        return mockBoardConfig
      }
      throw new Error(`Unmocked command: ${cmd}`)
    })
  })
  
  afterEach(() => {
    clearMocks()
    vi.clearAllMocks()
  })
  
  describe('fetchBoardConfig', () => {
    it('should fetch and cache board configuration', async () => {
      const config = await service.fetchBoardConfig('campaign')
      
      expect(config).toBeDefined()
      expect(config.boardType).toBe('campaign')
      expect(config.stages).toHaveLength(4)
      
      // Check it was cached
      const cached = service.getBoardConfig('campaign')
      expect(cached).toEqual(config)
    })
    
    it('should transform backend response to frontend format', async () => {
      const config = await service.fetchBoardConfig('campaign')
      
      const conceptStage = config.stages[0]
      expect(conceptStage.key).toBe('concept')
      expect(conceptStage.displayName).toBe('Concept')
      expect(conceptStage.requiredDocuments).toEqual(['campaign_pitch'])
      expect(conceptStage.metadata.completionMessage).toBe('Great! Your campaign pitch is complete.')
    })
  })
  
  describe('cacheBoard', () => {
    it('should cache a board configuration', () => {
      const config = {
        boardType: 'test',
        stages: [],
        transitions: {}
      }
      
      service.cacheBoard(config)
      
      const cached = service.getBoardConfig('test')
      expect(cached).toEqual(config)
    })
  })
  
  describe('getStageDefinition', () => {
    it('should return stage definition if exists', async () => {
      await service.fetchBoardConfig('campaign')
      
      const stage = service.getStageDefinition('campaign', 'session_zero')
      expect(stage).toBeDefined()
      expect(stage?.key).toBe('session_zero')
      expect(stage?.requiredDocuments).toContain('starting_scenario')
    })
    
    it('should return undefined for non-existent stage', async () => {
      await service.fetchBoardConfig('campaign')
      
      const stage = service.getStageDefinition('campaign', 'non_existent')
      expect(stage).toBeUndefined()
    })
    
    it('should return undefined for non-cached board', () => {
      const stage = service.getStageDefinition('unknown', 'concept')
      expect(stage).toBeUndefined()
    })
  })
  
  describe('getStageDocuments', () => {
    it('should return documents with metadata for a stage', async () => {
      await service.fetchBoardConfig('campaign')
      
      const docs = service.getStageDocuments('campaign', 'session_zero')
      
      // Should have required documents
      const requiredDocs = docs.filter(d => d.category === 'required')
      expect(requiredDocs).toHaveLength(5)
      
      const startingScenario = docs.find(d => d.templateId === 'starting_scenario')
      expect(startingScenario).toBeDefined()
      expect(startingScenario?.title).toBe('Starting Scenario')
      expect(startingScenario?.description).toBe('The opening situation that brings the party together')
      expect(startingScenario?.category).toBe('required')
      
      // Should have optional documents
      const optionalDocs = docs.filter(d => d.category === 'optional')
      expect(optionalDocs).toHaveLength(2)
      
      const safetyTools = docs.find(d => d.templateId === 'safety_tools')
      expect(safetyTools).toBeDefined()
      expect(safetyTools?.category).toBe('optional')
    })
    
    it('should provide fallback for unknown document types', async () => {
      // Modify the mock to return an unknown document
      mockIPC((cmd) => {
        if (cmd === 'get_board_configuration') {
          return {
            ...mockBoardConfig,
            data: {
              ...mockBoardConfig.data,
              stages: [{
                key: 'test',
                display_name: 'Test',
                description: 'Test stage',
                required_documents: ['unknown_document_type'],
                optional_documents: []
              }]
            }
          }
        }
        throw new Error(`Unmocked command: ${cmd}`)
      })
      
      const newService = new BoardConfigurationService()
      await newService.fetchBoardConfig('campaign')
      
      const docs = newService.getStageDocuments('campaign', 'test')
      const unknownDoc = docs[0]
      
      expect(unknownDoc.templateId).toBe('unknown_document_type')
      expect(unknownDoc.title).toBe('Unknown Document Type')
      expect(unknownDoc.description).toBe('Document: unknown_document_type')
    })
  })
  
  describe('calculateStageCompletion', () => {
    it('should calculate completion status correctly', async () => {
      await service.fetchBoardConfig('campaign')
      
      const stage = service.getStageDefinition('campaign', 'session_zero')!
      const completedDocs = new Set(['starting_scenario', 'world_primer'])
      
      const status = service.calculateStageCompletion(stage, completedDocs)
      
      expect(status.completed).toBe(2)
      expect(status.total).toBe(5)
      expect(status.percentage).toBe(40)
      expect(status.isComplete).toBe(false)
      expect(status.missingDocuments).toContain('character_guidelines')
      expect(status.missingDocuments).toContain('table_expectations')
      expect(status.missingDocuments).toContain('character_integration')
    })
    
    it('should handle fully completed stage', async () => {
      await service.fetchBoardConfig('campaign')
      
      const stage = service.getStageDefinition('campaign', 'concept')!
      const completedDocs = new Set(['campaign_pitch'])
      
      const status = service.calculateStageCompletion(stage, completedDocs)
      
      expect(status.completed).toBe(1)
      expect(status.total).toBe(1)
      expect(status.percentage).toBe(100)
      expect(status.isComplete).toBe(true)
      expect(status.missingDocuments).toHaveLength(0)
    })
    
    it('should handle stage with no required documents', async () => {
      await service.fetchBoardConfig('campaign')
      
      const stage = service.getStageDefinition('campaign', 'active')!
      const completedDocs = new Set<string>()
      
      const status = service.calculateStageCompletion(stage, completedDocs)
      
      expect(status.completed).toBe(0)
      expect(status.total).toBe(0)
      expect(status.percentage).toBe(0)
      expect(status.isComplete).toBe(false) // No required docs means not complete
    })
  })
  
  describe('getNextStage', () => {
    it('should return next stage in progression', async () => {
      await service.fetchBoardConfig('campaign')
      
      const nextStage = service.getNextStage('campaign', 'concept')
      expect(nextStage).toBe('session_zero')
    })
    
    it('should return undefined for last stage', async () => {
      await service.fetchBoardConfig('campaign')
      
      const nextStage = service.getNextStage('campaign', 'active')
      expect(nextStage).toBeUndefined()
    })
    
    it('should return undefined for invalid stage', async () => {
      await service.fetchBoardConfig('campaign')
      
      const nextStage = service.getNextStage('campaign', 'invalid')
      expect(nextStage).toBeUndefined()
    })
  })
})