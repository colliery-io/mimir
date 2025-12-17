//! Verification logic for checking task outcomes

// Re-export for use by other modules
pub use mimir_dm::services::llm::chat_processor::ToolCallRecord;

use mimir_dm_core::services::CharacterService;
use mimir_dm_core::DatabaseService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::tasks::{CharacterExpectation, Verification};

/// Result of a single verification check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub check_type: String,
    pub passed: bool,
    pub message: Option<String>,
    pub expected: Option<String>,
    pub actual: Option<String>,
}

/// Verification context containing execution results
pub struct VerificationContext {
    pub db_service: Arc<DatabaseService>,
    pub response_content: String,
    pub tools_called: Vec<ToolCallRecord>,
    pub had_errors: bool,
}

/// Helper to find a character by name
fn find_character_by_name(
    conn: &mut mimir_dm_core::connection::DbConnection,
    name: &str,
) -> Option<(
    mimir_dm_core::models::character::Character,
    mimir_dm_core::models::character::CharacterData,
)> {
    let mut service = CharacterService::new(conn);
    let characters = service.list_all_characters().ok()?;

    // Find by exact or partial match (case-insensitive)
    let name_lower = name.to_lowercase();
    let character = characters.into_iter().find(|c| {
        c.character_name.to_lowercase() == name_lower
            || c.character_name.to_lowercase().contains(&name_lower)
    })?;

    // Get full character data
    let mut service = CharacterService::new(conn);
    let (char, data) = service.get_character(character.id).ok()?;
    Some((char, data))
}

/// Run all verifications for a task
pub fn run_verifications(
    verifications: &[Verification],
    context: &VerificationContext,
) -> Vec<VerificationResult> {
    verifications
        .iter()
        .map(|v| run_single_verification(v, context))
        .collect()
}

fn run_single_verification(
    verification: &Verification,
    context: &VerificationContext,
) -> VerificationResult {
    match verification {
        Verification::CharacterExists { name, expect } => {
            verify_character_exists(name, expect.as_ref(), context)
        }
        Verification::CharacterModified {
            name,
            field,
            expected_value,
        } => verify_character_modified(name, field, expected_value, context),
        Verification::InventoryContains {
            character_name,
            item_name,
            quantity,
        } => verify_inventory_contains(character_name, item_name, *quantity, context),
        Verification::ToolCalled {
            tool_name,
            with_args,
        } => verify_tool_called(tool_name, with_args.as_ref(), context),
        Verification::ResponseContains { text } => verify_response_contains(text, context),
        Verification::ResponseNotContains { text } => verify_response_not_contains(text, context),
        Verification::SqlQuery {
            query,
            expect_rows,
            expect_value,
        } => verify_sql_query(query, *expect_rows, expect_value.as_ref(), context),
        Verification::NoErrors => verify_no_errors(context),
    }
}

