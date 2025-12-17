//! Development database seeder
//!
//! Seeds the database with test data for development and testing purposes.
//! This includes a sample campaign, players, and characters with various
//! levels and configurations.

use crate::connection::DbConnection;
use crate::dal::campaign::campaigns::CampaignRepository;
use crate::error::Result;
use crate::models::character::CharacterData;
use crate::models::character::{
    AbilityScores, ClassLevel, Currency, EquippedItems, FeatureReference, InventoryItem,
    Personality, Proficiencies, SpellData, SpellReference, SpellSlots,
};
use crate::services::{
    CampaignService, CharacterService, DocumentService, ModuleMonsterService, ModuleService,
    PlayerService,
};
use chrono::Utc;
use std::collections::HashMap;
use tracing::info;

/// Name of the test campaign used to check idempotency
const TEST_CAMPAIGN_NAME: &str = "The Lost Mine of Phandelver";

/// Check if dev seed data already exists
pub fn is_already_seeded(conn: &mut DbConnection) -> Result<bool> {
    let mut repo = CampaignRepository::new(conn);
    let campaigns = repo.list()?;
    Ok(campaigns.iter().any(|c| c.name == TEST_CAMPAIGN_NAME))
}

/// Clear existing dev seed data to allow re-seeding
fn clear_dev_seed_data(conn: &mut DbConnection) -> Result<()> {
    use crate::dal::campaign::documents::DocumentRepository;
    use crate::dal::campaign::module_monsters::ModuleMonsterRepository;
    use crate::dal::campaign::modules::ModuleRepository;
    use crate::dal::character::CharacterRepository;
    use crate::dal::player::PlayerRepository;

    // First, find the dev campaign
    let campaign_info = {
        let mut repo = CampaignRepository::new(conn);
        let campaigns = repo.list()?;
        campaigns
            .into_iter()
            .find(|c| c.name == TEST_CAMPAIGN_NAME)
            .map(|c| (c.id, c.directory_path.clone()))
    };

    let Some((campaign_id, directory_path)) = campaign_info else {
        return Ok(()); // No dev campaign exists, nothing to clear
    };

    info!(
        "Clearing existing dev seed data for campaign id: {}",
        campaign_id
    );

    // Get module IDs
    let module_ids: Vec<i32> = {
        let mut module_repo = ModuleRepository::new(conn);
        module_repo
            .list_by_campaign(campaign_id)?
            .into_iter()
            .map(|m| m.id)
            .collect()
    };

    // 1. Delete module monsters
    for module_id in &module_ids {
        let mut monster_repo = ModuleMonsterRepository::new(conn);
        monster_repo.delete_by_module(*module_id)?;
    }

    // 2. Delete documents
    let doc_ids: Vec<i32> = DocumentRepository::find_by_campaign(conn, campaign_id)?
        .into_iter()
        .map(|d| d.id)
        .collect();
    for doc_id in doc_ids {
        DocumentRepository::delete(conn, doc_id)?;
    }

    // 4. Delete characters for this campaign
    let character_ids: Vec<i32> = {
        let mut char_repo = CharacterRepository::new(conn);
        char_repo
            .list_for_campaign(campaign_id)?
            .into_iter()
            .map(|c| c.id)
            .collect()
    };
    for character_id in character_ids {
        let mut char_repo = CharacterRepository::new(conn);
        char_repo.delete(character_id)?;
    }

    // 5. Delete modules
    for module_id in module_ids {
        let mut module_repo = ModuleRepository::new(conn);
        module_repo.delete(module_id)?;
    }

    // 6. Delete players (they're created by dev seeder)
    let dev_player_names = ["Alice", "Bob", "Charlie", "Diana"];
    let player_ids: Vec<i32> = {
        let mut player_repo = PlayerRepository::new(conn);
        player_repo
            .list()?
            .into_iter()
            .filter(|p| dev_player_names.contains(&p.name.as_str()))
            .map(|p| p.id)
            .collect()
    };
    for player_id in player_ids {
        let mut player_repo = PlayerRepository::new(conn);
        player_repo.delete(player_id)?;
    }

    // 7. Delete campaign
    {
        let mut repo = CampaignRepository::new(conn);
        repo.delete(campaign_id)?;
    }

    // 8. Delete campaign directory
    if std::path::Path::new(&directory_path).exists() {
        if let Err(e) = std::fs::remove_dir_all(&directory_path) {
            info!("Note: Could not remove campaign directory: {}", e);
        }
    }

    info!("Cleared existing dev seed data");
    Ok(())
}

