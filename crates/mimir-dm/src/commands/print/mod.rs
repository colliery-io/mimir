//! Print command handlers for PDF generation.
//!
//! Contains Tauri commands for generating and managing PDF output
//! from Typst templates.

use crate::state::AppState;
use crate::types::ApiResponse;
use mimir_dm_print::PrintService;
use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::{debug, error, info};

/// Information about an available print template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintTemplateInfo {
    /// Template identifier (e.g., "character/sheet")
    pub id: String,
    /// Display name (e.g., "Character Sheet")
    pub name: String,
    /// Category (e.g., "character", "spell", "monster")
    pub category: String,
}

/// Result of PDF generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintResult {
    /// PDF data as base64-encoded string
    pub pdf_base64: String,
    /// Size of the PDF in bytes
    pub size_bytes: usize,
}

/// Get the templates root path.
///
/// In development, this uses the crate's templates directory.
/// In production, templates should be bundled with the app.
fn get_templates_root() -> std::path::PathBuf {
    // Try to use bundled templates first (production)
    // For now, fall back to the development path
    let dev_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("mimir-dm-print")
        .join("templates");

    if dev_path.exists() {
        dev_path
    } else {
        // In production, templates would be bundled differently
        // For now, log a warning and return the dev path
        tracing::warn!(
            "Templates directory not found at {:?}, PDF generation may fail",
            dev_path
        );
        dev_path
    }
}

/// Create a PrintService instance.
fn create_print_service() -> PrintService {
    PrintService::new(get_templates_root())
}

/// List all available print templates.
///
/// Returns a list of templates organized by category.
#[tauri::command]
pub async fn list_print_templates() -> Result<ApiResponse<Vec<PrintTemplateInfo>>, String> {
    debug!("Listing print templates");

    let service = create_print_service();

    match service.list_templates() {
        Ok(templates) => {
            let template_infos: Vec<PrintTemplateInfo> = templates
                .into_iter()
                .map(|t| PrintTemplateInfo {
                    id: t.id,
                    name: t.name,
                    category: t.category,
                })
                .collect();

            info!("Found {} print templates", template_infos.len());
            Ok(ApiResponse::success(template_infos))
        }
        Err(e) => {
            error!("Failed to list templates: {:?}", e);
            Ok(ApiResponse::error(format!(
                "Failed to list templates: {}",
                e
            )))
        }
    }
}

/// Generate a PDF from a template.
///
/// # Parameters
/// - `template_id` - Template identifier (e.g., "character/sheet.typ")
/// - `data` - JSON data to inject into the template
///
/// # Returns
/// Base64-encoded PDF data
#[tauri::command]
pub async fn generate_pdf(
    template_id: String,
    data: serde_json::Value,
) -> Result<ApiResponse<PrintResult>, String> {
    info!("Generating PDF from template: {}", template_id);
    debug!("Template data: {:?}", data);

    let service = create_print_service();

    // Ensure template has .typ extension
    let template_path = if template_id.ends_with(".typ") {
        template_id
    } else {
        format!("{}.typ", template_id)
    };

    match service.render_to_pdf(&template_path, data) {
        Ok(pdf_bytes) => {
            let size_bytes = pdf_bytes.len();
            let pdf_base64 = base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                &pdf_bytes,
            );

            info!("PDF generated successfully ({} bytes)", size_bytes);

            Ok(ApiResponse::success(PrintResult {
                pdf_base64,
                size_bytes,
            }))
        }
        Err(e) => {
            error!("Failed to generate PDF: {:?}", e);
            Ok(ApiResponse::error(format!("Failed to generate PDF: {}", e)))
        }
    }
}

