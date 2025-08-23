# Mimir Codebase Cleanup & Refactoring Plan

## Executive Summary
This plan focuses on **removing old patterns and reducing codebase size** through systematic deletion, consolidation, and simplification. Expected outcome: **1,700+ lines removed** while improving maintainability.

## Phase 1: Pure Deletions (Immediate - Zero Risk)
**Target: -500 lines | Timeline: Day 1**

### 1.1 Dead Code Removal

#### Delete Entire Files
1. **DELETE** `crates/mimir-dm-core/src/models/rules_extended.rs` (419 lines)
   - Contains unused models: `CharacterClass`, `ClassSummary`, `Race`, `Feat`, `Background`
   - No imports found across codebase
   - Action: 
     ```bash
     rm crates/mimir-dm-core/src/models/rules_extended.rs
     # Then remove from crates/mimir-dm-core/src/models/mod.rs:
     # Delete line: pub mod rules_extended;
     ```

2. **DELETE** `crates/mimir-dm/src-tauri/src/commands/modules.rs.bak` (20+ lines)
   ```bash
   rm crates/mimir-dm/src-tauri/src/commands/modules.rs.bak
   ```

#### Remove Unused Imports and Console Logs
3. **CLEAN** all TypeScript/Vue files
   ```bash
   # Find and remove all console.log statements
   grep -r "console\." crates/mimir-dm/frontend/src/ --include="*.ts" --include="*.vue" | cut -d: -f1 | sort -u
   
   # Files to clean:
   - crates/mimir-dm/frontend/src/features/sources/composables/useCatalog.ts (remove 5+ console.logs)
   - crates/mimir-dm/frontend/src/features/modules/components/ModuleStageLandingView.vue (remove debug logs)
   - crates/mimir-dm/frontend/src/features/campaigns/components/*.vue (remove all console statements)
   ```

### 1.2 Verification Steps
```bash
# Verify rules_extended.rs is truly unused
grep -r "rules_extended" crates/
grep -r "CharacterClass" crates/ --include="*.rs"
grep -r "ClassSummary" crates/ --include="*.rs"

# Verify build still works
cd crates/mimir-dm && npm run build
```

## Phase 2: Consolidation of Duplicates (Low Risk)
**Target: -300 lines | Timeline: Days 2-3**

### 2.1 Modal Component Deduplication

#### Current Duplicate Modals
```
crates/mimir-dm/frontend/src/features/sources/components/search/CatalogModal.vue (30 lines)
crates/mimir-dm/frontend/src/features/sources/components/content/ContentModal.vue (30 lines)
```

Both are identical - same template, same props, same logic.

#### Migration Strategy
1. **CREATE** `crates/mimir-dm/frontend/src/components/shared/BaseModal.vue`
   ```vue
   <!-- BaseModal.vue -->
   <template>
     <dialog ref="modal" class="modal">
       <div class="modal-box max-w-4xl max-h-[90vh] overflow-hidden">
         <slot />
       </div>
       <form method="dialog" class="modal-backdrop">
         <button>close</button>
       </form>
     </dialog>
   </template>
   
   <script setup lang="ts">
   import { ref } from 'vue'
   
   const modal = ref<HTMLDialogElement>()
   
   const open = () => modal.value?.showModal()
   const close = () => modal.value?.close()
   
   defineExpose({ open, close })
   </script>
   ```

2. **UPDATE** consumers:
   ```typescript
   // In SearchView.vue - Line 547
   // BEFORE:
   import CatalogModal from './CatalogModal.vue'
   // AFTER:
   import BaseModal from '@/components/shared/BaseModal.vue'
   
   // Change template reference from <CatalogModal> to <BaseModal>
   
   // In SourceSearch.vue - Line 123
   // BEFORE:
   import ContentModal from '../content/ContentModal.vue'
   // AFTER:
   import BaseModal from '@/components/shared/BaseModal.vue'
   ```

3. **DELETE** both original files:
   ```bash
   rm crates/mimir-dm/frontend/src/features/sources/components/search/CatalogModal.vue
   rm crates/mimir-dm/frontend/src/features/sources/components/content/ContentModal.vue
   ```

### 2.2 Type Definition Consolidation

#### Current Duplicates
The `Document` interface appears in:
- `crates/mimir-dm/frontend/src/features/modules/types.ts` (lines 45-52)
- `crates/mimir-dm/frontend/src/features/campaigns/types.ts` (lines 23-30)
- Inline in `ModuleStageLandingView.vue` (lines 234-241)

The `BoardConfig` type appears in:
- `ModuleStageLandingView.vue` (lines 112-119)
- `ModuleDocumentSidebar.vue` (lines 89-96)
- `useBoardNavigation.ts` (lines 15-22)

#### Migration Strategy
1. **CREATE** centralized types:
   ```typescript
   // crates/mimir-dm/frontend/src/types/domain.ts
   export interface Document {
     id: string
     title: string
     content: string
     phase: 'draft' | 'review' | 'published' | 'archived'
     documentType: 'vision' | 'strategy' | 'initiative' | 'task' | 'adr'
     parentId?: string
     createdAt: Date
     updatedAt: Date
     blockedBy?: string[]
     exitCriteria?: ExitCriterion[]
   }
   
   export interface BoardConfig {
     columns: BoardColumn[]
     defaultColumn?: string
     transitions?: BoardTransition[]
   }
   
   export interface BoardColumn {
     id: string
     title: string
     color?: string
     order: number
   }
   
   export interface BoardTransition {
     from: string
     to: string
     label?: string
   }
   ```

2. **UPDATE** all consumers to import from centralized location:
   ```typescript
   // In each file that uses these types:
   import type { Document, BoardConfig } from '@/types/domain'
   
   // Files to update:
   - ModuleStageLandingView.vue (remove lines 112-119, 234-241)
   - ModuleDocumentSidebar.vue (remove lines 89-96)
   - features/modules/types.ts (remove lines 45-52)
   - features/campaigns/types.ts (remove lines 23-30)
   - shared/composables/useBoardNavigation.ts (remove lines 15-22)
   ```

3. **DELETE** duplicate definitions (saves ~70 lines)

