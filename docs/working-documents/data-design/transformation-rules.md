# Data Transformation Rules (Denormalized)

## Overview
This document defines how to transform 5etools JSON data into the denormalized SQLite schema with multi-ruleset support.

## General Transformation Principles

### 1. ID Generation
- Primary keys are simple slugified names: `lowercase(name)`
- Replace spaces with hyphens
- Remove special characters except hyphens and apostrophes
- For subclasses/subraces: prefix with parent name
- Examples:
  - "Bag of Holding" → `bag-of-holding`
  - "High Elf" (subrace) → `high-elf`
  - "Champion" (Fighter subclass) → `fighter-champion`
  - "+1 Longsword" → `longsword-+1`

### 2. Rule System and Source Management
- All entities require `rule_system_id` (e.g., 'dnd5e-2014', 'dnd5e-2024')
- Map source abbreviations to the sources table
- Sources are linked to their rule system
- Handle `otherSources` array by creating additional reference entries

### 3. Denormalization Strategy
- **Races/Subraces**: Copy all base race data to subrace records
- **Classes/Subclasses**: Copy all base class data to subclass records  
- **Items/Variants**: Copy all base item data to variant records
- **Spells**: Store class list as JSON array (no junction table)
- **Creatures**: Store actions/traits/reactions as JSON arrays

### 4. Text Processing
- Preserve 5etools formatting tags in entries
- Extract plain text separately for future search indexing
- Handle nested entry objects recursively

## Entity-Specific Rules

### Races (Denormalized)
```python
def transform_race(race_json, rule_system_id):
    # Determine if this is a subrace
    is_subrace = 'raceName' in race_json or 'raceSource' in race_json
    parent_race_id = None
    
    if is_subrace:
        # For subraces, load parent race data first
        parent_race = load_parent_race(race_json)
        parent_race_id = generate_id(parent_race['name'], parent_race['source'], rule_system_id)
        
        # Merge parent data with subrace data
        merged_data = merge_race_data(parent_race, race_json)
    else:
        merged_data = race_json
    
    return {
        'id': generate_id(race_json['name'], race_json['source'], rule_system_id),
        'name': race_json['name'],
        'rule_system_id': rule_system_id,
        'source_id': race_json['source'],
        'page': race_json.get('page'),
        'race_type': 'subrace' if is_subrace else 'race',
        'parent_race_id': parent_race_id,
        # All fields populated from merged data
        'size': merged_data.get('size', ['M'])[0],
        'speed': json.dumps(merged_data.get('speed', {'walk': 30})),
        'ability_scores': json.dumps(merged_data.get('ability', [])),
        'age': json.dumps(merged_data.get('age')),
        'alignment_tendency': extract_alignment_text(merged_data),
        'language_proficiencies': json.dumps(merged_data.get('languageProficiencies', [])),
        'trait_tags': json.dumps(merged_data.get('traitTags', [])),
        'entries': json.dumps(race_json.get('entries', []))  # Keep original entries
    }
```

