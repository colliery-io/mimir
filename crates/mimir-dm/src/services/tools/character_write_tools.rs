//! Write-capable character tools for LLM interactions
//!
//! These tools allow LLMs to modify character data with user confirmation

use async_trait::async_trait;
use mimir_dm_core::{DatabaseService, services::CharacterService};
use mimir_dm_core::models::character::data::InventoryItem;
use mimir_dm_llm::ToolTrait;
use mimir_dm_llm::traits::{ActionDescription, ChangeDetail};
use serde_json::{json, Value};
use std::error::Error;
use std::sync::Arc;
use tracing::debug;

/// Tool for updating character HP (damage/healing)
pub struct UpdateCharacterHpTool {
    db_service: Arc<DatabaseService>,
}

impl UpdateCharacterHpTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for UpdateCharacterHpTool {
    fn name(&self) -> &str {
        "update_character_hp"
    }

    fn description(&self) -> &str {
        "Update a character's current HP (apply damage or healing).

Usage:
- Provide character_id and new_hp value
- Optionally provide reason for HP change
- Creates new character version snapshot
- Respects max HP limits

When to use:
- After combat encounters
- When healing spells or potions are used
- Recording damage during session
- Tracking character health status

Output:
- Updated character with new HP value
- Character version created for history"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "character_id": {
                    "type": "integer",
                    "description": "ID of the character to update"
                },
                "new_hp": {
                    "type": "integer",
                    "description": "New current HP value (0 to max_hp)"
                },
                "reason": {
                    "type": "string",
                    "description": "Reason for HP change (e.g., 'Took 10 damage from goblin', 'Healed 8 HP from potion')"
                }
            },
            "required": ["character_id", "new_hp"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        true
    }

    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let character_id = arguments.get("character_id")?.as_i64()?;
        let new_hp = arguments.get("new_hp")?.as_i64()?;
        let reason = arguments.get("reason")
            .and_then(|v| v.as_str())
            .unwrap_or("HP updated");

        // Try to get current character data for comparison
        let current_hp_info = if let Ok(mut conn) = self.db_service.get_connection() {
            let mut char_service = CharacterService::new(&mut conn);
            if let Ok((_, char_data)) = char_service.get_character(character_id as i32) {
                Some((char_data.character_name.clone(), char_data.current_hp, char_data.max_hp))
            } else {
                None
            }
        } else {
            None
        };

        let description = if let Some((name, current, max)) = current_hp_info {
            let change = new_hp as i32 - current;
            let change_desc = if change > 0 {
                format!("heal {} HP", change)
            } else if change < 0 {
                format!("take {} damage", -change)
            } else {
                "no change".to_string()
            };

            format!(
                "Update {}'s HP from {}/{} to {}/{} ({})\nReason: {}",
                name, current, max, new_hp, max, change_desc, reason
            )
        } else {
            format!(
                "Update character {} HP to {}\nReason: {}",
                character_id, new_hp, reason
            )
        };

        Some(ActionDescription {
            title: "Update Character HP".to_string(),
            description,
            changes: ChangeDetail::Generic {
                items: vec![
                    format!("Character ID: {}", character_id),
                    format!("New HP: {}", new_hp),
                    format!("Reason: {}", reason),
                ],
            },
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let character_id = arguments.get("character_id")
            .and_then(|v| v.as_i64())
            .ok_or("Missing 'character_id' parameter")? as i32;

        let new_hp = arguments.get("new_hp")
            .and_then(|v| v.as_i64())
            .ok_or("Missing 'new_hp' parameter")? as i32;

        let reason = arguments.get("reason")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut conn = self.db_service.get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let mut char_service = CharacterService::new(&mut conn);
        let (_, mut char_data) = char_service.get_character(character_id)
            .map_err(|e| format!("Failed to retrieve character: {}", e))?;

        let old_hp = char_data.current_hp;
        char_data.current_hp = new_hp.max(0).min(char_data.max_hp);

        let snapshot_reason = reason.unwrap_or_else(|| {
            format!("HP updated from {} to {}", old_hp, char_data.current_hp)
        });

        char_service.update_character(character_id, char_data.clone(), Some(snapshot_reason))
            .map_err(|e| format!("Failed to update character: {}", e))?;

        let result = json!({
            "success": true,
            "character_id": character_id,
            "character_name": char_data.character_name,
            "old_hp": old_hp,
            "new_hp": char_data.current_hp,
            "max_hp": char_data.max_hp,
            "message": format!("Updated {} HP from {} to {}", char_data.character_name, old_hp, char_data.current_hp)
        });

        debug!("Updated character {} HP: {} -> {}", character_id, old_hp, char_data.current_hp);
        Ok(serde_json::to_string_pretty(&result)?)
    }
}

