//! Character creation module for step-by-step character building
//!
//! Implements D&D 5e character creation process with validation and guidance.

use crate::error::{DbError, Result};
use crate::models::character::data::{
    AbilityScores, CharacterData, EquippedItems, InventoryItem, Personality, Proficiencies,
    SpellData,
};
use serde::{Deserialize, Serialize};

/// Method for determining ability scores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityScoreMethod {
    /// Point buy system (27 points)
    PointBuy {
        strength: i32,
        dexterity: i32,
        constitution: i32,
        intelligence: i32,
        wisdom: i32,
        charisma: i32,
    },

    /// Standard array: 15, 14, 13, 12, 10, 8
    StandardArray {
        strength: i32,
        dexterity: i32,
        constitution: i32,
        intelligence: i32,
        wisdom: i32,
        charisma: i32,
    },

    /// Manual/rolled scores
    Manual {
        strength: i32,
        dexterity: i32,
        constitution: i32,
        intelligence: i32,
        wisdom: i32,
        charisma: i32,
    },
}

/// Race information with ability score modifiers
#[derive(Debug, Clone)]
pub struct RaceInfo {
    pub name: String,
    pub ability_modifiers: Vec<(String, i32)>,
    pub speed: i32,
    pub languages: Vec<String>,
    pub traits: Vec<String>,
}

/// Background information
#[derive(Debug, Clone)]
pub struct BackgroundInfo {
    pub name: String,
    pub skill_proficiencies: Vec<String>,
    pub tool_proficiencies: Vec<String>,
    pub languages: i32, // number of additional languages
    pub starting_equipment: Vec<String>,
}

/// Character builder for step-by-step creation
#[derive(Debug, Clone)]
pub struct CharacterBuilder {
    // Required fields
    character_name: Option<String>,
    player_id: Option<i32>,

    // Step 1: Race
    race: Option<String>,
    subrace: Option<String>,
    race_info: Option<RaceInfo>,

    // Step 2: Class
    class: Option<String>,
    subclass: Option<String>,

    // Step 3: Ability Scores
    base_abilities: Option<AbilityScores>,
    ability_method: Option<AbilityScoreMethod>,

    // Step 4: Background
    background: Option<String>,
    background_info: Option<BackgroundInfo>,

    // Optional fields
    alignment: Option<String>,
    personality: Personality,

    // Derived/calculated fields
    proficiencies: Proficiencies,
    starting_equipment: Vec<InventoryItem>,
}

impl CharacterBuilder {
    /// Create a new character builder
    pub fn new() -> Self {
        Self {
            character_name: None,
            player_id: None,
            race: None,
            subrace: None,
            race_info: None,
            class: None,
            subclass: None,
            base_abilities: None,
            ability_method: None,
            background: None,
            background_info: None,
            alignment: None,
            personality: Personality::default(),
            proficiencies: Proficiencies::default(),
            starting_equipment: Vec::new(),
        }
    }

    /// Set character name and player
    pub fn set_identity(mut self, character_name: String, player_id: i32) -> Self {
        self.character_name = Some(character_name);
        self.player_id = Some(player_id);
        self
    }

    /// Set race (Step 1)
    pub fn set_race(mut self, race: &str, subrace: Option<String>) -> Result<Self> {
        let race_info = RaceInfo::get(race)
            .ok_or_else(|| DbError::InvalidData(format!("Unknown race: {}", race)))?;

        self.race = Some(race_info.name.clone());
        self.subrace = subrace;
        self.race_info = Some(race_info);

        Ok(self)
    }

    /// Set class (Step 2)
    pub fn set_class(mut self, class: &str, subclass: Option<String>) -> Result<Self> {
        // Validate class exists
        let class_info = crate::services::character::level_up::ClassInfo::get(class)
            .ok_or_else(|| DbError::InvalidData(format!("Unknown class: {}", class)))?;

        self.class = Some(class_info.name.clone());
        self.subclass = subclass;

        // Add class proficiencies
        self.add_class_proficiencies(class);

        Ok(self)
    }

    /// Set ability scores (Step 3)
    pub fn set_ability_scores(mut self, method: AbilityScoreMethod) -> Result<Self> {
        // Validate the method
        method.validate()?;

        // Get base scores from method
        let base_scores = method.to_ability_scores();

        self.base_abilities = Some(base_scores);
        self.ability_method = Some(method);

        Ok(self)
    }

    /// Set background (Step 4)
    pub fn set_background(mut self, background: &str) -> Result<Self> {
        let bg_info = BackgroundInfo::get(background)
            .ok_or_else(|| DbError::InvalidData(format!("Unknown background: {}", background)))?;

        // Add background proficiencies
        for skill in &bg_info.skill_proficiencies {
            if !self.proficiencies.skills.contains(skill) {
                self.proficiencies.skills.push(skill.clone());
            }
        }

        for tool in &bg_info.tool_proficiencies {
            if !self.proficiencies.tools.contains(tool) {
                self.proficiencies.tools.push(tool.clone());
            }
        }

        self.background = Some(bg_info.name.clone());
        self.background_info = Some(bg_info);

        Ok(self)
    }