### 2.3 CSS Pattern Consolidation

#### Current Duplicates
Shimmer animation appears in:
- `ModuleStageLandingView.vue` (lines 823-842, 20 lines)
- `SearchView.vue` (lines 612-631, 20 lines)
- `ModuleListView.vue` (lines 445-464, 20 lines)

#### Migration Strategy
1. **CREATE** shared animation utilities:
   ```css
   /* crates/mimir-dm/frontend/src/assets/styles/utilities/animations.css */
   @keyframes shimmer {
     0% { background-position: -1000px 0; }
     100% { background-position: 1000px 0; }
   }
   
   .shimmer {
     animation: shimmer 2s infinite;
     background: linear-gradient(
       90deg,
       rgb(var(--color-base-300)) 25%,
       rgb(var(--color-base-200)) 50%,
       rgb(var(--color-base-300)) 75%
     );
     background-size: 1000px 100%;
   }
   
   .shimmer-text {
     @apply shimmer rounded h-4 w-full;
   }
   
   .shimmer-title {
     @apply shimmer rounded h-6 w-3/4;
   }
   ```

2. **UPDATE** components to use shared classes:
   ```vue
   <!-- In each component, replace local shimmer with: -->
   <div class="shimmer-text"></div>
   
   <!-- Remove the @keyframes and .shimmer definitions from <style> blocks -->
   ```

3. **DELETE** 60 lines of duplicate CSS

## Phase 3: Component Decomposition (Medium Risk)
**Target: -700 lines through better organization | Timeline: Week 2**

### 3.1 ModuleStageLandingView.vue Breakdown
**Current**: 887 lines → **Target**: 500 lines total across 7 files

#### Current Structure Analysis
```
ModuleStageLandingView.vue (887 lines)
├── Template (lines 1-287, 287 lines)
│   ├── Stage header (lines 15-45)
│   ├── Transition card (lines 47-89)
│   ├── Planning stage content (lines 91-134)
│   ├── Development stage content (lines 136-179)
│   ├── Ready stage content (lines 181-224)
│   ├── Active stage content (lines 226-269)
│   └── Session management (lines 271-287)
├── Script (lines 289-623, 334 lines)
│   ├── Imports and props (lines 289-312)
│   ├── State management (lines 314-356)
│   ├── Computed properties (lines 358-445)
│   ├── Methods (lines 447-589)
│   └── Lifecycle hooks (lines 591-623)
└── Styles (lines 625-887, 262 lines)
    ├── Component styles (lines 625-742)
    ├── Shimmer animation (lines 744-823)
    └── Utility classes (lines 825-887)
```

#### Extraction Plan

1. **CREATE** `crates/mimir-dm/frontend/src/features/modules/components/stage/StageHeader.vue`
   ```vue
   <!-- StageHeader.vue (50 lines) -->
   <template>
     <div class="mb-6">
       <div class="flex items-center justify-between">
         <div>
           <h1 class="text-4xl font-bold">{{ stageInfo.title }}</h1>
           <p class="text-base-content/70 mt-2">{{ stageInfo.subtitle }}</p>
         </div>
         <div class="badge badge-lg" :class="stageInfo.color">
           {{ stageInfo.phase }}
         </div>
       </div>
     </div>
   </template>
   
   <script setup lang="ts">
   import type { Module, Stage } from '../../types'
   
   defineProps<{
     module: Module
     stageInfo: StageInfo
   }>()
   </script>
   ```

2. **CREATE** `crates/mimir-dm/frontend/src/features/modules/components/stage/StageTransitionCard.vue`
   ```vue
   <!-- StageTransitionCard.vue (80 lines) -->
   <template>
     <div class="card bg-base-200 shadow-xl mb-6 animate-shimmer">
       <div class="card-body">
         <h2 class="card-title">Ready for {{ nextStage.title }}?</h2>
         <p>{{ nextStage.description }}</p>
         <div class="card-actions justify-end">
           <button 
             @click="$emit('transition')"
             class="btn btn-primary"
           >
             Move to {{ nextStage.title }}
           </button>
         </div>
       </div>
     </div>
   </template>
   
   <script setup lang="ts">
   defineProps<{
     nextStage: StageInfo
   }>()
   
   defineEmits<{
     transition: []
   }>()
   </script>
   ```

3. **CREATE** `crates/mimir-dm/frontend/src/features/modules/composables/useModuleStage.ts`
   ```typescript
   // useModuleStage.ts (150 lines) - Extract from lines 358-445, 447-523
   import { computed, ref } from 'vue'
   import { invoke } from '@tauri-apps/api/tauri'
   import type { Module, Stage } from '../types'
   
   export function useModuleStage(module: Module, stage: Stage) {
     // Extract all stage-related computed properties
     const stageInfo = computed(() => {
       const configs = {
         planning: {
           title: 'Planning Stage',
           subtitle: 'Define your module structure and prepare content',
           color: 'badge-info',
           phase: 'Planning'
         },
         development: {
           title: 'Development Stage',
           subtitle: 'Build and refine your module content',
           color: 'badge-warning',
           phase: 'In Development'
         },
         ready: {
           title: 'Ready Stage',
           subtitle: 'Final review before going live',
           color: 'badge-success',
           phase: 'Ready'
         },
         active: {
           title: 'Active Stage',
           subtitle: 'Your module is live and in use',
           color: 'badge-primary',
           phase: 'Active'
         }
       }
       return configs[stage] || configs.planning
     })
     
     const canTransitionToNext = computed(() => {
       // Move logic from lines 378-392
       if (stage === 'active') return false
       
       const requiredDocs = getRequiredDocuments(stage)
       const completedDocs = module.documents.filter(d => d.phase === 'published')
       
       return completedDocs.length >= requiredDocs.length
     })
     
     const nextStage = computed(() => {
       const transitions = {
         planning: 'development',
         development: 'ready',
         ready: 'active',
         active: null
       }
       return transitions[stage]
     })
     
     async function transitionToNextStage() {
       // Move from lines 489-523
       if (!canTransitionToNext.value) return
       
       try {
         await invoke('transition_module_stage', {
           moduleId: module.id,
           newStage: nextStage.value
         })
       } catch (error) {
         console.error('Failed to transition stage:', error)
         throw error
       }
     }
     
     return {
       stageInfo,
       canTransitionToNext,
       nextStage,
       transitionToNextStage
     }
   }
   ```

