//! Main bundle import functionality

use crate::bundle::Bundle;
use crate::error::{ImportError, ImportResult};
use crate::progress::ImportProgress;
use mimir_dm_core::{
    dal::{
        rules::{
            backgrounds::BackgroundRepository, 
            classes::ClassRepository, 
            creatures::CreatureRepository, 
            feats::FeatRepository, 
            items::ItemRepository, 
            races::RaceRepository, 
            rule_systems::RuleSystemRepository, 
            sources::SourceRepository, 
            spells::SpellRepository
        },
        traits::AsyncRepository
    },
    models::rules::{
        backgrounds::Background, 
        classes::Class, 
        creatures::Creature, 
        feats::Feat, 
        items::{Item, NewItem}, 
        races::Race, 
        rule_systems::RuleSystem, 
        sources::Source, 
        spells::Spell
    },
};
use std::path::Path;
use tracing::{debug, info};

/// Main bundle importer
pub struct BundleImporter {
    db_url: String,
}

impl BundleImporter {
    /// Create a new bundle importer
    pub fn new(db_url: String) -> Self {
        Self { db_url }
    }

    /// Import a bundle into the database
    pub async fn import_bundle<P: AsRef<Path>>(&self, bundle_path: P) -> ImportResult<()> {
        let bundle_path = bundle_path.as_ref();
        info!("Starting import of bundle: {}", bundle_path.display());

        // Extract bundle
        let bundle = Bundle::from_archive(bundle_path).await?;
        info!("Extracted bundle: {}", bundle.manifest.bundle_name);

        // Use the direct import method
        self.import_bundle_direct(bundle).await
    }

    /// Import a bundle that has already been extracted
    pub async fn import_bundle_direct(&self, bundle: Bundle) -> ImportResult<()> {
        info!("Starting import of bundle: {}", bundle.manifest.bundle_name);

        // Create progress reporter
        let total_entities = bundle.manifest.total_entities();
        let mut progress = ImportProgress::new(total_entities);

        // Start database transaction
        progress.set_message("Starting database transaction...");
        
        // Import in dependency order
        self.import_rule_system(&bundle, &mut progress).await?;
        self.import_sources(&bundle, &mut progress).await?;
        self.import_races(&bundle, &mut progress).await?;
        self.import_classes(&bundle, &mut progress).await?;
        self.import_items(&bundle, &mut progress).await?;
        self.import_backgrounds(&bundle, &mut progress).await?;
        self.import_feats(&bundle, &mut progress).await?;
        self.import_spells(&bundle, &mut progress).await?;
        self.import_creatures(&bundle, &mut progress).await?;

        progress.finish_with_message("Import completed successfully!");
        info!("Bundle import completed successfully");
        
        Ok(())
    }

    /// Import rule system from manifest
    async fn import_rule_system(&self, bundle: &Bundle, progress: &mut ImportProgress) -> ImportResult<()> {
        progress.set_message("Importing rule system...");
        
        let rule_system_id = bundle.manifest.rule_system.clone();
        let rule_system = RuleSystem::new(
            rule_system_id.clone(),
            bundle.manifest.bundle_name.clone()
        ).with_version(bundle.manifest.bundle_version.clone());

        let repo = RuleSystemRepository::new(self.db_url.clone());
        repo.create(rule_system).await?;
        
        progress.inc(1);
        debug!("Imported rule system: {}", rule_system_id);
        Ok(())
    }