    /// Set alignment (optional)
    pub fn set_alignment(mut self, alignment: String) -> Self {
        self.alignment = Some(alignment);
        self
    }

    /// Set personality traits
    pub fn set_personality(mut self, personality: Personality) -> Self {
        self.personality = personality;
        self
    }

    /// Add additional skill proficiencies
    pub fn add_skill_proficiency(mut self, skill: String) -> Self {
        if !self.proficiencies.skills.contains(&skill) {
            self.proficiencies.skills.push(skill);
        }
        self
    }

    /// Add starting equipment
    pub fn add_equipment(mut self, item: InventoryItem) -> Self {
        self.starting_equipment.push(item);
        self
    }

    /// Build the final CharacterData
    pub fn build(mut self) -> Result<CharacterData> {
        // Validate required fields
        let character_name = self.character_name
            .ok_or_else(|| DbError::InvalidData("Character name is required".to_string()))?;

        let player_id = self.player_id
            .ok_or_else(|| DbError::InvalidData("Player ID is required".to_string()))?;

        let race = self.race
            .ok_or_else(|| DbError::InvalidData("Race is required".to_string()))?;

        let class = self.class
            .ok_or_else(|| DbError::InvalidData("Class is required".to_string()))?;

        let background = self.background
            .ok_or_else(|| DbError::InvalidData("Background is required".to_string()))?;

        let mut base_abilities = self.base_abilities
            .ok_or_else(|| DbError::InvalidData("Ability scores are required".to_string()))?;

        // Apply racial ability score modifiers
        if let Some(race_info) = &self.race_info {
            for (ability, modifier) in &race_info.ability_modifiers {
                apply_ability_modifier(&mut base_abilities, ability, *modifier)?;
            }
        }

        // Get class info for hit die
        let class_info = crate::services::character::level_up::ClassInfo::get(&class)
            .ok_or_else(|| DbError::InvalidData(format!("Unknown class: {}", class)))?;

        // Calculate starting HP: max hit die + CON modifier
        let max_hp = class_info.hit_die_value + base_abilities.con_modifier();

        // Add race languages to proficiencies
        if let Some(race_info) = &self.race_info {
            for lang in &race_info.languages {
                if !self.proficiencies.languages.contains(lang) {
                    self.proficiencies.languages.push(lang.clone());
                }
            }
        }

        Ok(CharacterData {
            character_name,
            player_id,
            level: 1,
            experience_points: 0,
            version: 1,
            snapshot_reason: Some("Initial character creation".to_string()),
            created_at: chrono::Utc::now().to_rfc3339(),
            race,
            subrace: self.subrace,
            class,
            subclass: self.subclass,
            background,
            alignment: self.alignment,
            abilities: base_abilities,
            max_hp,
            current_hp: max_hp,
            hit_dice_remaining: 1,
            hit_dice_type: class_info.hit_die,
            proficiencies: self.proficiencies,
            class_features: Vec::new(), // TODO: Add starting class features
            feats: Vec::new(),
            spells: SpellData::default(),
            inventory: self.starting_equipment,
            equipped: EquippedItems::default(),
            personality: self.personality,
        })
    }