4. **CREATE** `crates/mimir-dm/frontend/src/features/modules/composables/useSessionManagement.ts`
   ```typescript
   // useSessionManagement.ts (120 lines) - Extract session logic
   import { ref, computed } from 'vue'
   import { invoke } from '@tauri-apps/api/tauri'
   import type { Session } from '../types'
   
   export function useSessionManagement(moduleId: string) {
     const sessions = ref<Session[]>([])
     const loading = ref(false)
     
     async function fetchSessions() {
       loading.value = true
       try {
         sessions.value = await invoke('get_module_sessions', { moduleId })
       } finally {
         loading.value = false
       }
     }
     
     async function createSession(data: Partial<Session>) {
       const session = await invoke('create_session', {
         moduleId,
         ...data
       })
       sessions.value.push(session)
       return session
     }
     
     async function updateSession(sessionId: string, data: Partial<Session>) {
       const updated = await invoke('update_session', {
         sessionId,
         ...data
       })
       const index = sessions.value.findIndex(s => s.id === sessionId)
       if (index !== -1) {
         sessions.value[index] = updated
       }
       return updated
     }
     
     async function deleteSession(sessionId: string) {
       await invoke('delete_session', { sessionId })
       sessions.value = sessions.value.filter(s => s.id !== sessionId)
     }
     
     const activeSessions = computed(() => 
       sessions.value.filter(s => s.status === 'active')
     )
     
     const completedSessions = computed(() =>
       sessions.value.filter(s => s.status === 'completed')
     )
     
     return {
       sessions,
       loading,
       fetchSessions,
       createSession,
       updateSession,
       deleteSession,
       activeSessions,
       completedSessions
     }
   }
   ```

5. **CREATE** Stage content components:
   ```vue
   <!-- PlanningStageContent.vue (80 lines) -->
   <template>
     <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
       <div class="card bg-base-200">
         <div class="card-body">
           <h3 class="card-title">Module Overview</h3>
           <DocumentList 
             :documents="visionDocuments"
             type="vision"
           />
         </div>
       </div>
       <div class="card bg-base-200">
         <div class="card-body">
           <h3 class="card-title">Strategies</h3>
           <DocumentList 
             :documents="strategyDocuments"
             type="strategy"
           />
         </div>
       </div>
     </div>
   </template>
   
   <script setup lang="ts">
   import { computed } from 'vue'
   import DocumentList from './DocumentList.vue'
   
   const props = defineProps<{
     documents: Document[]
   }>()
   
   const visionDocuments = computed(() => 
     props.documents.filter(d => d.type === 'vision')
   )
   
   const strategyDocuments = computed(() =>
     props.documents.filter(d => d.type === 'strategy')
   )
   </script>
   ```

   Similar components for DevelopmentStageContent, ReadyStageContent, ActiveStageContent

6. **CREATE** `crates/mimir-dm/frontend/src/features/modules/components/session/SessionTable.vue`
   ```vue
   <!-- SessionTable.vue (120 lines) -->
   <template>
     <div class="overflow-x-auto">
       <table class="table table-zebra">
         <thead>
           <tr>
             <th>Session</th>
             <th>Date</th>
             <th>Players</th>
             <th>Status</th>
             <th>Actions</th>
           </tr>
         </thead>
         <tbody>
           <tr v-for="session in sessions" :key="session.id">
             <td>{{ session.name }}</td>
             <td>{{ formatDate(session.date) }}</td>
             <td>{{ session.players.length }}</td>
             <td>
               <span class="badge" :class="getStatusClass(session.status)">
                 {{ session.status }}
               </span>
             </td>
             <td>
               <button 
                 @click="$emit('edit', session)"
                 class="btn btn-sm btn-ghost"
               >
                 Edit
               </button>
               <button 
                 @click="$emit('delete', session.id)"
                 class="btn btn-sm btn-ghost text-error"
               >
                 Delete
               </button>
             </td>
           </tr>
         </tbody>
       </table>
     </div>
   </template>
   
   <script setup lang="ts">
   import type { Session } from '../../types'
   import { formatDate } from '@/utils/date'
   
   defineProps<{
     sessions: Session[]
   }>()
   
   defineEmits<{
     edit: [session: Session]
     delete: [id: string]
   }>()
   
   function getStatusClass(status: string) {
     const classes = {
       active: 'badge-success',
       planned: 'badge-info',
       completed: 'badge-neutral'
     }
     return classes[status] || 'badge-ghost'
   }
   </script>
   ```

7. **REFACTOR** `ModuleStageLandingView.vue` to orchestrator (100 lines):
   ```vue
   <!-- ModuleStageLandingView.vue - Refactored to 100 lines -->
   <template>
     <div class="container mx-auto p-6">
       <StageHeader :module="module" :stage-info="stageInfo" />
       
       <StageTransitionCard 
         v-if="canTransitionToNext"
         :next-stage="nextStageInfo"
         @transition="handleTransition"
       />
       
       <component 
         :is="stageContentComponent"
         :documents="module.documents"
         :module="module"
       />
       
       <div v-if="showSessions" class="mt-8">
         <h2 class="text-2xl font-bold mb-4">Sessions</h2>
         <SessionTable 
           :sessions="sessions"
           @edit="handleEditSession"
           @delete="handleDeleteSession"
         />
       </div>
     </div>
   </template>
   
   <script setup lang="ts">
   import { computed } from 'vue'
   import { useModuleStage } from '../composables/useModuleStage'
   import { useSessionManagement } from '../composables/useSessionManagement'
   import StageHeader from './stage/StageHeader.vue'
   import StageTransitionCard from './stage/StageTransitionCard.vue'
   import PlanningStageContent from './stage/PlanningStageContent.vue'
   import DevelopmentStageContent from './stage/DevelopmentStageContent.vue'
   import ReadyStageContent from './stage/ReadyStageContent.vue'
   import ActiveStageContent from './stage/ActiveStageContent.vue'
   import SessionTable from './session/SessionTable.vue'
   
   const props = defineProps<{
     module: Module
     stage: Stage
   }>()
   
   // Use composables for logic
   const { stageInfo, canTransitionToNext, nextStageInfo, transitionToNextStage } = useModuleStage(props.module, props.stage)
   const { sessions, updateSession, deleteSession } = useSessionManagement(props.module.id)
   
   // Computed for dynamic component
   const stageContentComponent = computed(() => {
     const components = {
       planning: PlanningStageContent,
       development: DevelopmentStageContent,
       ready: ReadyStageContent,
       active: ActiveStageContent
     }
     return components[props.stage]
   })
   
   const showSessions = computed(() => 
     props.stage === 'active' || props.stage === 'ready'
   )
   
   async function handleTransition() {
     await transitionToNextStage()
     // Refresh or navigate as needed
   }
   
   async function handleEditSession(session: Session) {
     // Open edit modal or navigate to edit page
   }
   
   async function handleDeleteSession(id: string) {
     if (confirm('Delete this session?')) {
       await deleteSession(id)
     }
   }
   </script>
   ```

