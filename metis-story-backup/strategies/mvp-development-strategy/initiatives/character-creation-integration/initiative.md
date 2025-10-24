---
id: character-creation-integration
level: initiative
title: "Character Creation Integration"
created_at: 2025-08-01T22:34:16.454451+00:00
updated_at: 2025-08-17T02:36:24.280302+00:00
parent: mvp-development-strategy
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/design"


exit_criteria_met: false
estimated_complexity: M
---

# Mimir Character Creation & Maintenance Implementation Plan

## Executive Summary

This document outlines the implementation plan for character creation and between-session maintenance in Mimir. The system focuses on persistent character data management - stats, equipment, spell preparation, and level progression - not in-game tracking. Players will use printed character sheets during sessions to track HP, spell slots, and other consumable resources.

## Architecture Overview

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  5etools Data   │────▶│ Rules Converter  │────▶│  Rules Database │
└─────────────────┘     └──────────────────┘     └────────┬────────┘
                                                           │
┌─────────────────┐     ┌──────────────────┐              │
│ Character Sheet │◀────│   Rules Engine   │◀─────────────┘
│    Generator    │     └──────────────────┘
└─────────────────┘              │
                                 ▼
                         ┌──────────────────┐
                         │ Character Data   │
                         └──────────────────┘
```

## Phase 1: Database Schema Implementation (Weeks 1-2)

### 1.1 Implementation Steps

1. **Create Migration Framework**
   ```python
   # migrations/001_character_tables.sql
   # migrations/002_rules_tables.sql
   # migrations/003_indexes_and_views.sql
   ```

2. **Core Data Structures**
   ```python
   # Character state JSON structure - persistent data only
   character_state = {
       "ability_scores": {"str": 10, "dex": 10, "con": 10, "int": 10, "wis": 10, "cha": 10},
       "proficiencies": {
           "armor": ["light", "medium"],
           "weapons": ["simple"],
           "tools": ["thieves_tools"],
           "skills": ["stealth", "acrobatics"],
           "saves": ["dex", "int"],
           "languages": ["common", "elvish"]
       },
       "features": {
           "racial": [{"name": "Darkvision", "description": "...", "range": 60}],
           "class": [{"name": "Sneak Attack", "level_gained": 1, "dice": "1d6"}],
           "background": [{"name": "Criminal Contact", "description": "..."}],
           "feats": []
       },
       "spell_casting": {
           "spell_ability": "int",
           "spell_save_dc": 15,
           "spell_attack_bonus": 7,
           "spells_known": ["mage-hand", "minor-illusion", "charm-person"],
           "spells_prepared": ["charm-person", "sleep", "detect-magic"],
           "ritual_caster": false,
           "spell_slots": {"1": 4, "2": 3, "3": 2}  # Maximum slots, not current
       }
   }
   ```

### 1.2 Character Data Tables

```sql
-- Core player character table - persistent data only
CREATE TABLE player_characters (
    id INTEGER PRIMARY KEY,
    campaign_id INTEGER REFERENCES campaigns(id),
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    name TEXT NOT NULL,
    player_name TEXT,
    race_id TEXT REFERENCES races(id),
    background_id TEXT REFERENCES backgrounds(id),
    alignment TEXT,
    experience_points INTEGER DEFAULT 0,
    level INTEGER DEFAULT 1,
    hit_points_max INTEGER,
    armor_class INTEGER,
    initiative_bonus INTEGER,
    speed INTEGER DEFAULT 30,
    ability_scores TEXT CHECK(json_valid(ability_scores)),
    character_state TEXT CHECK(json_valid(character_state)),
    notes TEXT,
    backstory TEXT,
    appearance TEXT,
    personality_traits TEXT,
    ideals TEXT,
    bonds TEXT,
    flaws TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Player character class progression
CREATE TABLE player_character_classes (
    player_character_id INTEGER REFERENCES player_characters(id),
    class_id TEXT REFERENCES classes(id),
    class_level INTEGER NOT NULL,
    subclass_id TEXT REFERENCES classes(id),
    hit_dice_count INTEGER NOT NULL,  -- How many hit dice of this type
    PRIMARY KEY (player_character_id, class_id)
);

-- Character equipment - what they own, not tracking usage
CREATE TABLE player_character_inventory (
    player_character_id INTEGER REFERENCES player_characters(id),
    item_id TEXT REFERENCES items(id),
    quantity INTEGER DEFAULT 1,
    equipped BOOLEAN DEFAULT FALSE,
    attunement_required BOOLEAN DEFAULT FALSE,
    attuned BOOLEAN DEFAULT FALSE,
    custom_name TEXT,
    notes TEXT,
    PRIMARY KEY (player_character_id, item_id)
);

-- Character spells - what they know/have prepared
CREATE TABLE player_character_spells (
    player_character_id INTEGER REFERENCES player_characters(id),
    spell_id TEXT REFERENCES spells(id),
    spell_source TEXT NOT NULL, -- 'class', 'race', 'feat', etc.
    source_id TEXT NOT NULL,
    always_prepared BOOLEAN DEFAULT FALSE,
    prepared BOOLEAN DEFAULT FALSE,  -- For next session
    PRIMARY KEY (player_character_id, spell_id, spell_source)
);

-- Store player selections from character creation
CREATE TABLE player_character_selections (
    id INTEGER PRIMARY KEY,
    player_character_id INTEGER REFERENCES player_characters(id),
    rule_id TEXT REFERENCES character_creation_rules(id),
    selection_type TEXT NOT NULL,
    selection_value TEXT NOT NULL CHECK(json_valid(selection_value)),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- View for character sheet data
CREATE VIEW player_character_sheet AS
SELECT 
    pc.id,
    pc.name,
    pc.level,
    r.name as race_name,
    b.name as background_name,
    GROUP_CONCAT(c.name || ' ' || pcc.class_level) as classes,
    pc.hit_points_max as hp_max,
    pc.armor_class,
    pc.ability_scores
FROM player_characters pc
LEFT JOIN races r ON pc.race_id = r.id
LEFT JOIN backgrounds b ON pc.background_id = b.id
LEFT JOIN player_character_classes pcc ON pc.id = pcc.player_character_id
LEFT JOIN classes c ON pcc.class_id = c.id
GROUP BY pc.id;
```

### 1.3 Rules Engine Tables

```sql
-- Character creation rules
CREATE TABLE character_creation_rules (
    id TEXT PRIMARY KEY,
    rule_system_id TEXT NOT NULL REFERENCES rule_systems(id),
    rule_type TEXT NOT NULL CHECK(rule_type IN ('grant', 'select', 'calculate', 'require', 'modify')),
    trigger_source TEXT NOT NULL,
    trigger_id TEXT,
    trigger_condition TEXT CHECK(json_valid(trigger_condition) OR trigger_condition IS NULL),
    rule_definition TEXT NOT NULL CHECK(json_valid(rule_definition)),
    priority INTEGER DEFAULT 100,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Session prep tables for DM
CREATE TABLE session_character_prep (
    session_id INTEGER REFERENCES sessions(id),
    player_character_id INTEGER REFERENCES player_characters(id),
    level_up_available BOOLEAN DEFAULT FALSE,
    milestone_reached BOOLEAN DEFAULT FALSE,
    notes_for_session TEXT,
    PRIMARY KEY (session_id, player_character_id)
);
```

## Phase 2: 5etools Data Conversion (Weeks 3-4)

### 2.1 Conversion Pipeline Architecture

```python
class ConversionPipeline:
    def __init__(self, rule_system_id: str):
        self.rule_system_id = rule_system_id
        self.converters = {
            'race': RaceConverter(),
            'class': ClassConverter(),
            'background': BackgroundConverter(),
            'feat': FeatConverter(),
            'spell': SpellConverter(),
            'item': ItemConverter()
        }
    
    def process_bundle(self, bundle_path: Path):
        # 1. Extract and validate bundle
        bundle_data = self.extract_bundle(bundle_path)
        self.validate_bundle_structure(bundle_data)
        
        # 2. Convert each entity type to rules
        all_rules = []
        for entity_type, converter in self.converters.items():
            if entity_type in bundle_data:
                rules = converter.convert_entities(
                    bundle_data[entity_type], 
                    self.rule_system_id
                )
                all_rules.extend(rules)
        
        # 3. Post-process rules
        all_rules = self.resolve_dependencies(all_rules)
        all_rules = self.assign_priorities(all_rules)
        
        return all_rules
```

### 2.2 Rule Conversion Focus

Since we're not tracking combat state, conversions focus on:
- **Permanent bonuses** (ability scores, proficiencies)
- **Maximum values** (HP max, spell slots per day)
- **Character options** (feats, class features)
- **Equipment properties** (AC bonuses, magical effects)

```python
class ClassConverter:
    def convert_entity(self, class_data: dict, rule_system_id: str) -> List[dict]:
        rules = []
        class_id = self.generate_id(class_data)
        
        # Hit points at first level
        if 'hd' in class_data:
            rules.append({
                'id': f"{rule_system_id}-class-{class_id}-hp-1",
                'rule_type': 'calculate',
                'trigger_source': 'class',
                'trigger_id': class_id,
                'trigger_condition': {'level': 1},
                'rule_definition': {
                    'calculations': [{
                        'target': 'hit_points_max',
                        'formula': f"{class_data['hd']['faces']} + {{ability_modifier:constitution}}"
                    }]
                }
            })
        
        # Spell slots (maximum per day)
        if 'spellcastingAbility' in class_data:
            rules.extend(self.create_spellcasting_rules(class_data, class_id, rule_system_id))
        
        # Class features by level
        if 'classFeatures' in class_data:
            for level, features in enumerate(class_data['classFeatures'], 1):
                for feature in features:
                    rules.append(self.create_feature_rule(
                        feature, class_id, level, rule_system_id
                    ))
        
        return rules
```

## Phase 3: Rules Engine Implementation (Weeks 5-6)

### 3.1 Core Engine Architecture

```python
class RulesEngine:
    def __init__(self, db_connection):
        self.db = db_connection
        self.handlers = {
            'grant': GrantHandler(),
            'select': SelectHandler(),
            'calculate': CalculateHandler(),
            'require': RequireHandler(),
            'modify': ModifyHandler()
        }
    
    def apply_rules(self, character: CharacterState, 
                   trigger_source: str, trigger_id: str = None):
        """Apply rules for character building, not combat tracking."""
        
        # 1. Fetch applicable rules
        rules = self.fetch_rules(
            character.rule_system_id, 
            trigger_source, 
            trigger_id
        )
        
        # 2. Filter by conditions
        applicable_rules = [
            rule for rule in rules
            if self.check_conditions(character, rule.get('trigger_condition'))
        ]
        
        # 3. Execute each rule
        for rule in sorted(applicable_rules, key=lambda r: r['priority']):
            self.execute_rule(character, rule)
    
    def calculate_sheet_values(self, character: CharacterState):
        """Calculate all derived values for character sheet."""
        
        # Ability modifiers
        for ability in ['str', 'dex', 'con', 'int', 'wis', 'cha']:
            score = character.ability_scores.get(ability, 10)
            character.ability_modifiers[ability] = (score - 10) // 2
        
        # Proficiency bonus
        character.proficiency_bonus = 2 + ((character.level - 1) // 4)
        
        # Skill bonuses
        self.calculate_skill_bonuses(character)
        
        # Save bonuses
        self.calculate_save_bonuses(character)
        
        # Initiative
        character.initiative = character.ability_modifiers['dex']
        
        # Spell save DC and attack bonus
        if character.spellcasting_ability:
            ability_mod = character.ability_modifiers[character.spellcasting_ability]
            character.spell_save_dc = 8 + character.proficiency_bonus + ability_mod
            character.spell_attack_bonus = character.proficiency_bonus + ability_mod
```

### 3.2 Simplified Handler Implementations

```python
class GrantHandler:
    """Handles permanent grants only - no temporary effects."""
    
    def execute(self, character: CharacterState, rule_def: dict):
        grants = rule_def.get('grants', [])
        
        for grant in grants:
            grant_type = grant['type']
            
            if grant_type == 'ability_score_increase':
                # Permanent ability increases
                ability = grant['ability']
                value = grant['value']
                character.ability_scores[ability] = min(
                    character.ability_scores.get(ability, 10) + value, 
                    20
                )
            
            elif grant_type == 'proficiency':
                # Add to proficiency lists
                prof_type = grant.get('proficiency_type')
                prof_value = grant.get('value')
                
                if prof_type not in character.proficiencies:
                    character.proficiencies[prof_type] = []
                
                if prof_value not in character.proficiencies[prof_type]:
                    character.proficiencies[prof_type].append(prof_value)
            
            elif grant_type == 'feature':
                # Class/race features
                character.features.append({
                    'name': grant['name'],
                    'source': grant.get('source', 'unknown'),
                    'level_gained': character.level,
                    'description': grant.get('description', '')
                })
            
            elif grant_type == 'spell_slots':
                # Maximum spell slots per level
                for level, count in grant.get('slots', {}).items():
                    character.spell_slots[level] = count


class CalculateHandler:
    """Calculate persistent values only."""
    
    def execute(self, character: CharacterState, rule_def: dict):
        calculations = rule_def.get('calculations', [])
        
        for calc in calculations:
            target = calc['target']
            formula = calc['formula']
            
            if target == 'hit_points_max':
                # Calculate max HP (not current)
                value = self.evaluate_formula(character, formula)
                character.hit_points_max = value
            
            elif target == 'armor_class':
                # Base armor class calculation
                value = self.evaluate_formula(character, formula)
                character.armor_class = value
            
            elif target == 'spell_slots':
                # Maximum spell slots
                level = calc.get('spell_level')
                value = self.evaluate_formula(character, formula)
                character.spell_slots[str(level)] = value
```

## Phase 4: Character Creation Workflow (Weeks 7-8)

### 4.1 Creation Focus

Since we're building characters for between sessions, the workflow emphasizes:
- **Complete character builds** before play starts
- **All selections resolved** (no pending choices during play)
- **Print-ready output** as the final step

```python
class CharacterCreationWorkflow:
    """Manages character creation for session prep."""
    
    PHASES = [
        'initialize',
        'ability_scores', 
        'race',
        'class',
        'background',
        'equipment',
        'spells',      # Spell selection/preparation
        'details',     # Name, backstory, etc.
        'review',      # Final review before printing
        'finalize'
    ]
    
    def complete_phase(self, character: CharacterState) -> dict:
        """Ensure phase is complete before advancing."""
        
        phase = character.creation_phase
        
        if phase == 'ability_scores':
            # All 6 abilities must be set
            if len(character.ability_scores) != 6:
                return {'complete': False, 'missing': 'ability scores'}
        
        elif phase == 'spells':
            # Spellcasters must prepare spells
            if character.has_spellcasting:
                prepared_count = len(character.spells_prepared)
                allowed_count = self.calculate_prepared_spells(character)
                if prepared_count < allowed_count:
                    return {
                        'complete': False, 
                        'missing': f'{allowed_count - prepared_count} prepared spells'
                    }
        
        elif phase == 'review':
            # Generate preview sheet
            preview = self.generate_character_sheet(character)
            return {
                'complete': True,
                'preview': preview,
                'can_print': True
            }
        
        return {'complete': True}
```

### 4.2 Equipment Management

```python
class EquipmentManager:
    """Manage character equipment between sessions."""
    
    def add_starting_equipment(self, character: CharacterState, 
                              selections: List[dict]):
        """Add selected starting equipment."""
        
        for selection in selections:
            for item in selection['items']:
                self.add_item(
                    character,
                    item_id=item['item_id'],
                    quantity=item.get('quantity', 1),
                    equipped=item.get('equipped', False)
                )
    
    def add_item(self, character: CharacterState, 
                 item_id: str, quantity: int = 1,
                 equipped: bool = False):
        """Add item to inventory."""
        
        # Check if stackable
        existing = next(
            (i for i in character.inventory if i['item_id'] == item_id),
            None
        )
        
        if existing and self.is_stackable(item_id):
            existing['quantity'] += quantity
        else:
            character.inventory.append({
                'item_id': item_id,
                'quantity': quantity,
                'equipped': equipped,
                'attuned': False
            })
        
        # Recalculate AC if armor/shield
        if equipped and self.is_armor_or_shield(item_id):
            self.recalculate_armor_class(character)
    
    def calculate_carrying_capacity(self, character: CharacterState) -> dict:
        """Calculate weight limits for reference."""
        
        strength = character.ability_scores.get('str', 10)
        
        return {
            'carry_capacity': strength * 15,
            'push_drag_lift': strength * 30,
            'encumbered': strength * 5,
            'heavily_encumbered': strength * 10,
            'current_weight': self.calculate_total_weight(character)
        }
```

## Phase 5: Character Maintenance (Weeks 9-10)

### 5.1 Between-Session Updates

```python
class CharacterMaintenance:
    """Handle character updates between sessions."""
    
    def __init__(self, rules_engine: RulesEngine):
        self.rules_engine = rules_engine
    
    def level_up(self, character: CharacterState, 
                 class_id: str, hp_roll: int = None) -> dict:
        """Process level advancement between sessions."""
        
        # Increase level
        character.level += 1
        
        # Update class level
        class_entry = next(
            (c for c in character.classes if c['class_id'] == class_id),
            None
        )
        
        if class_entry:
            class_entry['level'] += 1
        else:
            # Multiclassing
            character.classes.append({
                'class_id': class_id,
                'level': 1
            })
        
        # Apply level-based rules
        self.rules_engine.apply_rules(character, 'level', None)
        self.rules_engine.apply_rules(character, 'class_level', 
                                     f"{class_id}-{class_entry['level']}")
        
        # Calculate HP increase
        hit_die = self.get_class_hit_die(class_id)
        con_mod = character.ability_modifiers.get('con', 0)
        
        if hp_roll:
            hp_increase = max(1, hp_roll + con_mod)
        else:
            # Average HP
            hp_increase = (hit_die // 2 + 1) + con_mod
        
        character.hit_points_max += hp_increase
        
        # Check for ASI/Feat at appropriate levels
        if class_entry['level'] in [4, 8, 12, 16, 19]:
            return {
                'success': True,
                'hp_increase': hp_increase,
                'requires_asi_choice': True
            }
        
        return {
            'success': True,
            'hp_increase': hp_increase
        }
    
    def prepare_spells(self, character: CharacterState, 
                      spell_ids: List[str]) -> bool:
        """Update prepared spells for next session."""
        
        # Validate spell count
        max_prepared = self.calculate_prepared_spells(character)
        if len(spell_ids) > max_prepared:
            return False
        
        # Validate all spells are known
        known_spells = [s['spell_id'] for s in character.spells]
        for spell_id in spell_ids:
            if spell_id not in known_spells:
                return False
        
        # Update prepared list
        for spell in character.spells:
            spell['prepared'] = spell['spell_id'] in spell_ids
        
        return True
    
    def add_treasure(self, character: CharacterState, 
                    treasure: List[dict]) -> dict:
        """Add loot gained during session."""
        
        added_items = []
        
        for item in treasure:
            if item['type'] == 'currency':
                # Add to wealth tracking
                character.wealth[item['currency']] += item['amount']
            
            elif item['type'] == 'item':
                # Add to inventory
                self.add_item(character, item['item_id'], 
                            item.get('quantity', 1))
                added_items.append(item)
        
        return {
            'items_added': added_items,
            'new_total_weight': self.calculate_total_weight(character)
        }
```

### 5.2 Character Sheet Generation

```python
class CharacterSheetGenerator:
    """Generate printable character sheets."""
    
    def generate_sheet(self, character: CharacterState, 
                      format: str = 'standard') -> dict:
        """Generate complete character sheet data."""
        
        sheet = {
            'basic_info': {
                'name': character.name,
                'race': character.race_name,
                'classes': self.format_classes(character.classes),
                'level': character.level,
                'background': character.background_name,
                'alignment': character.alignment,
                'experience': character.experience_points
            },
            
            'abilities': self.generate_ability_block(character),
            
            'combat': {
                'armor_class': character.armor_class,
                'initiative': f"+{character.initiative}" if character.initiative >= 0 else str(character.initiative),
                'speed': character.speed,
                'hit_points': {
                    'maximum': character.hit_points_max,
                    'current': '___',  # Blank for player to track
                    'temporary': '___'
                },
                'hit_dice': self.format_hit_dice(character),
                'death_saves': {
                    'successes': '○ ○ ○',
                    'failures': '○ ○ ○'
                }
            },
            
            'proficiencies': {
                'proficiency_bonus': f"+{character.proficiency_bonus}",
                'saves': self.generate_saves(character),
                'skills': self.generate_skills(character)
            },
            
            'features': self.organize_features(character),
            
            'equipment': {
                'items': self.format_equipment(character),
                'weight': self.calculate_weight_summary(character),
                'currency': character.wealth
            },
            
            'spellcasting': self.generate_spellcasting_block(character) if character.has_spellcasting else None,
            
            'character_details': {
                'personality': character.personality_traits,
                'ideals': character.ideals,
                'bonds': character.bonds,
                'flaws': character.flaws,
                'backstory': character.backstory
            }
        }
        
        return sheet
    
    def generate_spellcasting_block(self, character: CharacterState) -> dict:
        """Generate spell section for character sheet."""
        
        spellcasting = {
            'ability': character.spellcasting_ability.upper(),
            'spell_save_dc': character.spell_save_dc,
            'spell_attack': f"+{character.spell_attack_bonus}",
            
            'spell_slots': {
                level: {
                    'total': slots,
                    'expended': '___'  # Blank for tracking
                }
                for level, slots in character.spell_slots.items()
                if slots > 0
            },
            
            'spells': {}
        }
        
        # Organize spells by level
        for spell_data in character.spells:
            spell = self.get_spell_details(spell_data['spell_id'])
            level = str(spell['level'])
            
            if level not in spellcasting['spells']:
                spellcasting['spells'][level] = {
                    'cantrips' if level == '0' else f'level_{level}': []
                }
            
            spell_entry = {
                'name': spell['name'],
                'prepared': spell_data.get('prepared', False),
                'ritual': spell.get('ritual', False),
                'concentration': spell.get('concentration', False),
                'casting_time': spell['casting_time'],
                'range': spell['range'],
                'components': spell['components'],
                'school': spell['school']
            }
            
            key = 'cantrips' if level == '0' else f'level_{level}'
            spellcasting['spells'][level][key].append(spell_entry)
        
        return spellcasting
    
    def export_to_pdf(self, character_sheet: dict) -> bytes:
        """Generate PDF character sheet."""
        # PDF generation logic here
        pass
    
    def export_to_form_fillable(self, character_sheet: dict) -> bytes:
        """Generate form-fillable PDF."""
        # Form-fillable PDF logic here
        pass
```

## Phase 6: API and Integration (Weeks 11-12)

### 6.1 Simplified API

```python
class CharacterAPI:
    """API focused on character building and sheet generation."""
    
    @app.route('/api/characters', methods=['POST'])
    def create_character():
        """Start new character creation."""
        data = request.json
        
        character = character_service.create_character(
            player_name=data['player_name'],
            campaign_id=data['campaign_id'],
            rule_system_id=data['rule_system_id']
        )
        
        return jsonify({
            'character_id': character.id,
            'creation_phase': 'ability_scores'
        })
    
    @app.route('/api/characters/<int:id>/sheet', methods=['GET'])
    def get_character_sheet(id):
        """Get printable character sheet."""
        character = character_service.get_character(id)
        sheet = sheet_generator.generate_sheet(character)
        
        format = request.args.get('format', 'json')
        
        if format == 'pdf':
            pdf_data = sheet_generator.export_to_pdf(sheet)
            return send_file(
                io.BytesIO(pdf_data),
                mimetype='application/pdf',
                as_attachment=True,
                download_name=f'{character.name}_character_sheet.pdf'
            )
        
        return jsonify(sheet)
    
    @app.route('/api/characters/<int:id>/level-up', methods=['POST'])
    def level_up_character(id):
        """Process level advancement."""
        data = request.json
        character = character_service.get_character(id)
        
        result = character_maintenance.level_up(
            character,
            class_id=data['class_id'],
            hp_roll=data.get('hp_roll')
        )
        
        return jsonify(result)
    
    @app.route('/api/characters/<int:id>/prepare-spells', methods=['POST'])
    def prepare_spells(id):
        """Update prepared spells for next session."""
        data = request.json
        character = character_service.get_character(id)
        
        success = character_maintenance.prepare_spells(
            character,
            spell_ids=data['spell_ids']
        )
        
        return jsonify({'success': success})
    
    @app.route('/api/characters/<int:id>/equipment', methods=['POST'])
    def add_equipment(id):
        """Add equipment between sessions."""
        data = request.json
        character = character_service.get_character(id)
        
        equipment_manager.add_item(
            character,
            item_id=data['item_id'],
            quantity=data.get('quantity', 1),
            equipped=data.get('equipped', False)
        )
        
        return jsonify({
            'success': True,
            'new_ac': character.armor_class
        })
```

### 6.2 Campaign Integration

```python
class CampaignCharacterManager:
    """Manage characters within campaigns."""
    
    def get_party_summary(self, campaign_id: int) -> dict:
        """Get summary of all characters for session prep."""
        
        characters = self.get_campaign_characters(campaign_id)
        
        return {
            'party_composition': {
                'total_members': len(characters),
                'average_level': sum(c.level for c in characters) / len(characters),
                'classes': self.count_classes(characters),
                'roles': self.analyze_party_roles(characters)
            },
            'characters': [
                {
                    'id': c.id,
                    'name': c.name,
                    'player': c.player_name,
                    'level': c.level,
                    'classes': c.class_summary,
                    'hp_max': c.hit_points_max,
                    'ac': c.armor_class,
                    'passive_perception': 10 + c.skill_bonuses.get('perception', 0)
                }
                for c in characters
            ]
        }
    
    def prepare_session_handouts(self, session_id: int) -> List[dict]:
        """Generate character sheets for upcoming session."""
        
        session = self.get_session(session_id)
        characters = self.get_session_characters(session_id)
        
        handouts = []
        for character in characters:
            sheet = sheet_generator.generate_sheet(character)
            
            handouts.append({
                'character_id': character.id,
                'player_name': character.player_name,
                'sheet_data': sheet,
                'level_up_available': self.check_level_up(character),
                'notes': self.get_session_notes(session_id, character.id)
            })
        
        return handouts
```

### 6.3 Import/Export

```python
class CharacterPortability:
    """Import/export for character backup and sharing."""
    
    def export_character(self, character: CharacterState) -> dict:
        """Export character data for backup."""
        
        return {
            'version': '1.0',
            'exported_date': datetime.now().isoformat(),
            'character': {
                'basic_info': {
                    'name': character.name,
                    'level': character.level,
                    'race': character.race_id,
                    'background': character.background_id,
                    'classes': character.classes
                },
                'abilities': character.ability_scores,
                'proficiencies': character.proficiencies,
                'features': character.features,
                'equipment': character.inventory,
                'spells': character.spells,
                'wealth': character.wealth,
                'character_details': {
                    'personality': character.personality_traits,
                    'ideals': character.ideals,
                    'bonds': character.bonds,
                    'flaws': character.flaws,
                    'backstory': character.backstory
                }
            }
        }
    
    def import_character(self, data: dict, campaign_id: int) -> CharacterState:
        """Import character from backup."""
        
        char_data = data['character']
        
        # Create new character
        character = CharacterState(
            campaign_id=campaign_id,
            name=char_data['basic_info']['name'],
            level=char_data['basic_info']['level']
        )
        
        # Restore all persistent data
        character.race_id = char_data['basic_info']['race']
        character.background_id = char_data['basic_info']['background']
        character.classes = char_data['basic_info']['classes']
        character.ability_scores = char_data['abilities']
        character.proficiencies = char_data['proficiencies']
        character.features = char_data['features']
        character.inventory = char_data['equipment']
        character.spells = char_data['spells']
        character.wealth = char_data['wealth']
        
        # Restore character details
        details = char_data['character_details']
        character.personality_traits = details['personality']
        character.ideals = details['ideals']
        character.bonds = details['bonds']
        character.flaws = details['flaws']
        character.backstory = details['backstory']
        
        # Recalculate derived values
        self.rules_engine.calculate_sheet_values(character)
        
        return character
```

## Implementation Timeline Summary

### Month 1: Foundation (Weeks 1-4)
- **Week 1-2**: Database schema for persistent character data
- **Week 3-4**: 5etools converter for character building rules

### Month 2: Core Engine (Weeks 5-8)
- **Week 5-6**: Rules engine for character creation
- **Week 7-8**: Character creation workflow

### Month 3: Character Management (Weeks 9-12)
- **Week 9-10**: Level up and character maintenance
- **Week 11-12**: Sheet generation and printing

### Month 4: Polish and Integration
- **Week 13-14**: Campaign integration and party management
- **Week 15-16**: Import/export and final polish

## Success Criteria

1. **Character Creation**
   - Complete PHB character in < 10 minutes
   - All choices resolved before play
   - Professional character sheet output

2. **Between-Session Management**
   - Level up in < 2 minutes
   - Easy spell preparation
   - Simple inventory management