/// Tool for adding items to character inventory
pub struct AddInventoryItemTool {
    db_service: Arc<DatabaseService>,
}

impl AddInventoryItemTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for AddInventoryItemTool {
    fn name(&self) -> &str {
        "add_inventory_item"
    }

    fn description(&self) -> &str {
        "Add an item to a character's inventory.

Usage:
- Provide character_id, item name, and quantity
- Optionally provide weight, value, and notes
- Creates new character version snapshot

When to use:
- After looting enemies or treasure
- When characters purchase items
- Recording quest rewards
- Adding starting equipment

Output:
- Updated character with new inventory item
- Character version created for history"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "character_id": {
                    "type": "integer",
                    "description": "ID of the character"
                },
                "item_name": {
                    "type": "string",
                    "description": "Name of the item to add"
                },
                "quantity": {
                    "type": "integer",
                    "description": "Quantity to add (default: 1)"
                },
                "weight": {
                    "type": "number",
                    "description": "Weight per item in pounds (optional)"
                },
                "value": {
                    "type": "number",
                    "description": "Value per item in gold pieces (optional)"
                },
                "notes": {
                    "type": "string",
                    "description": "Additional notes about the item (optional)"
                }
            },
            "required": ["character_id", "item_name"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        true
    }

    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let character_id = arguments.get("character_id")?.as_i64()?;
        let item_name = arguments.get("item_name")?.as_str()?;
        let quantity = arguments.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1);
        let weight = arguments.get("weight").and_then(|v| v.as_f64());
        let value = arguments.get("value").and_then(|v| v.as_f64());

        let mut details = vec![
            format!("Character ID: {}", character_id),
            format!("Item: {}", item_name),
            format!("Quantity: {}", quantity),
        ];

        if let Some(w) = weight {
            details.push(format!("Weight: {} lb", w));
        }
        if let Some(v) = value {
            details.push(format!("Value: {} gp", v));
        }

        Some(ActionDescription {
            title: "Add Inventory Item".to_string(),
            description: format!("Add {} × {} to character inventory", quantity, item_name),
            changes: ChangeDetail::Generic { items: details },
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let character_id = arguments.get("character_id")
            .and_then(|v| v.as_i64())
            .ok_or("Missing 'character_id' parameter")? as i32;

        let item_name = arguments.get("item_name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'item_name' parameter")?;

        let quantity = arguments.get("quantity")
            .and_then(|v| v.as_i64())
            .unwrap_or(1) as i32;

        let weight = arguments.get("weight")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let value = arguments.get("value")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let notes = arguments.get("notes")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut conn = self.db_service.get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let mut char_service = CharacterService::new(&mut conn);
        let (_, mut char_data) = char_service.get_character(character_id)
            .map_err(|e| format!("Failed to retrieve character: {}", e))?;

        // Check if item already exists in inventory
        if let Some(existing) = char_data.inventory.iter_mut().find(|i| i.name == item_name) {
            existing.quantity += quantity;
        } else {
            char_data.inventory.push(InventoryItem {
                name: item_name.to_string(),
                quantity,
                weight,
                value,
                notes,
            });
        }

        let snapshot_reason = format!("Added {} × {} to inventory", quantity, item_name);
        char_service.update_character(character_id, char_data.clone(), Some(snapshot_reason))
            .map_err(|e| format!("Failed to update character: {}", e))?;

        let result = json!({
            "success": true,
            "character_id": character_id,
            "character_name": char_data.character_name,
            "item_added": item_name,
            "quantity": quantity,
            "message": format!("Added {} × {} to {}'s inventory", quantity, item_name, char_data.character_name)
        });

        debug!("Added item to character {}: {} × {}", character_id, quantity, item_name);
        Ok(serde_json::to_string_pretty(&result)?)
    }
}

/// Tool for casting spells (consumes spell slots)
pub struct CastSpellTool {
    db_service: Arc<DatabaseService>,
}

impl CastSpellTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for CastSpellTool {
    fn name(&self) -> &str {
        "cast_spell"
    }

