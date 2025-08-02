export interface Campaign {
  id: number
  name: string
  status: string
  directory_path: string
  created_at: string
}

export interface CreateCampaignRequest {
  name: string
  description?: string
  directory_location: string
}