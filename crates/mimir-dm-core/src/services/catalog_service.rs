//! Catalog import service for D&D 5e content
//!
//! This service handles automatic import of catalog content from uploaded books
//! into the SQLite database for fast searching and filtering.

use crate::models::catalog::{NewCatalogSpell, Spell, SpellData, NewCatalogAction, Action, NewCatalogCondition, Condition, ConditionData, Disease, DiseaseData, NewCatalogLanguage, LanguageData, NewCatalogReward, Reward, RewardData, NewCatalogBackground, BackgroundData, NewCatalogFeat, FeatData, NewCatalogRace, RaceData, NewCatalogObject, ObjectData, NewCatalogTrap, TrapData, HazardData, TrapOrHazard};
use crate::models::catalog::cult::{NewCatalogCult, CultData, BoonData, Cult, Boon};
use crate::schema::{catalog_spells, catalog_actions, catalog_conditions, catalog_languages, catalog_rewards, catalog_backgrounds, catalog_feats, catalog_races, catalog_objects, catalog_traps, catalog_cults};
use diesel::prelude::*;
use std::fs;
use std::path::Path;
use tracing::{error, info, warn, debug};

pub struct CatalogService;

impl CatalogService {
    /// Import all spell data from an uploaded book directory
    pub fn import_spells_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing spells from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        let spell_files = Self::find_spell_files(book_dir)?;
        
        if spell_files.is_empty() {
            info!("No spell files found in book directory");
            return Ok(0);
        }
        
        info!("Found {} spell files to process", spell_files.len());
        
