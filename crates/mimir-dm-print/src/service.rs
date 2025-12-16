//! PrintService - Core service for PDF generation

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};
use typst::diag::{SourceDiagnostic, Severity};

use crate::error::{PrintError, Result};
use crate::markdown::{parse_campaign_document, ParsedDocument};
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

    /// Render a campaign document (markdown with YAML frontmatter) to PDF
    ///
    /// # Arguments
    /// * `file_path` - Path to the markdown document file
    /// * `campaign_name` - Name of the campaign (optional, used in header)
    ///
    /// # Returns
    /// PDF file contents as bytes
    #[instrument(skip(self), fields(file = %file_path.display()))]
    pub fn render_campaign_document(
        &self,
        file_path: &PathBuf,
        campaign_name: Option<&str>,
    ) -> Result<Vec<u8>> {
        info!("Rendering campaign document to PDF");

        // Read the markdown file
        let markdown = std::fs::read_to_string(file_path)?;

        // Parse the document
        let parsed = parse_campaign_document(&markdown)?;

        // Build the data structure for the template
        let data = self.build_campaign_document_data(&parsed, campaign_name)?;

        // Render using the campaign document template
        self.render_to_pdf("campaign/document.typ", data)
    }

    /// Render multiple campaign documents as a single combined PDF
    ///
    /// # Arguments
    /// * `documents` - List of document file paths
    /// * `campaign_name` - Name of the campaign
    ///
    /// # Returns
    /// PDF file contents as bytes
    #[instrument(skip(self, documents), fields(count = documents.len()))]
    pub fn render_campaign_combined(
        &self,
        documents: &[PathBuf],
        campaign_name: &str,
    ) -> Result<Vec<u8>> {
        info!("Rendering {} campaign documents to combined PDF", documents.len());

        // Parse all documents
        let mut parsed_docs = Vec::new();
        for file_path in documents {
            debug!("Reading document: {:?}", file_path);
            let markdown = std::fs::read_to_string(file_path)?;
            let parsed = parse_campaign_document(&markdown)?;
            parsed_docs.push(parsed);
        }

        // Build the combined data structure
        let data = self.build_campaign_combined_data(&parsed_docs, campaign_name)?;

        // Render using the combined campaign template
        self.render_to_pdf("campaign/combined.typ", data)
    }

    /// Build the data structure for a single campaign document template
    fn build_campaign_document_data(
        &self,
        parsed: &ParsedDocument,
        campaign_name: Option<&str>,
    ) -> Result<serde_json::Value> {
        // Extract title from frontmatter or use default
        let title = parsed
            .frontmatter
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled Document");

        // Extract document type from frontmatter or use default
        let document_type = parsed
            .frontmatter
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("document");

        let mut data = serde_json::json!({
            "title": title,
            "document_type": document_type,
            "content": parsed.typst_content,
        });

        if let Some(name) = campaign_name {
            data["campaign_name"] = serde_json::Value::String(name.to_string());
        }

        Ok(data)
    }

    /// Build the data structure for the combined campaign template
    fn build_campaign_combined_data(
        &self,
        documents: &[ParsedDocument],
        campaign_name: &str,
    ) -> Result<serde_json::Value> {
        let docs: Vec<serde_json::Value> = documents
            .iter()
            .map(|parsed| {
                let title = parsed
                    .frontmatter
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Untitled Document");

                let document_type = parsed
                    .frontmatter
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("document");

                serde_json::json!({
                    "title": title,
                    "document_type": document_type,
                    "content": parsed.typst_content,
                })
            })
            .collect();

        Ok(serde_json::json!({
            "campaign_name": campaign_name,
            "documents": docs,
        }))
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

    #[test]
    fn test_render_monster_templates() {
        let templates_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
        let service = PrintService::new(templates_root);

        // Sample monster data (matches Monster struct format)
        let goblin = serde_json::json!({
            "name": "Goblin",
            "source": "MM",
            "size": ["S"],
            "creature_type": "humanoid",
            "alignment": ["N", "E"],
            "ac": [{"ac": 15, "from": ["leather armor", "shield"]}],
            "hp": {"average": 7, "formula": "2d6"},
            "speed": {"walk": 30},
            "str": 8,
            "dex": 14,
            "con": 10,
            "int": 10,
            "wis": 8,
            "cha": 8,
            "skill": {"stealth": "+6"},
            "senses": ["darkvision 60 ft."],
            "passive": 9,
            "languages": ["Common", "Goblin"],
            "cr": "1/4",
            "trait_entries": [{
                "name": "Nimble Escape",
                "entries": ["The goblin can take the Disengage or Hide action as a bonus action on each of its turns."]
            }],
            "action": [
                {
                    "name": "Scimitar",
                    "entries": ["Melee Weapon Attack: +4 to hit, reach 5 ft., one target. Hit: 5 (1d6 + 2) slashing damage."]
                },
                {
                    "name": "Shortbow",
                    "entries": ["Ranged Weapon Attack: +4 to hit, range 80/320 ft., one target. Hit: 5 (1d6 + 2) piercing damage."]
                }
            ]
        });

        let dragon = serde_json::json!({
            "name": "Adult Red Dragon",
            "source": "MM",
            "size": ["H"],
            "creature_type": {"type": "dragon"},
            "alignment": ["C", "E"],
            "ac": [{"ac": 19, "from": ["natural armor"]}],
            "hp": {"average": 256, "formula": "19d12 + 133"},
            "speed": {"walk": 40, "climb": 40, "fly": 80},
            "str": 27,
            "dex": 10,
            "con": 25,
            "int": 16,
            "wis": 13,
            "cha": 21,
            "save": {"dex": "+6", "con": "+13", "wis": "+7", "cha": "+11"},
            "skill": {"perception": "+13", "stealth": "+6"},
            "damage_immunities": ["fire"],
            "senses": ["blindsight 60 ft.", "darkvision 120 ft."],
            "passive": 23,
            "languages": ["Common", "Draconic"],
            "cr": "17",
            "trait_entries": [{
                "name": "Legendary Resistance (3/Day)",
                "entries": ["If the dragon fails a saving throw, it can choose to succeed instead."]
            }],
            "action": [
                {
                    "name": "Multiattack",
                    "entries": ["The dragon can use its Frightful Presence. It then makes three attacks: one with its bite and two with its claws."]
                },
                {
                    "name": "Bite",
                    "entries": ["Melee Weapon Attack: +14 to hit, reach 10 ft., one target. Hit: 19 (2d10 + 8) piercing damage plus 7 (2d6) fire damage."]
                },
                {
                    "name": "Fire Breath (Recharge 5-6)",
                    "entries": ["The dragon exhales fire in a 60-foot cone. Each creature in that area must make a DC 21 Dexterity saving throw, taking 63 (18d6) fire damage on a failed save, or half as much damage on a successful one."]
                }
            ],
            "legendary": [
                {
                    "name": "Detect",
                    "entries": ["The dragon makes a Wisdom (Perception) check."]
                },
                {
                    "name": "Tail Attack",
                    "entries": ["The dragon makes a tail attack."]
                },
                {
                    "name": "Wing Attack (Costs 2 Actions)",
                    "entries": ["The dragon beats its wings. Each creature within 10 feet must succeed on a DC 22 Dexterity saving throw or take 15 (2d6 + 8) bludgeoning damage and be knocked prone."]
                }
            ]
        });

        // Test single monster stat block
        let result = service.render_to_pdf("monsters/stat-block.typ", goblin.clone());
        assert!(result.is_ok(), "Monster stat-block render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Monster stat-block is not a valid PDF");

        // Test monster card
        let result = service.render_to_pdf("monsters/card.typ", goblin.clone());
        assert!(result.is_ok(), "Monster card render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Monster card is not a valid PDF");

        // Test encounter template
        let encounter_data = serde_json::json!({
            "title": "Goblin Ambush",
            "monsters": [goblin.clone(), goblin.clone(), goblin.clone()],
            "notes": "The goblins attack from hiding in the trees. They flee if two are killed."
        });

        let result = service.render_to_pdf("monsters/encounter.typ", encounter_data);
        assert!(result.is_ok(), "Encounter render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Encounter is not a valid PDF");

        // Test multi-up monster cards
        let multiup_data = serde_json::json!({
            "monsters": [goblin.clone(), goblin.clone(), dragon.clone()],
            "show_cut_lines": true
        });

        let result = service.render_to_pdf("monsters/cards-multiup.typ", multiup_data);
        assert!(result.is_ok(), "Multi-up monster cards render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Multi-up monster cards is not a valid PDF");

        // Test dragon stat block (with legendary actions)
        let result = service.render_to_pdf("monsters/stat-block.typ", dragon);
        assert!(result.is_ok(), "Dragon stat-block render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert!(pdf_bytes.len() > 5000, "Dragon stat-block PDF seems too small for complex creature");
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Dragon stat-block is not a valid PDF");
    }

    #[test]
    fn test_render_session_templates() {
        let templates_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
        let service = PrintService::new(templates_root);

        // Sample NPC data
        let sildar = serde_json::json!({
            "name": "Sildar Hallwinter",
            "race": "Human",
            "occupation": "Knight",
            "role": "Ally",
            "alignment": "Lawful Good",
            "location": "Phandalin",
            "appearance": "Middle-aged man with a graying beard and worn armor",
            "personality": "Honorable and direct, speaks plainly",
            "mannerisms": "Strokes his beard when thinking",
            "voice": "Deep and commanding",
            "goal": "Find Iarno Albrek and establish order in Phandalin",
            "motivation": "Duty to the Lords' Alliance",
            "bond": "Fellow Alliance members",
            "flaw": "Too trusting of authority figures",
            "secret": "Member of the Lords' Alliance on a secret mission",
            "key_info": "Knows location of Cragmaw Castle"
        });

        let gundren = serde_json::json!({
            "name": "Gundren Rockseeker",
            "race": "Dwarf",
            "occupation": "Prospector",
            "role": "Quest Giver",
            "alignment": "Neutral Good",
            "location": "Cragmaw Hideout (captured)",
            "appearance": "Stocky dwarf with a braided brown beard",
            "personality": "Enthusiastic and secretive about his discovery",
            "goal": "Reach Wave Echo Cave and claim his inheritance",
            "motivation": "Family legacy and wealth",
            "flaw": "Greed can cloud his judgment"
        });

        // Test NPC card
        let result = service.render_to_pdf("session/npc-card.typ", sildar.clone());
        assert!(result.is_ok(), "NPC card render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "NPC card is not a valid PDF");

        // Test multi-up NPC cards
        let multiup_data = serde_json::json!({
            "npcs": [sildar.clone(), gundren.clone(), sildar.clone(), gundren.clone()],
            "show_cut_lines": true
        });

        let result = service.render_to_pdf("session/npc-cards-multiup.typ", multiup_data);
        assert!(result.is_ok(), "Multi-up NPC cards render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Multi-up NPC cards is not a valid PDF");

        // Test session prep sheet
        let session_data = serde_json::json!({
            "title": "The Goblin Ambush",
            "module": "Lost Mine of Phandelver",
            "session_number": 1,
            "date": "2025-01-15",
            "summary": "The party escorts a wagon from Neverwinter to Phandalin but is ambushed by goblins on the Triboar Trail.",
            "npcs": [
                {"name": "Sildar Hallwinter", "role": "Ally", "notes": "Captured by goblins, needs rescue"},
                {"name": "Gundren Rockseeker", "role": "Quest Giver", "notes": "Hired the party, went ahead and was captured"}
            ],
            "locations": [
                {"name": "Triboar Trail", "type": "Road", "notes": "Site of goblin ambush"},
                {"name": "Cragmaw Hideout", "type": "Cave", "notes": "Goblin lair, Sildar is held here"}
            ],
            "encounters": [
                {
                    "name": "Goblin Ambush",
                    "type": "Combat",
                    "difficulty": "medium",
                    "monsters": [{"name": "Goblin", "count": 4}],
                    "notes": "Goblins hide behind dead horses"
                },
                {
                    "name": "Cave Entrance",
                    "type": "Combat",
                    "difficulty": "easy",
                    "monsters": [{"name": "Goblin", "count": 2}],
                    "notes": "Guards stationed at cave mouth"
                }
            ],
            "items": [
                {"name": "Gundren's Map", "description": "Shows location of Wave Echo Cave"},
                {"name": "Lionshield Coster Supplies", "description": "Trade goods bound for Phandalin"}
            ],
            "hooks": [
                "Rescue Sildar from the goblins",
                "Find out what happened to Gundren",
                "Deliver the wagon to Barthen's Provisions"
            ],
            "secrets": [
                "Gundren found the entrance to Wave Echo Cave",
                "The Black Spider is behind the goblin attacks"
            ],
            "notes": [
                "Remember to describe the dead horses on the trail",
                "Goblins flee if half their number are killed"
            ]
        });

        let result = service.render_to_pdf("session/prep.typ", session_data);
        assert!(result.is_ok(), "Session prep render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert!(pdf_bytes.len() > 5000, "Session prep PDF seems too small");
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Session prep is not a valid PDF");

        // Test handout template
        let handout_data = serde_json::json!({
            "title": "Letter from Gundren",
            "subtitle": "A hastily written note",
            "type": "letter",
            "author": "Gundren Rockseeker",
            "date": "15th of Mirtul",
            "style": "aged",
            "body": [
                "My dear friends,",
                "I write to you with urgent news. I have finally found what my brothers and I have been searching for - the entrance to Wave Echo Cave! This discovery will change everything.",
                "Meet me in Phandalin as soon as you can. Bring the supplies and tell no one of this letter.",
                "Your friend,",
                "Gundren"
            ],
            "footer": "The ink is smudged and the paper appears worn from travel."
        });

        let result = service.render_to_pdf("session/handout.typ", handout_data);
        assert!(result.is_ok(), "Handout render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Handout is not a valid PDF");

        // Test magical style handout
        let magical_handout = serde_json::json!({
            "title": "Scroll of Fireball",
            "type": "scroll",
            "style": "magical",
            "body": "The arcane words on this scroll glow faintly with an inner fire. Reading the incantation aloud will release a devastating explosion of flame.",
            "sections": [
                {"title": "Incantation", "content": "Ignis Magnus Explodere"},
                {"title": "Warning", "content": "Stand clear of the target area. The caster is not immune to the flames."}
            ]
        });

        let result = service.render_to_pdf("session/handout.typ", magical_handout);
        assert!(result.is_ok(), "Magical handout render failed: {:?}", result.err());
        let pdf_bytes = result.unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Magical handout is not a valid PDF");
    }

    #[test]
    fn test_render_campaign_document() {
        let temp = TempDir::new().unwrap();
        let templates_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
        let service = PrintService::new(templates_root);

        // Create a test markdown document
        let markdown = r#"---
title: Session 1 - The Beginning
type: session_outline
---

# The Adventure Begins

Our heroes gather at the **Yawning Portal** tavern in Waterdeep.

## Key NPCs

- *Durnan* - the barkeep
- *Volothamp Geddarm* - famous explorer

## Objectives

1. Meet with Volo
2. Accept the quest
3. Head to the warehouse district
"#;
        let doc_path = temp.path().join("session_1.md");
        fs::write(&doc_path, markdown).unwrap();

        // Test single document render
        let result = service.render_campaign_document(&doc_path, Some("Waterdeep Dragon Heist"));
        assert!(result.is_ok(), "Campaign document render failed: {:?}", result.err());

        let pdf_bytes = result.unwrap();
        assert!(pdf_bytes.len() > 1000, "Campaign document PDF seems too small");
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Output is not a valid PDF");
    }

    #[test]
    fn test_render_campaign_combined() {
        let temp = TempDir::new().unwrap();
        let templates_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
        let service = PrintService::new(templates_root);

        // Create multiple test documents
        let doc1 = r#"---
title: Session 1 - The Beginning
type: session_outline
---

# The Adventure Begins

Our heroes meet at the tavern.
"#;

        let doc2 = r#"---
title: Durnan
type: npc_profile
---

# Durnan

The gruff but fair barkeep of the Yawning Portal.

## Personality

- Taciturn
- Protective of his establishment
"#;

        let doc3 = r#"---
title: The Yawning Portal
type: location_guide
---

# The Yawning Portal

A famous tavern in Waterdeep built over the entrance to Undermountain.
"#;

        let doc1_path = temp.path().join("session_1.md");
        let doc2_path = temp.path().join("npc_durnan.md");
        let doc3_path = temp.path().join("location_yawning_portal.md");

        fs::write(&doc1_path, doc1).unwrap();
        fs::write(&doc2_path, doc2).unwrap();
        fs::write(&doc3_path, doc3).unwrap();

        // Test combined render
        let documents = vec![doc1_path, doc2_path, doc3_path];
        let result = service.render_campaign_combined(&documents, "Waterdeep Dragon Heist");
        assert!(result.is_ok(), "Combined campaign render failed: {:?}", result.err());

        let pdf_bytes = result.unwrap();
        assert!(pdf_bytes.len() > 2000, "Combined PDF seems too small for 3 documents");
        assert_eq!(&pdf_bytes[0..4], b"%PDF", "Output is not a valid PDF");
    }
}
