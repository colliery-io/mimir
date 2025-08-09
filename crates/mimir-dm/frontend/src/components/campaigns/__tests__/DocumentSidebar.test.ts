import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { mockIPC, clearMocks } from '@tauri-apps/api/mocks'
import DocumentSidebar from '../DocumentSidebar.vue'

describe('DocumentSidebar', () => {
  let mockDocuments: any[] = []
  
  beforeEach(() => {
    mockDocuments = []
    
    // Setup IPC mocks
    mockIPC((cmd, args) => {
      switch (cmd) {
        case 'get_campaign_documents':
          // Return existing documents
          return {
            data: mockDocuments,
            success: true
          }
          
        case 'create_document_from_template':
          // Create new document
          const newDoc = {
            id: Date.now(),
            campaign_id: args.campaignId,
            template_id: args.templateId,
            document_type: args.templateId,
            title: args.templateId.split('_').map(w => 
              w.charAt(0).toUpperCase() + w.slice(1)
            ).join(' '),
            file_path: `/test/${args.templateId}.md`,
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
            completed_at: null
          }
          mockDocuments.push(newDoc)
          return {
            success: true,
            data: newDoc
          }
        
        case 'get_campaign':
          return {
            success: true,
            data: {
              id: args.id,
              name: 'Test Campaign',
              status: 'concept',
              directory_path: '/test/campaign',
              created_at: '2024-01-01T00:00:00'
            }
          }
          
        default:
          console.warn(`Unmocked command: ${cmd}`)
          throw new Error(`Unmocked command: ${cmd}`)
      }
    })
  })
  
  afterEach(() => {
    clearMocks()
    vi.clearAllMocks()
  })
  
  const createWrapper = (props = {}) => {
    return mount(DocumentSidebar, {
      props: {
        campaignId: 1,
        campaignStage: 'concept',
        boardConfig: {
          stages: [
            {
              key: 'concept',
              display_name: 'Concept',
              required_documents: ['campaign_pitch'],
              optional_documents: []
            },
            {
              key: 'session_zero',
              display_name: 'Session Zero',
              required_documents: ['starting_scenario'],
              optional_documents: []
            }
          ]
        },
        ...props
      }
    })
  }
  
  describe('Document Loading', () => {
    it('should load documents on mount', async () => {
      // Add a document to mock data
      mockDocuments = [{
        id: 1,
        campaign_id: 1,
        template_id: 'campaign_pitch',
        document_type: 'campaign_pitch',
        title: 'Campaign Pitch',
        file_path: '/test/campaign_pitch.md',
        created_at: '2024-01-01',
        updated_at: '2024-01-01',
        completed_at: null
      }]
      
      const wrapper = createWrapper()
      await flushPromises()
      
      // Check that document appears in sidebar
      expect(wrapper.text()).toContain('Campaign Pitch')
      expect(wrapper.find('.document-item').exists()).toBe(true)
    })
    
    it('should show documents grouped by stage', async () => {
      const wrapper = createWrapper()
      await flushPromises()
      
      // Check stage headers
      expect(wrapper.text()).toContain('Concept')
      expect(wrapper.find('.stage-group').exists()).toBe(true)
    })
  })
  
  describe('Document Clicking', () => {
    it('should emit selectDocument when clicking existing document', async () => {
      // Setup with existing document
      mockDocuments = [{
        id: 1,
        campaign_id: 1,
        template_id: 'campaign_pitch',
        document_type: 'campaign_pitch',
        title: 'Campaign Pitch',
        file_path: '/test/campaign_pitch.md',
        created_at: '2024-01-01',
        updated_at: '2024-01-01',
        completed_at: null
      }]
      
      const wrapper = createWrapper()
      await flushPromises()
      
      // Click the document title
      const docTitle = wrapper.find('.document-title')
      await docTitle.trigger('click')
      await flushPromises()
      
      // Check emit was called with the document
      const emitted = wrapper.emitted('selectDocument')
      expect(emitted).toBeTruthy()
      expect(emitted![0][0]).toMatchObject({
        id: 1,
        template_id: 'campaign_pitch'
      })
    })
    
    it('should create document if it does not exist', async () => {
      // Start with no documents
      mockDocuments = []
      
      const wrapper = createWrapper()
      await flushPromises()
      
      // Find and click campaign_pitch (which doesn't have an instance)
      const docItems = wrapper.findAll('.document-item')
      expect(docItems.length).toBeGreaterThan(0)
      
      const campaignPitchTitle = wrapper.find('.document-title')
      await campaignPitchTitle.trigger('click')
      await flushPromises()
      
      // Should have created and selected the document
      const emitted = wrapper.emitted('selectDocument')
      expect(emitted).toBeTruthy()
      expect(emitted![0][0]).toMatchObject({
        template_id: 'campaign_pitch'
      })
    })
    
    it('should not allow clicking on locked stages', async () => {
      const wrapper = createWrapper({
        campaignStage: 'concept' // Still in concept, so session_zero should be locked
      })
      await flushPromises()
      
      // Try to find session_zero documents (should be visible but locked)
      const stageGroups = wrapper.findAll('.stage-group')
      
      // Find session_zero stage if visible
      const sessionZeroGroup = stageGroups.find(g => 
        g.text().includes('Session Zero')
      )
      
      if (sessionZeroGroup) {
        const lockedDoc = sessionZeroGroup.find('.document-item.locked')
        if (lockedDoc.exists()) {
          await lockedDoc.trigger('click')
          await flushPromises()
          
          // Should not emit anything
          const emitted = wrapper.emitted('selectDocument')
          expect(emitted).toBeFalsy()
        }
      }
    })
  })
  
  describe('Stage Accessibility', () => {
    it('should show concept stage as accessible', async () => {
      const wrapper = createWrapper({
        campaignStage: 'concept'
      })
      await flushPromises()
      
      // Get concept stage documents
      const conceptDocs = wrapper.findAll('.document-item').filter(
        item => !item.classes().includes('locked')
      )
      
      expect(conceptDocs.length).toBeGreaterThan(0)
    })
  })
  
  describe('Document Progress', () => {
    it('should show completion status in stage header', async () => {
      mockDocuments = [{
        id: 1,
        campaign_id: 1,
        template_id: 'campaign_pitch',
        document_type: 'campaign_pitch',
        title: 'Campaign Pitch',
        file_path: '/test/campaign_pitch.md',
        created_at: '2024-01-01',
        updated_at: '2024-01-01',
        completed_at: '2024-01-02' // Completed
      }]
      
      const wrapper = createWrapper()
      await flushPromises()
      
      // Should show 1/1 for concept stage
      const stageHeader = wrapper.find('.stage-header')
      expect(stageHeader.text()).toContain('(1/1)')
    })
  })
})