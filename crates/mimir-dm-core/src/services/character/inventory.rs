//! Character inventory management service

use super::CharacterService;
use crate::{
    connection::DbConnection,
    error::{DbError, Result},
    models::character::CharacterVersion,
};

/// Service for character inventory operations
pub struct CharacterInventoryService<'a> {
    conn: &'a mut DbConnection,
}

impl<'a> CharacterInventoryService<'a> {
    /// Create a new inventory service
    pub fn new(conn: &'a mut DbConnection) -> Self {
        Self { conn }
    }

    /// Add an item to character's inventory
    ///
    /// Validates that the item exists in the catalog_items database
    pub fn add_item(
        &mut self,
        character_id: i32,
        item_name: &str,
        item_source: &str,
        quantity: i32,
        notes: Option<String>,
    ) -> Result<CharacterVersion> {
        let mut char_service = CharacterService::new(self.conn);
        let (_character, mut char_data) = char_service.get_character(character_id)?;

        // Query item from database to get weight and value
        let mut item_service = crate::services::ItemService::new(self.conn);
        let item = item_service
            .get_item_by_name_and_source(item_name, item_source)
            .map_err(|e| DbError::InvalidData(format!("Failed to get item: {}", e)))?
            .ok_or_else(|| {
                DbError::InvalidData(format!(
                    "Item '{}' from '{}' not found in database. Please import the appropriate rulebook first.",
                    item_name, item_source
                ))
            })?;

        // Get weight and value from item
        let weight = item.weight.map(|w| w as f64).unwrap_or(0.0);
        let value = item.value.unwrap_or(0.0);

        // Check if item already exists in inventory
        let existing_item = char_data.inventory.iter_mut().find(|i| i.name == item_name);

        if let Some(existing) = existing_item {
            // Item exists - add to quantity
            existing.quantity += quantity;
        } else {
            // New item - add to inventory
            char_data
                .inventory
                .push(crate::models::character::data::InventoryItem {
                    name: item_name.to_string(),
                    source: Some(item_source.to_string()),
                    quantity,
                    weight,
                    value,
                    notes,
                });
        }

        // Create new version
        let snapshot_reason = Some(format!("Added {} x{} to inventory", item_name, quantity));
        let mut char_service = CharacterService::new(self.conn);
        char_service.update_character(character_id, char_data, snapshot_reason)
    }

    /// Remove an item from character's inventory
    pub fn remove_item(
        &mut self,
        character_id: i32,
        item_name: &str,
        quantity: i32,
    ) -> Result<CharacterVersion> {
        let mut char_service = CharacterService::new(self.conn);
        let (_character, mut char_data) = char_service.get_character(character_id)?;

        // Find the item
        let item_index = char_data
            .inventory
            .iter()
            .position(|i| i.name == item_name)
            .ok_or_else(|| {
                DbError::InvalidData(format!("Item '{}' not found in inventory", item_name))
            })?;

        // Reduce quantity or remove item
        let item = &mut char_data.inventory[item_index];
        if item.quantity <= quantity {
            // Remove item entirely
            char_data.inventory.remove(item_index);
        } else {
            // Reduce quantity
            item.quantity -= quantity;
        }

        // Create new version
        let snapshot_reason = Some(format!(
            "Removed {} x{} from inventory",
            item_name, quantity
        ));
        let mut char_service = CharacterService::new(self.conn);
        char_service.update_character(character_id, char_data, snapshot_reason)
    }

    /// Update character's currency
    ///
    /// Adds or subtracts currency. Use negative values to subtract.
    pub fn update_currency(
        &mut self,
        character_id: i32,
        copper: i32,
        silver: i32,
        electrum: i32,
        gold: i32,
        platinum: i32,
    ) -> Result<CharacterVersion> {
        let mut char_service = CharacterService::new(self.conn);
        let (_character, mut char_data) = char_service.get_character(character_id)?;

        // Update currency values
        char_data.currency.copper += copper;
        char_data.currency.silver += silver;
        char_data.currency.electrum += electrum;
        char_data.currency.gold += gold;
        char_data.currency.platinum += platinum;

        // Validate no negative currency
        if char_data.currency.copper < 0
            || char_data.currency.silver < 0
            || char_data.currency.electrum < 0
            || char_data.currency.gold < 0
            || char_data.currency.platinum < 0
        {
            return Err(DbError::InvalidData(
                "Currency cannot be negative".to_string(),
            ));
        }

        // Create new version
        let snapshot_reason = Some("Updated currency".to_string());
        let mut char_service = CharacterService::new(self.conn);
        char_service.update_character(character_id, char_data, snapshot_reason)
    }

    /// Update character's equipped items
    pub fn update_equipped(
        &mut self,
        character_id: i32,
        armor: Option<String>,
        shield: Option<String>,
        main_hand: Option<String>,
        off_hand: Option<String>,
    ) -> Result<CharacterVersion> {
        let mut char_service = CharacterService::new(self.conn);
        let (_character, mut char_data) = char_service.get_character(character_id)?;

        // Update equipped items
        char_data.equipped.armor = armor;
        char_data.equipped.shield = shield;
        char_data.equipped.main_hand = main_hand;
        char_data.equipped.off_hand = off_hand;

        // Create new version
        let snapshot_reason = Some("Updated equipped items".to_string());
        let mut char_service = CharacterService::new(self.conn);
        char_service.update_character(character_id, char_data, snapshot_reason)
    }
}