/// Seed development data into the database
///
/// Creates a test campaign with modules, sessions, players, and characters.
/// This function always overwrites existing dev seed data to ensure fresh state.
///
/// # Arguments
/// * `conn` - Database connection
/// * `campaigns_directory` - Base directory for campaign files
///
/// # Returns
/// * `Ok(bool)` - true if seeding was performed
pub fn seed_dev_data(conn: &mut DbConnection, campaigns_directory: &str) -> Result<bool> {
    // Always clear existing dev data first (overwrite mode)
    clear_dev_seed_data(conn)?;

    info!("Seeding development data...");

    // Create campaign
    let campaign = seed_campaign(conn, campaigns_directory)?;
    info!(
        "Created test campaign: {} (id={})",
        campaign.name, campaign.id
    );

    // Create modules
    let modules = seed_modules(conn, campaign.id)?;
    info!("Created {} modules", modules.len());

    // Transition first module to "ready" stage
    if let Some(first_module) = modules.first() {
        transition_module_to_ready(conn, first_module.id)?;
        info!(
            "Transitioned module '{}' to ready stage",
            first_module.name
        );
    }

    // Add monsters to modules
    seed_module_monsters(conn, &modules)?;
    info!("Added monsters to modules");

    // Fill in document content for modules
    seed_module_document_content(conn, &modules, &campaign.directory_path)?;
    info!("Populated module document content");

    // Create players
    let players = seed_players(conn)?;
    info!("Created {} test players", players.len());

    // Create characters
    let characters = seed_characters(conn, Some(campaign.id), &campaign.directory_path, &players)?;
    info!("Created {} test characters", characters.len());

    info!("Dev seed data created successfully");
    Ok(true)
}

/// Seed the test campaign
fn seed_campaign(
    conn: &mut DbConnection,
    campaigns_directory: &str,
) -> Result<crate::models::campaign::campaigns::Campaign> {
    let mut service = CampaignService::new(conn);
    let campaign = service.create_campaign(
        TEST_CAMPAIGN_NAME,
        Some("A classic D&D adventure for 4-5 characters of levels 1-5".to_string()),
        campaigns_directory,
    )?;

    // Transition through stages to create all stage documents
    // concept -> session_zero -> integration -> active
    info!("Transitioning campaign through stages to create all documents...");

    let mut service = CampaignService::new(conn);
    service.transition_campaign_stage(campaign.id, "session_zero")?;
    info!("  -> session_zero (created session zero documents)");

    let mut service = CampaignService::new(conn);
    service.transition_campaign_stage(campaign.id, "integration")?;
    info!("  -> integration (created integration documents)");

    let mut service = CampaignService::new(conn);
    let campaign = service.transition_campaign_stage(campaign.id, "active")?;
    info!("  -> active (campaign ready for play)");

    Ok(campaign)
}

/// Seed test modules
fn seed_modules(
    conn: &mut DbConnection,
    campaign_id: i32,
) -> Result<Vec<crate::models::campaign::modules::Module>> {
    let mut modules = Vec::new();

    let module_data = [
        ("Goblin Ambush", 2, Some("dungeon")),
        ("Cragmaw Hideout", 3, Some("dungeon")),
    ];

    for (name, expected_sessions, module_type) in module_data {
        let mut service = ModuleService::new(conn);
        let module = service.create_module_with_documents(
            campaign_id,
            name.to_string(),
            expected_sessions,
            module_type.map(String::from),
        )?;
        modules.push(module);
    }

    Ok(modules)
}

/// Transition a module through stages to "ready"
fn transition_module_to_ready(conn: &mut DbConnection, module_id: i32) -> Result<()> {
    // Module stages: planning -> development -> ready
    let stages = ["development", "ready"];

    for stage in stages {
        let mut service = ModuleService::new(conn);
        service.transition_module_stage(module_id, stage)?;
    }

    Ok(())
}

/// Seed module monsters with encounter tags
fn seed_module_monsters(
    conn: &mut DbConnection,
    modules: &[crate::models::campaign::modules::Module],
) -> Result<()> {
    // Monster data for each module: (module_name, monsters)
    // Each monster: (name, source, quantity, encounter_tag)
    let module_monsters: Vec<(&str, Vec<(&str, &str, i32, Option<&str>)>)> = vec![
        (
            "Goblin Ambush",
            vec![
                ("Goblin", "MM", 4, Some("Ambush - Road")),
                ("Goblin", "MM", 2, Some("Ambush - Woods")),
                ("Wolf", "MM", 2, Some("Ambush - Road")),
            ],
        ),
        (
            "Cragmaw Hideout",
            vec![
                ("Goblin", "MM", 6, Some("Cave Entrance")),
                ("Goblin", "MM", 3, Some("Guard Post")),
                ("Goblin", "MM", 4, Some("Main Chamber")),
                ("Wolf", "MM", 2, Some("Kennel")),
                ("Bugbear", "MM", 1, Some("Boss Chamber")),
                ("Goblin", "MM", 2, Some("Boss Chamber")),
            ],
        ),
    ];

    for module in modules {
        // Find matching monster data for this module
        if let Some((_, monsters)) = module_monsters.iter().find(|(name, _)| *name == module.name) {
            for (monster_name, source, quantity, encounter_tag) in monsters {
                let mut service = ModuleMonsterService::new(conn);
                service.add_monster(
                    module.id,
                    monster_name.to_string(),
                    source.to_string(),
                    *quantity,
                    encounter_tag.map(String::from),
                )?;
            }
        }
    }

    Ok(())
}