/// Generate a character sheet PDF.
///
/// Convenience command for character sheet generation with proper data structure.
/// When include_spell_cards is true (default), spell cards are appended to the PDF.
/// Feature details are always fetched from the catalog.
#[tauri::command]
pub async fn generate_character_sheet(
    state: State<'_, AppState>,
    character_id: i32,
    template: Option<String>,
    include_spell_cards: Option<bool>,
) -> Result<ApiResponse<PrintResult>, String> {
    use mimir_dm_core::models::catalog::class::{ClassFeature, SubclassFeature};
    use mimir_dm_core::models::catalog::item::Item;
    use mimir_dm_core::models::catalog::Spell;
    use mimir_dm_core::services::{CharacterService, ClassService, ItemService, SpellService};
    use std::collections::HashSet;

    info!("Generating character sheet for character {}", character_id);

    // Get character data from database
    let mut conn = state
        .db
        .get_connection()
        .map_err(|e| format!("Database error: {}", e))?;

    let mut char_service = CharacterService::new(&mut conn);
    let (_character, character_data) = char_service
        .get_character(character_id)
        .map_err(|e| format!("Failed to get character: {}", e))?;

    // Determine if we should include spell cards (default to true)
    let should_include_spells = include_spell_cards.unwrap_or(true);

    // Collect all unique spell references from the character
    let mut spell_details: Vec<Spell> = Vec::new();

    if should_include_spells {
        let mut seen_spells: HashSet<(String, String)> = HashSet::new();

        // Helper closure to add spells
        let mut add_spell_refs = |refs: &[mimir_dm_core::models::character::SpellReference]| {
            for spell_ref in refs {
                let key = (spell_ref.name.clone(), spell_ref.source.clone());
                if !seen_spells.contains(&key) {
                    seen_spells.insert(key);
                }
            }
        };

        // Collect from all spell lists
        add_spell_refs(&character_data.spells.cantrips);
        add_spell_refs(&character_data.spells.known_spells);
        add_spell_refs(&character_data.spells.prepared_spells);

        // Fetch full spell details from catalog
        let mut spell_conn = state
            .db
            .get_connection()
            .map_err(|e| format!("Database error: {}", e))?;

        for (name, source) in seen_spells {
            match SpellService::get_spell_details(&mut spell_conn, &name, &source) {
                Ok(Some(spell)) => {
                    debug!("Fetched spell details for: {} from {}", name, source);
                    spell_details.push(spell);
                }
                Ok(None) => {
                    debug!("Spell not found in catalog: {} from {}", name, source);
                }
                Err(e) => {
                    error!("Failed to fetch spell {}: {}", name, e);
                }
            }
        }

        // Sort spells by level then name for consistent output
        spell_details.sort_by(|a, b| {
            a.level.cmp(&b.level).then_with(|| a.name.cmp(&b.name))
        });

        info!(
            "Fetched {} spell details for character sheet",
            spell_details.len()
        );
    }

    // Fetch feature details from catalog
    let mut class_feature_details: Vec<ClassFeature> = Vec::new();
    let mut subclass_feature_details: Vec<SubclassFeature> = Vec::new();

    {
        let mut feature_conn = state
            .db
            .get_connection()
            .map_err(|e| format!("Database error: {}", e))?;
        let mut class_service = ClassService::new(&mut feature_conn);

        for feature_ref in &character_data.class_features {
            if let Some(ref subclass_name) = feature_ref.subclass_name {
                // Try to fetch as subclass feature
                match class_service.get_subclass_feature(
                    &feature_ref.name,
                    &feature_ref.class_name,
                    subclass_name,
                    &feature_ref.source,
                ) {
                    Ok(Some(feature)) => {
                        debug!(
                            "Fetched subclass feature details for: {} ({} {})",
                            feature_ref.name, feature_ref.class_name, subclass_name
                        );
                        subclass_feature_details.push(feature);
                    }
                    Ok(None) => {
                        debug!(
                            "Subclass feature not found in catalog: {} ({} {})",
                            feature_ref.name, feature_ref.class_name, subclass_name
                        );
                    }
                    Err(e) => {
                        error!("Failed to fetch subclass feature {}: {}", feature_ref.name, e);
                    }
                }
            } else {
                // Fetch as class feature
                match class_service.get_class_feature(
                    &feature_ref.name,
                    &feature_ref.class_name,
                    &feature_ref.source,
                ) {
                    Ok(Some(feature)) => {
                        debug!(
                            "Fetched class feature details for: {} ({})",
                            feature_ref.name, feature_ref.class_name
                        );
                        class_feature_details.push(feature);
                    }
                    Ok(None) => {
                        debug!(
                            "Class feature not found in catalog: {} ({})",
                            feature_ref.name, feature_ref.class_name
                        );
                    }
                    Err(e) => {
                        error!("Failed to fetch class feature {}: {}", feature_ref.name, e);
                    }
                }
            }
        }

        info!(
            "Fetched {} class features and {} subclass features for character sheet",
            class_feature_details.len(),
            subclass_feature_details.len()
        );
    }

    // Fetch item details from catalog for inventory items
    let mut item_details: Vec<Item> = Vec::new();
    {
        let mut item_conn = state
            .db
            .get_connection()
            .map_err(|e| format!("Database error: {}", e))?;
        let mut item_service = ItemService::new(&mut item_conn);

        for inventory_item in &character_data.inventory {
            let source = inventory_item.source.as_deref().unwrap_or("PHB");
            match item_service.get_item_by_name_and_source(&inventory_item.name, source) {
                Ok(Some(item)) => {
                    debug!(
                        "Fetched item details for: {} from {}",
                        inventory_item.name, source
                    );
                    item_details.push(item);
                }
                Ok(None) => {
                    debug!(
                        "Item not found in catalog: {} from {}",
                        inventory_item.name, source
                    );
                }
                Err(e) => {
                    error!("Failed to fetch item {}: {}", inventory_item.name, e);
                }
            }
        }

        info!(
            "Fetched {} item details for character sheet",
            item_details.len()
        );
    }

    // Convert character to JSON
    let character_json = serde_json::to_value(&character_data)
        .map_err(|e| format!("Failed to serialize character: {}", e))?;

    // Convert spells to JSON
    let spells_json = serde_json::to_value(&spell_details)
        .map_err(|e| format!("Failed to serialize spells: {}", e))?;

    // Convert features to JSON
    let class_features_json = serde_json::to_value(&class_feature_details)
        .map_err(|e| format!("Failed to serialize class features: {}", e))?;
    let subclass_features_json = serde_json::to_value(&subclass_feature_details)
        .map_err(|e| format!("Failed to serialize subclass features: {}", e))?;

    // Convert item details to JSON
    let item_details_json = serde_json::to_value(&item_details)
        .map_err(|e| format!("Failed to serialize item details: {}", e))?;

    // Build combined data structure
    let data = serde_json::json!({
        "character": character_json,
        "spells": spells_json,
        "class_features_details": class_features_json,
        "subclass_features_details": subclass_features_json,
        "item_details": item_details_json,
        "include_spell_cards": should_include_spells && !spell_details.is_empty()
    });

    // Always use the combined template which handles spells, equipment, and features
    // The template conditionally shows sections based on what data is available
    let template_id = template.unwrap_or_else(|| "character/sheet-with-spells".to_string());

    generate_pdf(template_id, data).await
}

