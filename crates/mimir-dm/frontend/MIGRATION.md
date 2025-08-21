# Frontend Reorganization Migration

## Goal
Unify "book" and "catalog" concepts into a single "sources" feature where all D&D content is treated as source material with different access patterns (reading, searching, browsing).

## Migration Status

### ✅ Phase 1: Directory Structure Created
- Created `features/sources/` structure
- Created `shared/` structure  
- Created `app/` and `config/` directories

### 🔄 Phase 2: Shared Components Migration

#### Layout Components
- [x] `components/layout/ThreePanelLayout.vue` → `shared/components/layout/` (copied)
- [x] `components/layout/TwoPanelLayout.vue` → `shared/components/layout/` (copied)
- [x] `components/layout/Panel.vue` → `shared/components/layout/` (copied)
- [ ] `components/layout/AppHeader.vue` → Keep in app-level
- [ ] `components/layout/MainLayout.vue` → Keep in app-level

#### UI Components  
- [x] `components/ui/LoadingSpinner.vue` → `shared/components/ui/` (copied)
- [ ] `components/ui/WelcomeMessage.vue` → Determine if needed

### 📋 Phase 3: Catalog → Sources/Search Migration (Pending)

#### Components to Migrate
- [ ] `components/catalog/CatalogPanel.vue` → `features/sources/components/search/SearchPanel.vue`
- [ ] `components/catalog/CatalogModal.vue` → `features/sources/components/content/ReferenceModal.vue`
- [ ] `components/catalog/tables/SpellTable.vue` → `features/sources/components/search/results/SpellResults.vue`
- [ ] `components/catalog/tables/ItemTable.vue` → `features/sources/components/search/results/ItemResults.vue`
- [ ] `components/catalog/tables/MonsterTable.vue` → `features/sources/components/search/results/MonsterResults.vue`

#### Composables to Migrate
- [ ] `composables/catalog/useCatalog.ts` → `features/sources/composables/useSourceSearch.ts`

#### Utilities to Migrate
- [ ] `utils/catalog/spellFormatterEnhanced.ts` → `features/sources/formatters/spellFormatter.ts`
- [ ] `utils/catalog/itemFormatterEnhanced.ts` → `features/sources/formatters/itemFormatter.ts`
- [ ] `utils/catalog/monsterFormatterEnhanced.ts` → `features/sources/formatters/monsterFormatter.ts`

### 📋 Phase 4: Book → Sources/Reader Migration (Pending)

#### Components to Migrate
- [ ] `components/book/BookReader.vue` → `features/sources/components/reader/ReaderView.vue`
- [ ] `components/book/BookTableOfContents.vue` → `features/sources/components/reader/TableOfContents.vue`
- [ ] `components/book/BookContentViewer.vue` → `features/sources/components/reader/ChapterContent.vue`
- [ ] `components/book/BookLibrary.vue` → `features/sources/components/library/SourceLibrary.vue`

### 📋 Phase 5: Create Unified Components (Pending)
- [ ] Create `ContentRenderer.vue` that can display any content type
- [ ] Create unified search interface
- [ ] Create source management interface

### 📋 Phase 6: Update Routes and Imports (Pending)
- [ ] Update Vue Router paths
- [ ] Update all import statements
- [ ] Update Tauri command calls if needed

### 📋 Phase 7: Cleanup (Pending)
- [ ] Remove old directories after confirming everything works
- [ ] Remove duplicate code
- [ ] Update tests

## Notes
- Using COPY instead of MOVE initially to avoid breaking the app
- Will remove old files only after confirming new structure works
- Each phase should be tested before moving to the next