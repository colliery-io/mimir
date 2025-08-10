import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { mockIPC, clearMocks } from '@tauri-apps/api/mocks'
import StageLandingView from '../StageLandingView.vue'
import * as boardConfigServiceModule from '../../../services/boardConfigService'

// Mock the board config service
vi.mock('../../../services/boardConfigService', () => {
  const mockService = {
    fetchBoardConfig: vi.fn(),
    getBoardConfig: vi.fn(),
    cacheBoard: vi.fn(),
    getStageDocuments: vi.fn(),
    getStageDefinition: vi.fn(),
    calculateStageCompletion: vi.fn()
  }
  
  return {
    boardConfigService: mockService,
    BoardConfigurationService: vi.fn(() => mockService)
  }
})

describe('StageLandingView', () => {
  const mockBoardConfig = {
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
        required_documents: [
          'starting_scenario',
          'world_primer', 
          'character_guidelines',
          'table_expectations',
          'character_integration'
        ],
        optional_documents: ['safety_tools', 'house_rules']
      },
      {
        key: 'integration',
        display_name: 'Integration',
        description: 'Integrating player feedback',
        required_documents: ['campaign_bible', 'major_npc_tracker'],
        optional_documents: ['player_secrets', 'faction_overview']
      },
      {
        key: 'active',
        display_name: 'Active',
        description: 'Campaign is actively being played',
        required_documents: [],
        optional_documents: ['session_notes', 'player_handouts']
      }
    ]
  }
  
  const mockDocuments = [
    {
      id: 1,
      template_id: 'campaign_pitch',
      title: 'Campaign Pitch',
      completed_at: null
    }
  ]
  
  beforeEach(() => {
    // Setup board config service mock responses
    boardConfigServiceModule.boardConfigService.getStageDocuments.mockReturnValue([
      {
        templateId: 'campaign_pitch',
        title: 'Campaign Pitch',
        description: 'Your compelling one-page pitch for the campaign',
        category: 'required'
      }
    ])
    
    boardConfigServiceModule.boardConfigService.getBoardConfig.mockReturnValue(null)
    boardConfigServiceModule.boardConfigService.cacheBoard.mockImplementation(() => {})
  })
  
  afterEach(() => {
    vi.clearAllMocks()
    clearMocks()
  })
  
  const createWrapper = (props = {}) => {
    return mount(StageLandingView, {
      props: {
        stage: 'concept',
        documents: mockDocuments,
        campaign: { id: 1, name: 'Test Campaign' },
        boardConfig: mockBoardConfig,
        ...props
      }
    })
  }
  
  describe('Board Configuration Integration', () => {
    it('should cache board config on mount', async () => {
      createWrapper()
      await flushPromises()
      
      // Should have called cacheBoard with transformed config
      expect(boardConfigServiceModule.boardConfigService.cacheBoard).toHaveBeenCalledWith(
        expect.objectContaining({
          boardType: 'campaign',
          stages: expect.arrayContaining([
            expect.objectContaining({
              key: 'concept',
              displayName: 'Concept'
            })
          ])
        })
      )
    })
    
    it('should not cache if already cached', async () => {
      boardConfigServiceModule.boardConfigService.getBoardConfig.mockReturnValue({
        boardType: 'campaign',
        stages: []
      })
      
      createWrapper()
      await flushPromises()
      
      // Should not call cacheBoard if already cached
      expect(boardConfigServiceModule.boardConfigService.cacheBoard).not.toHaveBeenCalled()
    })
    
    it('should use board config service for document metadata', async () => {
      const wrapper = createWrapper({ stage: 'session_zero' })
      
      // Mock service to return session zero documents
      boardConfigServiceModule.boardConfigService.getStageDocuments.mockReturnValue([
        {
          templateId: 'starting_scenario',
          title: 'Starting Scenario',
          description: 'The opening situation that brings the party together',
          category: 'required'
        },
        {
          templateId: 'world_primer',
          title: 'World Primer',
          description: 'Essential information about your campaign setting',
          category: 'required'
        },
        {
          templateId: 'character_guidelines',
          title: 'Character Guidelines',
          description: 'Rules and suggestions for character creation',
          category: 'required'
        },
        {
          templateId: 'table_expectations',
          title: 'Table Expectations',
          description: 'Gameplay style, safety tools, and social contract',
          category: 'required'
        },
        {
          templateId: 'character_integration',
          title: 'Character Integration',
          description: 'How player characters connect to the world and story',
          category: 'required'
        }
      ])
      
      await flushPromises()
      
      // Should call getStageDocuments with correct parameters
      expect(boardConfigServiceModule.boardConfigService.getStageDocuments).toHaveBeenCalledWith('campaign', 'session_zero')
    })
  })
  
  describe('Stage Display', () => {
    it('should display stage title and subtitle from board config', () => {
      const wrapper = createWrapper()
      
      expect(wrapper.find('.stage-header h2').text()).toBe('Concept')
      expect(wrapper.find('.stage-subtitle').text()).toBe('Initial campaign planning')
    })
    
    it('should show correct documents for session zero stage', async () => {
      boardConfigServiceModule.boardConfigService.getStageDocuments.mockReturnValue([
        {
          templateId: 'starting_scenario',
          title: 'Starting Scenario',
          description: 'The opening situation that brings the party together',
          category: 'required'
        },
        {
          templateId: 'character_integration',
          title: 'Character Integration',
          description: 'How player characters connect to the world and story',
          category: 'required'
        }
      ])
      
      const wrapper = createWrapper({ stage: 'session_zero' })
      await flushPromises()
      
      // Should display all required documents including character_integration
      const docCards = wrapper.findAll('.document-card')
      expect(docCards.length).toBeGreaterThan(0)
      
      const titles = docCards.map(card => card.find('h4').text())
      expect(titles).toContain('Starting Scenario')
      expect(titles).toContain('Character Integration')
    })
  })
  
  describe('Document Progress', () => {
    it('should calculate progress based on board config required documents', () => {
      const wrapper = createWrapper({
        stage: 'session_zero',
        documents: [
          { template_id: 'starting_scenario', completed_at: '2024-01-01' },
          { template_id: 'world_primer', completed_at: '2024-01-01' }
        ],
        boardConfig: mockBoardConfig
      })
      
      // Progress should be 2/5 = 40%
      const progressCard = wrapper.find('.progress-card')
      expect(progressCard.text()).toContain('2') // completed
      expect(progressCard.text()).toContain('5') // total
      expect(progressCard.text()).toContain('40%')
    })
    
    it('should show 100% when all required documents are complete', () => {
      const wrapper = createWrapper({
        stage: 'concept',
        documents: [
          { template_id: 'campaign_pitch', completed_at: '2024-01-01' }
        ],
        boardConfig: mockBoardConfig
      })
      
      const progressCard = wrapper.find('.progress-card')
      expect(progressCard.text()).toContain('1') // completed
      expect(progressCard.text()).toContain('1') // total
      expect(progressCard.text()).toContain('100%')
    })
  })
  
  describe('Stage Transitions', () => {
    it('should show next stage button when all required documents are complete', () => {
      const wrapper = createWrapper({
        stage: 'concept',
        documents: [
          { template_id: 'campaign_pitch', completed_at: '2024-01-01' }
        ],
        boardConfig: mockBoardConfig
      })
      
      const nextSteps = wrapper.find('.next-steps')
      expect(nextSteps.exists()).toBe(true)
      expect(nextSteps.text()).toContain('Ready for Next Stage')
      expect(nextSteps.text()).toContain('Advance to Session Zero')
    })
    
    it('should not show next stage button when documents are incomplete', () => {
      const wrapper = createWrapper({
        stage: 'concept',
        documents: [
          { template_id: 'campaign_pitch', completed_at: null }
        ],
        boardConfig: mockBoardConfig
      })
      
      const nextSteps = wrapper.find('.next-steps')
      expect(nextSteps.exists()).toBe(false)
    })
    
    it('should emit transitionStage when clicking advance button', async () => {
      const wrapper = createWrapper({
        stage: 'concept',
        documents: [
          { template_id: 'campaign_pitch', completed_at: '2024-01-01' }
        ],
        boardConfig: mockBoardConfig
      })
      
      const advanceButton = wrapper.find('.btn-primary')
      await advanceButton.trigger('click')
      
      expect(wrapper.emitted('transitionStage')).toBeTruthy()
      expect(wrapper.emitted('transitionStage')![0]).toEqual(['session_zero'])
    })
  })
})