    // Helper method to add class proficiencies
    fn add_class_proficiencies(&mut self, class: &str) {
        match class.to_lowercase().as_str() {
            "barbarian" => {
                self.proficiencies.armor.extend(vec!["Light armor".to_string(), "Medium armor".to_string(), "Shields".to_string()]);
                self.proficiencies.weapons.extend(vec!["Simple weapons".to_string(), "Martial weapons".to_string()]);
                self.proficiencies.saves.extend(vec!["Strength".to_string(), "Constitution".to_string()]);
            }
            "bard" => {
                self.proficiencies.armor.push("Light armor".to_string());
                self.proficiencies.weapons.extend(vec!["Simple weapons".to_string(), "Hand crossbows".to_string(), "Longswords".to_string(), "Rapiers".to_string(), "Shortswords".to_string()]);
                self.proficiencies.saves.extend(vec!["Dexterity".to_string(), "Charisma".to_string()]);
            }
            "cleric" => {
                self.proficiencies.armor.extend(vec!["Light armor".to_string(), "Medium armor".to_string(), "Shields".to_string()]);
                self.proficiencies.weapons.push("Simple weapons".to_string());
                self.proficiencies.saves.extend(vec!["Wisdom".to_string(), "Charisma".to_string()]);
            }
            "druid" => {
                self.proficiencies.armor.extend(vec!["Light armor".to_string(), "Medium armor".to_string(), "Shields".to_string()]);
                self.proficiencies.weapons.extend(vec!["Clubs".to_string(), "Daggers".to_string(), "Darts".to_string(), "Javelins".to_string(), "Maces".to_string(), "Quarterstaffs".to_string(), "Scimitars".to_string(), "Sickles".to_string(), "Slings".to_string(), "Spears".to_string()]);
                self.proficiencies.saves.extend(vec!["Intelligence".to_string(), "Wisdom".to_string()]);
            }
            "fighter" => {
                self.proficiencies.armor.extend(vec!["All armor".to_string(), "Shields".to_string()]);
                self.proficiencies.weapons.extend(vec!["Simple weapons".to_string(), "Martial weapons".to_string()]);
                self.proficiencies.saves.extend(vec!["Strength".to_string(), "Constitution".to_string()]);
            }
            "monk" => {
                self.proficiencies.weapons.extend(vec!["Simple weapons".to_string(), "Shortswords".to_string()]);
                self.proficiencies.saves.extend(vec!["Strength".to_string(), "Dexterity".to_string()]);
            }
            "paladin" => {
                self.proficiencies.armor.extend(vec!["All armor".to_string(), "Shields".to_string()]);
                self.proficiencies.weapons.extend(vec!["Simple weapons".to_string(), "Martial weapons".to_string()]);
                self.proficiencies.saves.extend(vec!["Wisdom".to_string(), "Charisma".to_string()]);
            }
            "ranger" => {
                self.proficiencies.armor.extend(vec!["Light armor".to_string(), "Medium armor".to_string(), "Shields".to_string()]);
                self.proficiencies.weapons.extend(vec!["Simple weapons".to_string(), "Martial weapons".to_string()]);
                self.proficiencies.saves.extend(vec!["Strength".to_string(), "Dexterity".to_string()]);
            }
            "rogue" => {
                self.proficiencies.armor.push("Light armor".to_string());
                self.proficiencies.weapons.extend(vec!["Simple weapons".to_string(), "Hand crossbows".to_string(), "Longswords".to_string(), "Rapiers".to_string(), "Shortswords".to_string()]);
                self.proficiencies.saves.extend(vec!["Dexterity".to_string(), "Intelligence".to_string()]);
            }
            "sorcerer" => {
                self.proficiencies.weapons.extend(vec!["Daggers".to_string(), "Darts".to_string(), "Slings".to_string(), "Quarterstaffs".to_string(), "Light crossbows".to_string()]);
                self.proficiencies.saves.extend(vec!["Constitution".to_string(), "Charisma".to_string()]);
            }
            "warlock" => {
                self.proficiencies.armor.push("Light armor".to_string());
                self.proficiencies.weapons.push("Simple weapons".to_string());
                self.proficiencies.saves.extend(vec!["Wisdom".to_string(), "Charisma".to_string()]);
            }
            "wizard" => {
                self.proficiencies.weapons.extend(vec!["Daggers".to_string(), "Darts".to_string(), "Slings".to_string(), "Quarterstaffs".to_string(), "Light crossbows".to_string()]);
                self.proficiencies.saves.extend(vec!["Intelligence".to_string(), "Wisdom".to_string()]);
            }
            _ => {}
        }
    }
}

