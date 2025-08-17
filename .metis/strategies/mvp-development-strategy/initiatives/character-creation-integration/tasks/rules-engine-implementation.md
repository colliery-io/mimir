---
id: rules-engine-implementation
level: task
title: "Rules Engine Implementation"
created_at: 2025-08-17T02:40:00.000000+00:00
updated_at: 2025-08-17T02:40:00.000000+00:00
parent: character-creation-integration
blocked_by: ["character-models-dal", "character-creation-rules-tables"]
archived: false

tags:
  - "#task"
  - "#phase/todo"

exit_criteria_met: false
---

# Rules Engine Implementation

## Overview
Build the rules engine that applies character creation rules to characters. This engine processes grants, calculations, requirements, and modifications to build complete character sheets from choices.

## Acceptance Criteria

- [ ] Rules engine core with handler pattern
- [ ] Grant handler for abilities, proficiencies, features
- [ ] Calculate handler for HP, AC, saves
- [ ] Select handler for player choices
- [ ] Require handler for prerequisites
- [ ] Modify handler for conditional changes
- [ ] Rule priority and dependency resolution
- [ ] Comprehensive test coverage

## Architecture

### Core Engine
```rust
pub struct RulesEngine {
    db_connection: Arc<dyn Database>,
    handlers: HashMap<RuleType, Box<dyn RuleHandler>>,
}

impl RulesEngine {
    pub fn new(db: Arc<dyn Database>) -> Self {
        let mut handlers = HashMap::new();
        handlers.insert(RuleType::Grant, Box::new(GrantHandler::new()));
        handlers.insert(RuleType::Calculate, Box::new(CalculateHandler::new()));
        handlers.insert(RuleType::Select, Box::new(SelectHandler::new()));
        handlers.insert(RuleType::Require, Box::new(RequireHandler::new()));
        handlers.insert(RuleType::Modify, Box::new(ModifyHandler::new()));
        
        Self {
            db_connection: db,
            handlers,
        }
    }
    
    pub async fn apply_rules(
        &self, 
        character: &mut CharacterState,
        trigger_source: &str,
        trigger_id: Option<&str>
    ) -> Result<()> {
        // Fetch applicable rules
        let rules = self.fetch_rules(
            &character.rule_system_id,
            trigger_source,
            trigger_id
        ).await?;
        
        // Sort by priority
        let sorted_rules = self.sort_by_dependencies(rules);
        
        // Apply each rule
        for rule in sorted_rules {
            if self.check_conditions(character, &rule.trigger_condition) {
                self.execute_rule(character, &rule).await?;
            }
        }
        
        Ok(())
    }
}
```

### Rule Handlers

#### Grant Handler
```rust
pub struct GrantHandler;

impl RuleHandler for GrantHandler {
    fn execute(&self, character: &mut CharacterState, rule_def: &RuleDefinition) -> Result<()> {
        for grant in &rule_def.grants {
            match grant.grant_type {
                GrantType::AbilityScoreIncrease => {
                    let ability = &grant.ability;
                    let value = grant.value;
                    character.modify_ability(ability, value);
                }
                GrantType::Proficiency => {
                    let prof_type = &grant.proficiency_type;
                    let prof_value = &grant.value;
                    character.add_proficiency(prof_type, prof_value);
                }
                GrantType::Feature => {
                    character.add_feature(Feature {
                        name: grant.name.clone(),
                        source: grant.source.clone(),
                        description: grant.description.clone(),
                    });
                }
                GrantType::SpellSlots => {
                    for (level, count) in &grant.spell_slots {
                        character.set_spell_slots(*level, *count);
                    }
                }
            }
        }
        Ok(())
    }
}
```

#### Calculate Handler
```rust
pub struct CalculateHandler;

impl RuleHandler for CalculateHandler {
    fn execute(&self, character: &mut CharacterState, rule_def: &RuleDefinition) -> Result<()> {
        for calculation in &rule_def.calculations {
            let value = self.evaluate_formula(character, &calculation.formula)?;
            
            match calculation.target.as_str() {
                "hit_points_max" => character.hit_points_max = value,
                "armor_class" => character.armor_class = value,
                "spell_save_dc" => character.spell_save_dc = value,
                "spell_attack_bonus" => character.spell_attack_bonus = value,
                _ => return Err(anyhow!("Unknown calculation target")),
            }
        }
        Ok(())
    }
    
    fn evaluate_formula(&self, character: &CharacterState, formula: &str) -> Result<i32> {
        // Parse and evaluate formula with character context
        // Support for dice notation, ability modifiers, etc.
    }
}
```

### Derived Values Calculator
```rust
impl RulesEngine {
    pub fn calculate_derived_values(&self, character: &mut CharacterState) -> Result<()> {
        // Ability modifiers
        character.calculate_ability_modifiers();
        
        // Proficiency bonus
        character.proficiency_bonus = 2 + ((character.level - 1) / 4);
        
        // Skill bonuses
        for skill in &character.proficiencies.skills {
            let ability = SKILL_ABILITIES.get(skill).unwrap();
            let modifier = character.get_ability_modifier(ability);
            character.skill_bonuses.insert(
                skill.clone(),
                modifier + character.proficiency_bonus
            );
        }
        
        // Saving throws
        for save in &character.proficiencies.saves {
            let modifier = character.get_ability_modifier(save);
            character.save_bonuses.insert(
                save.clone(),
                modifier + character.proficiency_bonus
            );
        }
        
        // Initiative
        character.initiative = character.get_ability_modifier("dex");
        
        // Spell calculations
        if let Some(ref mut spellcasting) = character.spell_casting {
            let ability_mod = character.get_ability_modifier(&spellcasting.ability);
            spellcasting.spell_save_dc = 8 + character.proficiency_bonus + ability_mod;
            spellcasting.spell_attack_bonus = character.proficiency_bonus + ability_mod;
        }
        
        Ok(())
    }
}
```

## Implementation Steps

1. **Create rules engine module**
   - Define RuleType enum
   - Create RuleHandler trait
   - Implement core engine struct

2. **Implement handlers**
   - GrantHandler for permanent additions
   - CalculateHandler for derived values
   - SelectHandler for player choices
   - RequireHandler for prerequisites
   - ModifyHandler for conditional changes

3. **Add formula evaluation**
   - Parse mathematical expressions
   - Support dice notation (1d6+2)
   - Variable substitution
   - Min/max constraints

4. **Implement rule fetching**
   - Query rules by trigger
   - Filter by conditions
   - Sort by priority/dependencies

5. **Add validation**
   - Prerequisite checking
   - Conflict detection
   - Error reporting

## Testing Requirements

- [ ] Unit tests for each handler
- [ ] Formula evaluation tests
- [ ] Integration tests with sample characters
- [ ] Edge case handling (multiclass, etc.)
- [ ] Performance tests with many rules

## Dependencies
- Character models and DAL
- Rules tables in database
- Rule converter (for test data)

## Estimated Effort
3-4 days

## Notes
- Keep handlers focused and single-purpose
- Consider rule caching for performance
- Prepare for complex multiclass scenarios
- Document formula syntax clearly