/// Seed module document content with Lost Mine of Phandelver content
fn seed_module_document_content(
    conn: &mut DbConnection,
    modules: &[crate::models::campaign::modules::Module],
    campaign_directory: &str,
) -> Result<()> {
    use std::path::PathBuf;

    let module_content: Vec<(&str, &str)> = vec![
        (
            "Goblin Ambush",
            r#"---
title: "Goblin Ambush - Module Overview"
type: module_overview
---

# Goblin Ambush

## Overview

The party encounters a goblin ambush on the Triboar Trail while escorting supplies to Phandalin. This serves as the adventure's opening encounter and hook into the main plot.

## Key Objectives

- Survive the goblin ambush
- Discover the captured dwarf Gundren Rockseeker was taken to Cragmaw Hideout
- Follow the goblin trail to rescue Gundren's bodyguard Sildar Hallwinter

## Encounters

### The Ambush (Road)
- **Trigger**: The party discovers two dead horses blocking the trail
- **Enemies**: 4 Goblins hiding in the woods, 2 Wolves
- **Tactics**: Goblins attack from cover, wolves rush melee targets
- **Difficulty**: Medium for level 1 party

### Secondary Ambush (Woods)
- **Location**: If party pursues fleeing goblins
- **Enemies**: 2 Goblins with snare traps
- **Complication**: Pit trap (DC 10 Perception to spot)

## Important NPCs

- **Gundren Rockseeker** (mentioned) - Dwarf entrepreneur who hired the party
- **Sildar Hallwinter** (mentioned) - Human warrior escorting Gundren

## Treasure

- 25 gp in a belt pouch on one of the horses
- Empty map case (map was taken by goblins)
- Trail supplies worth 50 gp

## Hooks to Next Module

- Goblin trail leads northeast to Cragmaw Hideout
- One goblin can be captured and interrogated for information
- Horse brands identify them as belonging to Gundren Rockseeker

## DM Notes

- This encounter establishes the threat level and introduces combat
- Allow creative solutions - parley, stealth, or combat all work
- Emphasize the mystery of who was taken and why
"#,
        ),
        (
            "Cragmaw Hideout",
            r#"---
title: "Cragmaw Hideout - Module Overview"
type: module_overview
---

# Cragmaw Hideout

## Overview

A goblin lair hidden in a cave system where Sildar Hallwinter is being held prisoner. The hideout is controlled by Klarg, a bugbear working for the mysterious Black Spider.

## Key Objectives

- Rescue Sildar Hallwinter from the goblins
- Learn about the Black Spider's involvement
- Discover Gundren was taken to Cragmaw Castle
- Recover stolen supplies and treasure

## Dungeon Overview

The hideout consists of several connected cave chambers:

1. **Cave Entrance** - Thicket-hidden entrance with goblin guards
2. **Kennel** - Wolves chained as guard animals
3. **Guard Post** - Elevated platform with archer goblins
4. **Twin Pools** - Water reservoir with flood trap potential
5. **Main Chamber** - Goblin common area
6. **Boss Chamber** - Klarg's lair with Sildar prisoner

## Encounters

### Cave Entrance
- **Enemies**: 2 Goblin sentries
- **Tactics**: One flees to warn others if spotted
- **Hazard**: Stream makes stealthy approach difficult

### Kennel
- **Enemies**: 2 Wolves (chained)
- **Note**: Wolves alert goblins with howling if agitated
- **Opportunity**: Can be bypassed or fed to pacify

### Guard Post
- **Enemies**: 3 Goblins with shortbows
- **Advantage**: Elevated position, half cover
- **Tactics**: Fire at intruders, call for reinforcements

### Main Chamber
- **Enemies**: 6 Goblins led by Yeemik (goblin boss)
- **Complication**: Yeemik threatens to kill Sildar
- **Opportunity**: Negotiate - Yeemik wants Klarg dead

### Boss Chamber
- **Enemies**: Klarg (Bugbear), 2 Goblins, Wolf (pet)
- **Treasure**: Stolen goods, Klarg's treasure chest
- **Difficulty**: Deadly for level 1, hard for level 2

## Important NPCs

- **Sildar Hallwinter** - Captive, member of Lords' Alliance, knows about Wave Echo Cave
- **Klarg** - Bugbear boss, vain and cruel, serves the Black Spider
- **Yeemik** - Ambitious goblin, wants to overthrow Klarg

## Treasure

- 600 cp, 110 sp, 2 potions of healing
- Jade statuette of a frog (40 gp)
- Stolen Lionshield Coster supplies (50 gp reward)
- Sildar's gear (longsword, chainmail)

## Environmental Features

- **Flood Trap**: Dam in Twin Pools can be released
- **Chimney**: Natural shaft to Boss Chamber
- **Fissure**: Connects Guard Post to Twin Pools

## Hooks to Next Module

- Sildar asks party to escort him to Phandalin
- Information about Cragmaw Castle location
- Mention of the Black Spider seeking Wave Echo Cave
- Lionshield supplies can be returned for reward in Phandalin
"#,
        ),
    ];

    for module in modules {
        if let Some((_, content)) = module_content.iter().find(|(name, _)| *name == module.name) {
            // Build the file path for the module overview
            let module_dir = PathBuf::from(campaign_directory)
                .join("modules")
                .join(format!("module_{:02}", module.module_number));
            let overview_path = module_dir.join("module-overview.md");

            // Write the content
            let doc_service = DocumentService::new(conn);
            doc_service.save_document_file(&overview_path.to_string_lossy(), content)?;
        }
    }

    Ok(())
}

