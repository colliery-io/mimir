//! PrintService - Core service for PDF generation

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};
use typst::diag::{SourceDiagnostic, Severity};

use crate::error::{PrintError, Result};
use crate::world::MimirTypstWorld;

/// Information about an available template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    /// Template identifier (relative path without .typ extension)
    pub id: String,
    /// Display name
    pub name: String,
    /// Template category (e.g., "character", "spell", "monster")
    pub category: String,
    /// Description of what this template produces
    pub description: Option<String>,
}

/// Service for generating PDFs from Typst templates
pub struct PrintService {
    /// Root directory containing templates
    templates_root: PathBuf,
}

impl PrintService {
    /// Create a new PrintService
    ///
    /// # Arguments
    /// * `templates_root` - Root directory containing Typst templates
    pub fn new(templates_root: PathBuf) -> Self {
        Self { templates_root }
    }

    /// Get the templates root directory
    pub fn templates_root(&self) -> &PathBuf {
        &self.templates_root
    }

    /// Render a template to PDF bytes
    ///
    /// # Arguments
    /// * `template_path` - Path to template relative to templates root (e.g., "character/sheet.typ")
    /// * `data` - JSON data to inject into the template
    ///
    /// # Returns
    /// PDF file contents as bytes
    #[instrument(skip(self, data), fields(template = %template_path))]
    pub fn render_to_pdf(
        &self,
        template_path: &str,
        data: serde_json::Value,
    ) -> Result<Vec<u8>> {
        info!("Rendering template to PDF");

        // Create world with template and data
        let world = MimirTypstWorld::new(
            self.templates_root.clone(),
            template_path,
            data,
        )?;

        // Compile the document
        debug!("Compiling Typst document");
        let warned = typst::compile(&world);

        // Log any warnings
        for warning in &warned.warnings {
            tracing::warn!("Typst warning: {}", warning.message);
        }

        match warned.output {
            Ok(document) => {
                // Generate PDF
                debug!("Generating PDF from compiled document");
                let pdf_result = typst_pdf::pdf(&document, &typst_pdf::PdfOptions::default());

                match pdf_result {
                    Ok(pdf_bytes) => {
                        info!("PDF generated successfully ({} bytes)", pdf_bytes.len());
                        Ok(pdf_bytes)
                    }
                    Err(errors) => {
                        let error_msg = format_diagnostics(&errors);
                        Err(PrintError::PdfError(error_msg))
                    }
                }
            }
            Err(errors) => {
                let error_msg = format_diagnostics(&errors);
                Err(PrintError::CompilationError(error_msg))
            }
        }
    }

    /// Save PDF bytes to a file
    #[instrument(skip(self, pdf_bytes))]
    pub fn save_pdf(&self, path: &PathBuf, pdf_bytes: &[u8]) -> Result<()> {
        info!("Saving PDF to {:?}", path);
        std::fs::write(path, pdf_bytes)?;
        Ok(())
    }

    /// List all available templates
    #[instrument(skip(self))]
    pub fn list_templates(&self) -> Result<Vec<TemplateInfo>> {
        let mut templates = Vec::new();

        if !self.templates_root.exists() {
            return Ok(templates);
        }

        for entry in walkdir::WalkDir::new(&self.templates_root)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Skip directories and non-.typ files
            if !path.is_file() || path.extension().map_or(true, |e| e != "typ") {
                continue;
            }

            // Skip files in _shared directory (these are imports, not templates)
            if path.components().any(|c| c.as_os_str() == "_shared") {
                continue;
            }

            // Get relative path without extension
            if let Ok(rel_path) = path.strip_prefix(&self.templates_root) {
                let id = rel_path
                    .with_extension("")
                    .to_string_lossy()
                    .replace('\\', "/");

                // Determine category from first directory component
                let category = rel_path
                    .components()
                    .next()
                    .map(|c| c.as_os_str().to_string_lossy().into_owned())
                    .unwrap_or_else(|| "general".to_string());

                // Use filename as display name
                let name = path
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| id.clone());

                templates.push(TemplateInfo {
                    id,
                    name: titlecase(&name),
                    category,
                    description: None,
                });
            }
        }

        templates.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(templates)
    }

    /// Check if a template exists
    pub fn template_exists(&self, template_path: &str) -> bool {
        self.templates_root.join(template_path).exists()
    }
}