        for spell_file in spell_files {
            debug!("Processing spell file: {:?}", spell_file);
            
            match Self::import_spells_from_file(conn, &spell_file, source) {
                Ok(count) => {
                    info!("Imported {} spells from {:?}", count, spell_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import spells from {:?}: {}", spell_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }
        
        info!("Successfully imported {} total spells from {}", total_imported, source);
        Ok(total_imported)
    }
    
    /// Find all spell JSON files in a book directory
    fn find_spell_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut spell_files = Vec::new();
        
        // Common locations for spell data
        let search_dirs = vec![
            book_dir.join("data"),
            book_dir.join("spells"),  
            book_dir.join("book"),
            book_dir.to_path_buf(),
        ];
        
        for dir in search_dirs {
            if !dir.exists() {
                continue;
            }
            
            debug!("Searching for spell files in: {:?}", dir);
            
            // Look for spell-specific files and general book files
            let file_patterns = vec!["spells-*.json", "spell*.json", "book-*.json"];
            
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    
                    if path.is_file() {
                        let file_name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        
                        // Check if file matches any of our patterns
                        for pattern in &file_patterns {
                            if Self::matches_pattern(file_name, pattern) {
                                debug!("Found potential spell file: {:?}", path);
                                spell_files.push(path);
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        spell_files.sort();
        spell_files.dedup();
        Ok(spell_files)
    }
    
    /// Import spells from a single JSON file
    fn import_spells_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading spell file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
        
        let json_value: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON from {:?}: {}", file_path, e))?;
        
        let mut spells_to_import = Vec::new();
        
        // Try to parse as SpellData structure first (spells-*.json files)
        if let Ok(spell_data) = serde_json::from_value::<SpellData>(json_value.clone()) {
            debug!("Parsed as SpellData, found {} spells", spell_data.spell.len());
            
            for spell in spell_data.spell {
                spells_to_import.push(spell);
            }
        }
        // Try to extract spells from book structure (book-*.json files)
        else if let Some(spell_array) = json_value.get("spell").and_then(|v| v.as_array()) {
            debug!("Found spell array in book structure with {} entries", spell_array.len());
            
            for spell_value in spell_array {
                match serde_json::from_value::<Spell>(spell_value.clone()) {
                    Ok(spell) => spells_to_import.push(spell),
                    Err(e) => {
                        warn!("Failed to parse spell from book structure: {}", e);
                        continue;
                    }
                }
            }
        }
        // Check for nested data structure
        else if let Some(data_array) = json_value.get("data").and_then(|v| v.as_array()) {
            debug!("Checking nested data structure");
            
            for data_section in data_array {
                if let Some(spell_array) = data_section.get("spell").and_then(|v| v.as_array()) {
                    debug!("Found nested spell array with {} entries", spell_array.len());
                    
                    for spell_value in spell_array {
                        match serde_json::from_value::<Spell>(spell_value.clone()) {
                            Ok(spell) => spells_to_import.push(spell),
                            Err(e) => {
                                warn!("Failed to parse nested spell: {}", e);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        
        if spells_to_import.is_empty() {
            debug!("No spells found in file: {:?}", file_path);
            return Ok(0);
        }
        
        debug!("Processing {} spells for database import", spells_to_import.len());
        
        // Transform spells to database format
        let catalog_spells: Vec<NewCatalogSpell> = spells_to_import
            .into_iter()
            .map(|spell| NewCatalogSpell::from_spell(spell, source))
            .collect();
        
        // Batch insert spells
        Self::batch_insert_spells(conn, catalog_spells)
    }
    
    /// Batch insert spells into the database
    fn batch_insert_spells(
        conn: &mut SqliteConnection,
        spells: Vec<NewCatalogSpell>
    ) -> Result<usize, String> {
        if spells.is_empty() {
            return Ok(0);
        }
        
        debug!("Batch inserting {} spells into database", spells.len());
        
        // Use INSERT OR IGNORE to handle duplicates gracefully
        let inserted = diesel::insert_or_ignore_into(catalog_spells::table)
            .values(&spells)
            .execute(conn)
            .map_err(|e| format!("Failed to insert spells: {}", e))?;
        
        debug!("Successfully inserted {} spells (duplicates ignored)", inserted);
        Ok(inserted)
    }
    
    /// Remove all spells from a specific source
    pub fn remove_spells_by_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing all spells from source: {}", source);
        
        let deleted = diesel::delete(catalog_spells::table.filter(catalog_spells::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete spells from source {}: {}", source, e))?;
        
        info!("Removed {} spells from source: {}", deleted, source);
        Ok(deleted)
    }
    
    /// Import all action data from an uploaded book directory
    pub fn import_actions_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing actions from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        let action_files = Self::find_action_files(book_dir)?;
        
        if action_files.is_empty() {
            info!("No action files found in book directory");
            return Ok(0);
        }
        
        info!("Found {} action files to process", action_files.len());
        
        for action_file in action_files {
            debug!("Processing action file: {:?}", action_file);
            
            match Self::import_actions_from_file(conn, &action_file, source) {
                Ok(count) => {
                    info!("Imported {} actions from {:?}", count, action_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import actions from {:?}: {}", action_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }
        
        info!("Successfully imported {} total actions from {}", total_imported, source);
        Ok(total_imported)
    }

    /// Find action JSON files in a book directory
    fn find_action_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut action_files = Vec::new();
        
        let actions_dir = book_dir.join("actions");
        if !actions_dir.exists() || !actions_dir.is_dir() {
            debug!("No actions directory found in book: {:?}", book_dir);
            return Ok(action_files);
        }
        
        debug!("Found actions directory: {:?}", actions_dir);
        
        // Read all JSON files in the actions directory
        let entries = fs::read_dir(&actions_dir)
            .map_err(|e| format!("Failed to read actions directory: {}", e))?;
            
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            // Skip fluff files and non-JSON files
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with("fluff") || !filename.ends_with(".json") {
                    continue;
                }
            }

            if path.is_file() {
                debug!("Found action file: {:?}", path);
                action_files.push(path);
            }
        }
        
        debug!("Found {} action files in book directory", action_files.len());
        Ok(action_files)
    }

    /// Import actions from a specific JSON file
    fn import_actions_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path, 
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading action file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
            
        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON from {:?}: {}", file_path, e))?;
        
        let mut actions_to_import: Vec<Action> = Vec::new();
        
        // Handle different JSON structures
        if let Some(action_array) = json.get("action").and_then(|v| v.as_array()) {
            // Structure: { "action": [...] }
            for action_value in action_array {
                match serde_json::from_value::<Action>(action_value.clone()) {
                    Ok(mut action) => {
                        action.source = source.to_string();
                        actions_to_import.push(action);
                    }
                    Err(e) => {
                        warn!("Failed to parse action: {}", e);
                        continue;
                    }
                }
            }
        } else if json.is_array() {
            // Structure: [...]
            if let Some(action_array) = json.as_array() {
                for action_value in action_array {
                    match serde_json::from_value::<Action>(action_value.clone()) {
                        Ok(mut action) => {
                            action.source = source.to_string();
                            actions_to_import.push(action);
                        }
                        Err(e) => {
                            warn!("Failed to parse action: {}", e);
                            continue;
                        }
                    }
                }
            }
        } else if let Some(action_obj) = json.as_object() {
            // Look for nested action arrays in other keys
            for (key, value) in action_obj {
                if key.contains("action") || key == "data" {
                    if let Some(action_array) = value.as_array() {
                        for action_value in action_array {
                            match serde_json::from_value::<Action>(action_value.clone()) {
                                Ok(mut action) => {
                                    action.source = source.to_string();
                                    actions_to_import.push(action);
                                }
                                Err(e) => {
                                    warn!("Failed to parse nested action: {}", e);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if actions_to_import.is_empty() {
            debug!("No actions found in file: {:?}", file_path);
            return Ok(0);
        }
        
        debug!("Processing {} actions for database import", actions_to_import.len());
        
        // Transform actions to database format
        let catalog_actions: Vec<NewCatalogAction> = actions_to_import
            .into_iter()
            .map(NewCatalogAction::from)
            .collect();
        
        // Batch insert actions
        Self::batch_insert_actions(conn, catalog_actions)
    }
    
    /// Batch insert actions into the database
    fn batch_insert_actions(
        conn: &mut SqliteConnection,
        actions: Vec<NewCatalogAction>,
    ) -> Result<usize, String> {
        let mut total_inserted = 0;
        
        debug!("Inserting {} actions individually (SQLite limitation)", actions.len());
        
        for action in actions {
            let inserted = diesel::insert_into(catalog_actions::table)
                .values(&action)
                .on_conflict((catalog_actions::name, catalog_actions::source))
                .do_update()
                .set((
                    catalog_actions::time_type.eq(&action.time_type),
                    catalog_actions::description.eq(&action.description),
                    catalog_actions::see_also.eq(&action.see_also),
                    catalog_actions::full_action_json.eq(&action.full_action_json),
                ))
                .execute(conn)
                .map_err(|e| format!("Failed to insert action '{}': {}", action.name, e))?;
                
            total_inserted += inserted;
        }
        
        info!("Successfully imported {} actions into database", total_inserted);
        Ok(total_inserted)
    }

    /// Remove all actions from a specific source (for book removal)
    pub fn remove_actions_by_source(
        conn: &mut SqliteConnection,
        source: &str,
    ) -> Result<usize, String> {
        info!("Removing actions from source: {}", source);
        
        let deleted = diesel::delete(catalog_actions::table.filter(catalog_actions::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete actions from source {}: {}", source, e))?;
        
        info!("Removed {} actions from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Import all condition and disease data from an uploaded book directory
    pub fn import_conditions_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        info!("Importing conditions/diseases from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        
        // Import conditions from conditions/ subdirectory  
        if let Ok(count) = Self::import_conditions_from_subdirectory(conn, book_dir, "conditions", source) {
            total_imported += count;
        }
        
        // Import diseases from diseases/ subdirectory
        if let Ok(count) = Self::import_diseases_from_subdirectory(conn, book_dir, "diseases", source) {
            total_imported += count;
        }
        
        info!("Successfully imported {} total conditions/diseases from {}", total_imported, source);
        Ok(total_imported)
    }
    
    /// Import conditions from the conditions subdirectory
    fn import_conditions_from_subdirectory(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        subdir: &str,
        source: &str
    ) -> Result<usize, String> {
        let conditions_dir = book_dir.join(subdir);
        if !conditions_dir.exists() || !conditions_dir.is_dir() {
            debug!("No {} directory found in book: {:?}", subdir, book_dir);
            return Ok(0);
        }
        
        debug!("Found {} directory in book: {:?}", subdir, book_dir);
        
        let mut total_imported = 0;
        
        // Load all JSON files in the conditions directory
        if let Ok(entries) = fs::read_dir(&conditions_dir) {
            for entry in entries.flatten() {
                let condition_file = entry.path();
                
                if condition_file.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }
                
                let filename = condition_file.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                
                // Skip fluff files for now
                if filename.starts_with("fluff-") {
                    continue;
                }
                
                match Self::import_conditions_from_file(conn, &condition_file, source) {
                    Ok(count) => {
                        debug!("Imported {} conditions from {:?}", count, condition_file);
                        total_imported += count;
                    }
                    Err(e) => {
                        error!("Failed to import conditions from {:?}: {}", condition_file, e);
                    }
                }
            }
        }
        
        Ok(total_imported)
    }
    
    /// Import diseases from the diseases subdirectory
    fn import_diseases_from_subdirectory(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        subdir: &str,
        source: &str
    ) -> Result<usize, String> {
        let diseases_dir = book_dir.join(subdir);
        if !diseases_dir.exists() || !diseases_dir.is_dir() {
            debug!("No {} directory found in book: {:?}", subdir, book_dir);
            return Ok(0);
        }
        
        debug!("Found {} directory in book: {:?}", subdir, book_dir);
        
        let mut total_imported = 0;
        
        // Load all JSON files in the diseases directory
        if let Ok(entries) = fs::read_dir(&diseases_dir) {
            for entry in entries.flatten() {
                let disease_file = entry.path();
                
                if disease_file.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }
                
                match Self::import_diseases_from_file(conn, &disease_file, source) {
                    Ok(count) => {
                        debug!("Imported {} diseases from {:?}", count, disease_file);
                        total_imported += count;
                    }
                    Err(e) => {
                        error!("Failed to import diseases from {:?}: {}", disease_file, e);
                    }
                }
            }
        }
        
        Ok(total_imported)
    }
    
    /// Import conditions from a specific JSON file
    fn import_conditions_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading condition file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
        
        let condition_data: ConditionData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse ConditionData from {:?}: {}", file_path, e))?;
        
        let mut conditions_to_import: Vec<Condition> = Vec::new();
        
        if let Some(conditions) = condition_data.condition {
            if !conditions.is_empty() {
                debug!("Found {} conditions in file {:?}", conditions.len(), file_path);
                for mut condition in conditions {
                    // Ensure the source is set correctly
                    condition.source = source.to_string();
                    conditions_to_import.push(condition);
                }
            }
        }
        
        if conditions_to_import.is_empty() {
            debug!("No conditions found in file: {:?}", file_path);
            return Ok(0);
        }
        
        // Transform conditions to database format
        let catalog_conditions: Vec<NewCatalogCondition> = conditions_to_import
            .into_iter()
            .map(NewCatalogCondition::from)
            .collect();
        
        // Batch insert conditions
        Self::batch_insert_conditions(conn, catalog_conditions)
    }
    
    /// Import diseases from a specific JSON file
    fn import_diseases_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading disease file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
        
        let disease_data: DiseaseData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse DiseaseData from {:?}: {}", file_path, e))?;
        
        let mut diseases_to_import: Vec<Disease> = Vec::new();
        
        if let Some(diseases) = disease_data.disease {
            if !diseases.is_empty() {
                debug!("Found {} diseases in file {:?}", diseases.len(), file_path);
                for mut disease in diseases {
                    // Ensure the source is set correctly
                    disease.source = source.to_string();
                    diseases_to_import.push(disease);
                }
            }
        }
        
        if diseases_to_import.is_empty() {
            debug!("No diseases found in file: {:?}", file_path);
            return Ok(0);
        }
        
        // Transform diseases to database format
        let catalog_conditions: Vec<NewCatalogCondition> = diseases_to_import
            .into_iter()
            .map(NewCatalogCondition::from)
            .collect();
        
        // Batch insert diseases
        Self::batch_insert_conditions(conn, catalog_conditions)
    }
    
    /// Batch insert conditions/diseases into the database
    fn batch_insert_conditions(
        conn: &mut SqliteConnection,
        conditions: Vec<NewCatalogCondition>,
    ) -> Result<usize, String> {
        let mut total_inserted = 0;
        
        debug!("Inserting {} conditions individually (SQLite limitation)", conditions.len());
        
        for condition in conditions {
            let inserted = diesel::insert_into(catalog_conditions::table)
                .values(&condition)
                .on_conflict((catalog_conditions::name, catalog_conditions::source))
                .do_update()
                .set((
                    catalog_conditions::item_type.eq(&condition.item_type),
                    catalog_conditions::description.eq(&condition.description),
                    catalog_conditions::full_condition_json.eq(&condition.full_condition_json),
                ))
                .execute(conn)
                .map_err(|e| format!("Failed to insert condition '{}': {}", condition.name, e))?;
                
            total_inserted += inserted;
        }
        
        info!("Successfully imported {} conditions into database", total_inserted);
        Ok(total_inserted)
    }
    
    /// Remove all conditions from a specific source (for book removal)
    pub fn remove_conditions_by_source(
        conn: &mut SqliteConnection,
        source: &str,
    ) -> Result<usize, String> {
        info!("Removing conditions from source: {}", source);
        
        let deleted = diesel::delete(catalog_conditions::table.filter(catalog_conditions::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete conditions from source {}: {}", source, e))?;
        
        info!("Removed {} conditions from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Import all language data from an uploaded book directory  
    pub fn import_languages_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str,
    ) -> Result<usize, String> {
        info!("Importing languages from book directory: {:?} (source: {})", book_dir, source);
        
        let languages_dir = book_dir.join("languages");
        if !languages_dir.exists() || !languages_dir.is_dir() {
            debug!("No languages directory found in book: {:?}", book_dir);
            return Ok(0);
        }

        let mut imported_count = 0;

        // Read all JSON files in the languages directory
        let entries = fs::read_dir(&languages_dir)
            .map_err(|e| format!("Failed to read languages directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            // Skip fluff files and non-JSON files
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with("fluff") || !filename.ends_with(".json") {
                    continue;
                }
            }

            debug!("Processing language file: {:?}", path);
            
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;
                
            let language_data: LanguageData = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse language data from {:?}: {}", path, e))?;

            if let Some(languages) = language_data.language {
                let new_languages: Vec<NewCatalogLanguage> = languages
                    .into_iter()
                    .map(|mut lang| {
                        lang.source = source.to_string();
                        NewCatalogLanguage::from(lang)
                    })
                    .collect();

                if !new_languages.is_empty() {
                    let inserted = diesel::insert_into(catalog_languages::table)
                        .values(&new_languages)
                        .execute(conn)
                        .map_err(|e| format!("Failed to insert languages: {}", e))?;
                    
                    imported_count += inserted;
                    info!("Imported {} languages from {:?}", inserted, path);
                }
            }
        }

        info!("Successfully imported {} languages from source: {}", imported_count, source);
        Ok(imported_count)
    }

    /// Remove all languages from a specific source
    pub fn remove_languages_by_source(
        conn: &mut SqliteConnection,
        source: &str,
    ) -> Result<usize, String> {
        info!("Removing languages from source: {}", source);
        
        let deleted = diesel::delete(catalog_languages::table.filter(catalog_languages::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete languages from source {}: {}", source, e))?;
        
        info!("Removed {} languages from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Import all reward data from an uploaded book directory  
    pub fn import_rewards_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str,
    ) -> Result<usize, String> {
        info!("Importing rewards from book directory: {:?} (source: {})", book_dir, source);
        
        let rewards_dir = book_dir.join("rewards");
        if !rewards_dir.exists() || !rewards_dir.is_dir() {
            debug!("No rewards directory found in book: {:?}", book_dir);
            return Ok(0);
        }

        let mut imported_count = 0;

        // Read all JSON files in the rewards directory
        let entries = fs::read_dir(&rewards_dir)
            .map_err(|e| format!("Failed to read rewards directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            // Skip fluff files and non-JSON files
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with("fluff") || !filename.ends_with(".json") {
                    continue;
                }
            }

            debug!("Processing reward file: {:?}", path);
            
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;
                
            let reward_data: RewardData = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse reward data from {:?}: {}", path, e))?;

            if let Some(rewards) = reward_data.reward {
                let new_rewards: Vec<NewCatalogReward> = rewards
                    .into_iter()
                    .map(|mut reward| {
                        reward.source = source.to_string();
                        NewCatalogReward::from(reward)
                    })
                    .collect();

                if !new_rewards.is_empty() {
                    let inserted = diesel::insert_into(catalog_rewards::table)
                        .values(&new_rewards)
                        .execute(conn)
                        .map_err(|e| format!("Failed to insert rewards: {}", e))?;
                    
                    imported_count += inserted;
                    info!("Imported {} rewards from {:?}", inserted, path);
                }
            }
        }

        info!("Successfully imported {} rewards from source: {}", imported_count, source);
        Ok(imported_count)
    }

    /// Remove all rewards from a specific source
    pub fn remove_rewards_by_source(
        conn: &mut SqliteConnection,
        source: &str,
    ) -> Result<usize, String> {
        info!("Removing rewards from source: {}", source);
        
        let deleted = diesel::delete(catalog_rewards::table.filter(catalog_rewards::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete rewards from source {}: {}", source, e))?;
        
        info!("Removed {} rewards from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Import all background data from an uploaded book directory  
    pub fn import_backgrounds_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing backgrounds from book directory: {:?} (source: {})", book_dir, source);
        let mut imported_count = 0;
        
        let backgrounds_dir = book_dir.join("backgrounds");
        if !backgrounds_dir.exists() || !backgrounds_dir.is_dir() {
            debug!("No backgrounds directory found in book: {:?}", book_dir);
            return Ok(0);
        }
        
        info!("Found backgrounds directory: {:?}", backgrounds_dir);
        
        // Read all JSON files in the backgrounds directory
        let entries = fs::read_dir(&backgrounds_dir)
            .map_err(|e| format!("Failed to read backgrounds directory: {}", e))?;
            
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            
            debug!("Processing background file: {:?}", path);
            
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read background file {:?}: {}", path, e))?;
            
            let background_data: BackgroundData = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse background data from {:?}: {}", path, e))?;
            
            if let Some(backgrounds) = background_data.background {
                let new_backgrounds: Vec<NewCatalogBackground> = backgrounds
                    .into_iter()
                    .map(|mut background| {
                        background.source = source.to_string();
                        NewCatalogBackground::from(&background)
                    })
                    .collect();
                
                if !new_backgrounds.is_empty() {
                    let inserted = diesel::insert_into(catalog_backgrounds::table)
                        .values(&new_backgrounds)
                        .execute(conn)
                        .map_err(|e| format!("Failed to insert backgrounds: {}", e))?;
                    
                    imported_count += inserted;
                    info!("Imported {} backgrounds from {:?}", inserted, path);
                }
            }
        }
        
        info!("Successfully imported {} backgrounds from source: {}", imported_count, source);
        Ok(imported_count)
    }

    /// Import all feat data from an uploaded book directory  
    pub fn import_feats_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing feats from book directory: {:?} (source: {})", book_dir, source);
        let mut imported_count = 0;
        
        let feats_dir = book_dir.join("feats");
        if !feats_dir.exists() || !feats_dir.is_dir() {
            debug!("No feats directory found in book: {:?}", book_dir);
            return Ok(0);
        }
        
        info!("Found feats directory: {:?}", feats_dir);
        
        // Read all JSON files in the feats directory
        let entries = fs::read_dir(&feats_dir)
            .map_err(|e| format!("Failed to read feats directory: {}", e))?;
            
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            
            debug!("Processing feat file: {:?}", path);
            
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read feat file {:?}: {}", path, e))?;
            
            let feat_data: FeatData = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse feat data from {:?}: {}", path, e))?;
            
            if let Some(feats) = feat_data.feat {
                let new_feats: Vec<NewCatalogFeat> = feats
                    .into_iter()
                    .map(|mut feat| {
                        feat.source = source.to_string();
                        NewCatalogFeat::from(&feat)
                    })
                    .collect();
                
                if !new_feats.is_empty() {
                    let inserted = diesel::insert_into(catalog_feats::table)
                        .values(&new_feats)
                        .execute(conn)
                        .map_err(|e| format!("Failed to insert feats: {}", e))?;
                    
                    imported_count += inserted;
                    info!("Imported {} feats from {:?}", inserted, path);
                }
            }
        }
        
        info!("Successfully imported {} feats from source: {}", imported_count, source);
        Ok(imported_count)
    }

    /// Remove all feats from a specific source
    pub fn remove_feats_by_source(
        conn: &mut SqliteConnection, 
        source: &str
    ) -> Result<usize, String> {
        info!("Removing feats from source: {}", source);
        
        let deleted = diesel::delete(catalog_feats::table)
            .filter(catalog_feats::source.eq(source))
            .execute(conn)
            .map_err(|e| format!("Failed to delete feats: {}", e))?;
            
        info!("Removed {} feats from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Import all race data from an uploaded book directory  
    pub fn import_races_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing races from book directory: {:?} (source: {})", book_dir, source);
        let mut imported_count = 0;
        
        let races_dir = book_dir.join("races");
        if !races_dir.exists() || !races_dir.is_dir() {
            debug!("No races directory found in book: {:?}", book_dir);
            return Ok(0);
        }
        
        info!("Found races directory: {:?}", races_dir);
        
        // Read all JSON files in the races directory
        let entries = fs::read_dir(&races_dir)
            .map_err(|e| format!("Failed to read races directory: {}", e))?;
            
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
                
            // Skip fluff files - we only want race data
            if filename.starts_with("fluff-") {
                debug!("Skipping fluff file: {}", filename);
                continue;
            }
            
            debug!("Processing race file: {}", filename);
            
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;
                
            let race_data: RaceData = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse race file {}: {}", filename, e))?;
                
            // Import main races
            if let Some(races) = race_data.race {
                for race in &races {
                    let new_race = NewCatalogRace::from(race);
                    
                    match diesel::insert_into(catalog_races::table)
                        .values(&new_race)
                        .on_conflict((catalog_races::name, catalog_races::source))
                        .do_update()
                        .set((
                            catalog_races::size.eq(&new_race.size),
                            catalog_races::speed.eq(&new_race.speed),
                            catalog_races::ability_bonuses.eq(&new_race.ability_bonuses),
                            catalog_races::traits_count.eq(&new_race.traits_count),
                            catalog_races::full_race_json.eq(&new_race.full_race_json),
                        ))
                        .execute(conn) {
                        Ok(_) => {
                            imported_count += 1;
                            debug!("Imported race: {} ({})", race.name, source);
                        }
                        Err(e) => {
                            error!("Failed to insert race {}: {}", race.name, e);
                        }
                    }
                }
            }
            
            // Import subraces
            if let Some(subraces) = race_data.subrace {
                for subrace in &subraces {
                    // Skip unnamed subraces
                    if subrace.name.is_none() {
                        continue;
                    }
                    
                    let new_subrace = NewCatalogRace::from(subrace);
                    
                    match diesel::insert_into(catalog_races::table)
                        .values(&new_subrace)
                        .on_conflict((catalog_races::name, catalog_races::source))
                        .do_update()
                        .set((
                            catalog_races::size.eq(&new_subrace.size),
                            catalog_races::speed.eq(&new_subrace.speed),
                            catalog_races::ability_bonuses.eq(&new_subrace.ability_bonuses),
                            catalog_races::traits_count.eq(&new_subrace.traits_count),
                            catalog_races::full_race_json.eq(&new_subrace.full_race_json),
                        ))
                        .execute(conn) {
                        Ok(_) => {
                            imported_count += 1;
                            debug!("Imported subrace: {} ({})", new_subrace.name, source);
                        }
                        Err(e) => {
                            error!("Failed to insert subrace {}: {}", new_subrace.name, e);
                        }
                    }
                }
            }
        }
        
        info!("Successfully imported {} races/subraces from source: {}", imported_count, source);
        Ok(imported_count)
    }

    /// Remove all races from a specific source
    pub fn remove_races_by_source(
        conn: &mut SqliteConnection, 
        source: &str
    ) -> Result<usize, String> {
        info!("Removing races from source: {}", source);
        
        let deleted = diesel::delete(catalog_races::table)
            .filter(catalog_races::source.eq(source))
            .execute(conn)
            .map_err(|e| format!("Failed to delete races: {}", e))?;
            
        info!("Removed {} races from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Import all object data from an uploaded book directory  
    pub fn import_objects_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing objects from book directory: {:?} (source: {})", book_dir, source);
        let mut imported_count = 0;
        
        let objects_dir = book_dir.join("objects");
        if !objects_dir.exists() || !objects_dir.is_dir() {
            debug!("No objects directory found in book: {:?}", book_dir);
            return Ok(0);
        }
        
        info!("Found objects directory: {:?}", objects_dir);
        
        // Read all JSON files in the objects directory
        let entries = fs::read_dir(&objects_dir)
            .map_err(|e| format!("Failed to read objects directory: {}", e))?;
            
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
                
            debug!("Processing object file: {}", filename);
            
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;
                
            let object_data: ObjectData = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse object file {}: {}", filename, e))?;
                
            // Import objects
            if let Some(objects) = object_data.object {
                for obj in &objects {
                    let new_object = NewCatalogObject::from(obj);
                    
                    match diesel::insert_into(catalog_objects::table)
                        .values(&new_object)
                        .on_conflict((catalog_objects::name, catalog_objects::source))
                        .do_update()
                        .set((
                            catalog_objects::object_type.eq(&new_object.object_type),
                            catalog_objects::size.eq(&new_object.size),
                            catalog_objects::ac.eq(&new_object.ac),
                            catalog_objects::hp.eq(&new_object.hp),
                            catalog_objects::full_object_json.eq(&new_object.full_object_json),
                        ))
                        .execute(conn) {
                        Ok(_) => {
                            imported_count += 1;
                            debug!("Imported object: {} ({})", obj.name, source);
                        }
                        Err(e) => {
                            error!("Failed to insert object {}: {}", obj.name, e);
                        }
                    }
                }
            }
        }
        
        info!("Successfully imported {} objects from source: {}", imported_count, source);
        Ok(imported_count)
    }

    /// Remove all objects from a specific source
    pub fn remove_objects_by_source(
        conn: &mut SqliteConnection, 
        source: &str
    ) -> Result<usize, String> {
        info!("Removing objects from source: {}", source);
        
        let deleted = diesel::delete(catalog_objects::table)
            .filter(catalog_objects::source.eq(source))
            .execute(conn)
            .map_err(|e| format!("Failed to delete objects: {}", e))?;
            
        info!("Removed {} objects from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Remove all backgrounds from a specific source
    pub fn remove_backgrounds_by_source(
        conn: &mut SqliteConnection, 
        source: &str
    ) -> Result<usize, String> {
        info!("Removing backgrounds from source: {}", source);
        
        let deleted = diesel::delete(catalog_backgrounds::table.filter(catalog_backgrounds::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete backgrounds from source {}: {}", source, e))?;
        
        info!("Removed {} backgrounds from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Import all trap and hazard data from an uploaded book directory
    pub fn import_traps_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        info!("Importing traps from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        
        // Import traps
        let traps_dir = book_dir.join("traps");
        if traps_dir.exists() && traps_dir.is_dir() {
            info!("Found traps directory: {:?}", traps_dir);
            total_imported += Self::import_traps_from_directory(conn, &traps_dir, source)?;
        } else {
            debug!("No traps directory found in book: {:?}", book_dir);
        }
        
        // Import hazards
        let hazards_dir = book_dir.join("hazards");
        if hazards_dir.exists() && hazards_dir.is_dir() {
            info!("Found hazards directory: {:?}", hazards_dir);
            total_imported += Self::import_hazards_from_directory(conn, &hazards_dir, source)?;
        } else {
            debug!("No hazards directory found in book: {:?}", book_dir);
        }
        
        info!("Successfully imported {} total traps/hazards from {}", total_imported, source);
        Ok(total_imported)
    }

    /// Import traps from a directory
    fn import_traps_from_directory(
        conn: &mut SqliteConnection,
        traps_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        let mut total_imported = 0;
        
        for entry in fs::read_dir(traps_dir).map_err(|e| format!("Failed to read traps directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let file_path = entry.path();
            
            if !file_path.is_file() || file_path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            
            debug!("Processing trap file: {:?}", file_path.file_name().unwrap_or_default());
            
            match Self::import_traps_from_file(conn, &file_path, source) {
                Ok(count) => {
                    info!("Imported {} traps from {:?}", count, file_path);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import traps from {:?}: {}", file_path, e);
                }
            }
        }
        
        Ok(total_imported)
    }

    /// Import hazards from a directory
    fn import_hazards_from_directory(
        conn: &mut SqliteConnection,
        hazards_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        let mut total_imported = 0;
        
        for entry in fs::read_dir(hazards_dir).map_err(|e| format!("Failed to read hazards directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let file_path = entry.path();
            
            if !file_path.is_file() || file_path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            
            debug!("Processing hazard file: {:?}", file_path.file_name().unwrap_or_default());
            
            match Self::import_hazards_from_file(conn, &file_path, source) {
                Ok(count) => {
                    info!("Imported {} hazards from {:?}", count, file_path);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import hazards from {:?}: {}", file_path, e);
                }
            }
        }
        
        Ok(total_imported)
    }

    /// Import traps from a single JSON file
    fn import_traps_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading trap file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read trap file: {}", e))?;
            
        let trap_data: TrapData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse trap JSON: {}", e))?;
            
        if let Some(traps) = trap_data.trap {
            let traps: Vec<TrapOrHazard> = traps.into_iter().map(TrapOrHazard::Trap).collect();
            let new_traps: Vec<NewCatalogTrap> = traps.iter().map(|trap| NewCatalogTrap::from(trap)).collect();
            
            debug!("Inserting {} traps individually (SQLite limitation)", new_traps.len());
            
            for trap in &new_traps {
                let result = diesel::insert_into(catalog_traps::table)
                    .values(trap)
                    .on_conflict((catalog_traps::name, catalog_traps::source))
                    .do_nothing()
                    .execute(conn);
                    
                if let Err(e) = result {
                    warn!("Failed to insert trap {}: {}", trap.name, e);
                }
            }
            
            info!("Successfully imported {} traps into database", new_traps.len());
            Ok(new_traps.len())
        } else {
            Ok(0)
        }
    }

    /// Import hazards from a single JSON file
    fn import_hazards_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading hazard file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read hazard file: {}", e))?;
            
        let hazard_data: HazardData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse hazard JSON: {}", e))?;
            
        if let Some(hazards) = hazard_data.hazard {
            let hazards: Vec<TrapOrHazard> = hazards.into_iter().map(TrapOrHazard::Hazard).collect();
            let new_hazards: Vec<NewCatalogTrap> = hazards.iter().map(|hazard| NewCatalogTrap::from(hazard)).collect();
            
            debug!("Inserting {} hazards individually (SQLite limitation)", new_hazards.len());
            
            for hazard in &new_hazards {
                let result = diesel::insert_into(catalog_traps::table)
                    .values(hazard)
                    .on_conflict((catalog_traps::name, catalog_traps::source))
                    .do_nothing()
                    .execute(conn);
                    
                if let Err(e) = result {
                    warn!("Failed to insert hazard {}: {}", hazard.name, e);
                }
            }
            
            info!("Successfully imported {} hazards into database", new_hazards.len());
            Ok(new_hazards.len())
        } else {
            Ok(0)
        }
    }

    /// Remove all traps from a specific source
    pub fn remove_traps_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing traps from source: {}", source);
        
        let deleted = diesel::delete(catalog_traps::table.filter(catalog_traps::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete traps from source {}: {}", source, e))?;
        
        info!("Removed {} traps from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Import cults from a book directory
    pub fn import_cults_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        info!("Importing cults from book directory: {:?} (source: {})", book_dir, source);
        let mut total_imported = 0;
        
        // Import cults
        let cults_dir = book_dir.join("cults");
        if cults_dir.exists() {
            info!("Found cults directory: {:?}", cults_dir);
            let mut cult_entries = fs::read_dir(&cults_dir)
                .map_err(|e| format!("Failed to read cults directory: {}", e))?;
                
            while let Some(entry) = cult_entries.next() {
                let entry = entry.map_err(|e| format!("Failed to read cult directory entry: {}", e))?;
                let path = entry.path();
                
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    debug!("Processing cult file: {:?}", path.file_name().unwrap_or_default());
                    match Self::import_cults_from_file(conn, &path, source) {
                        Ok(count) => {
                            info!("Imported {} cults from {:?}", count, path);
                            total_imported += count;
                        }
                        Err(e) => {
                            error!("Failed to import cults from {:?}: {}", path, e);
                            return Err(e);
                        }
                    }
                }
            }
        }
        
        // Import boons from dedicated boons directory
        let boons_dir = book_dir.join("boons");
        if boons_dir.exists() {
            info!("Found boons directory: {:?}", boons_dir);
            let mut boon_entries = fs::read_dir(&boons_dir)
                .map_err(|e| format!("Failed to read boons directory: {}", e))?;
                
            while let Some(entry) = boon_entries.next() {
                let entry = entry.map_err(|e| format!("Failed to read boon directory entry: {}", e))?;
                let path = entry.path();
                
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    debug!("Processing boon file: {:?}", path.file_name().unwrap_or_default());
                    match Self::import_boons_from_file(conn, &path, source) {
                        Ok(count) => {
                            info!("Imported {} boons from {:?}", count, path);
                            total_imported += count;
                        }
                        Err(e) => {
                            error!("Failed to import boons from {:?}: {}", path, e);
                            return Err(e);
                        }
                    }
                }
            }
        }
        
        
        info!("Successfully imported {} total cults/boons from {}", total_imported, source);
        Ok(total_imported)
    }

    fn import_cults_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading cult file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read cult file: {}", e))?;
            
        let cult_data: CultData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse cult JSON: {}", e))?;
            
        if let Some(cults) = cult_data.cult {
            let new_cults: Vec<NewCatalogCult> = cults.iter().map(|cult| {
                let mut new_cult = NewCatalogCult::from(cult);
                if new_cult.source.is_empty() {
                    new_cult.source = source.to_string();
                }
                new_cult
            }).collect();
            
            debug!("Inserting {} cults individually (SQLite limitation)", new_cults.len());
            
            for cult in &new_cults {
                diesel::insert_into(catalog_cults::table)
                    .values(cult)
                    .on_conflict((catalog_cults::name, catalog_cults::source))
                    .do_nothing()
                    .execute(conn)
                    .map_err(|e| format!("Failed to insert cult: {}", e))?;
            }
            
            info!("Successfully imported {} cults into database", new_cults.len());
            Ok(new_cults.len())
        } else {
            Ok(0)
        }
    }

    fn import_boons_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading boon file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read boon file: {}", e))?;
            
        let boon_data: BoonData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse boon JSON: {}", e))?;
            
        if let Some(boons) = boon_data.boon {
            let new_boons: Vec<NewCatalogCult> = boons.iter().map(|boon| {
                let mut new_boon = NewCatalogCult::from(boon);
                if new_boon.source.is_empty() {
                    new_boon.source = source.to_string();
                }
                new_boon
            }).collect();
            
            debug!("Inserting {} boons individually (SQLite limitation)", new_boons.len());
            
            for boon in &new_boons {
                diesel::insert_into(catalog_cults::table)
                    .values(boon)
                    .on_conflict((catalog_cults::name, catalog_cults::source))
                    .do_nothing()
                    .execute(conn)
                    .map_err(|e| format!("Failed to insert boon: {}", e))?;
            }
            
            info!("Successfully imported {} boons into database", new_boons.len());
            Ok(new_boons.len())
        } else {
            Ok(0)
        }
    }


    /// Remove all cults/boons from a specific source
    pub fn remove_cults_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing cults/boons from source: {}", source);
        
        let deleted = diesel::delete(catalog_cults::table.filter(catalog_cults::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete cults/boons from source {}: {}", source, e))?;
        
        info!("Removed {} cults/boons from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Check if a filename matches a simple glob pattern
    fn matches_pattern(filename: &str, pattern: &str) -> bool {
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                return filename.starts_with(parts[0]) && filename.ends_with(parts[1]);
            }
        }
        filename == pattern
    }
}