/// Seed test players
fn seed_players(conn: &mut DbConnection) -> Result<Vec<crate::models::player::Player>> {
    let mut players = Vec::new();

    let player_data = [
        (
            "Alice",
            Some("alice@test.com"),
            Some("Experienced player, loves tactical combat"),
        ),
        (
            "Bob",
            Some("bob@test.com"),
            Some("Creative roleplayer, enjoys magic users"),
        ),
        (
            "Charlie",
            Some("charlie@test.com"),
            Some("New to D&D, learning the ropes"),
        ),
        (
            "Diana",
            Some("diana@test.com"),
            Some("Forever DM trying player side"),
        ),
    ];

    for (name, email, notes) in player_data {
        let mut service = PlayerService::new(conn);
        let player =
            service.create_player(name, email.map(String::from), notes.map(String::from))?;
        players.push(player);
    }

    Ok(players)
}

/// Seed test characters
fn seed_characters(
    conn: &mut DbConnection,
    campaign_id: Option<i32>,
    base_directory: &str,
    players: &[crate::models::player::Player],
) -> Result<Vec<crate::models::character::Character>> {
    let mut characters = Vec::new();
    let now = Utc::now().to_rfc3339();

    // Map player names to IDs for character assignment
    let player_map: HashMap<&str, i32> = players.iter().map(|p| (p.name.as_str(), p.id)).collect();

    // Thorin Ironforge - Level 5 Dwarf Fighter (Alice's character)
    if let Some(&player_id) = player_map.get("Alice") {
        let character_data = create_thorin(player_id, &now);
        let mut service = CharacterService::new(conn);
        let character =
            service.create_character(campaign_id, Some(player_id), false, base_directory, character_data)?;
        characters.push(character);
    }

    // Elara Moonwhisper - Level 5 Elf Wizard (Bob's character)
    if let Some(&player_id) = player_map.get("Bob") {
        let character_data = create_elara(player_id, &now);
        let mut service = CharacterService::new(conn);
        let character =
            service.create_character(campaign_id, Some(player_id), false, base_directory, character_data)?;
        characters.push(character);
    }

    // Finn Lightfoot - Level 1 Halfling Rogue (Charlie's character)
    if let Some(&player_id) = player_map.get("Charlie") {
        let character_data = create_finn(player_id, &now);
        let mut service = CharacterService::new(conn);
        let character =
            service.create_character(campaign_id, Some(player_id), false, base_directory, character_data)?;
        characters.push(character);
    }

    // Sister Helena - Level 10 Human Cleric (Diana's character)
    if let Some(&player_id) = player_map.get("Diana") {
        let character_data = create_helena(player_id, &now);
        let mut service = CharacterService::new(conn);
        let character =
            service.create_character(campaign_id, Some(player_id), false, base_directory, character_data)?;
        characters.push(character);
    }

    Ok(characters)
}

/// Create Thorin Ironforge - Level 5 Dwarf Fighter
fn create_thorin(player_id: i32, created_at: &str) -> CharacterData {
    CharacterData {
        character_name: "Thorin Ironforge".to_string(),
        player_id: Some(player_id),
        level: 5,
        experience_points: 6500,
        version: 1,
        snapshot_reason: Some("Dev seed character".to_string()),
        created_at: created_at.to_string(),
        race: "Dwarf".to_string(),
        subrace: Some("Mountain".to_string()),
        classes: vec![ClassLevel {
            class_name: "Fighter".to_string(),
            level: 5,
            subclass: Some("Champion".to_string()),
            hit_dice_type: "d10".to_string(),
            hit_dice_remaining: 5,
        }],
        background: "Soldier".to_string(),
        alignment: Some("Lawful Good".to_string()),
        abilities: AbilityScores {
            strength: 18,
            dexterity: 12,
            constitution: 16,
            intelligence: 10,
            wisdom: 13,
            charisma: 8,
        },
        max_hp: 49,
        current_hp: 49,
        speed: 25,
        proficiencies: Proficiencies {
            skills: vec![
                "Athletics".to_string(),
                "Intimidation".to_string(),
                "Perception".to_string(),
                "Survival".to_string(),
            ],
            saves: vec!["Strength".to_string(), "Constitution".to_string()],
            armor: vec![
                "Light armor".to_string(),
                "Medium armor".to_string(),
                "Heavy armor".to_string(),
                "Shields".to_string(),
            ],
            weapons: vec!["Simple weapons".to_string(), "Martial weapons".to_string()],
            tools: vec!["Smith's tools".to_string(), "Dice set".to_string()],
            languages: vec!["Common".to_string(), "Dwarvish".to_string()],
        },
        class_features: vec![
            FeatureReference::new("Fighting Style", "Fighter", "PHB", 1),
            FeatureReference::new("Second Wind", "Fighter", "PHB", 1),
            FeatureReference::new("Action Surge", "Fighter", "PHB", 2),
            FeatureReference::with_subclass("Improved Critical", "Fighter", "Champion", "PHB", 3),
            FeatureReference::new("Extra Attack", "Fighter", "PHB", 5),
        ],
        feats: vec![],
        spells: SpellData::default(),
        inventory: vec![
            InventoryItem {
                name: "Chain Mail".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 55.0,
                value: 75.0,
                notes: Some("AC 16".to_string()),
            },
            InventoryItem {
                name: "Battleaxe".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 4.0,
                value: 10.0,
                notes: Some("1d8 slashing, versatile (1d10)".to_string()),
            },
            InventoryItem {
                name: "Shield".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 6.0,
                value: 10.0,
                notes: Some("+2 AC".to_string()),
            },
            InventoryItem {
                name: "Handaxe".to_string(),
                source: Some("PHB".to_string()),
                quantity: 2,
                weight: 2.0,
                value: 5.0,
                notes: Some("1d6 slashing, light, thrown".to_string()),
            },
            InventoryItem {
                name: "Explorer's Pack".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 59.0,
                value: 10.0,
                notes: None,
            },
        ],
        currency: Currency {
            copper: 0,
            silver: 15,
            electrum: 0,
            gold: 45,
            platinum: 0,
        },
        equipped: EquippedItems {
            armor: Some("Chain Mail".to_string()),
            shield: Some("Shield".to_string()),
            main_hand: Some("Battleaxe".to_string()),
            off_hand: None,
        },
        personality: Personality {
            traits: Some("I face problems head-on. A simple, direct solution is the best path to success.".to_string()),
            ideals: Some("Responsibility. I do what I must and obey just authority.".to_string()),
            bonds: Some("I would still lay down my life for the people I served with.".to_string()),
            flaws: Some("I made a terrible mistake in battle that cost many lives, and I would do anything to keep that mistake secret.".to_string()),
        },
        npc_role: None,
        npc_location: None,
        npc_faction: None,
        npc_notes: None,
    }
}