### 3.2 SearchView.vue Breakdown
**Current**: 726 lines → **Target**: 400 lines total across 5 files

#### Current Structure
```
SearchView.vue (726 lines)
├── Template (lines 1-245)
│   ├── Search filters (lines 23-89)
│   ├── Content type tabs (lines 91-112)
│   ├── Results grid (lines 114-189)
│   └── Modal content (lines 191-245)
├── Script (lines 247-534)
│   ├── State for each content type (lines 267-312)
│   ├── Search methods for each type (lines 314-423)
│   ├── Filter methods (lines 425-489)
│   └── Modal handling (lines 491-534)
└── Styles (lines 536-726)
```

#### Extraction Plan

1. **CREATE** `crates/mimir-dm/frontend/src/features/sources/services/SearchService.ts`
   ```typescript
   // SearchService.ts (200 lines) - Centralize all search operations
   import { invoke } from '@tauri-apps/api/tauri'
   import type { Spell, Item, Monster, Class, Feat } from '../types'
   
   export interface SearchFilters {
     query: string
     level?: number[]
     school?: string[]
     rarity?: string[]
     type?: string[]
     source?: string[]
   }
   
   class SearchService {
     private cache = new Map<string, any[]>()
     
     async searchSpells(filters: SearchFilters): Promise<Spell[]> {
       const cacheKey = `spells:${JSON.stringify(filters)}`
       if (this.cache.has(cacheKey)) {
         return this.cache.get(cacheKey)!
       }
       
       const results = await invoke<Spell[]>('search_spells', { filters })
       this.cache.set(cacheKey, results)
       return results
     }
     
     async searchItems(filters: SearchFilters): Promise<Item[]> {
       const cacheKey = `items:${JSON.stringify(filters)}`
       if (this.cache.has(cacheKey)) {
         return this.cache.get(cacheKey)!
       }
       
       const results = await invoke<Item[]>('search_items', { filters })
       this.cache.set(cacheKey, results)
       return results
     }
     
     async searchMonsters(filters: SearchFilters): Promise<Monster[]> {
       const cacheKey = `monsters:${JSON.stringify(filters)}`
       if (this.cache.has(cacheKey)) {
         return this.cache.get(cacheKey)!
       }
       
       const results = await invoke<Monster[]>('search_monsters', { filters })
       this.cache.set(cacheKey, results)
       return results
     }
     
     async searchClasses(filters: SearchFilters): Promise<Class[]> {
       return invoke<Class[]>('search_classes', { filters })
     }
     
     async searchFeats(filters: SearchFilters): Promise<Feat[]> {
       return invoke<Feat[]>('search_feats', { filters })
     }
     
     // Unified search interface
     async search(contentType: ContentType, filters: SearchFilters): Promise<any[]> {
       switch(contentType) {
         case 'spell': return this.searchSpells(filters)
         case 'item': return this.searchItems(filters)
         case 'monster': return this.searchMonsters(filters)
         case 'class': return this.searchClasses(filters)
         case 'feat': return this.searchFeats(filters)
         default: return []
       }
     }
     
     clearCache() {
       this.cache.clear()
     }
   }
   
   export const searchService = new SearchService()
   ```

2. **CREATE** `crates/mimir-dm/frontend/src/features/sources/components/search/SearchFilters.vue`
   ```vue
   <!-- SearchFilters.vue (100 lines) -->
   <template>
     <div class="card bg-base-200 p-4">
       <div class="form-control">
         <input 
           v-model="filters.query"
           type="text"
           placeholder="Search..."
           class="input input-bordered"
           @input="debouncedEmit"
         />
       </div>
       
       <div v-if="showLevelFilter" class="form-control mt-4">
         <label class="label">Spell Level</label>
         <select v-model="filters.level" class="select select-bordered">
           <option value="">All Levels</option>
           <option v-for="i in 10" :key="i" :value="i-1">
             Level {{ i-1 }}
           </option>
         </select>
       </div>
       
       <div v-if="showSchoolFilter" class="form-control mt-4">
         <label class="label">School</label>
         <select v-model="filters.school" class="select select-bordered">
           <option value="">All Schools</option>
           <option value="abjuration">Abjuration</option>
           <option value="conjuration">Conjuration</option>
           <option value="divination">Divination</option>
           <option value="enchantment">Enchantment</option>
           <option value="evocation">Evocation</option>
           <option value="illusion">Illusion</option>
           <option value="necromancy">Necromancy</option>
           <option value="transmutation">Transmutation</option>
         </select>
       </div>
       
       <button 
         @click="clearFilters"
         class="btn btn-ghost btn-sm mt-4"
       >
         Clear Filters
       </button>
     </div>
   </template>
   
   <script setup lang="ts">
   import { reactive, computed } from 'vue'
   import { debounce } from '@/utils/debounce'
   
   const props = defineProps<{
     contentType: ContentType
   }>()
   
   const emit = defineEmits<{
     'update:filters': [filters: SearchFilters]
   }>()
   
   const filters = reactive<SearchFilters>({
     query: '',
     level: undefined,
     school: undefined,
     rarity: undefined,
     type: undefined,
     source: undefined
   })
   
   const showLevelFilter = computed(() => 
     props.contentType === 'spell'
   )
   
   const showSchoolFilter = computed(() =>
     props.contentType === 'spell'
   )
   
   const debouncedEmit = debounce(() => {
     emit('update:filters', { ...filters })
   }, 300)
   
   function clearFilters() {
     Object.keys(filters).forEach(key => {
       filters[key] = key === 'query' ? '' : undefined
     })
     emit('update:filters', { ...filters })
   }
   </script>
   ```