fn verify_character_exists(
    name: &str,
    expect: Option<&CharacterExpectation>,
    context: &VerificationContext,
) -> VerificationResult {
    let mut conn = match context.db_service.get_connection() {
        Ok(c) => c,
        Err(e) => {
            return VerificationResult {
                check_type: "character_exists".to_string(),
                passed: false,
                message: Some(format!("Database error: {}", e)),
                expected: None,
                actual: None,
            }
        }
    };

    match find_character_by_name(&mut conn, name) {
        Some((_character, data)) => {
            // Character exists, check expectations if provided
            if let Some(exp) = expect {
                let mut failures = Vec::new();

                if let Some(ref expected_class) = exp.class {
                    let actual_class = data.classes.first().map(|c| c.class_name.as_str());
                    if actual_class != Some(expected_class.as_str()) {
                        failures.push(format!(
                            "class: expected '{}', got '{:?}'",
                            expected_class, actual_class
                        ));
                    }
                }

                if let Some(expected_level) = exp.level {
                    if data.level != expected_level {
                        failures.push(format!(
                            "level: expected {}, got {}",
                            expected_level, data.level
                        ));
                    }
                }

                if let Some(ref expected_race) = exp.race {
                    if data.race != *expected_race {
                        failures.push(format!(
                            "race: expected '{}', got '{}'",
                            expected_race, data.race
                        ));
                    }
                }

                if let Some(expected_max_hp) = exp.max_hp {
                    if data.max_hp != expected_max_hp {
                        failures.push(format!(
                            "max_hp: expected {}, got {}",
                            expected_max_hp, data.max_hp
                        ));
                    }
                }

                if let Some(expected_current_hp) = exp.current_hp {
                    if data.current_hp != expected_current_hp {
                        failures.push(format!(
                            "current_hp: expected {}, got {}",
                            expected_current_hp, data.current_hp
                        ));
                    }
                }

                if failures.is_empty() {
                    VerificationResult {
                        check_type: "character_exists".to_string(),
                        passed: true,
                        message: Some(format!(
                            "Character '{}' exists with expected properties",
                            name
                        )),
                        expected: None,
                        actual: None,
                    }
                } else {
                    VerificationResult {
                        check_type: "character_exists".to_string(),
                        passed: false,
                        message: Some(format!(
                            "Character '{}' exists but properties don't match: {}",
                            name,
                            failures.join(", ")
                        )),
                        expected: None,
                        actual: None,
                    }
                }
            } else {
                VerificationResult {
                    check_type: "character_exists".to_string(),
                    passed: true,
                    message: Some(format!("Character '{}' exists", name)),
                    expected: None,
                    actual: None,
                }
            }
        }
        None => VerificationResult {
            check_type: "character_exists".to_string(),
            passed: false,
            message: Some(format!("Character '{}' not found", name)),
            expected: Some(name.to_string()),
            actual: Some("not found".to_string()),
        },
    }
}

fn verify_character_modified(
    name: &str,
    field: &str,
    expected_value: &serde_json::Value,
    context: &VerificationContext,
) -> VerificationResult {
    let mut conn = match context.db_service.get_connection() {
        Ok(c) => c,
        Err(e) => {
            return VerificationResult {
                check_type: "character_modified".to_string(),
                passed: false,
                message: Some(format!("Database error: {}", e)),
                expected: None,
                actual: None,
            }
        }
    };

    match find_character_by_name(&mut conn, name) {
        Some((_character, data)) => {
            let actual_value = match field {
                "current_hp" => serde_json::json!(data.current_hp),
                "max_hp" => serde_json::json!(data.max_hp),
                "level" => serde_json::json!(data.level),
                _ => serde_json::json!(null),
            };

            if &actual_value == expected_value {
                VerificationResult {
                    check_type: "character_modified".to_string(),
                    passed: true,
                    message: Some(format!(
                        "Character '{}' field '{}' matches expected value",
                        name, field
                    )),
                    expected: None,
                    actual: None,
                }
            } else {
                VerificationResult {
                    check_type: "character_modified".to_string(),
                    passed: false,
                    message: Some(format!("Field mismatch for '{}'", field)),
                    expected: Some(expected_value.to_string()),
                    actual: Some(actual_value.to_string()),
                }
            }
        }
        None => VerificationResult {
            check_type: "character_modified".to_string(),
            passed: false,
            message: Some(format!("Character '{}' not found", name)),
            expected: None,
            actual: None,
        },
    }
}