/// Create Elara Moonwhisper - Level 5 Elf Wizard
fn create_elara(player_id: i32, created_at: &str) -> CharacterData {
    let mut spell_slots = HashMap::new();
    spell_slots.insert(1, SpellSlots::new(4));
    spell_slots.insert(2, SpellSlots::new(3));
    spell_slots.insert(3, SpellSlots::new(2));

    CharacterData {
        character_name: "Elara Moonwhisper".to_string(),
        player_id: Some(player_id),
        level: 5,
        experience_points: 6500,
        version: 1,
        snapshot_reason: Some("Dev seed character".to_string()),
        created_at: created_at.to_string(),
        race: "Elf".to_string(),
        subrace: Some("High".to_string()),
        classes: vec![ClassLevel {
            class_name: "Wizard".to_string(),
            level: 5,
            subclass: Some("School of Evocation".to_string()),
            hit_dice_type: "d6".to_string(),
            hit_dice_remaining: 5,
        }],
        background: "Sage".to_string(),
        alignment: Some("Neutral Good".to_string()),
        abilities: AbilityScores {
            strength: 8,
            dexterity: 14,
            constitution: 13,
            intelligence: 18,
            wisdom: 12,
            charisma: 10,
        },
        max_hp: 27,
        current_hp: 27,
        speed: 30,
        proficiencies: Proficiencies {
            skills: vec![
                "Arcana".to_string(),
                "History".to_string(),
                "Investigation".to_string(),
                "Perception".to_string(),
            ],
            saves: vec!["Intelligence".to_string(), "Wisdom".to_string()],
            armor: vec![],
            weapons: vec![
                "Daggers".to_string(),
                "Darts".to_string(),
                "Slings".to_string(),
                "Quarterstaffs".to_string(),
                "Light crossbows".to_string(),
                "Longsword".to_string(),
                "Shortsword".to_string(),
                "Shortbow".to_string(),
                "Longbow".to_string(),
            ],
            tools: vec![],
            languages: vec![
                "Common".to_string(),
                "Elvish".to_string(),
                "Draconic".to_string(),
                "Celestial".to_string(),
            ],
        },
        class_features: vec![
            FeatureReference::new("Arcane Recovery", "Wizard", "PHB", 1),
            FeatureReference::with_subclass("Evocation Savant", "Wizard", "Evocation", "PHB", 2),
            FeatureReference::with_subclass("Sculpt Spells", "Wizard", "Evocation", "PHB", 2),
        ],
        feats: vec![],
        spells: SpellData {
            cantrips: vec![
                SpellReference::new("Fire Bolt", "PHB"),
                SpellReference::new("Light", "PHB"),
                SpellReference::new("Mage Hand", "PHB"),
                SpellReference::new("Prestidigitation", "PHB"),
            ],
            known_spells: vec![
                SpellReference::new("Magic Missile", "PHB"),
                SpellReference::new("Shield", "PHB"),
                SpellReference::new("Mage Armor", "PHB"),
                SpellReference::new("Detect Magic", "PHB"),
                SpellReference::new("Identify", "PHB"),
                SpellReference::new("Misty Step", "PHB"),
                SpellReference::new("Scorching Ray", "PHB"),
                SpellReference::new("Shatter", "PHB"),
                SpellReference::new("Fireball", "PHB"),
                SpellReference::new("Counterspell", "PHB"),
            ],
            prepared_spells: vec![
                SpellReference::new("Magic Missile", "PHB"),
                SpellReference::new("Shield", "PHB"),
                SpellReference::new("Mage Armor", "PHB"),
                SpellReference::new("Misty Step", "PHB"),
                SpellReference::new("Fireball", "PHB"),
                SpellReference::new("Counterspell", "PHB"),
            ],
            spell_slots,
        },
        inventory: vec![
            InventoryItem {
                name: "Quarterstaff".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 4.0,
                value: 0.2,
                notes: Some("Arcane focus".to_string()),
            },
            InventoryItem {
                name: "Spellbook".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 3.0,
                value: 50.0,
                notes: Some("Contains all known spells".to_string()),
            },
            InventoryItem {
                name: "Component Pouch".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 2.0,
                value: 25.0,
                notes: None,
            },
            InventoryItem {
                name: "Scholar's Pack".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 10.0,
                value: 40.0,
                notes: None,
            },
        ],
        currency: Currency {
            copper: 0,
            silver: 0,
            electrum: 0,
            gold: 75,
            platinum: 0,
        },
        equipped: EquippedItems {
            armor: None,
            shield: None,
            main_hand: Some("Quarterstaff".to_string()),
            off_hand: None,
        },
        personality: Personality {
            traits: Some("I use polysyllabic words that convey the impression of great erudition.".to_string()),
            ideals: Some("Knowledge. The path to power and self-improvement is through knowledge.".to_string()),
            bonds: Some("I have an ancient text that holds terrible secrets that must not fall into the wrong hands.".to_string()),
            flaws: Some("I overlook obvious solutions in favor of complicated ones.".to_string()),
        },
        npc_role: None,
        npc_location: None,
        npc_faction: None,
        npc_notes: None,
    }
}