### Classes (Denormalized)
```python
def transform_class(class_json, rule_system_id, parent_class=None):
    is_subclass = parent_class is not None
    
    if is_subclass:
        # Merge parent class data
        merged_data = merge_class_data(parent_class, class_json)
        class_id = generate_id(f"{parent_class['name']}-{class_json['name']}", 
                              class_json['source'], rule_system_id)
        parent_class_id = generate_id(parent_class['name'], parent_class['source'], rule_system_id)
    else:
        merged_data = class_json
        class_id = generate_id(class_json['name'], class_json['source'], rule_system_id)
        parent_class_id = None
    
    return {
        'id': class_id,
        'name': class_json['name'],
        'rule_system_id': rule_system_id,
        'source_id': class_json['source'],
        'page': class_json.get('page'),
        'class_type': 'subclass' if is_subclass else 'class',
        'parent_class_id': parent_class_id,
        # Base class fields (populated for all)
        'hit_die': merged_data.get('hd', {}).get('faces'),
        'primary_abilities': json.dumps(merged_data.get('primaryAbility', [])),
        'saving_throws': json.dumps(merged_data.get('proficiency', [])),
        'skill_proficiency_count': merged_data.get('skills', {}).get('choose', {}).get('count'),
        'skill_proficiency_choices': json.dumps(merged_data.get('skills', {})),
        'starting_proficiencies': json.dumps(merged_data.get('startingProficiencies', {})),
        'starting_equipment': json.dumps(merged_data.get('startingEquipment', {})),
        'spell_ability': merged_data.get('spellcastingAbility'),
        'caster_progression': merged_data.get('casterProgression'),
        # Subclass specific
        'subclass_title': class_json.get('subclassTitle') if is_subclass else None,
        'subclass_level': class_json.get('subclassLevel') if is_subclass else None,
        # Shared fields
        'features': json.dumps(merge_features(merged_data)),
        'spell_slots': json.dumps(merged_data.get('classTableGroups', [])),
        'entries': json.dumps(class_json.get('entries', []))
    }
```

### Items (Denormalized)
```python
def transform_item(item_json, rule_system_id):
    # Check if this is a variant of a base item
    base_item_ref = item_json.get('baseItem')
    base_item_id = None
    
    if base_item_ref:
        # Load base item data
        base_item = load_base_item(base_item_ref)
        base_item_id = generate_id(base_item['name'], base_item['source'], rule_system_id)
        
        # Copy all base item properties
        merged_data = {**base_item, **item_json}
    else:
        merged_data = item_json
    
    # Determine item type
    item_type = merged_data.get('type')
    if not item_type and 'wondrous' in merged_data:
        item_type = 'W'
    
    # Calculate value in copper
    value_cp = None
    if 'value' in merged_data:
        value_cp = convert_to_copper(merged_data['value'])
    
    # Extract damage for weapons
    damage = None
    if 'damage' in merged_data:
        damage = {
            'dice': merged_data['damage'],
            'type': merged_data.get('damageType')
        }
    
    return {
        'id': generate_id(item_json['name'], item_json['source'], rule_system_id),
        'name': item_json['name'],
        'rule_system_id': rule_system_id,
        'source_id': item_json['source'],
        'page': item_json.get('page'),
        'base_item_id': base_item_id,
        # All fields populated from merged data
        'type': item_type,
        'weight_lb': merged_data.get('weight'),
        'value_cp': value_cp,
        'armor_class': merged_data.get('ac'),
        'damage': json.dumps(damage) if damage else None,
        'properties': json.dumps(merged_data.get('property', [])),
        # Magic item fields
        'rarity': item_json.get('rarity'),  # Use original, not base
        'requires_attunement': bool(item_json.get('reqAttune')),
        'attunement_prereq': json.dumps(item_json.get('reqAttuneTags')),
        'magic_bonus': item_json.get('bonusWeapon') or item_json.get('bonusAc'),
        'additional_properties': json.dumps(extract_magic_properties(item_json)),
        'entries': json.dumps(item_json.get('entries', []))
    }
```

