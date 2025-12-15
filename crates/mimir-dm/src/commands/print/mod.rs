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
#[tauri::command]
pub async fn generate_character_sheet(
    state: State<'_, AppState>,
    character_id: i32,
    template: Option<String>,
) -> Result<ApiResponse<PrintResult>, String> {
    use mimir_dm_core::services::CharacterService;

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

    // Convert to JSON
    let data = serde_json::to_value(&character_data)
        .map_err(|e| format!("Failed to serialize character: {}", e))?;

    // Use specified template or default
    let template_id = template.unwrap_or_else(|| "character/sheet".to_string());

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