/// Create Finn Lightfoot - Level 1 Halfling Rogue
fn create_finn(player_id: i32, created_at: &str) -> CharacterData {
    CharacterData {
        character_name: "Finn Lightfoot".to_string(),
        player_id: Some(player_id),
        level: 1,
        experience_points: 0,
        version: 1,
        snapshot_reason: Some("Dev seed character".to_string()),
        created_at: created_at.to_string(),
        race: "Halfling".to_string(),
        subrace: Some("Lightfoot".to_string()),
        classes: vec![ClassLevel {
            class_name: "Rogue".to_string(),
            level: 1,
            subclass: None,
            hit_dice_type: "d8".to_string(),
            hit_dice_remaining: 1,
        }],
        background: "Criminal".to_string(),
        alignment: Some("Chaotic Neutral".to_string()),
        abilities: AbilityScores {
            strength: 8,
            dexterity: 17,
            constitution: 12,
            intelligence: 13,
            wisdom: 10,
            charisma: 14,
        },
        max_hp: 9,
        current_hp: 9,
        speed: 25,
        proficiencies: Proficiencies {
            skills: vec![
                "Acrobatics".to_string(),
                "Deception".to_string(),
                "Sleight of Hand".to_string(),
                "Stealth".to_string(),
            ],
            saves: vec!["Dexterity".to_string(), "Intelligence".to_string()],
            armor: vec!["Light armor".to_string()],
            weapons: vec![
                "Simple weapons".to_string(),
                "Hand crossbows".to_string(),
                "Longswords".to_string(),
                "Rapiers".to_string(),
                "Shortswords".to_string(),
            ],
            tools: vec!["Thieves' tools".to_string(), "Dice set".to_string()],
            languages: vec![
                "Common".to_string(),
                "Halfling".to_string(),
                "Thieves' Cant".to_string(),
            ],
        },
        class_features: vec![
            FeatureReference::new("Expertise", "Rogue", "PHB", 1),
            FeatureReference::new("Sneak Attack", "Rogue", "PHB", 1),
            FeatureReference::new("Thieves' Cant", "Rogue", "PHB", 1),
        ],
        feats: vec![],
        spells: SpellData::default(),
        inventory: vec![
            InventoryItem {
                name: "Leather Armor".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 10.0,
                value: 10.0,
                notes: Some("AC 11 + Dex".to_string()),
            },
            InventoryItem {
                name: "Shortsword".to_string(),
                source: Some("PHB".to_string()),
                quantity: 2,
                weight: 2.0,
                value: 10.0,
                notes: Some("1d6 piercing, finesse, light".to_string()),
            },
            InventoryItem {
                name: "Thieves' Tools".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 1.0,
                value: 25.0,
                notes: None,
            },
            InventoryItem {
                name: "Burglar's Pack".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 44.0,
                value: 16.0,
                notes: None,
            },
        ],
        currency: Currency {
            copper: 0,
            silver: 0,
            electrum: 0,
            gold: 15,
            platinum: 0,
        },
        equipped: EquippedItems {
            armor: Some("Leather Armor".to_string()),
            shield: None,
            main_hand: Some("Shortsword".to_string()),
            off_hand: Some("Shortsword".to_string()),
        },
        personality: Personality {
            traits: Some("I always have a plan for what to do when things go wrong.".to_string()),
            ideals: Some(
                "Freedom. Chains are meant to be broken, as are those who would forge them."
                    .to_string(),
            ),
            bonds: Some(
                "I'm trying to pay off an old debt I owe to a generous benefactor.".to_string(),
            ),
            flaws: Some(
                "When I see something valuable, I can't think about anything but how to steal it."
                    .to_string(),
            ),
        },
        npc_role: None,
        npc_location: None,
        npc_faction: None,
        npc_notes: None,
    }
}