    fn description(&self) -> &str {
        "Cast a spell and consume the appropriate spell slot.

Usage:
- Provide character_id, spell_name, and spell_level
- Automatically reduces available spell slots
- Creates new character version snapshot
- Validates character has available slots

When to use:
- During combat when spells are cast
- Recording spell usage in sessions
- Tracking spell slot consumption
- Before rest/long rest recovery

Output:
- Updated character with reduced spell slots
- Character version created for history"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "character_id": {
                    "type": "integer",
                    "description": "ID of the character casting the spell"
                },
                "spell_name": {
                    "type": "string",
                    "description": "Name of the spell being cast"
                },
                "spell_level": {
                    "type": "integer",
                    "description": "Spell level (1-9, use 0 for cantrips)"
                }
            },
            "required": ["character_id", "spell_name", "spell_level"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        true
    }

    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let character_id = arguments.get("character_id")?.as_i64()?;
        let spell_name = arguments.get("spell_name")?.as_str()?;
        let spell_level = arguments.get("spell_level")?.as_i64()?;

        // Try to get current character data for slot availability
        let slot_info = if let Ok(mut conn) = self.db_service.get_connection() {
            let mut char_service = CharacterService::new(&mut conn);
            if let Ok((_, char_data)) = char_service.get_character(character_id as i32) {
                char_data.spells.spell_slots.get(&(spell_level as i32))
                    .map(|slots| (char_data.character_name.clone(), slots.current, slots.max))
            } else {
                None
            }
        } else {
            None
        };

        let description = if spell_level == 0 {
            format!("Cast cantrip: {} (no slot required)", spell_name)
        } else if let Some((name, current, max)) = slot_info {
            format!(
                "{} casts {} (level {})\nCurrent level {} slots: {}/{}\nAfter cast: {}/{}",
                name, spell_name, spell_level, spell_level, current, max, current - 1, max
            )
        } else {
            format!(
                "Cast {} (level {}) - consumes 1 spell slot",
                spell_name, spell_level
            )
        };

        Some(ActionDescription {
            title: "Cast Spell".to_string(),
            description,
            changes: ChangeDetail::Generic {
                items: vec![
                    format!("Character ID: {}", character_id),
                    format!("Spell: {}", spell_name),
                    format!("Level: {}", spell_level),
                    if spell_level > 0 {
                        "Consumes 1 spell slot".to_string()
                    } else {
                        "Cantrip (no slot consumed)".to_string()
                    },
                ],
            },
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let character_id = arguments.get("character_id")
            .and_then(|v| v.as_i64())
            .ok_or("Missing 'character_id' parameter")? as i32;

        let spell_name = arguments.get("spell_name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'spell_name' parameter")?;

        let spell_level = arguments.get("spell_level")
            .and_then(|v| v.as_i64())
            .ok_or("Missing 'spell_level' parameter")? as i32;

        if spell_level == 0 {
            return Ok(json!({
                "success": true,
                "message": format!("Cast cantrip {} (no slot consumed)", spell_name)
            }).to_string());
        }

        let mut conn = self.db_service.get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let mut char_service = CharacterService::new(&mut conn);
        let (_, mut char_data) = char_service.get_character(character_id)
            .map_err(|e| format!("Failed to retrieve character: {}", e))?;

        // Check if character has spell slots for this level
        let slots = char_data.spells.spell_slots.get_mut(&spell_level)
            .ok_or(format!("Character has no level {} spell slots", spell_level))?;

        if slots.current <= 0 {
            return Err(format!(
                "No level {} spell slots remaining (0/{})",
                spell_level, slots.max
            ).into());
        }

        // Consume spell slot
        slots.current -= 1;

        // Capture values before the borrow ends
        let slots_remaining = slots.current;
        let slots_max = slots.max;

        let snapshot_reason = format!(
            "Cast {} (level {}) - {} slots remaining",
            spell_name, spell_level, slots_remaining
        );

        char_service.update_character(character_id, char_data.clone(), Some(snapshot_reason))
            .map_err(|e| format!("Failed to update character: {}", e))?;

        let result = json!({
            "success": true,
            "character_id": character_id,
            "character_name": char_data.character_name,
            "spell_cast": spell_name,
            "spell_level": spell_level,
            "slots_remaining": slots_remaining,
            "slots_max": slots_max,
            "message": format!(
                "{} cast {} (level {}). {} slots remaining.",
                char_data.character_name, spell_name, spell_level, slots_remaining
            )
        });

        debug!("Character {} cast spell: {} (level {})", character_id, spell_name, spell_level);
        Ok(serde_json::to_string_pretty(&result)?)
    }
}