/// Format Typst diagnostics into a readable error message
fn format_diagnostics(diagnostics: &[SourceDiagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diag| {
            let severity = match diag.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
            };
            format!("{}: {}", severity, diag.message)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Convert a string to title case
fn titlecase(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' {
            result.push(' ');
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_template(dir: &TempDir) -> PathBuf {
        let template = r#"
#set page(width: 8.5in, height: 11in, margin: 1in)
#set text(font: "DejaVu Sans", size: 12pt)

= Hello World

This is a test document.

#if data != none [
  Data received: #data
]
"#;
        let template_path = dir.path().join("test/hello.typ");
        fs::create_dir_all(template_path.parent().unwrap()).unwrap();
        fs::write(&template_path, template).unwrap();
        template_path
    }

    #[test]
    fn test_service_creation() {
        let temp = TempDir::new().unwrap();
        let service = PrintService::new(temp.path().to_path_buf());
        assert_eq!(service.templates_root(), temp.path());
    }

    #[test]
    fn test_list_templates_empty() {
        let temp = TempDir::new().unwrap();
        let service = PrintService::new(temp.path().to_path_buf());
        let templates = service.list_templates().unwrap();
        assert!(templates.is_empty());
    }

    #[test]
    fn test_list_templates() {
        let temp = TempDir::new().unwrap();
        setup_test_template(&temp);

        let service = PrintService::new(temp.path().to_path_buf());
        let templates = service.list_templates().unwrap();

        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].id, "test/hello");
        assert_eq!(templates[0].category, "test");
    }

    #[test]
    fn test_template_exists() {
        let temp = TempDir::new().unwrap();
        setup_test_template(&temp);

        let service = PrintService::new(temp.path().to_path_buf());
        assert!(service.template_exists("test/hello.typ"));
        assert!(!service.template_exists("nonexistent.typ"));
    }

    #[test]
    fn test_titlecase() {
        assert_eq!(titlecase("hello_world"), "Hello World");
        assert_eq!(titlecase("character-sheet"), "Character Sheet");
        assert_eq!(titlecase("test"), "Test");
    }

    #[test]
    fn test_render_to_pdf() {
        let temp = TempDir::new().unwrap();

        // Create a simple template that doesn't require specific fonts
        let template = r#"
#set page(width: 8.5in, height: 11in, margin: 1in)

= Hello World

This is a test document.

#if "name" in data [
  Name: #data.name
]
"#;
        let template_path = temp.path().join("test/simple.typ");
        fs::create_dir_all(template_path.parent().unwrap()).unwrap();
        fs::write(&template_path, template).unwrap();

        let service = PrintService::new(temp.path().to_path_buf());
        let data = serde_json::json!({
            "name": "Test User"
        });

        let result = service.render_to_pdf("test/simple.typ", data);
        assert!(result.is_ok(), "PDF generation failed: {:?}", result.err());

        let pdf_bytes = result.unwrap();
        // PDF files start with %PDF
        assert!(pdf_bytes.len() > 100, "PDF seems too small");
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Output is not a valid PDF");
    }

    #[test]
    fn test_render_with_shared_components() {
        // Use the actual templates directory from the crate
        let templates_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");

        let service = PrintService::new(templates_root);
        let data = serde_json::json!({});

        let result = service.render_to_pdf("test/components-test.typ", data);
        assert!(result.is_ok(), "Component test template failed: {:?}", result.err());

        let pdf_bytes = result.unwrap();
        assert!(pdf_bytes.len() > 1000, "PDF seems too small for a multi-page document");
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Output is not a valid PDF");
    }

    #[test]
    fn test_render_character_sheet() {
        let templates_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
        let service = PrintService::new(templates_root);

        // Sample character data matching CharacterData structure
        let data = serde_json::json!({
            "character_name": "Thorin Ironforge",
            "player_id": 1,
            "level": 5,
            "experience_points": 6500,
            "version": 1,
            "created_at": "2025-01-01",
            "race": "Dwarf",
            "subrace": "Mountain",
            "classes": [{
                "class_name": "Fighter",
                "level": 5,
                "subclass": "Champion",
                "hit_dice_type": "d10",
                "hit_dice_remaining": 5
            }],
            "background": "Soldier",
            "alignment": "Lawful Good",
            "abilities": {
                "strength": 18,
                "dexterity": 12,
                "constitution": 16,
                "intelligence": 10,
                "wisdom": 13,
                "charisma": 8
            },
            "max_hp": 44,
            "current_hp": 44,
            "speed": 25,
            "proficiencies": {
                "skills": ["Athletics", "Intimidation", "Perception"],
                "saves": ["Strength", "Constitution"],
                "armor": ["All armor", "Shields"],
                "weapons": ["Simple weapons", "Martial weapons"],
                "tools": ["Smith's tools"],
                "languages": ["Common", "Dwarvish"]
            },
            "class_features": [
                "Fighting Style (Defense)",
                "Second Wind",
                "Action Surge",
                "Improved Critical"
            ],
            "feats": [],
            "spells": {
                "cantrips": [],
                "prepared_spells": [],
                "known_spells": [],
                "spell_slots": {}
            },
            "inventory": [
                {"name": "Rations", "quantity": 10, "weight": 20.0, "value": 5.0},
                {"name": "Rope (50 ft)", "quantity": 1, "weight": 10.0, "value": 1.0}
            ],
            "currency": {
                "copper": 15,
                "silver": 30,
                "gold": 50,
                "platinum": 0
            },
            "equipped": {
                "armor": "Chain Mail",
                "shield": "Shield",
                "main_hand": "Warhammer",
                "off_hand": null
            },
            "personality": {
                "traits": "I'm always polite and respectful.",
                "ideals": "Responsibility. I do what I must and obey authority.",
                "bonds": "I would still lay down my life for the people I served with.",
                "flaws": "I made a terrible mistake in battle that cost lives."
            }
        });

        let result = service.render_to_pdf("character/sheet.typ", data.clone());
        assert!(result.is_ok(), "Character sheet render failed: {:?}", result.err());

        let pdf_bytes = result.unwrap();
        assert!(pdf_bytes.len() > 5000, "Character sheet PDF seems too small");
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Output is not a valid PDF");

        // Also test summary template
        let result = service.render_to_pdf("character/summary.typ", data);
        assert!(result.is_ok(), "Character summary render failed: {:?}", result.err());

        let pdf_bytes = result.unwrap();
        assert!(pdf_bytes.len() > 1000, "Character summary PDF seems too small");
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Output is not a valid PDF");
    }

    #[test]
    fn test_render_spell_templates() {
        let templates_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
        let service = PrintService::new(templates_root);

        // Sample spell data (simplified SpellSummary format)
        let fireball = serde_json::json!({
            "name": "Fireball",
            "level": 3,
            "school": "Evocation",
            "source": "PHB",
            "casting_time": "1 action",
            "range": "150 feet",
            "components": "V, S, M",
            "concentration": false,
            "ritual": false,
            "description": "A bright streak flashes from your pointing finger to a point you choose within range and then blossoms with a low roar into an explosion of flame. Each creature in a 20-foot-radius sphere centered on that point must make a Dexterity saving throw. A target takes 8d6 fire damage on a failed save, or half as much damage on a successful one.",
            "classes": ["Sorcerer", "Wizard"]
        });

        // Test single spell card
        let result = service.render_to_pdf("spells/card.typ", fireball.clone());
        assert!(result.is_ok(), "Spell card render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Spell card is not a valid PDF");

        // Test spell list
        let spell_list_data = serde_json::json!({
            "title": "Wizard Spells",
            "show_description": false,
            "spells": [
                {
                    "name": "Fire Bolt",
                    "level": 0,
                    "school": "Evocation",
                    "casting_time": "1 action",
                    "range": "120 feet",
                    "components": "V, S",
                    "concentration": false,
                    "ritual": false,
                    "description": "You hurl a mote of fire at a creature or object within range."
                },
                {
                    "name": "Magic Missile",
                    "level": 1,
                    "school": "Evocation",
                    "casting_time": "1 action",
                    "range": "120 feet",
                    "components": "V, S",
                    "concentration": false,
                    "ritual": false,
                    "description": "You create three glowing darts of magical force."
                },
                fireball.clone()
            ]
        });

        let result = service.render_to_pdf("spells/list.typ", spell_list_data.clone());
        assert!(result.is_ok(), "Spell list render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Spell list is not a valid PDF");

        // Test multi-up cards
        let multiup_data = serde_json::json!({
            "spells": spell_list_data["spells"],
            "show_cut_lines": true
        });

        let result = service.render_to_pdf("spells/cards-multiup.typ", multiup_data);
        assert!(result.is_ok(), "Multi-up cards render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Multi-up cards is not a valid PDF");
    }
}