### Spells (Denormalized with Classes)
```python
def transform_spell(spell_json, rule_system_id):
    # Extract duration info
    duration_data = spell_json.get('duration', [{}])[0]
    is_concentration = duration_data.get('concentration', False)
    
    # Extract components
    components = spell_json.get('components', {})
    
    # Build class list - NO JUNCTION TABLE
    classes = extract_spell_classes(spell_json)
    
    return {
        'id': generate_id(spell_json['name'], spell_json['source'], rule_system_id),
        'name': spell_json['name'],
        'rule_system_id': rule_system_id,
        'source_id': spell_json['source'],
        'page': spell_json.get('page'),
        'level': spell_json.get('level', 0),
        'school': spell_json.get('school'),
        'casting_time': json.dumps(spell_json.get('time', [])),
        'range': json.dumps(spell_json.get('range')),
        'components': json.dumps(components),
        'duration': json.dumps(spell_json.get('duration', [])),
        'is_ritual': spell_json.get('meta', {}).get('ritual', False),
        'is_concentration': is_concentration,
        'saving_throw': json.dumps(spell_json.get('savingThrow', [])),
        'damage_type': json.dumps(spell_json.get('damageInflict', [])),
        'entries': json.dumps(spell_json.get('entries', [])),
        'upcast_info': json.dumps(spell_json.get('entriesHigherLevel', [])),
        'classes': json.dumps(classes)  # Stored as JSON array
    }

def extract_spell_classes(spell_json):
    """Extract all classes that can cast this spell."""
    classes = []
    
    # From main classes
    if 'classes' in spell_json:
        for class_info in spell_json['classes'].get('fromClassList', []):
            classes.append(class_info['name'].lower())
    
    # From subclasses  
    if 'classes' in spell_json and 'fromSubclass' in spell_json['classes']:
        for subclass_info in spell_json['classes']['fromSubclass']:
            # Include both for searching
            classes.append(f"{subclass_info['class']['name'].lower()}")
    
    return list(set(classes))  # Remove duplicates
```

### Creatures (Denormalized with Actions)
```python
def transform_creature(creature_json, rule_system_id):
    # Handle complex AC format
    ac_value = creature_json.get('ac')
    if isinstance(ac_value, list):
        ac_json = ac_value
    else:
        ac_json = [{'ac': ac_value}]
    
    # Calculate proficiency bonus if not provided
    cr = creature_json.get('cr', '0')
    prof_bonus = calculate_proficiency_bonus(cr)
    
    return {
        'id': generate_id(creature_json['name'], creature_json['source'], rule_system_id),
        'name': creature_json['name'],
        'rule_system_id': rule_system_id,
        'source_id': creature_json['source'],
        'page': creature_json.get('page'),
        'size': creature_json.get('size', ['M'])[0],
        'type': extract_creature_type(creature_json.get('type')),
        'type_tags': json.dumps(extract_type_tags(creature_json.get('type'))),
        'alignment': json.dumps(creature_json.get('alignment', [])),
        'armor_class': json.dumps(ac_json),
        'hit_points': json.dumps(creature_json.get('hp')),
        'speed': json.dumps(creature_json.get('speed')),
        'ability_scores': json.dumps({
            'str': creature_json.get('str', 10),
            'dex': creature_json.get('dex', 10),
            'con': creature_json.get('con', 10),
            'int': creature_json.get('int', 10),
            'wis': creature_json.get('wis', 10),
            'cha': creature_json.get('cha', 10)
        }),
        'saving_throws': json.dumps(creature_json.get('save')),
        'skills': json.dumps(creature_json.get('skill')),
        'damage_resistances': json.dumps(creature_json.get('resist')),
        'damage_immunities': json.dumps(creature_json.get('immune')),
        'condition_immunities': json.dumps(creature_json.get('conditionImmune')),
        'senses': json.dumps(creature_json.get('senses', [])),
        'languages': json.dumps(creature_json.get('languages', [])),
        'challenge_rating': str(cr),
        'proficiency_bonus': prof_bonus,
        # Actions stored as JSON arrays - NO SEPARATE TABLE
        'traits': json.dumps(creature_json.get('trait', [])),
        'actions': json.dumps(creature_json.get('action', [])),
        'reactions': json.dumps(creature_json.get('reaction', [])),
        'legendary_actions': json.dumps(creature_json.get('legendary', [])),
        'lair_actions': json.dumps(creature_json.get('lairActions', [])),
        'regional_effects': json.dumps(creature_json.get('regionalEffects', [])),
        'entries': json.dumps(creature_json.get('entries', [])),
        'environment': json.dumps(creature_json.get('environment', [])),
        'is_npc': creature_json.get('isNpc', False)
    }
```

