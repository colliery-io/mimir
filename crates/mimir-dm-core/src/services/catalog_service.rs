//! Catalog import service for D&D 5e content
//!
//! This service handles automatic import of catalog content from uploaded books
//! into the SQLite database for fast searching and filtering.

use crate::models::catalog::{NewCatalogSpell, Spell, SpellData, NewCatalogLanguage, LanguageData, NewCatalogReward, RewardData, NewCatalogBackground, BackgroundData, NewCatalogFeat, FeatData, NewCatalogRace, RaceData, NewCatalogObject, ObjectData, NewCatalogTrap, TrapData, HazardData, TrapOrHazard};
use crate::models::catalog::cult::{NewCatalogCult, CultData, BoonData};
use crate::models::catalog::variant_rule::{NewCatalogVariantRule, VariantRuleData};
use crate::models::catalog::optionalfeature::{NewCatalogOptionalFeature, OptionalFeatureData};
use crate::models::catalog::item::{NewCatalogItem, ItemData};
use crate::models::catalog::monster::{NewCatalogMonster, MonsterData, MonsterFluff, MonsterFluffData};
use crate::models::catalog::deity::{NewCatalogDeity, DeityData};
use crate::models::catalog::vehicle::{NewCatalogVehicle, VehicleData};
use crate::schema::{catalog_spells, catalog_languages, catalog_rewards, catalog_backgrounds, catalog_feats, catalog_races, catalog_objects, catalog_traps, catalog_cults, catalog_variant_rules, catalog_optional_features, catalog_items, catalog_monsters, catalog_deities, catalog_vehicles};
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
        _source: &str
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
        _source: &str
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

    /// Import all variant rule data from an uploaded book directory
    pub fn import_variant_rules_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing variant rules from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        let variant_rule_files = Self::find_variant_rule_files(book_dir)?;
        
        if variant_rule_files.is_empty() {
            info!("No variant rule files found in book directory");
            return Ok(0);
        }
        
        info!("Found {} variant rule files to process", variant_rule_files.len());
        
        for variant_rule_file in variant_rule_files {
            debug!("Processing variant rule file: {:?}", variant_rule_file);
            
            match Self::import_variant_rules_from_file(conn, &variant_rule_file, source) {
                Ok(count) => {
                    info!("Imported {} variant rules from {:?}", count, variant_rule_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import variant rules from {:?}: {}", variant_rule_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }
        
        info!("Successfully imported {} total variant rules from {}", total_imported, source);
        Ok(total_imported)
    }

    fn find_variant_rule_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut files = Vec::new();
        
        // Look for variantrules directory
        let variantrules_dir = book_dir.join("variantrules");
        if variantrules_dir.exists() && variantrules_dir.is_dir() {
            let entries = fs::read_dir(&variantrules_dir)
                .map_err(|e| format!("Failed to read variantrules directory: {}", e))?;
                
            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
                let path = entry.path();
                
                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                    files.push(path);
                }
            }
        }
        
        Ok(files)
    }

    fn import_variant_rules_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading variant rule file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read variant rule file: {}", e))?;
            
        let variant_rule_data: VariantRuleData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse variant rule JSON: {}", e))?;
            
        if let Some(variant_rules) = variant_rule_data.variantrule {
            let new_variant_rules: Vec<NewCatalogVariantRule> = variant_rules.iter().map(|rule| {
                let mut new_rule = NewCatalogVariantRule::from(rule);
                // Always override the source with the book source to ensure consistency
                new_rule.source = source.to_string();
                new_rule
            }).collect();
            
            debug!("Inserting {} variant rules individually (SQLite limitation)", new_variant_rules.len());
            
            for rule in &new_variant_rules {
                diesel::insert_into(catalog_variant_rules::table)
                    .values(rule)
                    .on_conflict((catalog_variant_rules::name, catalog_variant_rules::source))
                    .do_nothing()
                    .execute(conn)
                    .map_err(|e| format!("Failed to insert variant rule: {}", e))?;
            }
            
            info!("Successfully imported {} variant rules into database", new_variant_rules.len());
            Ok(new_variant_rules.len())
        } else {
            Ok(0)
        }
    }

    /// Import all optional feature data from an uploaded book directory
    pub fn import_optional_features_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing optional features from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        let optional_feature_files = Self::find_optional_feature_files(book_dir)?;
        
        if optional_feature_files.is_empty() {
            info!("No optional feature files found in book directory");
            return Ok(0);
        }
        
        info!("Found {} optional feature files to process", optional_feature_files.len());
        
        for optional_feature_file in optional_feature_files {
            debug!("Processing optional feature file: {:?}", optional_feature_file);
            
            match Self::import_optional_features_from_file(conn, &optional_feature_file, source) {
                Ok(count) => {
                    info!("Imported {} optional features from {:?}", count, optional_feature_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import optional features from {:?}: {}", optional_feature_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }
        
        info!("Total optional features imported: {}", total_imported);
        Ok(total_imported)
    }

    /// Find optional feature files in a book directory
    fn find_optional_feature_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut files = Vec::new();
        
        // Look for optionalfeatures directory (matching existing catalog structure from previous implementation)
        let optionalfeatures_dir = book_dir.join("optionalfeatures");
        if optionalfeatures_dir.exists() && optionalfeatures_dir.is_dir() {
            let entries = fs::read_dir(&optionalfeatures_dir)
                .map_err(|e| format!("Failed to read optionalfeatures directory: {}", e))?;
                
            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
                let path = entry.path();
                
                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                    debug!("Found optional feature file: {:?}", path);
                    files.push(path);
                }
            }
        }
        
        // Also check the main data directory for optionalfeatures.json files
        let search_dirs = vec![
            book_dir.join("data"),
            book_dir.to_path_buf(),
        ];
        
        for search_dir in search_dirs {
            if !search_dir.exists() || !search_dir.is_dir() {
                continue;
            }
            
            if let Ok(entries) = fs::read_dir(&search_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        let filename = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                            
                        if path.is_file() && 
                           path.extension().and_then(|e| e.to_str()) == Some("json") &&
                           (filename.contains("optionalfeature") || filename.contains("optional-feature")) {
                            debug!("Found optional feature file: {:?}", path);
                            files.push(path);
                        }
                    }
                }
            }
        }
        
        Ok(files)
    }

    /// Import optional features from a single JSON file
    fn import_optional_features_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading optional features from file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
        
        let data: OptionalFeatureData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON from {:?}: {}", file_path, e))?;
        
        if let Some(optional_features) = data.optional_features {
            let new_optional_features: Vec<NewCatalogOptionalFeature> = optional_features.iter().map(|feature| {
                let mut new_feature = NewCatalogOptionalFeature::from(feature);
                // Always override the source with the book source to ensure consistency
                new_feature.source = source.to_string();
                new_feature
            }).collect();
            
            debug!("Inserting {} optional features individually (SQLite limitation)", new_optional_features.len());
            
            for feature in &new_optional_features {
                diesel::insert_into(catalog_optional_features::table)
                    .values(feature)
                    .on_conflict((catalog_optional_features::name, catalog_optional_features::source))
                    .do_nothing()
                    .execute(conn)
                    .map_err(|e| format!("Failed to insert optional feature: {}", e))?;
            }
            
            info!("Successfully imported {} optional features into database", new_optional_features.len());
            Ok(new_optional_features.len())
        } else {
            Ok(0)
        }
    }

    /// Remove all optional features from a specific source
    pub fn remove_optional_features_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing optional features from source: {}", source);
        
        let deleted = diesel::delete(catalog_optional_features::table.filter(catalog_optional_features::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete optional features from source {}: {}", source, e))?;
        
        info!("Removed {} optional features from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Import all item data from an uploaded book directory
    pub fn import_items_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing items from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        let item_files = Self::find_item_files(book_dir)?;
        
        if item_files.is_empty() {
            info!("No item files found in book directory");
            return Ok(0);
        }
        
        info!("Found {} item files to process", item_files.len());
        
        for item_file in item_files {
            debug!("Processing item file: {:?}", item_file);
            
            match Self::import_items_from_file(conn, &item_file, source) {
                Ok(count) => {
                    info!("Imported {} items from {:?}", count, item_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import items from {:?}: {}", item_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }
        
        info!("Total items imported: {}", total_imported);
        Ok(total_imported)
    }

    /// Find item files in a book directory (reusing existing logic from catalog.rs)
    fn find_item_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut files = Vec::new();
        
        // Check the items directory
        let items_dir = book_dir.join("items");
        if items_dir.exists() && items_dir.is_dir() {
            let entries = fs::read_dir(&items_dir)
                .map_err(|e| format!("Failed to read items directory: {}", e))?;
                
            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
                let path = entry.path();
                
                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    
                    // Skip fluff files, index files, and foundry files
                    if filename.starts_with("fluff-") || filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }
                    
                    debug!("Found item file: {:?}", path);
                    files.push(path);
                }
            }
        }
        
        Ok(files)
    }

    /// Import items from a single JSON file
    fn import_items_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading items from file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
        
        let data: ItemData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON from {:?}: {}", file_path, e))?;
        
        if !data.item.is_empty() {
            let new_items: Vec<NewCatalogItem> = data.item.iter().map(|item| {
                let mut new_item = NewCatalogItem::from(item);
                // Always override the source with the book source to ensure consistency
                new_item.source = source.to_string();
                
                // Also update the source in the full_item_json to maintain consistency
                if let Ok(mut item_json) = serde_json::from_str::<serde_json::Value>(&new_item.full_item_json) {
                    if let Some(obj) = item_json.as_object_mut() {
                        obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                        if let Ok(updated_json) = serde_json::to_string(&item_json) {
                            new_item.full_item_json = updated_json;
                        }
                    }
                }
                
                new_item
            }).collect();
            
            debug!("Inserting {} items individually (SQLite limitation)", new_items.len());
            
            for item in &new_items {
                diesel::insert_into(catalog_items::table)
                    .values(item)
                    .on_conflict((catalog_items::name, catalog_items::source))
                    .do_nothing()
                    .execute(conn)
                    .map_err(|e| format!("Failed to insert item: {}", e))?;
            }
            
            info!("Successfully imported {} items into database", new_items.len());
            Ok(new_items.len())
        } else {
            Ok(0)
        }
    }

    /// Import all monster data from an uploaded book directory
    pub fn import_monsters_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing monsters from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        let monster_files = Self::find_monster_files(book_dir)?;
        
        if monster_files.is_empty() {
            info!("No monster files found in book directory");
            return Ok(0);
        }
        
        info!("Found {} monster files to process", monster_files.len());
        
        for monster_file in monster_files {
            debug!("Processing monster file: {:?}", monster_file);
            
            match Self::import_monsters_from_file(conn, &monster_file, source) {
                Ok(count) => {
                    info!("Imported {} monsters from {:?}", count, monster_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import monsters from {:?}: {}", monster_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }
        
        info!("Total monsters imported: {}", total_imported);
        Ok(total_imported)
    }

    /// Find monster files in a book directory (bestiary/*.json files)
    fn find_monster_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut files = Vec::new();
        
        // Check the bestiary directory
        let bestiary_dir = book_dir.join("bestiary");
        if bestiary_dir.exists() && bestiary_dir.is_dir() {
            let entries = fs::read_dir(&bestiary_dir)
                .map_err(|e| format!("Failed to read bestiary directory: {}", e))?;
                
            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
                let path = entry.path();
                
                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    
                    // Skip fluff files, index files, and foundry files
                    if filename.starts_with("fluff-") || filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }
                    
                    debug!("Found monster file: {:?}", path);
                    files.push(path);
                }
            }
        }
        
        Ok(files)
    }

    /// Load monster fluff data from corresponding fluff file
    fn load_monster_fluff_data(monster_file_path: &Path) -> Option<std::collections::HashMap<String, MonsterFluff>> {
        // Get the bestiary directory and filename
        let bestiary_dir = monster_file_path.parent()?;
        let filename = monster_file_path.file_name()?.to_str()?;
        
        // Convert bestiary-*.json to fluff-bestiary-*.json
        if !filename.starts_with("bestiary-") {
            return None;
        }
        
        let fluff_filename = filename.replace("bestiary-", "fluff-bestiary-");
        let fluff_file = bestiary_dir.join(&fluff_filename);
        
        if !fluff_file.exists() {
            debug!("No fluff file found at: {:?}", fluff_file);
            return None;
        }
        
        debug!("Loading fluff data from: {:?}", fluff_file);
        
        match fs::read_to_string(&fluff_file) {
            Ok(fluff_content) => {
                match serde_json::from_str::<MonsterFluffData>(&fluff_content) {
                    Ok(fluff_data) => {
                        let mut fluff_map = std::collections::HashMap::new();
                        
                        for fluff in fluff_data.monster_fluff {
                            fluff_map.insert(fluff.name.to_lowercase(), fluff);
                        }
                        
                        debug!("Loaded fluff data for {} monsters", fluff_map.len());
                        Some(fluff_map)
                    }
                    Err(e) => {
                        warn!("Failed to parse fluff file {:?}: {}", fluff_file, e);
                        None
                    }
                }
            }
            Err(e) => {
                warn!("Failed to read fluff file {:?}: {}", fluff_file, e);
                None
            }
        }
    }

    /// Import monsters from a single JSON file
    fn import_monsters_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading monsters from file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
        
        let data: MonsterData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON from {:?}: {}", file_path, e))?;
        
        // Load fluff data if available
        let fluff_data = Self::load_monster_fluff_data(file_path);
        
        if !data.monster.is_empty() {
            let new_monsters: Vec<NewCatalogMonster> = data.monster.iter().map(|monster| {
                let mut new_monster = NewCatalogMonster::from(monster);
                // Always override the source with the book source to ensure consistency
                new_monster.source = source.to_string();
                
                // Also update the source in the full_monster_json to maintain consistency
                if let Ok(mut monster_json) = serde_json::from_str::<serde_json::Value>(&new_monster.full_monster_json) {
                    if let Some(obj) = monster_json.as_object_mut() {
                        obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                        if let Ok(updated_json) = serde_json::to_string(&monster_json) {
                            new_monster.full_monster_json = updated_json;
                        }
                    }
                }
                
                // Add fluff data if available for this monster
                if let Some(ref fluff_map) = fluff_data {
                    let monster_name_lower = monster.name.to_lowercase();
                    if let Some(monster_fluff) = fluff_map.get(&monster_name_lower) {
                        if let Ok(fluff_json) = serde_json::to_string(monster_fluff) {
                            new_monster.fluff_json = Some(fluff_json);
                        }
                    }
                }
                
                new_monster
            }).collect();
            
            debug!("Inserting {} monsters individually (SQLite limitation)", new_monsters.len());
            
            for monster in &new_monsters {
                diesel::insert_into(catalog_monsters::table)
                    .values(monster)
                    .on_conflict((catalog_monsters::name, catalog_monsters::source))
                    .do_nothing()
                    .execute(conn)
                    .map_err(|e| format!("Failed to insert monster: {}", e))?;
            }
            
            info!("Successfully imported {} monsters into database", new_monsters.len());
            Ok(new_monsters.len())
        } else {
            Ok(0)
        }
    }

    /// Remove all monsters from a specific source
    pub fn remove_monsters_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing monsters from source: {}", source);
        
        let deleted = diesel::delete(catalog_monsters::table.filter(catalog_monsters::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete monsters from source {}: {}", source, e))?;
        
        info!("Removed {} monsters from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Remove all items from a specific source
    pub fn remove_items_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing items from source: {}", source);
        
        let deleted = diesel::delete(catalog_items::table.filter(catalog_items::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete items from source {}: {}", source, e))?;
        
        info!("Removed {} items from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Remove all variant rules from a specific source
    pub fn remove_variant_rules_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing variant rules from source: {}", source);
        
        let deleted = diesel::delete(catalog_variant_rules::table.filter(catalog_variant_rules::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete variant rules from source {}: {}", source, e))?;
        
        info!("Removed {} variant rules from source: {}", deleted, source);
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

    /// Import all deity data from an uploaded book directory
    pub fn import_deities_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing deities from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        let deity_files = Self::find_deity_files(book_dir)?;
        
        if deity_files.is_empty() {
            info!("No deity files found in book directory");
            return Ok(0);
        }
        
        info!("Found {} deity files to process", deity_files.len());
        
        for deity_file in deity_files {
            debug!("Processing deity file: {:?}", deity_file);
            
            match Self::import_deities_from_file(conn, &deity_file, source) {
                Ok(count) => {
                    info!("Imported {} deities from {:?}", count, deity_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import deities from {:?}: {}", deity_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }
        
        info!("Total deities imported: {}", total_imported);
        Ok(total_imported)
    }

    /// Find deity files in a book directory (deities/*.json files)
    fn find_deity_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut files = Vec::new();
        
        // Check the deities directory
        let deities_dir = book_dir.join("deities");
        if deities_dir.exists() && deities_dir.is_dir() {
            let entries = fs::read_dir(&deities_dir)
                .map_err(|e| format!("Failed to read deities directory: {}", e))?;
                
            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
                let path = entry.path();
                
                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    
                    // Skip index files and foundry files
                    if filename == "index.json" || filename == "foundry.json" {
                        continue;
                    }
                    
                    debug!("Found deity file: {:?}", path);
                    files.push(path);
                }
            }
        }
        
        Ok(files)
    }

    /// Import deities from a single JSON file
    fn import_deities_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading deities from file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
        
        let data: DeityData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON from {:?}: {}", file_path, e))?;
        
        if let Some(deities) = data.deity {
            if !deities.is_empty() {
                let new_deities: Vec<NewCatalogDeity> = deities.iter().map(|deity| {
                    let mut new_deity = NewCatalogDeity::from(deity);
                    // Always override the source with the book source to ensure consistency
                    new_deity.source = source.to_string();
                    
                    // Also update the source in the full_deity_json to maintain consistency
                    if let Ok(mut deity_json) = serde_json::from_str::<serde_json::Value>(&new_deity.full_deity_json) {
                        if let Some(obj) = deity_json.as_object_mut() {
                            obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                            if let Ok(updated_json) = serde_json::to_string(&deity_json) {
                                new_deity.full_deity_json = updated_json;
                            }
                        }
                    }
                    
                    new_deity
                }).collect();
                
                debug!("Inserting {} deities individually (SQLite limitation)", new_deities.len());
                
                for deity in &new_deities {
                    diesel::insert_into(catalog_deities::table)
                        .values(deity)
                        .on_conflict((catalog_deities::name, catalog_deities::source))
                        .do_nothing()
                        .execute(conn)
                        .map_err(|e| format!("Failed to insert deity: {}", e))?;
                }
                
                info!("Successfully imported {} deities into database", new_deities.len());
                Ok(new_deities.len())
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }

    /// Remove all deities from a specific source
    pub fn remove_deities_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing deities from source: {}", source);
        
        let deleted = diesel::delete(catalog_deities::table.filter(catalog_deities::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete deities from source {}: {}", source, e))?;
        
        info!("Removed {} deities from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Import all vehicle data from an uploaded book directory
    pub fn import_vehicles_from_book(
        conn: &mut SqliteConnection, 
        book_dir: &Path, 
        source: &str
    ) -> Result<usize, String> {
        info!("Importing vehicles from book directory: {:?} (source: {})", book_dir, source);
        
        let mut total_imported = 0;
        let vehicle_files = Self::find_vehicle_files(book_dir)?;
        
        if vehicle_files.is_empty() {
            info!("No vehicle files found in book directory");
            return Ok(0);
        }
        
        info!("Found {} vehicle files to process", vehicle_files.len());
        
        for vehicle_file in vehicle_files {
            debug!("Processing vehicle file: {:?}", vehicle_file);
            
            match Self::import_vehicles_from_file(conn, &vehicle_file, source) {
                Ok(count) => {
                    info!("Imported {} vehicles from {:?}", count, vehicle_file);
                    total_imported += count;
                }
                Err(e) => {
                    error!("Failed to import vehicles from {:?}: {}", vehicle_file, e);
                    // Continue processing other files instead of failing completely
                }
            }
        }
        
        info!("Total vehicles imported: {}", total_imported);
        Ok(total_imported)
    }

    /// Find vehicle files in a book directory (vehicles/*.json files)
    fn find_vehicle_files(book_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut files = Vec::new();
        
        // Check the vehicles directory
        let vehicles_dir = book_dir.join("vehicles");
        if vehicles_dir.exists() && vehicles_dir.is_dir() {
            let entries = fs::read_dir(&vehicles_dir)
                .map_err(|e| format!("Failed to read vehicles directory: {}", e))?;
                
            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
                let path = entry.path();
                
                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
                    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    
                    // Skip fluff files for now
                    if !filename.starts_with("fluff-") {
                        files.push(path);
                    }
                }
            }
        }
        
        if files.is_empty() {
            debug!("No vehicle files found in {:?}", vehicles_dir);
        } else {
            debug!("Found {} vehicle files in {:?}", files.len(), vehicles_dir);
        }
        
        Ok(files)
    }

    /// Import vehicles from a single JSON file
    fn import_vehicles_from_file(
        conn: &mut SqliteConnection,
        file_path: &Path,
        source: &str
    ) -> Result<usize, String> {
        debug!("Reading vehicles from file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
        
        let data: VehicleData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON from {:?}: {}", file_path, e))?;
        
        if let Some(vehicles) = data.vehicle {
            if !vehicles.is_empty() {
                let new_vehicles: Vec<NewCatalogVehicle> = vehicles.iter().map(|vehicle| {
                    let mut new_vehicle = NewCatalogVehicle::from(vehicle);
                    // Always override the source with the book source to ensure consistency
                    new_vehicle.source = source.to_string();
                    
                    // Also update the source in the full_vehicle_json to maintain consistency
                    if let Ok(mut vehicle_json) = serde_json::from_str::<serde_json::Value>(&new_vehicle.full_vehicle_json) {
                        if let Some(obj) = vehicle_json.as_object_mut() {
                            obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                            if let Ok(updated_json) = serde_json::to_string(&vehicle_json) {
                                new_vehicle.full_vehicle_json = updated_json;
                            }
                        }
                    }
                    
                    new_vehicle
                }).collect();
                
                // Use batch insert with conflict resolution for better performance
                for new_vehicle in &new_vehicles {
                    diesel::insert_into(catalog_vehicles::table)
                        .values(new_vehicle)
                        .on_conflict((catalog_vehicles::name, catalog_vehicles::source))
                        .do_nothing()
                        .execute(conn)
                        .map_err(|e| format!("Failed to insert vehicle: {}", e))?;
                }
                
                info!("Successfully imported {} vehicles into database", new_vehicles.len());
                Ok(new_vehicles.len())
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }

    /// Remove all vehicles from a specific source
    pub fn remove_vehicles_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        info!("Removing vehicles from source: {}", source);
        
        let deleted = diesel::delete(catalog_vehicles::table.filter(catalog_vehicles::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete vehicles from source {}: {}", source, e))?;
        
        info!("Removed {} vehicles from source: {}", deleted, source);
        Ok(deleted)
    }

    /// Load class fluff data from corresponding fluff file
    fn load_class_fluff_data(book_dir: &Path, source: &str) -> Option<std::collections::HashMap<String, crate::models::catalog::class::ClassFluff>> {
        use crate::models::catalog::class::ClassFluffData;
        
        // Look for fluff files in class directory
        let search_paths = [
            book_dir.join("class").join(format!("fluff-{}.json", source.to_lowercase())),
            book_dir.join("classes").join(format!("fluff-{}.json", source.to_lowercase())),
            book_dir.join("class").join(format!("fluff-class-{}.json", source.to_lowercase())),
            book_dir.join("classes").join(format!("fluff-class-{}.json", source.to_lowercase())),
        ];
        
        for fluff_file in &search_paths {
            if !fluff_file.exists() {
                continue;
            }
            
            debug!("Loading class fluff data from: {:?}", fluff_file);
            
            match fs::read_to_string(&fluff_file) {
                Ok(fluff_content) => {
                    match serde_json::from_str::<ClassFluffData>(&fluff_content) {
                        Ok(fluff_data) => {
                            let mut fluff_map = std::collections::HashMap::new();
                            
                            if let Some(class_fluff) = fluff_data.class_fluff {
                                for fluff in class_fluff {
                                    fluff_map.insert(fluff.name.to_lowercase(), fluff);
                                }
                            }
                            
                            debug!("Loaded class fluff data for {} classes", fluff_map.len());
                            return Some(fluff_map);
                        }
                        Err(e) => {
                            warn!("Failed to parse class fluff file {:?}: {}", fluff_file, e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to read class fluff file {:?}: {}", fluff_file, e);
                }
            }
        }
        
        debug!("No class fluff data found for source: {}", source);
        None
    }

    /// Load subclass fluff data from corresponding fluff file
    fn load_subclass_fluff_data(book_dir: &Path, source: &str) -> Option<std::collections::HashMap<String, crate::models::catalog::class::SubclassFluff>> {
        use crate::models::catalog::class::ClassFluffData;
        
        // Look for subclass fluff files in class directory
        let search_paths = [
            book_dir.join("class").join(format!("subclass-fluff-{}.json", source.to_lowercase())),
            book_dir.join("classes").join(format!("subclass-fluff-{}.json", source.to_lowercase())),
            book_dir.join("class").join(format!("fluff-{}.json", source.to_lowercase())),
            book_dir.join("classes").join(format!("fluff-{}.json", source.to_lowercase())),
        ];
        
        for fluff_file in &search_paths {
            if !fluff_file.exists() {
                continue;
            }
            
            debug!("Loading subclass fluff data from: {:?}", fluff_file);
            
            match fs::read_to_string(&fluff_file) {
                Ok(fluff_content) => {
                    match serde_json::from_str::<ClassFluffData>(&fluff_content) {
                        Ok(fluff_data) => {
                            let mut fluff_map = std::collections::HashMap::new();
                            
                            if let Some(subclass_fluff) = fluff_data.subclass_fluff {
                                for fluff in subclass_fluff {
                                    let key = format!("{}|{}", fluff.class_name.to_lowercase(), fluff.name.to_lowercase());
                                    fluff_map.insert(key, fluff);
                                }
                            }
                            
                            debug!("Loaded subclass fluff data for {} subclasses", fluff_map.len());
                            return Some(fluff_map);
                        }
                        Err(e) => {
                            warn!("Failed to parse subclass fluff file {:?}: {}", fluff_file, e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to read subclass fluff file {:?}: {}", fluff_file, e);
                }
            }
        }
        
        debug!("No subclass fluff data found for source: {}", source);
        None
    }

    /// Import classes from a book directory
    pub fn import_classes_from_book(
        conn: &mut SqliteConnection,
        book_dir: &Path,
        source: &str
    ) -> Result<usize, String> {
        use crate::schema::{catalog_classes, catalog_subclasses, catalog_class_features, catalog_subclass_features};
        use crate::models::catalog::class::{
            Class, ClassData, ClassFeatureData,
            NewCatalogClass, NewCatalogSubclass, NewCatalogClassFeature, NewCatalogSubclassFeature
        };
        
        info!("Importing classes from book: {}", source);
        
        // Load fluff data for classes and subclasses
        let class_fluff_data = Self::load_class_fluff_data(book_dir, source);
        let subclass_fluff_data = Self::load_subclass_fluff_data(book_dir, source);
        
        let mut total_imported = 0;
        
        // Search for class files in multiple possible locations
        let search_dirs = [
            book_dir.join("class"),
            book_dir.join("classes"),
            book_dir.join("data"),
            book_dir.to_path_buf(),
        ];
        
        for search_dir in &search_dirs {
            if !search_dir.exists() {
                continue;
            }
            
            debug!("Searching for class files in: {:?}", search_dir);
            
            let entries = fs::read_dir(search_dir)
                .map_err(|e| format!("Failed to read directory {:?}: {}", search_dir, e))?;
            
            for entry in entries.flatten() {
                let path = entry.path();
                
                // Skip if not a JSON file
                if !path.extension().map_or(false, |ext| ext == "json") {
                    continue;
                }
                
                let filename = entry.file_name();
                let filename_str = filename.to_string_lossy();
                
                // Check if this might be a class file based on naming patterns
                let is_main_class_file = search_dir.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n == "class" || n == "classes")
                    .unwrap_or(false) && 
                    !filename_str.contains("fluff") &&
                    !filename_str.contains("feature");
                
                let is_class_named_file = filename_str.contains("class") && 
                                  !filename_str.contains("fluff") &&
                                  !filename_str.contains("feature") &&
                                  !filename_str.contains("subclass-feature");
                
                let is_main_book_file = filename_str == format!("{}.json", source.to_lowercase());
                
                if is_main_class_file || is_class_named_file || is_main_book_file {
                    debug!("Processing class file: {:?}", path);
                    
                    let content = fs::read_to_string(&path)
                        .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;
                    
                    // Try to parse as ClassData structure first
                    if let Ok(class_data) = serde_json::from_str::<ClassData>(&content) {
                        // Import classes
                        if !class_data.classes.is_empty() {
                            for class in &class_data.classes {
                                let mut new_class = NewCatalogClass::from(class);
                                new_class.source = source.to_string();
                                
                                // Update source in JSON
                                if let Ok(mut class_json) = serde_json::from_str::<serde_json::Value>(&new_class.full_class_json) {
                                    if let Some(obj) = class_json.as_object_mut() {
                                        obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                                        if let Ok(updated_json) = serde_json::to_string(&class_json) {
                                            new_class.full_class_json = updated_json;
                                        }
                                    }
                                }
                                
                                // Add fluff data if available
                                if let Some(ref class_fluff_map) = class_fluff_data {
                                    let class_name_lower = class.name.to_lowercase();
                                    if let Some(class_fluff) = class_fluff_map.get(&class_name_lower) {
                                        if let Ok(fluff_json) = serde_json::to_string(class_fluff) {
                                            new_class.fluff_json = Some(fluff_json);
                                        }
                                    }
                                }
                                
                                diesel::insert_into(catalog_classes::table)
                                    .values(&new_class)
                                    .on_conflict((catalog_classes::name, catalog_classes::source))
                                    .do_nothing()
                                    .execute(conn)
                                    .map_err(|e| format!("Failed to insert class: {}", e))?;
                                
                                total_imported += 1;
                                debug!("Imported class: {} ({})", class.name, source);
                            }
                        }
                        
                        // Import subclasses
                        if let Some(subclasses) = &class_data.subclass {
                            for subclass in subclasses {
                                let mut new_subclass = NewCatalogSubclass::from(subclass);
                                new_subclass.source = source.to_string();
                                
                                // Update source in JSON
                                if let Ok(mut subclass_json) = serde_json::from_str::<serde_json::Value>(&new_subclass.full_subclass_json) {
                                    if let Some(obj) = subclass_json.as_object_mut() {
                                        obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                                        if let Ok(updated_json) = serde_json::to_string(&subclass_json) {
                                            new_subclass.full_subclass_json = updated_json;
                                        }
                                    }
                                }
                                
                                // Add fluff data if available
                                if let Some(ref subclass_fluff_map) = subclass_fluff_data {
                                    let subclass_key = format!("{}|{}", subclass.class_name.to_lowercase(), subclass.name.to_lowercase());
                                    if let Some(subclass_fluff) = subclass_fluff_map.get(&subclass_key) {
                                        if let Ok(fluff_json) = serde_json::to_string(subclass_fluff) {
                                            new_subclass.fluff_json = Some(fluff_json);
                                        }
                                    }
                                }
                                
                                diesel::insert_into(catalog_subclasses::table)
                                    .values(&new_subclass)
                                    .on_conflict((catalog_subclasses::name, catalog_subclasses::class_name, catalog_subclasses::source))
                                    .do_nothing()
                                    .execute(conn)
                                    .map_err(|e| format!("Failed to insert subclass: {}", e))?;
                                
                                debug!("Imported subclass: {} ({})", subclass.name, source);
                            }
                        }
                        
                        // Import class features
                        if let Some(features) = &class_data.class_features {
                            for feature in features {
                                let mut new_feature = NewCatalogClassFeature::from(feature);
                                new_feature.source = source.to_string();
                                
                                // Update source in JSON
                                if let Ok(mut feature_json) = serde_json::from_str::<serde_json::Value>(&new_feature.full_feature_json) {
                                    if let Some(obj) = feature_json.as_object_mut() {
                                        obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                                        if let Ok(updated_json) = serde_json::to_string(&feature_json) {
                                            new_feature.full_feature_json = updated_json;
                                        }
                                    }
                                }
                                
                                diesel::insert_into(catalog_class_features::table)
                                    .values(&new_feature)
                                    .on_conflict_do_nothing()
                                    .execute(conn)
                                    .map_err(|e| format!("Failed to insert class feature: {}", e))?;
                                
                                debug!("Imported class feature: {} ({})", feature.name, source);
                            }
                        }
                        
                        // Import subclass features
                        if let Some(subclass_features) = &class_data.subclass_features {
                            for feature in subclass_features {
                                let mut new_feature = NewCatalogSubclassFeature::from(feature);
                                new_feature.source = source.to_string();
                                
                                // Update source in JSON
                                if let Ok(mut feature_json) = serde_json::from_str::<serde_json::Value>(&new_feature.full_feature_json) {
                                    if let Some(obj) = feature_json.as_object_mut() {
                                        obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                                        if let Ok(updated_json) = serde_json::to_string(&feature_json) {
                                            new_feature.full_feature_json = updated_json;
                                        }
                                    }
                                }
                                
                                diesel::insert_into(catalog_subclass_features::table)
                                    .values(&new_feature)
                                    .on_conflict_do_nothing()
                                    .execute(conn)
                                    .map_err(|e| format!("Failed to insert subclass feature: {}", e))?;
                                
                                debug!("Imported subclass feature: {} ({})", feature.name, source);
                            }
                        }
                    } else if let Ok(classes) = serde_json::from_str::<Vec<Class>>(&content) {
                        // Handle direct array of classes
                        for class in &classes {
                            let mut new_class = NewCatalogClass::from(class);
                            new_class.source = source.to_string();
                            
                            // Update source in JSON
                            if let Ok(mut class_json) = serde_json::from_str::<serde_json::Value>(&new_class.full_class_json) {
                                if let Some(obj) = class_json.as_object_mut() {
                                    obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                                    if let Ok(updated_json) = serde_json::to_string(&class_json) {
                                        new_class.full_class_json = updated_json;
                                    }
                                }
                            }
                            
                            diesel::insert_into(catalog_classes::table)
                                .values(&new_class)
                                .on_conflict((catalog_classes::name, catalog_classes::source))
                                .do_nothing()
                                .execute(conn)
                                .map_err(|e| format!("Failed to insert class: {}", e))?;
                            
                            total_imported += 1;
                            debug!("Imported class: {} ({})", class.name, source);
                        }
                    }
                }
                
                // Check for separate feature files
                let is_feature_file = filename_str.starts_with("features-") || 
                                     filename_str.starts_with("class-features-") ||
                                     (filename_str.contains("feature") && 
                                      !filename_str.contains("fluff") && 
                                      !filename_str.starts_with("subclass-features"));
                
                let is_subclass_feature_file = filename_str.starts_with("subclass-features-");
                
                if is_feature_file {
                    debug!("Processing class feature file: {:?}", path);
                    
                    let content = fs::read_to_string(&path)
                        .map_err(|e| format!("Failed to read feature file {:?}: {}", path, e))?;
                    
                    if let Ok(feature_data) = serde_json::from_str::<ClassFeatureData>(&content) {
                        if let Some(features) = &feature_data.class_feature {
                            for feature in features {
                                let mut new_feature = NewCatalogClassFeature::from(feature);
                                new_feature.source = source.to_string();
                                
                                // Update source in JSON
                                if let Ok(mut feature_json) = serde_json::from_str::<serde_json::Value>(&new_feature.full_feature_json) {
                                    if let Some(obj) = feature_json.as_object_mut() {
                                        obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                                        if let Ok(updated_json) = serde_json::to_string(&feature_json) {
                                            new_feature.full_feature_json = updated_json;
                                        }
                                    }
                                }
                                
                                diesel::insert_into(catalog_class_features::table)
                                    .values(&new_feature)
                                    .on_conflict_do_nothing()
                                    .execute(conn)
                                    .map_err(|e| format!("Failed to insert class feature: {}", e))?;
                                
                                debug!("Imported class feature: {} ({})", feature.name, source);
                            }
                        }
                    }
                }
                
                if is_subclass_feature_file {
                    debug!("Processing subclass feature file: {:?}", path);
                    
                    let content = fs::read_to_string(&path)
                        .map_err(|e| format!("Failed to read subclass feature file {:?}: {}", path, e))?;
                    
                    if let Ok(feature_data) = serde_json::from_str::<ClassFeatureData>(&content) {
                        if let Some(subclass_features) = &feature_data.subclass_feature {
                            for feature in subclass_features {
                                let mut new_feature = NewCatalogSubclassFeature::from(feature);
                                new_feature.source = source.to_string();
                                
                                // Update source in JSON
                                if let Ok(mut feature_json) = serde_json::from_str::<serde_json::Value>(&new_feature.full_feature_json) {
                                    if let Some(obj) = feature_json.as_object_mut() {
                                        obj.insert("source".to_string(), serde_json::Value::String(source.to_string()));
                                        if let Ok(updated_json) = serde_json::to_string(&feature_json) {
                                            new_feature.full_feature_json = updated_json;
                                        }
                                    }
                                }
                                
                                diesel::insert_into(catalog_subclass_features::table)
                                    .values(&new_feature)
                                    .on_conflict_do_nothing()
                                    .execute(conn)
                                    .map_err(|e| format!("Failed to insert subclass feature: {}", e))?;
                                
                                debug!("Imported subclass feature: {} ({})", feature.name, source);
                            }
                        }
                    }
                }
            }
        }
        
        info!("Successfully imported {} total class-related items from source: {}", total_imported, source);
        Ok(total_imported)
    }

    /// Remove all classes and related data from a specific source
    pub fn remove_classes_from_source(
        conn: &mut SqliteConnection,
        source: &str
    ) -> Result<usize, String> {
        use crate::schema::{catalog_classes, catalog_subclasses, catalog_class_features, catalog_subclass_features};
        
        info!("Removing classes from source: {}", source);
        
        // Delete in reverse dependency order
        let subclass_features_deleted = diesel::delete(catalog_subclass_features::table.filter(catalog_subclass_features::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete subclass features from source {}: {}", source, e))?;
        
        let class_features_deleted = diesel::delete(catalog_class_features::table.filter(catalog_class_features::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete class features from source {}: {}", source, e))?;
        
        let subclasses_deleted = diesel::delete(catalog_subclasses::table.filter(catalog_subclasses::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete subclasses from source {}: {}", source, e))?;
        
        let classes_deleted = diesel::delete(catalog_classes::table.filter(catalog_classes::source.eq(source)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete classes from source {}: {}", source, e))?;
        
        let total_deleted = classes_deleted + subclasses_deleted + class_features_deleted + subclass_features_deleted;
        info!("Removed {} total class-related items from source: {}", total_deleted, source);
        Ok(total_deleted)
    }
}