    /// Import sources from sources.json
    async fn import_sources(&self, bundle: &Bundle, progress: &mut ImportProgress) -> ImportResult<()> {
        if !bundle.has_file("sources.json") {
            debug!("No sources.json found, skipping sources import");
            return Ok(());
        }

        progress.set_message("Importing sources...");
        
        let sources_data: serde_json::Value = bundle.parse_json_file("sources.json")?;
        let sources_array = sources_data["sources"].as_array()
            .ok_or_else(|| ImportError::invalid_entity_data("sources.json", "Missing 'sources' array"))?;

        let repo = SourceRepository::new(self.db_url.clone());
        let rule_system_id = bundle.manifest.rule_system.clone();
        
        for source_value in sources_array {
            let source_obj = source_value.as_object()
                .ok_or_else(|| ImportError::invalid_entity_data("sources.json", "Source is not an object"))?;
            
            let mut source = Source::new(
                source_obj["id"].as_str().unwrap_or_default().to_string(),
                rule_system_id.clone(),
                source_obj["full_name"].as_str().unwrap_or_default().to_string(),
            );
            
            if let Some(abbr) = source_obj.get("abbreviation").and_then(|v| v.as_str()) {
                source = source.with_abbreviation(abbr.to_string());
            }
            
            if let Some(version) = source_obj.get("version").and_then(|v| v.as_str()) {
                source = source.with_version(version.to_string());
            }
            
            if let Some(is_official) = source_obj.get("is_official").and_then(|v| v.as_bool()) {
                source = source.with_official(is_official);
            }
            
            if let Some(is_srd) = source_obj.get("is_srd").and_then(|v| v.as_bool()) {
                source = source.with_srd(is_srd);
            }
            
            // Parse published_date if present
            if let Some(date_str) = source_obj.get("published_date").and_then(|v| v.as_str()) {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                    source = source.with_published_date(date);
                }
            }
            
            repo.create(source).await?;
            progress.inc(1);
        }