/// Generate a spell card or list PDF.
///
/// # Parameters
/// - `template` - Template to use (card, list, cards-multiup)
/// - `spells` - Array of spell data
/// - `options` - Additional options (title, show_description, show_cut_lines)
#[tauri::command]
pub async fn generate_spell_pdf(
    template: String,
    spells: Vec<serde_json::Value>,
    options: Option<serde_json::Value>,
) -> Result<ApiResponse<PrintResult>, String> {
    info!("Generating spell PDF with template: {}", template);

    let template_id = format!("spells/{}", template);

    // Build data structure based on template
    let data = match template.as_str() {
        "card" => {
            // Single spell card - use first spell
            spells.into_iter().next().unwrap_or(serde_json::json!({}))
        }
        "list" => {
            let mut data = serde_json::json!({
                "spells": spells
            });
            if let Some(opts) = options {
                if let serde_json::Value::Object(opts_map) = opts {
                    if let serde_json::Value::Object(ref mut data_map) = data {
                        for (k, v) in opts_map {
                            data_map.insert(k, v);
                        }
                    }
                }
            }
            data
        }
        "cards-multiup" => {
            let mut data = serde_json::json!({
                "spells": spells,
                "show_cut_lines": true
            });
            if let Some(opts) = options {
                if let serde_json::Value::Object(opts_map) = opts {
                    if let serde_json::Value::Object(ref mut data_map) = data {
                        for (k, v) in opts_map {
                            data_map.insert(k, v);
                        }
                    }
                }
            }
            data
        }
        _ => serde_json::json!({ "spells": spells }),
    };

    generate_pdf(template_id, data).await
}