/// Create Sister Helena - Level 10 Human Cleric
fn create_helena(player_id: i32, created_at: &str) -> CharacterData {
    let mut spell_slots = HashMap::new();
    spell_slots.insert(1, SpellSlots::new(4));
    spell_slots.insert(2, SpellSlots::new(3));
    spell_slots.insert(3, SpellSlots::new(3));
    spell_slots.insert(4, SpellSlots::new(3));
    spell_slots.insert(5, SpellSlots::new(2));

    CharacterData {
        character_name: "Sister Helena".to_string(),
        player_id: Some(player_id),
        level: 10,
        experience_points: 64000,
        version: 1,
        snapshot_reason: Some("Dev seed character".to_string()),
        created_at: created_at.to_string(),
        race: "Human".to_string(),
        subrace: None,
        classes: vec![ClassLevel {
            class_name: "Cleric".to_string(),
            level: 10,
            subclass: Some("Life Domain".to_string()),
            hit_dice_type: "d8".to_string(),
            hit_dice_remaining: 10,
        }],
        background: "Acolyte".to_string(),
        alignment: Some("Lawful Good".to_string()),
        abilities: AbilityScores {
            strength: 14,
            dexterity: 10,
            constitution: 14,
            intelligence: 10,
            wisdom: 18,
            charisma: 12,
        },
        max_hp: 73,
        current_hp: 73,
        speed: 30,
        proficiencies: Proficiencies {
            skills: vec![
                "Insight".to_string(),
                "Medicine".to_string(),
                "Persuasion".to_string(),
                "Religion".to_string(),
            ],
            saves: vec!["Wisdom".to_string(), "Charisma".to_string()],
            armor: vec![
                "Light armor".to_string(),
                "Medium armor".to_string(),
                "Heavy armor".to_string(),
                "Shields".to_string(),
            ],
            weapons: vec!["Simple weapons".to_string()],
            tools: vec![],
            languages: vec![
                "Common".to_string(),
                "Celestial".to_string(),
                "Dwarvish".to_string(),
            ],
        },
        class_features: vec![
            FeatureReference::with_subclass("Disciple of Life", "Cleric", "Life", "PHB", 1),
            FeatureReference::new("Channel Divinity", "Cleric", "PHB", 2),
            FeatureReference::with_subclass("Preserve Life", "Cleric", "Life", "PHB", 2),
            FeatureReference::with_subclass("Blessed Healer", "Cleric", "Life", "PHB", 6),
            FeatureReference::with_subclass("Divine Strike", "Cleric", "Life", "PHB", 8),
            FeatureReference::new("Destroy Undead", "Cleric", "PHB", 5),
            FeatureReference::new("Divine Intervention", "Cleric", "PHB", 10),
        ],
        feats: vec!["War Caster".to_string()],
        spells: SpellData {
            cantrips: vec![
                SpellReference::new("Guidance", "PHB"),
                SpellReference::new("Light", "PHB"),
                SpellReference::new("Sacred Flame", "PHB"),
                SpellReference::new("Spare the Dying", "PHB"),
                SpellReference::new("Thaumaturgy", "PHB"),
            ],
            known_spells: vec![
                // Domain spells (always prepared)
                SpellReference::new("Bless", "PHB"),
                SpellReference::new("Cure Wounds", "PHB"),
                SpellReference::new("Lesser Restoration", "PHB"),
                SpellReference::new("Spiritual Weapon", "PHB"),
                SpellReference::new("Beacon of Hope", "PHB"),
                SpellReference::new("Revivify", "PHB"),
                SpellReference::new("Death Ward", "PHB"),
                SpellReference::new("Guardian of Faith", "PHB"),
                SpellReference::new("Mass Cure Wounds", "PHB"),
                SpellReference::new("Raise Dead", "PHB"),
                // Other prepared spells
                SpellReference::new("Healing Word", "PHB"),
                SpellReference::new("Shield of Faith", "PHB"),
                SpellReference::new("Aid", "PHB"),
                SpellReference::new("Prayer of Healing", "PHB"),
                SpellReference::new("Spirit Guardians", "PHB"),
                SpellReference::new("Banishment", "PHB"),
                SpellReference::new("Holy Weapon", "XGE"),
            ],
            prepared_spells: vec![
                SpellReference::new("Bless", "PHB"),
                SpellReference::new("Cure Wounds", "PHB"),
                SpellReference::new("Healing Word", "PHB"),
                SpellReference::new("Shield of Faith", "PHB"),
                SpellReference::new("Lesser Restoration", "PHB"),
                SpellReference::new("Spiritual Weapon", "PHB"),
                SpellReference::new("Aid", "PHB"),
                SpellReference::new("Beacon of Hope", "PHB"),
                SpellReference::new("Revivify", "PHB"),
                SpellReference::new("Spirit Guardians", "PHB"),
                SpellReference::new("Death Ward", "PHB"),
                SpellReference::new("Banishment", "PHB"),
                SpellReference::new("Mass Cure Wounds", "PHB"),
                SpellReference::new("Holy Weapon", "XGE"),
            ],
            spell_slots,
        },
        inventory: vec![
            InventoryItem {
                name: "Plate Armor".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 65.0,
                value: 1500.0,
                notes: Some("AC 18".to_string()),
            },
            InventoryItem {
                name: "Shield".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 6.0,
                value: 10.0,
                notes: Some("+2 AC, holy symbol emblazoned".to_string()),
            },
            InventoryItem {
                name: "Mace".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 4.0,
                value: 5.0,
                notes: Some("1d6 bludgeoning".to_string()),
            },
            InventoryItem {
                name: "Holy Symbol".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 0.0,
                value: 5.0,
                notes: Some("Amulet of Lathander".to_string()),
            },
            InventoryItem {
                name: "Priest's Pack".to_string(),
                source: Some("PHB".to_string()),
                quantity: 1,
                weight: 24.0,
                value: 19.0,
                notes: None,
            },
            InventoryItem {
                name: "Diamond".to_string(),
                source: Some("PHB".to_string()),
                quantity: 3,
                weight: 0.0,
                value: 300.0,
                notes: Some("For Revivify spell component".to_string()),
            },
        ],
        currency: Currency {
            copper: 0,
            silver: 0,
            electrum: 0,
            gold: 250,
            platinum: 10,
        },
        equipped: EquippedItems {
            armor: Some("Plate Armor".to_string()),
            shield: Some("Shield".to_string()),
            main_hand: Some("Mace".to_string()),
            off_hand: None,
        },
        personality: Personality {
            traits: Some("I see omens in every event and action. The gods try to speak to us, we just need to listen.".to_string()),
            ideals: Some("Charity. I always try to help those in need, no matter what the personal cost.".to_string()),
            bonds: Some("I will do anything to protect the temple where I served.".to_string()),
            flaws: Some("I put too much trust in those who wield power within my temple's hierarchy.".to_string()),
        },
        npc_role: None,
        npc_location: None,
        npc_faction: None,
        npc_notes: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_connection;
    use crate::seed::template_seeder::seed_templates;

    #[test]
    fn test_seed_dev_data_creates_expected_data() {
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        seed_templates(&mut conn).unwrap();

        // Create temp directory for test campaign files
        let temp_dir = tempfile::tempdir().unwrap();
        let campaigns_dir = temp_dir.path().to_str().unwrap();

        // First seed should create data
        let result = seed_dev_data(&mut conn, campaigns_dir).unwrap();
        assert!(result, "First seed should return true");

        // Verify campaign created
        let mut repo = CampaignRepository::new(&mut conn);
        let campaigns = repo.list().unwrap();
        assert_eq!(campaigns.len(), 1);
        assert_eq!(campaigns[0].name, TEST_CAMPAIGN_NAME);

        // Verify players created
        use crate::dal::player::PlayerRepository;
        let mut player_repo = PlayerRepository::new(&mut conn);
        let players = player_repo.list().unwrap();
        assert_eq!(players.len(), 4);
        let player_names: Vec<&str> = players.iter().map(|p| p.name.as_str()).collect();
        assert!(player_names.contains(&"Alice"));
        assert!(player_names.contains(&"Bob"));
        assert!(player_names.contains(&"Charlie"));
        assert!(player_names.contains(&"Diana"));

        // Verify characters created
        use crate::dal::character::CharacterRepository;
        let mut char_repo = CharacterRepository::new(&mut conn);
        let characters = char_repo.list_all().unwrap();
        assert_eq!(characters.len(), 4);
        let char_names: Vec<&str> = characters
            .iter()
            .map(|c| c.character_name.as_str())
            .collect();
        assert!(char_names.contains(&"Thorin Ironforge"));
        assert!(char_names.contains(&"Elara Moonwhisper"));
        assert!(char_names.contains(&"Finn Lightfoot"));
        assert!(char_names.contains(&"Sister Helena"));

        // Verify character levels
        let thorin = characters
            .iter()
            .find(|c| c.character_name == "Thorin Ironforge")
            .unwrap();
        assert_eq!(thorin.current_level, 5);

        let finn = characters
            .iter()
            .find(|c| c.character_name == "Finn Lightfoot")
            .unwrap();
        assert_eq!(finn.current_level, 1);

        let helena = characters
            .iter()
            .find(|c| c.character_name == "Sister Helena")
            .unwrap();
        assert_eq!(helena.current_level, 10);
    }

    #[test]
    fn test_seed_dev_data_overwrites_existing() {
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();
        seed_templates(&mut conn).unwrap();

        let temp_dir = tempfile::tempdir().unwrap();
        let campaigns_dir = temp_dir.path().to_str().unwrap();

        // First seed
        let first_result = seed_dev_data(&mut conn, campaigns_dir).unwrap();
        assert!(first_result, "First seed should return true");

        // Second seed should overwrite (also returns true)
        let second_result = seed_dev_data(&mut conn, campaigns_dir).unwrap();
        assert!(second_result, "Second seed should also return true (overwrite)");

        // Verify still only one campaign (old one was deleted and re-created)
        let mut repo = CampaignRepository::new(&mut conn);
        let campaigns = repo.list().unwrap();
        assert_eq!(campaigns.len(), 1, "Should still have only one campaign");
    }

    #[test]
    fn test_is_already_seeded() {
        let mut conn = establish_connection(":memory:").unwrap();
        crate::run_migrations(&mut conn).unwrap();

        // Initially not seeded
        assert!(!is_already_seeded(&mut conn).unwrap());

        // Manually create a campaign with the test name
        use crate::dal::campaign::campaigns::CampaignRepository;
        use crate::models::campaign::campaigns::NewCampaign;
        let mut repo = CampaignRepository::new(&mut conn);
        repo.create(NewCampaign {
            name: TEST_CAMPAIGN_NAME.to_string(),
            status: "concept".to_string(),
            directory_path: "/tmp/test".to_string(),
        })
        .unwrap();

        // Now should be seeded
        assert!(is_already_seeded(&mut conn).unwrap());
    }
}
