import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('../views/HomeView.vue')
  },
  {
    path: '/campaigns',
    name: 'campaigns',
    component: () => import('../views/campaigns/CampaignListView.vue')
  },
  {
    path: '/campaigns/new',
    name: 'campaign-new',
    component: () => import('../views/campaigns/CampaignCreateView.vue')
  },
  {
    path: '/campaigns/:id',
    name: 'campaign-detail',
    component: () => import('../views/campaigns/CampaignDetailView.vue'),
    props: true
  },
  {
    path: '/campaigns/:id/modules',
    name: 'campaign-modules',
    component: () => import('../views/modules/ModuleListView.vue'),
    props: true
  },
  {
    path: '/modules/:id',
    name: 'module-detail',
    component: () => import('../views/modules/ModuleDetailView.vue'),
    props: true
  },
  {
    path: '/modules/:id/sessions',
    name: 'module-sessions',
    component: () => import('../views/sessions/SessionListView.vue'),
    props: true
  },
  {
    path: '/sessions/:id',
    name: 'session-detail',
    component: () => import('../views/sessions/SessionDetailView.vue'),
    props: true
  },
  {
    path: '/templates',
    name: 'templates',
    component: () => import('../views/templates/TemplateListView.vue')
  },
  {
    path: '/templates/:id',
    name: 'template-detail',
    component: () => import('../views/templates/TemplateDetailView.vue'),
    props: true
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('../views/SettingsView.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router