/// Generate a monster stat block or card PDF.
///
/// # Parameters
/// - `template` - Template to use (stat-block, card, encounter, cards-multiup)
/// - `monsters` - Array of monster data
/// - `options` - Additional options (title, notes, show_cut_lines)
#[tauri::command]
pub async fn generate_monster_pdf(
    template: String,
    monsters: Vec<serde_json::Value>,
    options: Option<serde_json::Value>,
) -> Result<ApiResponse<PrintResult>, String> {
    info!("Generating monster PDF with template: {}", template);

    let template_id = format!("monsters/{}", template);

    // Build data structure based on template
    let data = match template.as_str() {
        "stat-block" | "card" => {
            // Single monster - use first
            monsters.into_iter().next().unwrap_or(serde_json::json!({}))
        }
        "encounter" | "cards-multiup" => {
            let mut data = serde_json::json!({
                "monsters": monsters
            });
            if let Some(opts) = options {
                if let serde_json::Value::Object(opts_map) = opts {
                    if let serde_json::Value::Object(ref mut data_map) = data {
                        for (k, v) in opts_map {
                            data_map.insert(k, v);
                        }
                    }
                }
            }
            data
        }
        _ => serde_json::json!({ "monsters": monsters }),
    };

    generate_pdf(template_id, data).await
}

/// Generate a session prep sheet or NPC card PDF.
///
/// # Parameters
/// - `template` - Template to use (prep, npc-card, npc-cards-multiup, handout)
/// - `data` - Session or NPC data
#[tauri::command]
pub async fn generate_session_pdf(
    template: String,
    data: serde_json::Value,
) -> Result<ApiResponse<PrintResult>, String> {
    info!("Generating session PDF with template: {}", template);

    let template_id = format!("session/{}", template);
    generate_pdf(template_id, data).await
}

/// Save a PDF to the file system.
///
/// # Parameters
/// - `pdf_base64` - Base64-encoded PDF data
/// - `path` - File path to save to
#[tauri::command]
pub async fn save_pdf(pdf_base64: String, path: String) -> Result<ApiResponse<()>, String> {
    info!("Saving PDF to: {}", path);

    let pdf_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &pdf_base64)
        .map_err(|e| format!("Failed to decode PDF: {}", e))?;

    std::fs::write(&path, &pdf_bytes).map_err(|e| format!("Failed to write file: {}", e))?;

    info!("PDF saved successfully ({} bytes)", pdf_bytes.len());
    Ok(ApiResponse::success(()))
}

