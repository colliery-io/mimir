//! Character sheet markdown renderer
//!
//! Generates human-readable markdown character sheets from CharacterData.

use crate::models::character::CharacterData;
use crate::models::character::data::Currency;

/// Trait for rendering character sheets in various formats
pub trait CharacterRenderer {
    fn render(&self, character: &CharacterData) -> String;
}

/// Markdown renderer for character sheets
pub struct MarkdownRenderer;

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self
    }

    fn render_header(&self, character: &CharacterData) -> String {
        let subclass = character
            .subclass
            .as_ref()
            .map(|s| format!(" ({})", s))
            .unwrap_or_default();

        format!(
            "# {} - Level {} {}{}\n\n",
            character.character_name, character.level, character.class, subclass
        )
    }

    fn render_metadata(&self, character: &CharacterData) -> String {
        let mut output = String::new();

        output.push_str(&format!("**Race:** {}", character.race));
        if let Some(subrace) = &character.subrace {
            output.push_str(&format!(" ({})", subrace));
        }
        output.push_str("  \n");

        output.push_str(&format!("**Background:** {}  \n", character.background));

        if let Some(alignment) = &character.alignment {
            output.push_str(&format!("**Alignment:** {}  \n", alignment));
        }

        output.push_str(&format!("**Experience:** {} XP  \n", character.experience_points));

        if let Some(reason) = &character.snapshot_reason {
            output.push_str(&format!("**Version:** {} ({})  \n", character.version, reason));
        }

        output.push_str(&format!("**Created:** {}\n\n", character.created_at));

        output
    }

    fn render_ability_scores(&self, character: &CharacterData) -> String {
        let mut output = String::from("## Ability Scores\n\n");
        output.push_str("| STR | DEX | CON | INT | WIS | CHA |\n");
        output.push_str("|-----|-----|-----|-----|-----|-----|\n");

        let abilities = &character.abilities;
        output.push_str(&format!(
            "| {} ({:+}) | {} ({:+}) | {} ({:+}) | {} ({:+}) | {} ({:+}) | {} ({:+}) |\n\n",
            abilities.strength,
            abilities.str_modifier(),
            abilities.dexterity,
            abilities.dex_modifier(),
            abilities.constitution,
            abilities.con_modifier(),
            abilities.intelligence,
            abilities.int_modifier(),
            abilities.wisdom,
            abilities.wis_modifier(),
            abilities.charisma,
            abilities.cha_modifier()
        ));

        output
    }

    fn render_combat_stats(&self, character: &CharacterData) -> String {
        let mut output = String::from("## Combat Stats\n\n");
        output.push_str(&format!("- **HP:** {} / {}\n", character.current_hp, character.max_hp));
        output.push_str(&format!(
            "- **Hit Dice:** {}{} remaining\n",
            character.hit_dice_remaining, character.hit_dice_type
        ));
        output.push_str(&format!("- **Proficiency Bonus:** +{}\n\n", character.proficiency_bonus()));

        output
    }

    fn render_proficiencies(&self, character: &CharacterData) -> String {
        let mut output = String::from("## Proficiencies\n\n");
        let prof = &character.proficiencies;

        if !prof.skills.is_empty() {
            output.push_str(&format!("**Skills:** {}  \n", prof.skills.join(", ")));
        }

        if !prof.saves.is_empty() {
            output.push_str(&format!("**Saves:** {}  \n", prof.saves.join(", ")));
        }

        if !prof.armor.is_empty() {
            output.push_str(&format!("**Armor:** {}  \n", prof.armor.join(", ")));
        }

        if !prof.weapons.is_empty() {
            output.push_str(&format!("**Weapons:** {}  \n", prof.weapons.join(", ")));
        }

        if !prof.tools.is_empty() {
            output.push_str(&format!("**Tools:** {}  \n", prof.tools.join(", ")));
        }

        if !prof.languages.is_empty() {
            output.push_str(&format!("**Languages:** {}  \n", prof.languages.join(", ")));
        }

        output.push('\n');
        output
    }

    fn render_class_features(&self, character: &CharacterData) -> String {
        if character.class_features.is_empty() {
            return String::new();
        }

        let mut output = String::from("## Class Features\n\n");
        for feature in &character.class_features {
            output.push_str(&format!("- {}\n", feature));
        }
        output.push('\n');

        output
    }

    fn render_feats(&self, character: &CharacterData) -> String {
        if character.feats.is_empty() {
            return String::new();
        }

        let mut output = String::from("## Feats\n\n");
        for feat in &character.feats {
            output.push_str(&format!("- {}\n", feat));
        }
        output.push('\n');

        output
    }

    fn render_spells(&self, character: &CharacterData) -> String {
        let spells = &character.spells;

        // Only render if character has any spells
        if spells.cantrips.is_empty()
            && spells.known_spells.is_empty()
            && spells.spell_slots.is_empty()
        {
            return String::new();
        }

        let mut output = String::from("## Spells\n\n");

        // Spell slots
        if !spells.spell_slots.is_empty() {
            output.push_str("**Spell Slots:**\n");
            let mut levels: Vec<_> = spells.spell_slots.keys().collect();
            levels.sort();

            for level in levels {
                if let Some(slots) = spells.spell_slots.get(level) {
                    output.push_str(&format!(
                        "- Level {}: {} / {}\n",
                        level, slots.current, slots.max
                    ));
                }
            }
            output.push('\n');
        }

        // Cantrips
        if !spells.cantrips.is_empty() {
            output.push_str("**Cantrips:**  \n");
            output.push_str(&format!("{}  \n\n", spells.cantrips.join(", ")));
        }

        // Known spells
        if !spells.known_spells.is_empty() {
            output.push_str("**Known Spells:**  \n");
            output.push_str(&format!("{}  \n\n", spells.known_spells.join(", ")));
        }

        // Prepared spells
        if !spells.prepared_spells.is_empty() {
            output.push_str("**Prepared Spells:**  \n");
            output.push_str(&format!("{}  \n\n", spells.prepared_spells.join(", ")));
        }

        output
    }

    fn render_equipment(&self, character: &CharacterData) -> String {
        let equipped = &character.equipped;

        // Check if any equipment is present
        if equipped.armor.is_none()
            && equipped.shield.is_none()
            && equipped.main_hand.is_none()
            && equipped.off_hand.is_none()
        {
            return String::new();
        }

        let mut output = String::from("## Equipment\n\n");

        if let Some(armor) = &equipped.armor {
            output.push_str(&format!("- **Armor:** {}\n", armor));
        }

        if let Some(shield) = &equipped.shield {
            output.push_str(&format!("- **Shield:** {}\n", shield));
        }

        if let Some(main_hand) = &equipped.main_hand {
            output.push_str(&format!("- **Main Hand:** {}\n", main_hand));
        }

        if let Some(off_hand) = &equipped.off_hand {
            output.push_str(&format!("- **Off Hand:** {}\n", off_hand));
        }

        output.push('\n');
        output
    }

    fn render_inventory(&self, character: &CharacterData) -> String {
        if character.inventory.is_empty() {
            return String::new();
        }

        let mut output = String::from("## Inventory\n\n");
        output.push_str("| Item | Qty | Weight | Value | Notes |\n");
        output.push_str("|------|-----|--------|-------|-------|\n");

        for item in &character.inventory {
            output.push_str(&format!(
                "| {} | {} | {:.1} lbs | {:.1} gp | {} |\n",
                item.name,
                item.quantity,
                item.weight,
                item.value,
                item.notes.as_deref().unwrap_or("")
            ));
        }

        output.push('\n');
        output
    }

    fn render_personality(&self, character: &CharacterData) -> String {
        let personality = &character.personality;

        // Check if any personality traits are present
        if personality.traits.is_none()
            && personality.ideals.is_none()
            && personality.bonds.is_none()
            && personality.flaws.is_none()
        {
            return String::new();
        }

        let mut output = String::from("## Personality\n\n");

        if let Some(traits) = &personality.traits {
            output.push_str(&format!("**Traits:** {}  \n", traits));
        }

        if let Some(ideals) = &personality.ideals {
            output.push_str(&format!("**Ideals:** {}  \n", ideals));
        }

        if let Some(bonds) = &personality.bonds {
            output.push_str(&format!("**Bonds:** {}  \n", bonds));
        }

        if let Some(flaws) = &personality.flaws {
            output.push_str(&format!("**Flaws:** {}  \n", flaws));
        }

        output.push('\n');
        output
    }
}