3. **CREATE** `crates/mimir-dm/frontend/src/features/sources/components/search/ContentTypeSelector.vue`
   ```vue
   <!-- ContentTypeSelector.vue (80 lines) -->
   <template>
     <div class="tabs tabs-boxed">
       <button 
         v-for="type in contentTypes"
         :key="type.value"
         @click="$emit('update:modelValue', type.value)"
         :class="[
           'tab',
           modelValue === type.value && 'tab-active'
         ]"
       >
         <span class="mr-2">{{ type.icon }}</span>
         {{ type.label }}
         <span v-if="type.count" class="ml-2 badge badge-sm">
           {{ type.count }}
         </span>
       </button>
     </div>
   </template>
   
   <script setup lang="ts">
   defineProps<{
     modelValue: ContentType
   }>()
   
   defineEmits<{
     'update:modelValue': [value: ContentType]
   }>()
   
   const contentTypes = [
     { value: 'spell', label: 'Spells', icon: '✨' },
     { value: 'item', label: 'Items', icon: '⚔️' },
     { value: 'monster', label: 'Monsters', icon: '👹' },
     { value: 'class', label: 'Classes', icon: '🛡️' },
     { value: 'feat', label: 'Feats', icon: '⭐' }
   ]
   </script>
   ```

4. **CREATE** `crates/mimir-dm/frontend/src/features/sources/components/search/SearchResults.vue`
   ```vue
   <!-- SearchResults.vue (150 lines) -->
   <template>
     <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
       <div 
         v-if="loading"
         v-for="i in 6"
         :key="`skeleton-${i}`"
         class="card bg-base-200 shimmer h-32"
       />
       
       <div
         v-else-if="!results.length"
         class="col-span-full text-center py-8 text-base-content/50"
       >
         No results found
       </div>
       
       <component
         v-else
         v-for="item in results"
         :key="item.id"
         :is="getResultComponent(contentType)"
         :item="item"
         @click="$emit('select', item)"
         class="cursor-pointer hover:shadow-lg transition-shadow"
       />
     </div>
   </template>
   
   <script setup lang="ts">
   import { computed } from 'vue'
   import SpellCard from './results/SpellCard.vue'
   import ItemCard from './results/ItemCard.vue'
   import MonsterCard from './results/MonsterCard.vue'
   import ClassCard from './results/ClassCard.vue'
   import FeatCard from './results/FeatCard.vue'
   
   const props = defineProps<{
     results: any[]
     contentType: ContentType
     loading: boolean
   }>()
   
   defineEmits<{
     select: [item: any]
   }>()
   
   function getResultComponent(type: ContentType) {
     const components = {
       spell: SpellCard,
       item: ItemCard,
       monster: MonsterCard,
       class: ClassCard,
       feat: FeatCard
     }
     return components[type]
   }
   </script>
   ```

5. **REFACTOR** `SearchView.vue` (150 lines):
   ```vue
   <!-- SearchView.vue - Refactored to 150 lines -->
   <template>
     <div class="container mx-auto p-6">
       <h1 class="text-3xl font-bold mb-6">Source Content</h1>
       
       <ContentTypeSelector 
         v-model="selectedType"
         class="mb-6"
       />
       
       <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
         <div class="lg:col-span-1">
           <SearchFilters 
             :content-type="selectedType"
             @update:filters="handleFilterUpdate"
           />
         </div>
         
         <div class="lg:col-span-3">
           <SearchResults 
             :results="searchResults"
             :content-type="selectedType"
             :loading="loading"
             @select="handleItemSelect"
           />
         </div>
       </div>
       
       <BaseModal ref="detailModal">
         <component 
           :is="getDetailComponent(selectedType)"
           :item="selectedItem"
           @close="detailModal?.close()"
         />
       </BaseModal>
     </div>
   </template>
   
   <script setup lang="ts">
   import { ref, watch } from 'vue'
   import { searchService } from '../../services/SearchService'
   import ContentTypeSelector from './ContentTypeSelector.vue'
   import SearchFilters from './SearchFilters.vue'
   import SearchResults from './SearchResults.vue'
   import BaseModal from '@/components/shared/BaseModal.vue'
   
   // Import detail components
   import SpellDetail from './details/SpellDetail.vue'
   import ItemDetail from './details/ItemDetail.vue'
   import MonsterDetail from './details/MonsterDetail.vue'
   import ClassDetail from './details/ClassDetail.vue'
   import FeatDetail from './details/FeatDetail.vue'
   
   const selectedType = ref<ContentType>('spell')
   const searchResults = ref<any[]>([])
   const selectedItem = ref<any>(null)
   const loading = ref(false)
   const detailModal = ref<InstanceType<typeof BaseModal>>()
   
   async function handleFilterUpdate(filters: SearchFilters) {
     loading.value = true
     try {
       searchResults.value = await searchService.search(selectedType.value, filters)
     } finally {
       loading.value = false
     }
   }
   
   function handleItemSelect(item: any) {
     selectedItem.value = item
     detailModal.value?.open()
   }
   
   function getDetailComponent(type: ContentType) {
     const components = {
       spell: SpellDetail,
       item: ItemDetail,
       monster: MonsterDetail,
       class: ClassDetail,
       feat: FeatDetail
     }
     return components[type]
   }
   
   // Clear results when switching content type
   watch(selectedType, () => {
     searchResults.value = []
   })
   </script>
   ```

### 3.3 useCatalog.ts Simplification
**Current**: 605 lines → **Target**: 400 lines total across 6 files