## Cross-Reference Extraction

### Tag Patterns
```python
TAG_PATTERNS = {
    'creature': r'{@creature ([^}|]+)(?:\|([^}]+))?}',
    'spell': r'{@spell ([^}|]+)(?:\|([^}]+))?}',
    'item': r'{@item ([^}|]+)(?:\|([^}]+))?}',
    'condition': r'{@condition ([^}]+)}',
    'skill': r'{@skill ([^}]+)}',
    'damage': r'{@damage ([^}]+)}',
    'dice': r'{@dice ([^}]+)}',
    'dc': r'{@dc ([^}]+)}',
}

def extract_references(text, source_entity_id, source_entity_type, rule_system_id):
    references = []
    
    for ref_type, pattern in TAG_PATTERNS.items():
        matches = re.finditer(pattern, text)
        for match in matches:
            name = match.group(1)
            source = match.group(2) if match.lastindex >= 2 else None
            
            references.append({
                'source_entity_id': source_entity_id,
                'source_entity_type': source_entity_type,
                'target_entity_id': None,  # Resolved in post-processing
                'target_entity_type': ref_type,
                'rule_system_id': rule_system_id,
                'reference_type': 'mentions',
                'context': text[max(0, match.start()-50):match.end()+50]
            })
    
    return references
```

## Helper Functions

### Currency Conversion
```python
def convert_to_copper(value_obj):
    """Convert currency object to copper pieces."""
    if isinstance(value_obj, int):
        return value_obj
    
    total_cp = 0
    if 'cp' in value_obj:
        total_cp += value_obj['cp']
    if 'sp' in value_obj:
        total_cp += value_obj['sp'] * 10
    if 'ep' in value_obj:
        total_cp += value_obj['ep'] * 50
    if 'gp' in value_obj:
        total_cp += value_obj['gp'] * 100
    if 'pp' in value_obj:
        total_cp += value_obj['pp'] * 1000
    
    return total_cp
```

### CR to Proficiency Bonus
```python
def calculate_proficiency_bonus(cr):
    """Calculate proficiency bonus from CR."""
    if isinstance(cr, str) and '/' in cr:
        # Handle fractional CR
        num, den = cr.split('/')
        cr_value = float(num) / float(den)
    else:
        cr_value = float(cr)
    
    if cr_value < 5:
        return 2
    elif cr_value < 9:
        return 3
    elif cr_value < 13:
        return 4
    elif cr_value < 17:
        return 5
    elif cr_value < 21:
        return 6
    elif cr_value < 25:
        return 7
    elif cr_value < 29:
        return 8
    else:
        return 9
```

## Import Order

To maintain referential integrity:

1. **Rule Systems** - Define available rule systems
2. **Sources** - Import books/supplements for each rule system
3. **Base Classes** - Must exist before subclasses
4. **Core Entities** (can be parallel):
   - Base Races (before subraces)
   - Base Items (before variants)
   - Backgrounds
   - Feats
5. **Dependent Entities**:
   - Subraces (after base races)
   - Subclasses (after base classes)
   - Item Variants (after base items)
6. **Spells** - After classes (for validation)
7. **Creatures** - Can be imported anytime after sources
8. **Cross-references** - After all entities exist

## Error Handling

### Missing References
- Log missing base items/races/classes
- Skip variants if base doesn't exist
- Store partial reference data for later resolution

### Invalid Data
- Validate required fields (name, source, rule_system)
- Use sensible defaults for missing optional fields
- Log validation errors with file and line context

### Duplicate Keys
- Check for existing records before insert
- For same rule system: update if source is newer
- For different rule systems: both can coexist

### Data Merging Conflicts
- Subrace/subclass properties override base properties
- Item variant properties override base properties
- Log conflicts for manual review