        debug!("Imported {} sources", sources_array.len());
        Ok(())
    }

    /// Import races from races.json
    async fn import_races(&self, bundle: &Bundle, progress: &mut ImportProgress) -> ImportResult<()> {
        if !bundle.has_file("races.json") {
            debug!("No races.json found, skipping races import");
            return Ok(());
        }

        progress.set_message("Importing races...");
        
        let races_data: serde_json::Value = bundle.parse_json_file("races.json")?;
        let races_array = races_data["races"].as_array()
            .ok_or_else(|| ImportError::invalid_entity_data("races.json", "Missing 'races' array"))?;

        let repo = RaceRepository::new(self.db_url.clone());
        let rule_system_id = bundle.manifest.rule_system.clone();
        let now = chrono::Utc::now().naive_utc();
        
        for race_value in races_array {
            let race_obj = race_value.as_object()
                .ok_or_else(|| ImportError::invalid_entity_data("races.json", "Race is not an object"))?;
            
            let race = Race {
                id: race_obj["id"].as_str().unwrap_or_default().to_string(),
                name: race_obj["name"].as_str().unwrap_or_default().to_string(),
                rule_system_id: rule_system_id.clone(),
                source_id: race_obj["source"].as_str().unwrap_or_default().to_string(),
                page: race_obj.get("page").and_then(|v| v.as_i64()).map(|p| p as i32),
                race_type: race_obj["race_type"].as_str().unwrap_or("race").to_string(),
                parent_race_id: race_obj.get("parent_race_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
                size: race_obj.get("size").and_then(|v| v.as_str()).map(|s| s.to_string()),
                speed: race_obj.get("speed").map(|v| v.to_string()),
                ability_scores: race_obj.get("ability_scores").map(|v| v.to_string()),
                age: race_obj.get("age").map(|v| v.to_string()),
                alignment_tendency: race_obj.get("alignment_tendency").and_then(|v| v.as_str()).map(|s| s.to_string()),
                language_proficiencies: race_obj.get("language_proficiencies").map(|v| v.to_string()),
                trait_tags: race_obj.get("trait_tags").map(|v| v.to_string()),
                entries: race_obj.get("entries").map(|v| v.to_string()).unwrap_or_else(|| "[]".to_string()),
                created_at: now,
                updated_at: now,
            };
            
            repo.create(race).await?;
            progress.inc(1);
        }

        debug!("Imported {} races", races_array.len());
        Ok(())
    }

    /// Import classes from classes.json
    async fn import_classes(&self, bundle: &Bundle, progress: &mut ImportProgress) -> ImportResult<()> {
        if !bundle.has_file("classes.json") {
            debug!("No classes.json found, skipping classes import");
            return Ok(());
        }

        progress.set_message("Importing classes...");
        
        let classes_data: serde_json::Value = bundle.parse_json_file("classes.json")?;
        let classes_array = classes_data["classes"].as_array()
            .ok_or_else(|| ImportError::invalid_entity_data("classes.json", "Missing 'classes' array"))?;

        let repo = ClassRepository::new(self.db_url.clone());
        let rule_system_id = bundle.manifest.rule_system.clone();
        
        for class_value in classes_array {
            let class_obj = class_value.as_object()
                .ok_or_else(|| ImportError::invalid_entity_data("classes.json", "Class is not an object"))?;
            
            let entries = class_obj.get("entries")
                .cloned()
                .unwrap_or_else(|| serde_json::Value::Array(vec![]));
            let mut class = Class::new(
                class_obj["id"].as_str().unwrap_or_default().to_string(),
                class_obj["name"].as_str().unwrap_or_default().to_string(),
                rule_system_id.clone(),
                class_obj["source"].as_str().unwrap_or_default().to_string(),
                class_obj.get("class_type").and_then(|v| v.as_str()).unwrap_or("class").to_string(),
                entries,
            ).map_err(|e| ImportError::JsonParsing {
                filename: "classes.json".to_string(),
                source: e,
            })?;
            
            // Set optional fields
            if let Some(page) = class_obj.get("page").and_then(|v| v.as_i64()) {
                class = class.with_page(page as i32);
            }
            
            if let Some(parent_id) = class_obj.get("parent_class_id").and_then(|v| v.as_str()) {
                class = class.with_parent_class(parent_id.to_string());
            }
            
            if let Some(hit_die) = class_obj.get("hit_die").and_then(|v| v.as_i64()) {
                class = class.with_hit_die(hit_die as i32);
            }
            
            if let Some(primary_abilities) = class_obj.get("primary_abilities").and_then(|v| v.as_array()) {
                let abilities: Vec<String> = primary_abilities.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                class = class.with_primary_abilities(abilities).map_err(|e| ImportError::JsonParsing {
                    filename: "classes.json".to_string(),
                    source: e,
                })?;
            }
            
            if let Some(saving_throws) = class_obj.get("saving_throws").and_then(|v| v.as_array()) {
                let saves: Vec<String> = saving_throws.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                class = class.with_saving_throws(saves).map_err(|e| ImportError::JsonParsing {
                    filename: "classes.json".to_string(),
                    source: e,
                })?;
            }
            
            if let Some(skill_count) = class_obj.get("skill_proficiency_count").and_then(|v| v.as_i64()) {
                class = class.with_skill_proficiency_count(skill_count as i32);
            }
            
            if let Some(skill_choices) = class_obj.get("skill_proficiency_choices").and_then(|v| v.as_array()) {
                let choices: Vec<String> = skill_choices.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                class = class.with_skill_proficiency_choices(choices).map_err(|e| ImportError::JsonParsing {
                    filename: "classes.json".to_string(),
                    source: e,
                })?;
            }
            
            // For complex JSON structures, store as JSON strings
            if let Some(profs) = class_obj.get("starting_proficiencies") {
                class.starting_proficiencies = Some(profs.to_string());
            }
            
            if let Some(equip) = class_obj.get("starting_equipment") {
                class.starting_equipment = Some(equip.to_string());
            }
            
            if let Some(spell_ability) = class_obj.get("spell_ability").and_then(|v| v.as_str()) {
                class = class.with_spell_ability(spell_ability.to_string());
            }
            
            if let Some(caster) = class_obj.get("caster_progression").and_then(|v| v.as_str()) {
                class = class.with_caster_progression(caster.to_string());
            }
            
            if let Some(title) = class_obj.get("subclass_title").and_then(|v| v.as_str()) {
                class = class.with_subclass_title(title.to_string());
            }
            
            if let Some(level) = class_obj.get("subclass_level").and_then(|v| v.as_i64()) {
                class = class.with_subclass_level(level as i32);
            }
            
            if let Some(features) = class_obj.get("features") {
                class.features = Some(features.to_string());
            }
            
            if let Some(spell_slots) = class_obj.get("spell_slots") {
                class.spell_slots = Some(spell_slots.to_string());
            }
            
            repo.create(class).await?;
            progress.inc(1);
        }

        debug!("Imported {} classes", classes_array.len());
        Ok(())
    }

    /// Import items from items.json
    async fn import_items(&self, bundle: &Bundle, progress: &mut ImportProgress) -> ImportResult<()> {
        if !bundle.has_file("items.json") {
            debug!("No items.json found, skipping items import");
            return Ok(());
        }

        progress.set_message("Importing items...");
        
        let items_data: serde_json::Value = bundle.parse_json_file("items.json")?;
        let items_array = items_data["items"].as_array()
            .ok_or_else(|| ImportError::invalid_entity_data("items.json", "Missing 'items' array"))?;

        let repo = ItemRepository::new(self.db_url.clone());
        let rule_system_id = bundle.manifest.rule_system.clone();
        
        for item_value in items_array {
            let item_obj = item_value.as_object()
                .ok_or_else(|| ImportError::invalid_entity_data("items.json", "Item is not an object"))?;
            
            let entries = item_obj.get("entries")
                .cloned()
                .unwrap_or_else(|| serde_json::Value::Array(vec![]));
            
            let mut new_item = NewItem::new(
                item_obj["id"].as_str().unwrap_or_default().to_string(),
                item_obj["name"].as_str().unwrap_or_default().to_string(),
                rule_system_id.clone(),
                item_obj["source"].as_str().unwrap_or_default().to_string(),
                entries,
            ).map_err(|e| ImportError::JsonParsing {
                filename: "items.json".to_string(),
                source: e,
            })?;
            
            // Set optional fields
            if let Some(page) = item_obj.get("page").and_then(|v| v.as_i64()) {
                new_item = new_item.with_page(page as i32);
            }
            
            if let Some(base_item_id) = item_obj.get("base_item_id").and_then(|v| v.as_str()) {
                new_item = new_item.with_base_item(base_item_id.to_string());
            }
            
            if let Some(item_type) = item_obj.get("type").and_then(|v| v.as_str()) {
                new_item = new_item.with_type(item_type.to_string());
            }
            
            if let Some(weight) = item_obj.get("weight_lb").and_then(|v| v.as_f64()) {
                new_item = new_item.with_weight(weight as f32);
            }
            
            if let Some(value) = item_obj.get("value_cp").and_then(|v| v.as_i64()) {
                new_item = new_item.with_value(value as i32);
            }
            
            if let Some(ac) = item_obj.get("armor_class").and_then(|v| v.as_i64()) {
                new_item = new_item.with_armor_class(ac as i32);
            }
            
            if let Some(damage) = item_obj.get("damage") {
                new_item.damage = Some(damage.to_string());
            }
            
            if let Some(props) = item_obj.get("properties") {
                new_item.properties = Some(props.to_string());
            }
            
            if let Some(rarity) = item_obj.get("rarity").and_then(|v| v.as_str()) {
                new_item = new_item.with_rarity(rarity.to_string());
            }
            
            new_item.requires_attunement = item_obj.get("requires_attunement").and_then(|v| v.as_bool()).unwrap_or(false);
            
            if let Some(prereq) = item_obj.get("attunement_prereq") {
                new_item.attunement_prereq = Some(prereq.to_string());
            }
            
            if let Some(bonus) = item_obj.get("magic_bonus").and_then(|v| v.as_i64()) {
                new_item = new_item.with_magic_bonus(bonus as i32);
            }
            
            if let Some(additional) = item_obj.get("additional_properties") {
                new_item.additional_properties = Some(additional.to_string());
            }
            
            // Convert NewItem to Item for the repository (it expects Item)
            let item = Item {
                id: new_item.id,
                name: new_item.name,
                rule_system_id: new_item.rule_system_id,
                source_id: new_item.source_id,
                page: new_item.page,
                base_item_id: new_item.base_item_id,
                item_type: new_item.item_type,
                weight_lb: new_item.weight_lb,
                value_cp: new_item.value_cp,
                armor_class: new_item.armor_class,
                damage: new_item.damage,
                properties: new_item.properties,
                rarity: new_item.rarity,
                requires_attunement: new_item.requires_attunement,
                attunement_prereq: new_item.attunement_prereq,
                magic_bonus: new_item.magic_bonus,
                additional_properties: new_item.additional_properties,
                entries: new_item.entries,
                is_magic: false, // This is a generated column, value doesn't matter
                created_at: new_item.created_at,
                updated_at: new_item.updated_at,
            };
            
            repo.create(item).await?;
            progress.inc(1);
        }

        debug!("Imported {} items", items_array.len());
        Ok(())
    }

    /// Import backgrounds from backgrounds.json
    async fn import_backgrounds(&self, bundle: &Bundle, progress: &mut ImportProgress) -> ImportResult<()> {
        if !bundle.has_file("backgrounds.json") {
            debug!("No backgrounds.json found, skipping backgrounds import");
            return Ok(());
        }

        progress.set_message("Importing backgrounds...");
        
        let backgrounds_data: serde_json::Value = bundle.parse_json_file("backgrounds.json")?;
        let backgrounds_array = backgrounds_data["backgrounds"].as_array()
            .ok_or_else(|| ImportError::invalid_entity_data("backgrounds.json", "Missing 'backgrounds' array"))?;

        let repo = BackgroundRepository::new(self.db_url.clone());
        let rule_system_id = bundle.manifest.rule_system.clone();
        
        for background_value in backgrounds_array {
            let background_obj = background_value.as_object()
                .ok_or_else(|| ImportError::invalid_entity_data("backgrounds.json", "Background is not an object"))?;
            
            let entries = background_obj.get("entries")
                .cloned()
                .unwrap_or_else(|| serde_json::Value::Array(vec![]));
            let mut background = Background::new(
                background_obj["id"].as_str().unwrap_or_default().to_string(),
                background_obj["name"].as_str().unwrap_or_default().to_string(),
                rule_system_id.clone(),
                background_obj["source"].as_str().unwrap_or_default().to_string(),
                entries,
            ).map_err(|e| ImportError::JsonParsing {
                filename: "backgrounds.json".to_string(),
                source: e,
            })?;
            
            // Set optional fields
            if let Some(page) = background_obj.get("page").and_then(|v| v.as_i64()) {
                background = background.with_page(page as i32);
            }
            
            if let Some(skills) = background_obj.get("skill_proficiencies") {
                background.skill_proficiencies = Some(skills.to_string());
            }
            
            if let Some(langs) = background_obj.get("language_proficiencies") {
                background.language_proficiencies = Some(langs.to_string());
            }
            
            if let Some(tools) = background_obj.get("tool_proficiencies") {
                background.tool_proficiencies = Some(tools.to_string());
            }
            
            if let Some(equip) = background_obj.get("starting_equipment") {
                background.starting_equipment = Some(equip.to_string());
            }
            
            if let Some(feat_name) = background_obj.get("feature_name").and_then(|v| v.as_str()) {
                background.feature_name = Some(feat_name.to_string());
            }
            
            if let Some(feat_text) = background_obj.get("feature_text") {
                background.feature_text = Some(feat_text.to_string());
            }
            
            repo.create(background).await?;
            progress.inc(1);
        }

        debug!("Imported {} backgrounds", backgrounds_array.len());
        Ok(())
    }

    /// Import feats from feats.json
    async fn import_feats(&self, bundle: &Bundle, progress: &mut ImportProgress) -> ImportResult<()> {
        if !bundle.has_file("feats.json") {
            debug!("No feats.json found, skipping feats import");
            return Ok(());
        }

        progress.set_message("Importing feats...");
        
        let feats_data: serde_json::Value = bundle.parse_json_file("feats.json")?;
        let feats_array = feats_data["feats"].as_array()
            .ok_or_else(|| ImportError::invalid_entity_data("feats.json", "Missing 'feats' array"))?;

        let repo = FeatRepository::new(self.db_url.clone());
        let rule_system_id = bundle.manifest.rule_system.clone();
        
        for feat_value in feats_array {
            let feat_obj = feat_value.as_object()
                .ok_or_else(|| ImportError::invalid_entity_data("feats.json", "Feat is not an object"))?;
            
            let entries = feat_obj.get("entries")
                .cloned()
                .unwrap_or_else(|| serde_json::Value::Array(vec![]));
            let mut feat = Feat::new(
                feat_obj["id"].as_str().unwrap_or_default().to_string(),
                feat_obj["name"].as_str().unwrap_or_default().to_string(),
                rule_system_id.clone(),
                feat_obj["source"].as_str().unwrap_or_default().to_string(),
                entries,
            ).map_err(|e| ImportError::JsonParsing {
                filename: "feats.json".to_string(),
                source: e,
            })?;
            
            // Set optional fields
            if let Some(page) = feat_obj.get("page").and_then(|v| v.as_i64()) {
                feat = feat.with_page(page as i32);
            }
            
            if let Some(prereqs) = feat_obj.get("prerequisites") {
                feat.prerequisites = Some(prereqs.to_string());
            }
            
            if let Some(increases) = feat_obj.get("ability_increases") {
                feat.ability_increases = Some(increases.to_string());
            }
            
            if let Some(feat_type) = feat_obj.get("feat_type").and_then(|v| v.as_str()) {
                feat = feat.with_feat_type_str(feat_type.to_string());
            }
            
            repo.create(feat).await?;
            progress.inc(1);
        }

        debug!("Imported {} feats", feats_array.len());
        Ok(())
    }

    /// Import spells from spells.json
    async fn import_spells(&self, bundle: &Bundle, progress: &mut ImportProgress) -> ImportResult<()> {
        if !bundle.has_file("spells.json") {
            debug!("No spells.json found, skipping spells import");
            return Ok(());
        }

        progress.set_message("Importing spells...");
        
        let spells_data: serde_json::Value = bundle.parse_json_file("spells.json")?;
        let spells_array = spells_data["spells"].as_array()
            .ok_or_else(|| ImportError::invalid_entity_data("spells.json", "Missing 'spells' array"))?;

        let repo = SpellRepository::new(self.db_url.clone());
        let rule_system_id = bundle.manifest.rule_system.clone();
        
        for spell_value in spells_array {
            let spell_obj = spell_value.as_object()
                .ok_or_else(|| ImportError::invalid_entity_data("spells.json", "Spell is not an object"))?;
            
            let entries = spell_obj.get("entries")
                .cloned()
                .unwrap_or_else(|| serde_json::Value::Array(vec![]));
            let mut spell = Spell::new(
                spell_obj["id"].as_str().unwrap_or_default().to_string(),
                spell_obj["name"].as_str().unwrap_or_default().to_string(),
                rule_system_id.clone(),
                spell_obj["source"].as_str().unwrap_or_default().to_string(),
                entries,
            ).map_err(|e| ImportError::JsonParsing {
                filename: "spells.json".to_string(),
                source: e,
            })?;
            
            // Set optional fields
            if let Some(page) = spell_obj.get("page").and_then(|v| v.as_i64()) {
                spell = spell.with_page(page as i32);
            }
            
            if let Some(level) = spell_obj.get("level").and_then(|v| v.as_i64()) {
                spell = spell.with_level(level as i32);
            }
            
            if let Some(school) = spell_obj.get("school").and_then(|v| v.as_str()) {
                spell = spell.with_school_str(school.to_string());
            }
            
            if let Some(casting_time) = spell_obj.get("casting_time") {
                spell.casting_time = Some(casting_time.to_string());
            }
            
            if let Some(range) = spell_obj.get("range") {
                spell.range = Some(range.to_string());
            }
            
            if let Some(components) = spell_obj.get("components") {
                spell.components = Some(components.to_string());
            }
            
            if let Some(duration) = spell_obj.get("duration") {
                spell.duration = Some(duration.to_string());
            }
            
            spell.is_ritual = spell_obj.get("is_ritual").and_then(|v| v.as_bool()).unwrap_or(false);
            spell.is_concentration = spell_obj.get("is_concentration").and_then(|v| v.as_bool()).unwrap_or(false);
            
            if let Some(saving_throw) = spell_obj.get("saving_throw") {
                if let Some(st_str) = saving_throw.as_str() {
                    spell = spell.with_saving_throw(vec![st_str.to_string()]).map_err(|e| ImportError::JsonParsing {
                        filename: "spells.json".to_string(),
                        source: e,
                    })?;
                } else if let Some(st_arr) = saving_throw.as_array() {
                    let saves: Vec<String> = st_arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    spell = spell.with_saving_throw(saves).map_err(|e| ImportError::JsonParsing {
                        filename: "spells.json".to_string(),
                        source: e,
                    })?;
                }
            }
            
            if let Some(damage_type) = spell_obj.get("damage_type") {
                if let Some(dt_str) = damage_type.as_str() {
                    spell = spell.with_damage_type(vec![dt_str.to_string()]).map_err(|e| ImportError::JsonParsing {
                        filename: "spells.json".to_string(),
                        source: e,
                    })?;
                } else if let Some(dt_arr) = damage_type.as_array() {
                    let types: Vec<String> = dt_arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    spell = spell.with_damage_type(types).map_err(|e| ImportError::JsonParsing {
                        filename: "spells.json".to_string(),
                        source: e,
                    })?;
                }
            }
            
            if let Some(upcast) = spell_obj.get("upcast_info") {
                spell.upcast_info = Some(upcast.to_string());
            }
            
            if let Some(classes) = spell_obj.get("classes") {
                spell.classes = Some(classes.to_string());
            }
            
            repo.create(spell).await?;
            progress.inc(1);
        }

        debug!("Imported {} spells", spells_array.len());
        Ok(())
    }

    /// Import creatures from creatures.json
    async fn import_creatures(&self, bundle: &Bundle, progress: &mut ImportProgress) -> ImportResult<()> {
        if !bundle.has_file("creatures.json") {
            debug!("No creatures.json found, skipping creatures import");
            return Ok(());
        }

        progress.set_message("Importing creatures...");
        
        let creatures_data: serde_json::Value = bundle.parse_json_file("creatures.json")?;
        let creatures_array = creatures_data["creatures"].as_array()
            .ok_or_else(|| ImportError::invalid_entity_data("creatures.json", "Missing 'creatures' array"))?;

        let repo = CreatureRepository::new(self.db_url.clone());
        let rule_system_id = bundle.manifest.rule_system.clone();
        
        for creature_value in creatures_array {
            let creature_obj = creature_value.as_object()
                .ok_or_else(|| ImportError::invalid_entity_data("creatures.json", "Creature is not an object"))?;
            
            let mut creature = Creature::new(
                creature_obj["id"].as_str().unwrap_or_default().to_string(),
                creature_obj["name"].as_str().unwrap_or_default().to_string(),
                rule_system_id.clone(),
                creature_obj["source"].as_str().unwrap_or_default().to_string(),
            ).map_err(|e| ImportError::JsonParsing {
                filename: "creatures.json".to_string(),
                source: e,
            })?;
            
            // Set optional fields
            if let Some(page) = creature_obj.get("page").and_then(|v| v.as_i64()) {
                creature.page = Some(page as i32);
            }
            
            if let Some(size) = creature_obj.get("size").and_then(|v| v.as_str()) {
                creature.size = Some(size.to_string());
            }
            
            if let Some(creature_type) = creature_obj.get("type").and_then(|v| v.as_str()) {
                creature.creature_type = Some(creature_type.to_string());
            }
            
            if let Some(type_tags) = creature_obj.get("type_tags") {
                creature.type_tags = Some(type_tags.to_string());
            }
            
            if let Some(alignment) = creature_obj.get("alignment").and_then(|v| v.as_str()) {
                creature.alignment = Some(alignment.to_string());
            }
            
            // Store complex fields as JSON strings
            if let Some(ac) = creature_obj.get("armor_class") {
                creature.armor_class = Some(ac.to_string());
            }
            
            if let Some(hp) = creature_obj.get("hit_points") {
                creature.hit_points = Some(hp.to_string());
            }
            
            if let Some(speed) = creature_obj.get("speed") {
                creature.speed = Some(speed.to_string());
            }
            
            if let Some(scores) = creature_obj.get("ability_scores") {
                creature.ability_scores = Some(scores.to_string());
            }
            
            if let Some(saves) = creature_obj.get("saving_throws") {
                creature.saving_throws = Some(saves.to_string());
            }
            
            if let Some(skills) = creature_obj.get("skills") {
                creature.skills = Some(skills.to_string());
            }
            
            if let Some(dr) = creature_obj.get("damage_resistances") {
                creature.damage_resistances = Some(dr.to_string());
            }
            
            if let Some(di) = creature_obj.get("damage_immunities") {
                creature.damage_immunities = Some(di.to_string());
            }
            
            if let Some(ci) = creature_obj.get("condition_immunities") {
                creature.condition_immunities = Some(ci.to_string());
            }
            
            if let Some(senses) = creature_obj.get("senses") {
                creature.senses = Some(senses.to_string());
            }
            
            if let Some(languages) = creature_obj.get("languages") {
                creature.languages = Some(languages.to_string());
            }
            
            if let Some(cr) = creature_obj.get("challenge_rating").and_then(|v| v.as_str()) {
                creature.challenge_rating = Some(cr.to_string());
            }
            
            if let Some(pb) = creature_obj.get("proficiency_bonus").and_then(|v| v.as_i64()) {
                creature.proficiency_bonus = Some(pb as i32);
            }
            
            if let Some(traits) = creature_obj.get("traits") {
                creature.traits = Some(traits.to_string());
            }
            
            if let Some(actions) = creature_obj.get("actions") {
                creature.actions = Some(actions.to_string());
            }
            
            if let Some(reactions) = creature_obj.get("reactions") {
                creature.reactions = Some(reactions.to_string());
            }
            
            if let Some(legendary) = creature_obj.get("legendary_actions") {
                creature.legendary_actions = Some(legendary.to_string());
            }
            
            if let Some(lair) = creature_obj.get("lair_actions") {
                creature.lair_actions = Some(lair.to_string());
            }
            
            if let Some(regional) = creature_obj.get("regional_effects") {
                creature.regional_effects = Some(regional.to_string());
            }
            
            if let Some(env) = creature_obj.get("environment") {
                creature.environment = Some(env.to_string());
            }
            
            creature.is_npc = creature_obj.get("is_npc").and_then(|v| v.as_bool()).unwrap_or(false);
            
            // Set entries - default to empty array if not present
            let entries = creature_obj.get("entries")
                .cloned()
                .unwrap_or_else(|| serde_json::Value::Array(vec![]));
            creature.entries = entries.to_string();
            
            repo.create(creature).await?;
            progress.inc(1);
        }

        debug!("Imported {} creatures", creatures_array.len());
        Ok(())
    }
}