impl Default for CharacterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AbilityScoreMethod {
    /// Validate the ability score method
    pub fn validate(&self) -> Result<()> {
        match self {
            AbilityScoreMethod::PointBuy { strength, dexterity, constitution, intelligence, wisdom, charisma } => {
                let scores = vec![*strength, *dexterity, *constitution, *intelligence, *wisdom, *charisma];

                // All scores must be between 8 and 15
                for score in &scores {
                    if *score < 8 || *score > 15 {
                        return Err(DbError::InvalidData(format!(
                            "Point buy scores must be between 8 and 15, got {}",
                            score
                        )));
                    }
                }

                // Calculate total points spent
                let total_cost: i32 = scores.iter().map(|s| point_buy_cost(*s)).sum();

                if total_cost != 27 {
                    return Err(DbError::InvalidData(format!(
                        "Point buy must total exactly 27 points, got {}",
                        total_cost
                    )));
                }
            }
            AbilityScoreMethod::StandardArray { strength, dexterity, constitution, intelligence, wisdom, charisma } => {
                let mut scores = vec![*strength, *dexterity, *constitution, *intelligence, *wisdom, *charisma];
                scores.sort();

                let expected = vec![8, 10, 12, 13, 14, 15];

                if scores != expected {
                    return Err(DbError::InvalidData(
                        "Standard array must use exactly: 15, 14, 13, 12, 10, 8".to_string()
                    ));
                }
            }
            AbilityScoreMethod::Manual { strength, dexterity, constitution, intelligence, wisdom, charisma } => {
                let scores = vec![*strength, *dexterity, *constitution, *intelligence, *wisdom, *charisma];

                // All scores must be between 1 and 20 (allowing for generous DM rolls)
                for score in &scores {
                    if *score < 1 || *score > 20 {
                        return Err(DbError::InvalidData(format!(
                            "Manual scores must be between 1 and 20, got {}",
                            score
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    /// Convert to AbilityScores struct
    pub fn to_ability_scores(&self) -> AbilityScores {
        match self {
            AbilityScoreMethod::PointBuy { strength, dexterity, constitution, intelligence, wisdom, charisma } |
            AbilityScoreMethod::StandardArray { strength, dexterity, constitution, intelligence, wisdom, charisma } |
            AbilityScoreMethod::Manual { strength, dexterity, constitution, intelligence, wisdom, charisma } => {
                AbilityScores {
                    strength: *strength,
                    dexterity: *dexterity,
                    constitution: *constitution,
                    intelligence: *intelligence,
                    wisdom: *wisdom,
                    charisma: *charisma,
                }
            }
        }
    }
}

/// Calculate point buy cost for a given score
fn point_buy_cost(score: i32) -> i32 {
    match score {
        8 => 0,
        9 => 1,
        10 => 2,
        11 => 3,
        12 => 4,
        13 => 5,
        14 => 7,
        15 => 9,
        _ => 0,
    }
}

/// Apply ability modifier to ability scores
fn apply_ability_modifier(abilities: &mut AbilityScores, ability_name: &str, modifier: i32) -> Result<()> {
    let score = match ability_name.to_lowercase().as_str() {
        "strength" => &mut abilities.strength,
        "dexterity" => &mut abilities.dexterity,
        "constitution" => &mut abilities.constitution,
        "intelligence" => &mut abilities.intelligence,
        "wisdom" => &mut abilities.wisdom,
        "charisma" => &mut abilities.charisma,
        _ => {
            return Err(DbError::InvalidData(format!(
                "Unknown ability: {}",
                ability_name
            )))
        }
    };

    *score += modifier;
    Ok(())
}

impl RaceInfo {
    /// Get race information by name
    pub fn get(race: &str) -> Option<Self> {
        match race.to_lowercase().as_str() {
            // PHB Races
            "human" => Some(RaceInfo {
                name: "Human".to_string(),
                ability_modifiers: vec![
                    ("Strength".to_string(), 1),
                    ("Dexterity".to_string(), 1),
                    ("Constitution".to_string(), 1),
                    ("Intelligence".to_string(), 1),
                    ("Wisdom".to_string(), 1),
                    ("Charisma".to_string(), 1),
                ],
                speed: 30,
                languages: vec!["Common".to_string()],
                traits: vec!["Extra Language".to_string()],
            }),
            "dwarf" => Some(RaceInfo {
                name: "Dwarf".to_string(),
                ability_modifiers: vec![("Constitution".to_string(), 2)],
                speed: 25,
                languages: vec!["Common".to_string(), "Dwarvish".to_string()],
                traits: vec!["Darkvision".to_string(), "Dwarven Resilience".to_string(), "Stonecunning".to_string()],
            }),
            "elf" => Some(RaceInfo {
                name: "Elf".to_string(),
                ability_modifiers: vec![("Dexterity".to_string(), 2)],
                speed: 30,
                languages: vec!["Common".to_string(), "Elvish".to_string()],
                traits: vec!["Darkvision".to_string(), "Fey Ancestry".to_string(), "Trance".to_string()],
            }),
            "halfling" => Some(RaceInfo {
                name: "Halfling".to_string(),
                ability_modifiers: vec![("Dexterity".to_string(), 2)],
                speed: 25,
                languages: vec!["Common".to_string(), "Halfling".to_string()],
                traits: vec!["Lucky".to_string(), "Brave".to_string(), "Halfling Nimbleness".to_string()],
            }),
            "dragonborn" => Some(RaceInfo {
                name: "Dragonborn".to_string(),
                ability_modifiers: vec![("Strength".to_string(), 2), ("Charisma".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string(), "Draconic".to_string()],
                traits: vec!["Draconic Ancestry".to_string(), "Breath Weapon".to_string(), "Damage Resistance".to_string()],
            }),
            "gnome" => Some(RaceInfo {
                name: "Gnome".to_string(),
                ability_modifiers: vec![("Intelligence".to_string(), 2)],
                speed: 25,
                languages: vec!["Common".to_string(), "Gnomish".to_string()],
                traits: vec!["Darkvision".to_string(), "Gnome Cunning".to_string()],
            }),
            "half-elf" => Some(RaceInfo {
                name: "Half-Elf".to_string(),
                ability_modifiers: vec![("Charisma".to_string(), 2)],
                speed: 30,
                languages: vec!["Common".to_string(), "Elvish".to_string()],
                traits: vec!["Darkvision".to_string(), "Fey Ancestry".to_string(), "Skill Versatility".to_string()],
            }),
            "half-orc" => Some(RaceInfo {
                name: "Half-Orc".to_string(),
                ability_modifiers: vec![("Strength".to_string(), 2), ("Constitution".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string(), "Orc".to_string()],
                traits: vec!["Darkvision".to_string(), "Relentless Endurance".to_string(), "Savage Attacks".to_string()],
            }),
            "tiefling" => Some(RaceInfo {
                name: "Tiefling".to_string(),
                ability_modifiers: vec![("Charisma".to_string(), 2), ("Intelligence".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string(), "Infernal".to_string()],
                traits: vec!["Darkvision".to_string(), "Hellish Resistance".to_string(), "Infernal Legacy".to_string()],
            }),

            // Volo's Guide to Monsters
            "aasimar" => Some(RaceInfo {
                name: "Aasimar".to_string(),
                ability_modifiers: vec![("Charisma".to_string(), 2)],
                speed: 30,
                languages: vec!["Common".to_string(), "Celestial".to_string()],
                traits: vec!["Darkvision".to_string(), "Celestial Resistance".to_string(), "Healing Hands".to_string(), "Light Bearer".to_string()],
            }),
            "firbolg" => Some(RaceInfo {
                name: "Firbolg".to_string(),
                ability_modifiers: vec![("Wisdom".to_string(), 2), ("Strength".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string(), "Elvish".to_string(), "Giant".to_string()],
                traits: vec!["Firbolg Magic".to_string(), "Hidden Step".to_string(), "Powerful Build".to_string(), "Speech of Beast and Leaf".to_string()],
            }),
            "goliath" => Some(RaceInfo {
                name: "Goliath".to_string(),
                ability_modifiers: vec![("Strength".to_string(), 2), ("Constitution".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string(), "Giant".to_string()],
                traits: vec!["Natural Athlete".to_string(), "Stone's Endurance".to_string(), "Powerful Build".to_string(), "Mountain Born".to_string()],
            }),
            "kenku" => Some(RaceInfo {
                name: "Kenku".to_string(),
                ability_modifiers: vec![("Dexterity".to_string(), 2), ("Wisdom".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string(), "Auran".to_string()],
                traits: vec!["Expert Forgery".to_string(), "Kenku Training".to_string(), "Mimicry".to_string()],
            }),
            "tabaxi" => Some(RaceInfo {
                name: "Tabaxi".to_string(),
                ability_modifiers: vec![("Dexterity".to_string(), 2), ("Charisma".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string()],
                traits: vec!["Darkvision".to_string(), "Feline Agility".to_string(), "Cat's Claws".to_string(), "Cat's Talent".to_string()],
            }),
            "triton" => Some(RaceInfo {
                name: "Triton".to_string(),
                ability_modifiers: vec![("Strength".to_string(), 1), ("Constitution".to_string(), 1), ("Charisma".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string(), "Primordial".to_string()],
                traits: vec!["Amphibious".to_string(), "Control Air and Water".to_string(), "Emissary of the Sea".to_string(), "Guardians of the Depths".to_string()],
            }),

            // Elemental Evil Player's Companion
            "aarakocra" => Some(RaceInfo {
                name: "Aarakocra".to_string(),
                ability_modifiers: vec![("Dexterity".to_string(), 2), ("Wisdom".to_string(), 1)],
                speed: 25,
                languages: vec!["Common".to_string(), "Aarakocra".to_string(), "Auran".to_string()],
                traits: vec!["Flight".to_string(), "Talons".to_string()],
            }),
            "genasi" => Some(RaceInfo {
                name: "Genasi".to_string(),
                ability_modifiers: vec![("Constitution".to_string(), 2)],
                speed: 30,
                languages: vec!["Common".to_string(), "Primordial".to_string()],
                traits: vec!["Elemental Legacy".to_string()],
            }),

            // Mordenkainen's Tome of Foes
            "gith" => Some(RaceInfo {
                name: "Gith".to_string(),
                ability_modifiers: vec![("Intelligence".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string(), "Gith".to_string()],
                traits: vec!["Githyanki or Githzerai features".to_string()],
            }),

            // Eberron: Rising from the Last War
            "changeling" => Some(RaceInfo {
                name: "Changeling".to_string(),
                ability_modifiers: vec![("Charisma".to_string(), 2)],
                speed: 30,
                languages: vec!["Common".to_string()],
                traits: vec!["Shapechanger".to_string(), "Changeling Instincts".to_string(), "Unsettling Visage".to_string(), "Divergent Persona".to_string()],
            }),
            "kalashtar" => Some(RaceInfo {
                name: "Kalashtar".to_string(),
                ability_modifiers: vec![("Wisdom".to_string(), 2), ("Charisma".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string(), "Quori".to_string()],
                traits: vec!["Dual Mind".to_string(), "Mental Discipline".to_string(), "Mind Link".to_string(), "Severed from Dreams".to_string()],
            }),
            "shifter" => Some(RaceInfo {
                name: "Shifter".to_string(),
                ability_modifiers: vec![("Dexterity".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string()],
                traits: vec!["Darkvision".to_string(), "Keen Senses".to_string(), "Shifting".to_string()],
            }),
            "warforged" => Some(RaceInfo {
                name: "Warforged".to_string(),
                ability_modifiers: vec![("Constitution".to_string(), 2)],
                speed: 30,
                languages: vec!["Common".to_string()],
                traits: vec!["Constructed Resilience".to_string(), "Sentry's Rest".to_string(), "Integrated Protection".to_string(), "Specialized Design".to_string()],
            }),

            // Mythic Odysseys of Theros
            "leonin" => Some(RaceInfo {
                name: "Leonin".to_string(),
                ability_modifiers: vec![("Constitution".to_string(), 2), ("Strength".to_string(), 1)],
                speed: 35,
                languages: vec!["Common".to_string(), "Leonin".to_string()],
                traits: vec!["Darkvision".to_string(), "Claws".to_string(), "Hunter's Instincts".to_string(), "Daunting Roar".to_string()],
            }),
            "satyr" => Some(RaceInfo {
                name: "Satyr".to_string(),
                ability_modifiers: vec![("Charisma".to_string(), 2), ("Dexterity".to_string(), 1)],
                speed: 35,
                languages: vec!["Common".to_string(), "Sylvan".to_string()],
                traits: vec!["Fey".to_string(), "Ram".to_string(), "Magic Resistance".to_string(), "Mirthful Leaps".to_string(), "Reveler".to_string()],
            }),

            // Fizban's Treasury of Dragons
            "gem dragonborn" => Some(RaceInfo {
                name: "Gem Dragonborn".to_string(),
                ability_modifiers: vec![("Strength".to_string(), 2), ("Charisma".to_string(), 1)],
                speed: 30,
                languages: vec!["Common".to_string(), "Draconic".to_string()],
                traits: vec!["Gem Ancestry".to_string(), "Breath Weapon".to_string(), "Draconic Resistance".to_string(), "Psionic Mind".to_string()],
            }),

            // Custom/Homebrew friendly option
            "custom lineage" => Some(RaceInfo {
                name: "Custom Lineage".to_string(),
                ability_modifiers: vec![], // Player chooses +2 to one ability
                speed: 30,
                languages: vec!["Common".to_string()],
                traits: vec!["Variable Trait".to_string(), "Feat".to_string()],
            }),

            _ => None,
        }
    }
}

impl BackgroundInfo {
    /// Get background information by name
    pub fn get(background: &str) -> Option<Self> {
        match background.to_lowercase().as_str() {
            // PHB Backgrounds
            "acolyte" => Some(BackgroundInfo {
                name: "Acolyte".to_string(),
                skill_proficiencies: vec!["Insight".to_string(), "Religion".to_string()],
                tool_proficiencies: vec![],
                languages: 2,
                starting_equipment: vec!["Holy symbol".to_string(), "Prayer book".to_string(), "Vestments".to_string()],
            }),
            "charlatan" => Some(BackgroundInfo {
                name: "Charlatan".to_string(),
                skill_proficiencies: vec!["Deception".to_string(), "Sleight of Hand".to_string()],
                tool_proficiencies: vec!["Disguise kit".to_string(), "Forgery kit".to_string()],
                languages: 0,
                starting_equipment: vec!["Fine clothes".to_string(), "Disguise kit".to_string(), "Con tools".to_string()],
            }),
            "criminal" => Some(BackgroundInfo {
                name: "Criminal".to_string(),
                skill_proficiencies: vec!["Deception".to_string(), "Stealth".to_string()],
                tool_proficiencies: vec!["Thieves' tools".to_string(), "Gaming set".to_string()],
                languages: 0,
                starting_equipment: vec!["Crowbar".to_string(), "Dark clothes".to_string()],
            }),
            "entertainer" => Some(BackgroundInfo {
                name: "Entertainer".to_string(),
                skill_proficiencies: vec!["Acrobatics".to_string(), "Performance".to_string()],
                tool_proficiencies: vec!["Disguise kit".to_string(), "Musical instrument".to_string()],
                languages: 0,
                starting_equipment: vec!["Musical instrument".to_string(), "Costume".to_string()],
            }),
            "folk hero" => Some(BackgroundInfo {
                name: "Folk Hero".to_string(),
                skill_proficiencies: vec!["Animal Handling".to_string(), "Survival".to_string()],
                tool_proficiencies: vec!["Artisan's tools".to_string(), "Vehicles (land)".to_string()],
                languages: 0,
                starting_equipment: vec!["Artisan's tools".to_string(), "Shovel".to_string()],
            }),
            "guild artisan" => Some(BackgroundInfo {
                name: "Guild Artisan".to_string(),
                skill_proficiencies: vec!["Insight".to_string(), "Persuasion".to_string()],
                tool_proficiencies: vec!["Artisan's tools".to_string()],
                languages: 1,
                starting_equipment: vec!["Artisan's tools".to_string(), "Letter of introduction".to_string()],
            }),
            "hermit" => Some(BackgroundInfo {
                name: "Hermit".to_string(),
                skill_proficiencies: vec!["Medicine".to_string(), "Religion".to_string()],
                tool_proficiencies: vec!["Herbalism kit".to_string()],
                languages: 1,
                starting_equipment: vec!["Scroll case".to_string(), "Winter blanket".to_string(), "Herbalism kit".to_string()],
            }),
            "noble" => Some(BackgroundInfo {
                name: "Noble".to_string(),
                skill_proficiencies: vec!["History".to_string(), "Persuasion".to_string()],
                tool_proficiencies: vec!["Gaming set".to_string()],
                languages: 1,
                starting_equipment: vec!["Fine clothes".to_string(), "Signet ring".to_string()],
            }),
            "outlander" => Some(BackgroundInfo {
                name: "Outlander".to_string(),
                skill_proficiencies: vec!["Athletics".to_string(), "Survival".to_string()],
                tool_proficiencies: vec!["Musical instrument".to_string()],
                languages: 1,
                starting_equipment: vec!["Staff".to_string(), "Hunting trap".to_string(), "Trophy".to_string()],
            }),
            "sage" => Some(BackgroundInfo {
                name: "Sage".to_string(),
                skill_proficiencies: vec!["Arcana".to_string(), "History".to_string()],
                tool_proficiencies: vec![],
                languages: 2,
                starting_equipment: vec!["Ink".to_string(), "Quill".to_string(), "Book".to_string()],
            }),
            "sailor" => Some(BackgroundInfo {
                name: "Sailor".to_string(),
                skill_proficiencies: vec!["Athletics".to_string(), "Perception".to_string()],
                tool_proficiencies: vec!["Navigator's tools".to_string(), "Vehicles (water)".to_string()],
                languages: 0,
                starting_equipment: vec!["Belaying pin".to_string(), "Silk rope".to_string(), "Lucky charm".to_string()],
            }),
            "soldier" => Some(BackgroundInfo {
                name: "Soldier".to_string(),
                skill_proficiencies: vec!["Athletics".to_string(), "Intimidation".to_string()],
                tool_proficiencies: vec!["Gaming set".to_string(), "Vehicles (land)".to_string()],
                languages: 0,
                starting_equipment: vec!["Rank insignia".to_string(), "Trophy".to_string()],
            }),
            "urchin" => Some(BackgroundInfo {
                name: "Urchin".to_string(),
                skill_proficiencies: vec!["Sleight of Hand".to_string(), "Stealth".to_string()],
                tool_proficiencies: vec!["Disguise kit".to_string(), "Thieves' tools".to_string()],
                languages: 0,
                starting_equipment: vec!["Small knife".to_string(), "Map of city".to_string(), "Pet mouse".to_string()],
            }),

            // Sword Coast Adventurer's Guide
            "city watch" => Some(BackgroundInfo {
                name: "City Watch".to_string(),
                skill_proficiencies: vec!["Athletics".to_string(), "Insight".to_string()],
                tool_proficiencies: vec![],
                languages: 2,
                starting_equipment: vec!["Uniform".to_string(), "Horn".to_string(), "Manacles".to_string()],
            }),
            "clan crafter" => Some(BackgroundInfo {
                name: "Clan Crafter".to_string(),
                skill_proficiencies: vec!["History".to_string(), "Insight".to_string()],
                tool_proficiencies: vec!["Artisan's tools".to_string()],
                languages: 1,
                starting_equipment: vec!["Artisan's tools".to_string(), "Maker's mark chisel".to_string()],
            }),
            "far traveler" => Some(BackgroundInfo {
                name: "Far Traveler".to_string(),
                skill_proficiencies: vec!["Insight".to_string(), "Perception".to_string()],
                tool_proficiencies: vec!["Musical instrument or gaming set".to_string()],
                languages: 1,
                starting_equipment: vec!["Traveler's clothes".to_string(), "Piece of jewelry".to_string(), "Maps".to_string()],
            }),
            "inheritor" => Some(BackgroundInfo {
                name: "Inheritor".to_string(),
                skill_proficiencies: vec!["Survival".to_string(), "Arcana, History, or Religion".to_string()],
                tool_proficiencies: vec!["Gaming set or musical instrument".to_string()],
                languages: 1,
                starting_equipment: vec!["Inheritance".to_string(), "Traveler's clothes".to_string()],
            }),

            // Xanathar's Guide to Everything
            "city watch investigator" => Some(BackgroundInfo {
                name: "City Watch Investigator".to_string(),
                skill_proficiencies: vec!["Insight".to_string(), "Investigation".to_string()],
                tool_proficiencies: vec![],
                languages: 2,
                starting_equipment: vec!["Uniform".to_string(), "Horn".to_string(), "Manacles".to_string()],
            }),

            // Guildmaster's Guide to Ravnica
            "azorius functionary" => Some(BackgroundInfo {
                name: "Azorius Functionary".to_string(),
                skill_proficiencies: vec!["Insight".to_string(), "Intimidation".to_string()],
                tool_proficiencies: vec![],
                languages: 2,
                starting_equipment: vec!["Law book".to_string(), "Ink and quill".to_string()],
            }),

            // Mythic Odysseys of Theros
            "athlete" => Some(BackgroundInfo {
                name: "Athlete".to_string(),
                skill_proficiencies: vec!["Acrobatics".to_string(), "Athletics".to_string()],
                tool_proficiencies: vec!["Vehicles (land)".to_string()],
                languages: 1,
                starting_equipment: vec!["Bronze discus or leather ball".to_string(), "Lucky charm".to_string()],
            }),

            // Strixhaven: A Curriculum of Chaos
            "strixhaven student" => Some(BackgroundInfo {
                name: "Strixhaven Student".to_string(),
                skill_proficiencies: vec!["Arcana, History, Investigation, Nature, or Religion".to_string(), "Choose one".to_string()],
                tool_proficiencies: vec!["Artisan's tools or musical instrument".to_string()],
                languages: 1,
                starting_equipment: vec!["Bottle of ink".to_string(), "Ink pen".to_string(), "Book".to_string(), "School uniform".to_string()],
            }),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_buy_validation() {
        let valid = AbilityScoreMethod::PointBuy {
            strength: 15,
            dexterity: 14,
            constitution: 13,
            intelligence: 12,
            wisdom: 10,
            charisma: 8,
        };
        assert!(valid.validate().is_ok());

        // 15 (9) + 15 (9) + 15 (9) = 27, but with 8,8,8 (0) = 27 total (valid)
        // Let's make a truly invalid one: 15+15+14+8+8+8 = 9+9+7+0+0+0 = 25 points
        let invalid_total = AbilityScoreMethod::PointBuy {
            strength: 15,
            dexterity: 15,
            constitution: 14,
            intelligence: 8,
            wisdom: 8,
            charisma: 8,
        };
        assert!(invalid_total.validate().is_err());

        let invalid_score = AbilityScoreMethod::PointBuy {
            strength: 16,
            dexterity: 14,
            constitution: 13,
            intelligence: 12,
            wisdom: 10,
            charisma: 8,
        };
        assert!(invalid_score.validate().is_err());
    }

    #[test]
    fn test_standard_array_validation() {
        let valid = AbilityScoreMethod::StandardArray {
            strength: 15,
            dexterity: 14,
            constitution: 13,
            intelligence: 12,
            wisdom: 10,
            charisma: 8,
        };
        assert!(valid.validate().is_ok());

        let invalid = AbilityScoreMethod::StandardArray {
            strength: 16,
            dexterity: 14,
            constitution: 13,
            intelligence: 12,
            wisdom: 10,
            charisma: 8,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_character_builder_full_creation() {
        let character_data = CharacterBuilder::new()
            .set_identity("Test Character".to_string(), 1)
            .set_race("Human", None).unwrap()
            .set_class("Fighter", None).unwrap()
            .set_ability_scores(AbilityScoreMethod::StandardArray {
                strength: 15,
                dexterity: 14,
                constitution: 13,
                intelligence: 12,
                wisdom: 10,
                charisma: 8,
            }).unwrap()
            .set_background("Soldier").unwrap()
            .set_alignment("Lawful Good".to_string())
            .build()
            .unwrap();

        assert_eq!(character_data.character_name, "Test Character");
        assert_eq!(character_data.race, "Human");
        assert_eq!(character_data.class, "Fighter");
        assert_eq!(character_data.background, "Soldier");
        assert_eq!(character_data.level, 1);

        // Human gets +1 to all abilities
        assert_eq!(character_data.abilities.strength, 16);
        assert_eq!(character_data.abilities.dexterity, 15);
        assert_eq!(character_data.abilities.constitution, 14);

        // Fighter has d10 hit die, so starting HP should be 10 + CON mod (2) = 12
        assert_eq!(character_data.max_hp, 12);

        // Check proficiencies
        assert!(character_data.proficiencies.skills.contains(&"Athletics".to_string()));
        assert!(character_data.proficiencies.skills.contains(&"Intimidation".to_string()));
        assert!(character_data.proficiencies.saves.contains(&"Strength".to_string()));
        assert!(character_data.proficiencies.saves.contains(&"Constitution".to_string()));
    }

    #[test]
    fn test_race_ability_modifiers() {
        let dwarf_info = RaceInfo::get("Dwarf").unwrap();
        assert_eq!(dwarf_info.ability_modifiers, vec![("Constitution".to_string(), 2)]);

        let elf_info = RaceInfo::get("Elf").unwrap();
        assert_eq!(elf_info.ability_modifiers, vec![("Dexterity".to_string(), 2)]);
    }

    #[test]
    fn test_builder_validation() {
        // Missing required fields should fail
        let result = CharacterBuilder::new()
            .set_race("Human", None).unwrap()
            .build();
        assert!(result.is_err());
    }
}