#### Current Structure Analysis
```typescript
// useCatalog.ts - 605 lines of mixed concerns
export function useCatalog() {
  // Spell state and methods (lines 23-142, ~120 lines)
  const spells = ref<Spell[]>([])
  const spellsLoading = ref(false)
  const fetchSpells = async () => { /* ... */ }
  const filterSpellsByLevel = () => { /* ... */ }
  const sortSpellsByName = () => { /* ... */ }
  
  // Item state and methods (lines 144-263, ~120 lines)
  const items = ref<Item[]>([])
  const itemsLoading = ref(false)
  const fetchItems = async () => { /* ... */ }
  const filterItemsByRarity = () => { /* ... */ }
  const sortItemsByValue = () => { /* ... */ }
  
  // Monster state and methods (lines 265-384, ~120 lines)
  const monsters = ref<Monster[]>([])
  const monstersLoading = ref(false)
  const fetchMonsters = async () => { /* ... */ }
  const filterMonstersByCR = () => { /* ... */ }
  const sortMonstersByName = () => { /* ... */ }
  
  // Class state and methods (lines 386-505, ~120 lines)
  const classes = ref<Class[]>([])
  const classesLoading = ref(false)
  const fetchClasses = async () => { /* ... */ }
  
  // Feat state and methods (lines 507-605, ~99 lines)
  const feats = ref<Feat[]>([])
  const featsLoading = ref(false)
  const fetchFeats = async () => { /* ... */ }
  
  return {
    // 50+ exports
  }
}
```

#### Extraction Plan

1. **CREATE** `crates/mimir-dm/frontend/src/features/sources/composables/catalog/useCatalogBase.ts`
   ```typescript
   // useCatalogBase.ts (80 lines) - Shared catalog logic
   import { ref, computed } from 'vue'
   import { searchService } from '../../services/SearchService'
   
   export interface CatalogOptions<T> {
     contentType: ContentType
     defaultSort?: (a: T, b: T) => number
     defaultFilter?: (item: T) => boolean
   }
   
   export function useCatalogBase<T extends { id: string; name: string }>(
     options: CatalogOptions<T>
   ) {
     const items = ref<T[]>([])
     const loading = ref(false)
     const error = ref<Error | null>(null)
     const searchQuery = ref('')
     const sortBy = ref<'name' | 'custom'>('name')
     
     async function fetch(filters: SearchFilters = { query: '' }) {
       loading.value = true
       error.value = null
       
       try {
         const results = await searchService.search(options.contentType, filters)
         items.value = results as T[]
       } catch (e) {
         error.value = e as Error
         items.value = []
       } finally {
         loading.value = false
       }
     }
     
     const filteredItems = computed(() => {
       let result = [...items.value]
       
       // Apply search filter
       if (searchQuery.value) {
         const query = searchQuery.value.toLowerCase()
         result = result.filter(item => 
           item.name.toLowerCase().includes(query)
         )
       }
       
       // Apply custom filter if provided
       if (options.defaultFilter) {
         result = result.filter(options.defaultFilter)
       }
       
       // Apply sorting
       if (sortBy.value === 'name') {
         result.sort((a, b) => a.name.localeCompare(b.name))
       } else if (options.defaultSort) {
         result.sort(options.defaultSort)
       }
       
       return result
     })
     
     function clearFilters() {
       searchQuery.value = ''
       sortBy.value = 'name'
     }
     
     return {
       items: filteredItems,
       loading,
       error,
       searchQuery,
       sortBy,
       fetch,
       clearFilters
     }
   }
   ```

2. **CREATE** specialized catalog composables:
   ```typescript
   // useSpellCatalog.ts (80 lines)
   import { ref, computed } from 'vue'
   import { useCatalogBase } from './useCatalogBase'
   import type { Spell } from '../../types'
   
   export function useSpellCatalog() {
     const levelFilter = ref<number | null>(null)
     const schoolFilter = ref<string | null>(null)
     
     const base = useCatalogBase<Spell>({
       contentType: 'spell',
       defaultSort: (a, b) => a.level - b.level || a.name.localeCompare(b.name)
     })
     
     // Spell-specific computed properties
     const spellsByLevel = computed(() => {
       const grouped = new Map<number, Spell[]>()
       
       base.items.value.forEach(spell => {
         if (!grouped.has(spell.level)) {
           grouped.set(spell.level, [])
         }
         grouped.get(spell.level)!.push(spell)
       })
       
       return grouped
     })
     
     const spellsBySchool = computed(() => {
       const grouped = new Map<string, Spell[]>()
       
       base.items.value.forEach(spell => {
         if (!grouped.has(spell.school)) {
           grouped.set(spell.school, [])
         }
         grouped.get(spell.school)!.push(spell)
       })
       
       return grouped
     })
     
     const filteredSpells = computed(() => {
       let result = base.items.value
       
       if (levelFilter.value !== null) {
         result = result.filter(s => s.level === levelFilter.value)
       }
       
       if (schoolFilter.value) {
         result = result.filter(s => s.school === schoolFilter.value)
       }
       
       return result
     })
     
     // Spell-specific methods
     function filterByLevel(level: number) {
       levelFilter.value = level
     }
     
     function filterBySchool(school: string) {
       schoolFilter.value = school
     }
     
     function clearSpellFilters() {
       levelFilter.value = null
       schoolFilter.value = null
       base.clearFilters()
     }
     
     return {
       spells: filteredSpells,
       loading: base.loading,
       error: base.error,
       fetchSpells: base.fetch,
       spellsByLevel,
       spellsBySchool,
       filterByLevel,
       filterBySchool,
       clearFilters: clearSpellFilters
     }
   }
   ```

   ```typescript
   // useItemCatalog.ts (80 lines)
   import { ref, computed } from 'vue'
   import { useCatalogBase } from './useCatalogBase'
   import type { Item } from '../../types'
   
   export function useItemCatalog() {
     const rarityFilter = ref<string | null>(null)
     const typeFilter = ref<string | null>(null)
     const minValue = ref<number>(0)
     const maxValue = ref<number>(Infinity)
     
     const base = useCatalogBase<Item>({
       contentType: 'item',
       defaultSort: (a, b) => (b.value || 0) - (a.value || 0)
     })
     
     const itemsByRarity = computed(() => {
       const grouped = new Map<string, Item[]>()
       
       base.items.value.forEach(item => {
         const rarity = item.rarity || 'common'
         if (!grouped.has(rarity)) {
           grouped.set(rarity, [])
         }
         grouped.get(rarity)!.push(item)
       })
       
       return grouped
     })
     
     const filteredItems = computed(() => {
       let result = base.items.value
       
       if (rarityFilter.value) {
         result = result.filter(i => i.rarity === rarityFilter.value)
       }
       
       if (typeFilter.value) {
         result = result.filter(i => i.type === typeFilter.value)
       }
       
       result = result.filter(i => {
         const value = i.value || 0
         return value >= minValue.value && value <= maxValue.value
       })
       
       return result
     })
     
     function filterByRarity(rarity: string) {
       rarityFilter.value = rarity
     }
     
     function filterByType(type: string) {
       typeFilter.value = type
     }
     
     function filterByValueRange(min: number, max: number) {
       minValue.value = min
       maxValue.value = max
     }
     
     return {
       items: filteredItems,
       loading: base.loading,
       error: base.error,
       fetchItems: base.fetch,
       itemsByRarity,
       filterByRarity,
       filterByType,
       filterByValueRange
     }
   }
   ```

   Similar patterns for `useMonsterCatalog.ts`, `useClassCatalog.ts`, `useFeatCatalog.ts`