impl CharacterRenderer for MarkdownRenderer {
    fn render(&self, character: &CharacterData) -> String {
        let mut output = String::new();

        output.push_str(&self.render_header(character));
        output.push_str(&self.render_metadata(character));
        output.push_str(&self.render_ability_scores(character));
        output.push_str(&self.render_combat_stats(character));
        output.push_str(&self.render_proficiencies(character));
        output.push_str(&self.render_class_features(character));
        output.push_str(&self.render_feats(character));
        output.push_str(&self.render_spells(character));
        output.push_str(&self.render_equipment(character));
        output.push_str(&self.render_inventory(character));
        output.push_str(&self.render_personality(character));

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::character::{
        AbilityScores, EquippedItems, InventoryItem, Personality, Proficiencies, SpellData,
        SpellSlots,
    };
    use std::collections::HashMap;

    fn create_sample_fighter() -> CharacterData {
        CharacterData {
            character_name: "Thorin Ironforge".to_string(),
            player_id: 1,
            level: 3,
            experience_points: 900,
            version: 1,
            snapshot_reason: Some("Initial creation".to_string()),
            created_at: "2025-01-15T10:30:00Z".to_string(),
            race: "Dwarf".to_string(),
            subrace: Some("Mountain".to_string()),
            class: "Fighter".to_string(),
            subclass: Some("Champion".to_string()),
            background: "Soldier".to_string(),
            alignment: Some("Lawful Good".to_string()),
            abilities: AbilityScores {
                strength: 16,
                dexterity: 12,
                constitution: 16,
                intelligence: 10,
                wisdom: 13,
                charisma: 8,
            },
            max_hp: 28,
            current_hp: 28,
            hit_dice_remaining: 3,
            hit_dice_type: "d10".to_string(),
            proficiencies: Proficiencies {
                skills: vec!["Athletics".to_string(), "Intimidation".to_string()],
                saves: vec!["Strength".to_string(), "Constitution".to_string()],
                armor: vec!["All armor".to_string(), "Shields".to_string()],
                weapons: vec!["Simple weapons".to_string(), "Martial weapons".to_string()],
                tools: vec!["Smith's tools".to_string()],
                languages: vec!["Common".to_string(), "Dwarvish".to_string()],
            },
            class_features: vec![
                "Fighting Style (Defense)".to_string(),
                "Second Wind".to_string(),
                "Action Surge".to_string(),
            ],
            feats: Vec::new(),
            spells: SpellData::default(),
            inventory: vec![
                InventoryItem {
                    name: "Rations".to_string(),
                    quantity: 10,
                    weight: 20.0,
                    value: 5.0,
                    notes: None,
                },
                InventoryItem {
                    name: "Healing Potion".to_string(),
                    quantity: 2,
                    weight: 1.0,
                    value: 50.0,
                    notes: Some("Greater healing".to_string()),
                },
            ],
            currency: Currency::default(),
            equipped: EquippedItems {
                armor: Some("Chain Mail".to_string()),
                shield: Some("Shield".to_string()),
                main_hand: Some("Warhammer".to_string()),
                off_hand: None,
            },
            personality: Personality {
                traits: Some("I'm always polite and respectful.".to_string()),
                ideals: Some("Responsibility.".to_string()),
                bonds: Some("I would still lay down my life for the people I served with.".to_string()),
                flaws: Some("I obey authority without question.".to_string()),
            },
        }
    }

    fn create_sample_wizard() -> CharacterData {
        let mut spell_slots = HashMap::new();
        spell_slots.insert(1, SpellSlots::new(4));
        spell_slots.insert(2, SpellSlots::new(2));

        CharacterData {
            character_name: "Elara Moonwhisper".to_string(),
            player_id: 2,
            level: 3,
            experience_points: 900,
            version: 1,
            snapshot_reason: None,
            created_at: "2025-01-15T11:00:00Z".to_string(),
            race: "Elf".to_string(),
            subrace: Some("High".to_string()),
            class: "Wizard".to_string(),
            subclass: Some("School of Evocation".to_string()),
            background: "Sage".to_string(),
            alignment: Some("Neutral Good".to_string()),
            abilities: AbilityScores {
                strength: 8,
                dexterity: 14,
                constitution: 12,
                intelligence: 16,
                wisdom: 13,
                charisma: 10,
            },
            max_hp: 15,
            current_hp: 15,
            hit_dice_remaining: 3,
            hit_dice_type: "d6".to_string(),
            proficiencies: Proficiencies {
                skills: vec!["Arcana".to_string(), "History".to_string(), "Investigation".to_string()],
                saves: vec!["Intelligence".to_string(), "Wisdom".to_string()],
                armor: Vec::new(),
                weapons: vec!["Simple weapons".to_string()],
                tools: Vec::new(),
                languages: vec!["Common".to_string(), "Elvish".to_string(), "Draconic".to_string()],
            },
            class_features: vec![
                "Arcane Recovery".to_string(),
                "Evocation Savant".to_string(),
                "Sculpt Spells".to_string(),
            ],
            feats: Vec::new(),
            spells: SpellData {
                cantrips: vec!["Fire Bolt".to_string(), "Mage Hand".to_string(), "Prestidigitation".to_string()],
                known_spells: vec![
                    "Magic Missile".to_string(),
                    "Shield".to_string(),
                    "Detect Magic".to_string(),
                    "Fireball".to_string(),
                    "Counterspell".to_string(),
                ],
                prepared_spells: vec![
                    "Magic Missile".to_string(),
                    "Shield".to_string(),
                    "Fireball".to_string(),
                ],
                spell_slots,
            },
            inventory: vec![InventoryItem {
                name: "Spellbook".to_string(),
                quantity: 1,
                weight: 3.0,
                value: 50.0,
                notes: Some("Contains all known spells".to_string()),
            }],
            currency: Currency::default(),
            equipped: EquippedItems {
                armor: None,
                shield: None,
                main_hand: Some("Quarterstaff".to_string()),
                off_hand: None,
            },
            personality: Personality::default(),
        }
    }

    #[test]
    fn test_render_fighter() {
        let renderer = MarkdownRenderer::new();
        let fighter = create_sample_fighter();
        let markdown = renderer.render(&fighter);

        // Check header
        assert!(markdown.contains("# Thorin Ironforge - Level 3 Fighter (Champion)"));

        // Check metadata
        assert!(markdown.contains("**Race:** Dwarf (Mountain)"));
        assert!(markdown.contains("**Background:** Soldier"));

        // Check abilities
        assert!(markdown.contains("| 16 (+3) | 12 (+1) | 16 (+3)"));

        // Check combat stats
        assert!(markdown.contains("**HP:** 28 / 28"));
        assert!(markdown.contains("**Proficiency Bonus:** +2"));

        // Check proficiencies
        assert!(markdown.contains("**Skills:** Athletics, Intimidation"));

        // Check class features
        assert!(markdown.contains("Fighting Style (Defense)"));

        // Check equipment
        assert!(markdown.contains("**Armor:** Chain Mail"));

        // Check inventory table
        assert!(markdown.contains("| Rations | 10 | 20.0 lbs"));

        // Check personality
        assert!(markdown.contains("**Traits:** I'm always polite and respectful."));
    }

    #[test]
    fn test_render_wizard_with_spells() {
        let renderer = MarkdownRenderer::new();
        let wizard = create_sample_wizard();
        let markdown = renderer.render(&wizard);

        // Check spells section exists
        assert!(markdown.contains("## Spells"));

        // Check spell slots
        assert!(markdown.contains("Level 1: 4 / 4"));
        assert!(markdown.contains("Level 2: 2 / 2"));

        // Check cantrips
        assert!(markdown.contains("**Cantrips:**"));
        assert!(markdown.contains("Fire Bolt"));

        // Check known spells
        assert!(markdown.contains("**Known Spells:**"));
        assert!(markdown.contains("Magic Missile"));

        // Check prepared spells
        assert!(markdown.contains("**Prepared Spells:**"));
        assert!(markdown.contains("Fireball"));
    }

    #[test]
    fn test_conditional_sections() {
        let renderer = MarkdownRenderer::new();

        // Create minimal character with no spells, feats, or personality
        let minimal = CharacterData {
            character_name: "Test".to_string(),
            player_id: 1,
            level: 1,
            experience_points: 0,
            version: 1,
            snapshot_reason: None,
            created_at: "2025-01-01".to_string(),
            race: "Human".to_string(),
            subrace: None,
            class: "Fighter".to_string(),
            subclass: None,
            background: "Folk Hero".to_string(),
            alignment: None,
            abilities: AbilityScores {
                strength: 15,
                dexterity: 14,
                constitution: 13,
                intelligence: 12,
                wisdom: 10,
                charisma: 8,
            },
            max_hp: 12,
            current_hp: 12,
            hit_dice_remaining: 1,
            hit_dice_type: "d10".to_string(),
            proficiencies: Proficiencies::default(),
            class_features: Vec::new(),
            feats: Vec::new(),
            spells: SpellData::default(),
            inventory: Vec::new(),
            currency: Currency::default(),
            equipped: EquippedItems::default(),
            personality: Personality::default(),
        };

        let markdown = renderer.render(&minimal);

        // Should not have these sections
        assert!(!markdown.contains("## Spells"));
        assert!(!markdown.contains("## Feats"));
        assert!(!markdown.contains("## Equipment"));
        assert!(!markdown.contains("## Inventory"));
        assert!(!markdown.contains("## Personality"));

        // Should still have core sections
        assert!(markdown.contains("## Ability Scores"));
        assert!(markdown.contains("## Combat Stats"));
    }
}
