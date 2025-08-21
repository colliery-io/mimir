import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('../../views/HomeView.vue')
  },
  {
    path: '/campaigns',
    name: 'campaigns',
    component: () => import('../../features/campaigns/views/CampaignListView.vue')
  },
  {
    path: '/campaigns/new',
    name: 'campaign-new',
    component: () => import('../../features/campaigns/views/CampaignCreateView.vue')
  },
  {
    path: '/campaigns/:id',
    name: 'campaign-detail',
    component: () => import('../../features/campaigns/views/CampaignDetailView.vue'),
    props: true
  },
  {
    path: '/campaigns/:id/board',
    name: 'campaign-board',
    component: () => import('../../features/campaigns/views/CampaignBoardView.vue'),
    props: true
  },
  {
    path: '/campaigns/:id/modules',
    name: 'campaign-modules',
    component: () => import('../../features/modules/views/ModuleListView.vue'),
    props: true
  },
  {
    path: '/modules/:id',
    name: 'module-detail',
    component: () => import('../../features/modules/views/ModuleDetailView.vue'),
    props: true
  },
  {
    path: '/modules/:id/board',
    name: 'module-board',
    component: () => import('../../features/modules/views/ModuleBoardView.vue'),
    props: true
  },
  {
    path: '/modules/:id/sessions',
    name: 'module-sessions',
    component: () => import('../../features/sessions/views/SessionListView.vue'),
    props: true
  },
  {
    path: '/sessions/:id',
    name: 'session-detail',
    component: () => import('../../features/sessions/views/SessionDetailView.vue'),
    props: true
  },
  {
    path: '/templates',
    name: 'templates',
    component: () => import('../../features/templates/views/TemplateListView.vue')
  },
  {
    path: '/templates/:id',
    name: 'template-detail',
    component: () => import('../../features/templates/views/TemplateDetailView.vue'),
    props: true
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('../../views/SettingsView.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router