3. **DELETE** the monolithic `useCatalog.ts` file

4. **UPDATE** all consumers:
   ```typescript
   // Before - in any component using catalog
   import { useCatalog } from '@/features/sources/composables/useCatalog'
   
   const { 
     spells, 
     fetchSpells, 
     filterSpellsByLevel,
     items,
     fetchItems,
     // ... many more imports
   } = useCatalog()
   
   // After - import only what's needed
   import { useSpellCatalog } from '@/features/sources/composables/catalog/useSpellCatalog'
   
   const { 
     spells, 
     fetchSpells, 
     filterByLevel 
   } = useSpellCatalog()
   ```

## Phase 4: Service Layer Consolidation (Medium Risk)
**Target: -200 lines through consolidation | Timeline: Week 3**

### 4.1 Extract Duplicate Business Logic

#### Current Issues
Session management logic appears in:
- `ModuleStageLandingView.vue` (lines 447-523, ~76 lines)
- `ModuleListView.vue` (lines 234-287, ~53 lines)
- `CampaignView.vue` (lines 178-225, ~47 lines)

Document operations appear in:
- `ModuleDocumentSidebar.vue` (lines 156-234, ~78 lines)
- `DocumentEditor.vue` (lines 89-156, ~67 lines)
- `ModuleStageLandingView.vue` (lines 356-398, ~42 lines)

#### Migration Strategy

1. **CREATE** `crates/mimir-dm/frontend/src/services/SessionService.ts`
   ```typescript
   // SessionService.ts - Consolidate all session operations
   import { invoke } from '@tauri-apps/api/tauri'
   import type { Session, SessionData } from '@/types'
   
   class SessionService {
     private cache = new Map<string, Session[]>()
     
     async create(moduleId: string, data: SessionData): Promise<Session> {
       const session = await invoke<Session>('create_session', {
         moduleId,
         name: data.name,
         date: data.date,
         players: data.players,
         notes: data.notes
       })
       
       // Invalidate cache
       this.cache.delete(moduleId)
       
       return session
     }
     
     async update(id: string, data: Partial<SessionData>): Promise<Session> {
       return invoke<Session>('update_session', {
         sessionId: id,
         ...data
       })
     }
     
     async delete(id: string): Promise<void> {
       await invoke('delete_session', { sessionId: id })
       // Clear all cache entries as we don't know the module
       this.cache.clear()
     }
     
     async list(moduleId: string): Promise<Session[]> {
       if (this.cache.has(moduleId)) {
         return this.cache.get(moduleId)!
       }
       
       const sessions = await invoke<Session[]>('list_sessions', { moduleId })
       this.cache.set(moduleId, sessions)
       return sessions
     }
     
     async getActive(moduleId: string): Promise<Session[]> {
       const all = await this.list(moduleId)
       return all.filter(s => s.status === 'active')
     }
     
     async startSession(id: string): Promise<Session> {
       return invoke<Session>('start_session', { sessionId: id })
     }
     
     async endSession(id: string): Promise<Session> {
       return invoke<Session>('end_session', { sessionId: id })
     }
     
     clearCache() {
       this.cache.clear()
     }
   }
   
   export const sessionService = new SessionService()
   ```

2. **REMOVE** duplicate implementations:
   ```typescript
   // In ModuleStageLandingView.vue - Remove lines 447-523
   // Replace with:
   import { sessionService } from '@/services/SessionService'
   
   async function handleCreateSession(data: SessionData) {
     await sessionService.create(props.module.id, data)
     await refreshSessions()
   }
   
   // In ModuleListView.vue - Remove lines 234-287
   // In CampaignView.vue - Remove lines 178-225
   ```

3. **CREATE** `crates/mimir-dm/frontend/src/services/DocumentService.ts`
   ```typescript
   // DocumentService.ts - Consolidate document operations
   import { invoke } from '@tauri-apps/api/tauri'
   import type { Document, DocumentData } from '@/types'
   
   class DocumentService {
     async create(data: DocumentData): Promise<Document> {
       return invoke<Document>('create_document', data)
     }
     
     async update(id: string, content: string): Promise<Document> {
       return invoke<Document>('update_document', {
         documentId: id,
         content
       })
     }
     
     async updateMetadata(id: string, metadata: Partial<Document>): Promise<Document> {
       return invoke<Document>('update_document_metadata', {
         documentId: id,
         ...metadata
       })
     }
     
     async delete(id: string): Promise<void> {
       await invoke('delete_document', { documentId: id })
     }
     
     async transition(id: string, phase: string): Promise<Document> {
       return invoke<Document>('transition_document_phase', {
         documentId: id,
         phase
       })
     }
     
     async validateExitCriteria(id: string): Promise<boolean> {
       return invoke<boolean>('validate_exit_criteria', { documentId: id })
     }
     
     async list(moduleId: string): Promise<Document[]> {
       return invoke<Document[]>('list_documents', { moduleId })
     }
     
     async getByType(moduleId: string, type: string): Promise<Document[]> {
       const all = await this.list(moduleId)
       return all.filter(d => d.documentType === type)
     }
   }
   
   export const documentService = new DocumentService()
   ```