/// Export a single campaign document to PDF.
///
/// Reads the markdown document from disk, converts to Typst, and generates PDF.
///
/// # Parameters
/// - `document_id` - Database ID of the document
///
/// # Returns
/// PrintResult with base64-encoded PDF
#[tauri::command]
pub async fn export_campaign_document(
    state: State<'_, AppState>,
    document_id: i32,
) -> Result<ApiResponse<PrintResult>, String> {
    use mimir_dm_core::dal::campaign::campaigns::CampaignRepository;
    use mimir_dm_core::dal::campaign::documents::DocumentRepository;

    info!("Exporting campaign document {} to PDF", document_id);

    // Get the document from the database
    let mut conn = state
        .db
        .get_connection()
        .map_err(|e| format!("Database error: {}", e))?;

    let document = DocumentRepository::find_by_id(&mut conn, document_id)
        .map_err(|e| format!("Failed to get document: {}", e))?;

    // Get the campaign name
    let mut campaign_repo = CampaignRepository::new(&mut conn);
    let campaign = campaign_repo
        .find_by_id(document.campaign_id)
        .map_err(|e| format!("Failed to get campaign: {}", e))?
        .ok_or_else(|| format!("Campaign {} not found", document.campaign_id))?;

    // Create the print service and render
    let service = create_print_service();
    let file_path = std::path::PathBuf::from(&document.file_path);

    match service.render_campaign_document(&file_path, Some(&campaign.name)) {
        Ok(pdf_bytes) => {
            let size_bytes = pdf_bytes.len();
            let pdf_base64 = base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                &pdf_bytes,
            );

            info!(
                "Campaign document PDF generated successfully ({} bytes)",
                size_bytes
            );

            Ok(ApiResponse::success(PrintResult {
                pdf_base64,
                size_bytes,
            }))
        }
        Err(e) => {
            error!("Failed to generate campaign document PDF: {:?}", e);
            Ok(ApiResponse::error(format!(
                "Failed to generate PDF: {}",
                e
            )))
        }
    }
}

/// Export all campaign documents as a combined PDF.
///
/// Reads all markdown documents for the campaign, converts to Typst,
/// and generates a single PDF with cover page and table of contents.
///
/// # Parameters
/// - `campaign_id` - Database ID of the campaign
///
/// # Returns
/// PrintResult with base64-encoded PDF
#[tauri::command]
pub async fn export_campaign_documents(
    state: State<'_, AppState>,
    campaign_id: i32,
) -> Result<ApiResponse<PrintResult>, String> {
    use mimir_dm_core::dal::campaign::campaigns::CampaignRepository;
    use mimir_dm_core::services::DocumentService;

    info!("Exporting all campaign {} documents to PDF", campaign_id);

    // Get the campaign
    let mut conn = state
        .db
        .get_connection()
        .map_err(|e| format!("Database error: {}", e))?;

    let mut campaign_repo = CampaignRepository::new(&mut conn);
    let campaign = campaign_repo
        .find_by_id(campaign_id)
        .map_err(|e| format!("Failed to get campaign: {}", e))?
        .ok_or_else(|| format!("Campaign {} not found", campaign_id))?;

    // Get all documents for the campaign
    let mut doc_conn = state
        .db
        .get_connection()
        .map_err(|e| format!("Database error: {}", e))?;

    let mut doc_service = DocumentService::new(&mut doc_conn);
    let documents = doc_service
        .get_campaign_documents(campaign_id)
        .map_err(|e| format!("Failed to get documents: {}", e))?;

    if documents.is_empty() {
        return Ok(ApiResponse::error("No documents to export".to_string()));
    }

    // Collect file paths for documents that exist
    let file_paths: Vec<std::path::PathBuf> = documents
        .iter()
        .filter_map(|doc| {
            let path = std::path::PathBuf::from(&doc.file_path);
            if path.exists() {
                Some(path)
            } else {
                debug!("Skipping non-existent document file: {:?}", path);
                None
            }
        })
        .collect();

    if file_paths.is_empty() {
        return Ok(ApiResponse::error(
            "No document files found on disk".to_string(),
        ));
    }

    info!(
        "Rendering {} documents for campaign '{}'",
        file_paths.len(),
        campaign.name
    );

    // Create the print service and render combined PDF
    let service = create_print_service();

    match service.render_campaign_combined(&file_paths, &campaign.name) {
        Ok(pdf_bytes) => {
            let size_bytes = pdf_bytes.len();
            let pdf_base64 = base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                &pdf_bytes,
            );

            info!(
                "Combined campaign PDF generated successfully ({} bytes, {} documents)",
                size_bytes,
                file_paths.len()
            );

            Ok(ApiResponse::success(PrintResult {
                pdf_base64,
                size_bytes,
            }))
        }
        Err(e) => {
            error!("Failed to generate combined campaign PDF: {:?}", e);
            Ok(ApiResponse::error(format!(
                "Failed to generate PDF: {}",
                e
            )))
        }
    }
}
