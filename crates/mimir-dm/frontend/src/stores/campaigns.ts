import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { ApiResponse, Campaign, NewCampaign } from '../types/api'

export const useCampaignStore = defineStore('campaigns', () => {
  const campaigns = ref<Campaign[]>([])
  const currentCampaign = ref<Campaign | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  
  // Fetch all campaigns
  const fetchCampaigns = async () => {
    loading.value = true
    error.value = null
    
    try {
      const response = await invoke<ApiResponse<Campaign[]>>('list_campaigns')
      if (response.success && response.data) {
        campaigns.value = response.data
      } else {
        error.value = response.error || 'Failed to fetch campaigns'
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error occurred'
    } finally {
      loading.value = false
    }
  }
  
  // Get campaign by ID
  const getCampaign = async (id: number) => {
    loading.value = true
    error.value = null
    
    try {
      const response = await invoke<ApiResponse<Campaign>>('get_campaign', { id })
      if (response.success && response.data) {
        currentCampaign.value = response.data
        return response.data
      } else {
        error.value = response.error || 'Failed to fetch campaign'
        return null
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error occurred'
      return null
    } finally {
      loading.value = false
    }
  }
  
  // Create new campaign
  const createCampaign = async (data: NewCampaign) => {
    loading.value = true
    error.value = null
    
    try {
      const response = await invoke<ApiResponse<Campaign>>('create_campaign', { request: data })
      if (response.success && response.data) {
        campaigns.value.push(response.data)
        return response.data
      } else {
        error.value = response.error || 'Failed to create campaign'
        return null
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error occurred'
      return null
    } finally {
      loading.value = false
    }
  }
  
  // Update campaign status
  const updateCampaignStatus = async (id: number, status: string) => {
    loading.value = true
    error.value = null
    
    try {
      const response = await invoke<ApiResponse<Campaign>>('update_campaign_status', { id, status })
      if (response.success && response.data) {
        // Update local state
        const index = campaigns.value.findIndex(c => c.id === id)
        if (index !== -1) {
          campaigns.value[index] = response.data
        }
        if (currentCampaign.value?.id === id) {
          currentCampaign.value = response.data
        }
        return response.data
      } else {
        error.value = response.error || 'Failed to update campaign'
        return null
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error occurred'
      return null
    } finally {
      loading.value = false
    }
  }
  
  // Delete campaign
  const deleteCampaign = async (id: number) => {
    loading.value = true
    error.value = null
    
    try {
      const response = await invoke<ApiResponse<void>>('delete_campaign', { id })
      if (response.success) {
        // Remove from local state
        campaigns.value = campaigns.value.filter(c => c.id !== id)
        if (currentCampaign.value?.id === id) {
          currentCampaign.value = null
        }
        return true
      } else {
        error.value = response.error || 'Failed to delete campaign'
        return false
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Unknown error occurred'
      return false
    } finally {
      loading.value = false
    }
  }
  
  return {
    campaigns,
    currentCampaign,
    loading,
    error,
    fetchCampaigns,
    getCampaign,
    createCampaign,
    updateCampaignStatus,
    deleteCampaign
  }
})