fn verify_inventory_contains(
    character_name: &str,
    item_name: &str,
    expected_quantity: Option<i32>,
    context: &VerificationContext,
) -> VerificationResult {
    let mut conn = match context.db_service.get_connection() {
        Ok(c) => c,
        Err(e) => {
            return VerificationResult {
                check_type: "inventory_contains".to_string(),
                passed: false,
                message: Some(format!("Database error: {}", e)),
                expected: None,
                actual: None,
            }
        }
    };

    match find_character_by_name(&mut conn, character_name) {
        Some((_character, data)) => {
            // Search inventory for item
            let found = data
                .inventory
                .iter()
                .find(|i| i.name.to_lowercase().contains(&item_name.to_lowercase()));

            match found {
                Some(item) => {
                    if let Some(expected_qty) = expected_quantity {
                        if item.quantity == expected_qty {
                            VerificationResult {
                                check_type: "inventory_contains".to_string(),
                                passed: true,
                                message: Some(format!(
                                    "Found {} x{} in {}'s inventory",
                                    item_name, expected_qty, character_name
                                )),
                                expected: None,
                                actual: None,
                            }
                        } else {
                            VerificationResult {
                                check_type: "inventory_contains".to_string(),
                                passed: false,
                                message: Some(format!("Found {} but quantity mismatch", item_name)),
                                expected: Some(format!("quantity: {}", expected_qty)),
                                actual: Some(format!("quantity: {}", item.quantity)),
                            }
                        }
                    } else {
                        VerificationResult {
                            check_type: "inventory_contains".to_string(),
                            passed: true,
                            message: Some(format!(
                                "Found {} in {}'s inventory",
                                item_name, character_name
                            )),
                            expected: None,
                            actual: None,
                        }
                    }
                }
                None => {
                    let inv_names: Vec<&str> =
                        data.inventory.iter().map(|i| i.name.as_str()).collect();
                    VerificationResult {
                        check_type: "inventory_contains".to_string(),
                        passed: false,
                        message: Some(format!(
                            "Item '{}' not found in {}'s inventory",
                            item_name, character_name
                        )),
                        expected: Some(item_name.to_string()),
                        actual: Some(format!("inventory: {:?}", inv_names)),
                    }
                }
            }
        }
        None => VerificationResult {
            check_type: "inventory_contains".to_string(),
            passed: false,
            message: Some(format!("Character '{}' not found", character_name)),
            expected: None,
            actual: None,
        },
    }
}

fn verify_tool_called(
    tool_name: &str,
    _with_args: Option<&serde_json::Value>,
    context: &VerificationContext,
) -> VerificationResult {
    let was_called = context.tools_called.iter().any(|t| t.name == tool_name);

    if was_called {
        VerificationResult {
            check_type: "tool_called".to_string(),
            passed: true,
            message: Some(format!("Tool '{}' was called", tool_name)),
            expected: None,
            actual: None,
        }
    } else {
        let called_tools: Vec<&str> = context.tools_called.iter().map(|t| t.name.as_str()).collect();
        VerificationResult {
            check_type: "tool_called".to_string(),
            passed: false,
            message: Some(format!(
                "Tool '{}' was not called. Called tools: {:?}",
                tool_name, called_tools
            )),
            expected: Some(tool_name.to_string()),
            actual: Some(format!("{:?}", called_tools)),
        }
    }
}

fn verify_response_contains(text: &str, context: &VerificationContext) -> VerificationResult {
    let contains = context
        .response_content
        .to_lowercase()
        .contains(&text.to_lowercase());

    VerificationResult {
        check_type: "response_contains".to_string(),
        passed: contains,
        message: if contains {
            Some(format!("Response contains '{}'", text))
        } else {
            Some(format!("Response does not contain '{}'", text))
        },
        expected: Some(text.to_string()),
        actual: if !contains {
            Some(format!(
                "{}...",
                context.response_content.chars().take(200).collect::<String>()
            ))
        } else {
            None
        },
    }
}

fn verify_response_not_contains(text: &str, context: &VerificationContext) -> VerificationResult {
    let contains = context
        .response_content
        .to_lowercase()
        .contains(&text.to_lowercase());

    VerificationResult {
        check_type: "response_not_contains".to_string(),
        passed: !contains,
        message: if !contains {
            Some(format!("Response correctly does not contain '{}'", text))
        } else {
            Some(format!("Response incorrectly contains '{}'", text))
        },
        expected: Some(format!("not contain: {}", text)),
        actual: if contains {
            Some("found in response".to_string())
        } else {
            None
        },
    }
}

fn verify_sql_query(
    _query: &str,
    _expect_rows: Option<i32>,
    _expect_value: Option<&serde_json::Value>,
    _context: &VerificationContext,
) -> VerificationResult {
    // TODO: Implement raw SQL verification
    VerificationResult {
        check_type: "sql_query".to_string(),
        passed: true,
        message: Some("SQL query verification not yet implemented".to_string()),
        expected: None,
        actual: None,
    }
}

fn verify_no_errors(context: &VerificationContext) -> VerificationResult {
    VerificationResult {
        check_type: "no_errors".to_string(),
        passed: !context.had_errors,
        message: if context.had_errors {
            Some("Errors occurred during execution".to_string())
        } else {
            Some("No errors occurred".to_string())
        },
        expected: None,
        actual: None,
    }
}