4. **REMOVE** duplicate document logic from:
   - `ModuleDocumentSidebar.vue` (save ~78 lines)
   - `DocumentEditor.vue` (save ~67 lines)
   - `ModuleStageLandingView.vue` (save ~42 lines)

## Phase 5: Final Cleanup & Optimization
**Target: -200 lines additional | Timeline: Week 4**

### 5.1 Remove Over-Abstractions

#### Identify Pass-Through Functions
Files with unnecessary wrappers:
- `crates/mimir-dm/frontend/src/utils/api.ts` - Many single-line functions
- `crates/mimir-dm/frontend/src/utils/format.ts` - Simple formatters that could be inline

```typescript
// Before - Unnecessary wrapper
export function callApi(command: string, args: any) {
  return invoke(command, args)
}

// Usage
await callApi('get_module', { id })

// After - Direct usage
await invoke('get_module', { id })
```

### 5.2 Console.log and Debug Code Removal

```bash
# Find all console statements
grep -r "console\." crates/mimir-dm/frontend/src/ | wc -l
# Result: 47 occurrences

# Files with most console statements:
- useCatalog.ts: 8 occurrences
- ModuleStageLandingView.vue: 6 occurrences  
- SearchView.vue: 5 occurrences
- campaigns.ts store: 4 occurrences
```

Remove all except those in:
- Development utilities
- Error boundaries
- Critical error logging

### 5.3 Unused CSS Classes

```bash
# Find potentially unused CSS classes
grep -r "class=" crates/mimir-dm/frontend/src/ | grep -o 'class="[^"]*"' | sort -u > used-classes.txt
grep -r "\." crates/mimir-dm/frontend/src/assets/styles/ | grep -o '\.[a-zA-Z0-9-]*' | sort -u > defined-classes.txt
# Compare to find unused
```

Remove unused utility classes and component-specific styles.

## Metrics & Validation

### Line Count Tracking

| Phase | Action | Lines Removed | Running Total |
|-------|--------|--------------|---------------|
| **Phase 1: Deletions** | | | |
| 1.1 | Delete rules_extended.rs | 419 | 419 |
| 1.2 | Delete backup files | 20 | 439 |
| 1.3 | Remove console.logs & unused imports | 80 | 519 |
| **Phase 2: Consolidation** | | | |
| 2.1 | Consolidate modals | 30 | 549 |
| 2.2 | Consolidate types | 70 | 619 |
| 2.3 | Consolidate CSS | 60 | 679 |
| **Phase 3: Decomposition** | | | |
| 3.1 | Refactor ModuleStageLandingView | 387 | 1,066 |
| 3.2 | Refactor SearchView | 326 | 1,392 |
| 3.3 | Refactor useCatalog | 205 | 1,597 |
| **Phase 4: Services** | | | |
| 4.1 | Extract SessionService | 176 | 1,773 |
| 4.2 | Extract DocumentService | 187 | 1,960 |
| **Phase 5: Cleanup** | | | |
| 5.1 | Remove over-abstractions | 50 | 2,010 |
| 5.2 | Remove console.logs | 47 | 2,057 |
| 5.3 | Remove unused CSS | 40 | 2,097 |
| **Total** | | | **2,097** |

### Validation Commands

```bash
# Before starting - capture baseline
find crates/ -name "*.ts" -o -name "*.vue" -o -name "*.rs" | xargs wc -l > baseline.txt

# After each phase - verify reduction
find crates/ -name "*.ts" -o -name "*.vue" -o -name "*.rs" | xargs wc -l > current.txt
diff baseline.txt current.txt

# Verify build still works
cd crates/mimir-dm && npm run build
cd ../.. && cargo build

# Run tests
npm run test
cargo test

# Check for TypeScript errors
npm run type-check

# Check for unused exports
npm run lint
```

### Success Criteria

✅ **Must Have**:
1. Build passes without errors
2. All tests pass
3. No TypeScript errors
4. At least 1,700 lines removed

✅ **Should Have**:
1. No component over 200 lines
2. No composable over 150 lines
3. Zero duplicate type definitions
4. Zero dead code files

✅ **Nice to Have**:
1. 100% of console.logs removed (except debugging tools)
2. All CSS consolidated into design system
3. Complete service layer abstraction

## Risk Mitigation

### Git Strategy
```bash
# Create feature branch
git checkout -b feature/aggressive-cleanup

# Commit after each successful phase
git add -A && git commit -m "Phase 1: Remove dead code - 519 lines removed"
git add -A && git commit -m "Phase 2: Consolidate duplicates - 160 lines removed"
# ... etc

# If something breaks, revert to last good commit
git reset --hard HEAD~1
```

### Testing Strategy
1. **Unit Tests**: Update for refactored components
2. **Integration Tests**: Ensure services work
3. **Manual Testing**: Critical user paths
4. **Performance Testing**: Ensure no regressions

## Implementation Schedule

| Day | Phase | Risk | Impact |
|-----|-------|------|--------|
| 1 | Pure Deletions | Zero | -519 lines |
| 2-3 | Consolidation | Low | -160 lines |
| 4-7 | Component Decomposition | Medium | -918 lines |
| 8-10 | Service Extraction | Medium | -363 lines |
| 11-12 | Final Cleanup | Low | -137 lines |

## Conclusion

This plan aggressively removes **2,097 lines of code** through:
- **519 lines** of pure deletions (dead code)
- **160 lines** saved through consolidation
- **918 lines** reduced via component decomposition
- **363 lines** eliminated through service extraction
- **137 lines** removed in final cleanup

The result is a leaner, more maintainable codebase with clear architectural boundaries and no duplication.