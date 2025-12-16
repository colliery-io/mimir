// Mimir Character Sheet with Spell Cards Template
// Multi-page document: Character sheet + Spell cards
// Spell cards printed 9 per page (3x3 grid)

#import "/_shared/styles.typ": *
#import "/_shared/components.typ": *
#import "/_shared/icons.typ": *

// Document setup
#show: mimir-doc.with(margin: 0.4in)

// Helper to safely get nested data
#let get(obj, key, default: none) = {
  if obj != none and key in obj { obj.at(key) } else { default }
}

// Helper for formatted modifier
#let fmt-mod(score) = {
  let mod = calc.floor((score - 10) / 2)
  if mod >= 0 { "+" + str(mod) } else { str(mod) }
}

// Calculate proficiency bonus from level
#let prof-bonus(level) = {
  if level <= 4 { 2 }
  else if level <= 8 { 3 }
  else if level <= 12 { 4 }
  else if level <= 16 { 5 }
  else { 6 }
}

// =============================================================================
// DATA EXTRACTION
// =============================================================================

// The data structure has: { character: {...}, spells: [...], class_features_details: [...], subclass_features_details: [...], include_spell_cards: bool }
#let char-data = get(data, "character", default: (:))
#let spell-list = get(data, "spells", default: ())
#let class-features-details = get(data, "class_features_details", default: ())
#let subclass-features-details = get(data, "subclass_features_details", default: ())
#let include-spell-cards = get(data, "include_spell_cards", default: false)

// Helper to find feature details by name
#let get-feature-description(feature-name) = {
  // First check class features
  let class-match = if type(class-features-details) == array {
    class-features-details.find((f) => get(f, "name", default: "") == feature-name)
  } else { none }

  if class-match != none {
    let entries = get(class-match, "entries", default: ())
    if type(entries) == array and entries.len() > 0 {
      // Extract first text entry as description
      let first-entry = entries.at(0)
      if type(first-entry) == str { first-entry }
      else { none }
    } else { none }
  } else {
    // Check subclass features
    let subclass-match = if type(subclass-features-details) == array {
      subclass-features-details.find((f) => get(f, "name", default: "") == feature-name)
    } else { none }

    if subclass-match != none {
      let entries = get(subclass-match, "entries", default: ())
      if type(entries) == array and entries.len() > 0 {
        let first-entry = entries.at(0)
        if type(first-entry) == str { first-entry }
        else { none }
      } else { none }
    } else { none }
  }
}

// =============================================================================
// CHARACTER SHEET (PAGE 1)
// =============================================================================

#let char-name = get(char-data, "character_name", default: "Unknown Character")
#let level = get(char-data, "level", default: 1)
#let race = get(char-data, "race", default: "Unknown")
#let subrace = get(char-data, "subrace", default: none)
#let background = get(char-data, "background", default: "Unknown")
#let alignment = get(char-data, "alignment", default: none)
#let classes = get(char-data, "classes", default: ())

// Build class string
#let class-str = if classes.len() > 0 {
  classes.map((c) => {
    let name = get(c, "class_name", default: "?")
    let lvl = get(c, "level", default: 0)
    let sub = get(c, "subclass", default: none)
    if sub != none { [#name (#sub) #lvl] } else { [#name #lvl] }
  }).join(" / ")
} else { "No Class" }

// Race string with subrace
#let race-str = if subrace != none { [#subrace #race] } else { race }

// Header block
#block(
  width: 100%,
  inset: spacing.md,
  stroke: (bottom: 2pt + colors.accent),
  {
    grid(
      columns: (1fr, auto),
      title-text(char-name),
      if classes.len() > 0 {
        let primary = classes.at(0)
        class-icon(get(primary, "class_name", default: "Fighter"), size: sizes.xxl)
      }
    )
    v(spacing.xs)
    text(size: sizes.md)[Level #level #race-str #class-str]
    if background != "Unknown" or alignment != none {
      linebreak()
      small-text[
        #if background != "Unknown" [#background]
        #if alignment != none [, #alignment]
      ]
    }
  }
)

#v(spacing.md)

// =============================================================================
// MAIN STATS ROW
// =============================================================================

#let abilities = get(char-data, "abilities", default: (
  strength: 10,
  dexterity: 10,
  constitution: 10,
  intelligence: 10,
  wisdom: 10,
  charisma: 10,
))
#let str-score = get(abilities, "strength", default: 10)
#let dex-score = get(abilities, "dexterity", default: 10)
#let con-score = get(abilities, "constitution", default: 10)
#let int-score = get(abilities, "intelligence", default: 10)
#let wis-score = get(abilities, "wisdom", default: 10)
#let cha-score = get(abilities, "charisma", default: 10)

#let max-hp = get(char-data, "max_hp", default: 10)
#let current-hp = get(char-data, "current_hp", default: 10)
#let speed = get(char-data, "speed", default: 30)

#let proficiencies = get(char-data, "proficiencies", default: ())
#let prof-saves = get(proficiencies, "saves", default: ())

// Calculate AC (basic formula: 10 + dex mod, or from equipped armor)
#let equipped = get(char-data, "equipped", default: ())
#let has-armor = get(equipped, "armor", default: none) != none
#let has-shield = get(equipped, "shield", default: none) != none
// Simplified AC calculation - would need armor data for real calculation
#let base-ac = 10 + calc.floor((dex-score - 10) / 2)
#let shield-bonus = if has-shield { 2 } else { 0 }
#let ac = base-ac + shield-bonus

#grid(
  columns: (2fr, 1fr),
  column-gutter: spacing.md,

  // Ability Scores
  ability-scores(
    str: str-score,
    dex: dex-score,
    con: con-score,
    int: int-score,
    wis: wis-score,
    cha: cha-score,
    layout: "grid"
  ),

  // Combat Stats
  {
    info-box(title: "Combat")[
      #grid(
        columns: (1fr, 1fr),
        row-gutter: spacing.sm,
        column-gutter: spacing.md,

        labeled-value("HP", [#current-hp / #max-hp]),
        labeled-value("AC", str(ac)),
        labeled-value("Speed", [#speed ft]),
        labeled-value("Prof", [+#prof-bonus(level)]),
      )
    ]

    v(spacing.sm)

    // Saving Throws
    info-box(title: "Saving Throws")[
      #let saves = (
        ("STR", str-score, "Strength" in prof-saves),
        ("DEX", dex-score, "Dexterity" in prof-saves),
        ("CON", con-score, "Constitution" in prof-saves),
        ("INT", int-score, "Intelligence" in prof-saves),
        ("WIS", wis-score, "Wisdom" in prof-saves),
        ("CHA", cha-score, "Charisma" in prof-saves),
      )
      #for (name, score, prof) in saves {
        let mod = calc.floor((score - 10) / 2)
        let total = mod + if prof { prof-bonus(level) } else { 0 }
        let sign = if total >= 0 { "+" } else { "" }
        text(size: sizes.sm)[
          #if prof { text(weight: "bold")[#name] } else { name }
          #h(1fr)
          #sign#total
        ]
        linebreak()
      }
    ]
  }
)

#v(spacing.md)

// =============================================================================
// PROFICIENCIES AND EQUIPMENT
// =============================================================================

#grid(
  columns: (1fr, 1fr),
  column-gutter: spacing.md,

  // Proficiencies
  {
    info-box(title: "Proficiencies")[
      #let prof-skills = get(proficiencies, "skills", default: ())
      #let prof-tools = get(proficiencies, "tools", default: ())
      #let prof-langs = get(proficiencies, "languages", default: ())
      #let prof-armor = get(proficiencies, "armor", default: ())
      #let prof-weapons = get(proficiencies, "weapons", default: ())

      #if prof-skills.len() > 0 [
        #label-text("Skills")
        #linebreak()
        #text(size: sizes.sm)[#prof-skills.join(", ")]
        #v(spacing.sm)
      ]

      #if prof-armor.len() > 0 or prof-weapons.len() > 0 [
        #label-text("Armor & Weapons")
        #linebreak()
        #text(size: sizes.sm)[#(prof-armor + prof-weapons).join(", ")]
        #v(spacing.sm)
      ]

      #if prof-tools.len() > 0 [
        #label-text("Tools")
        #linebreak()
        #text(size: sizes.sm)[#prof-tools.join(", ")]
        #v(spacing.sm)
      ]

      #if prof-langs.len() > 0 [
        #label-text("Languages")
        #linebreak()
        #text(size: sizes.sm)[#prof-langs.join(", ")]
      ]
    ]
  },

  // Equipment
  {
    info-box(title: "Equipment")[
      #let armor = get(equipped, "armor", default: none)
      #let shield = get(equipped, "shield", default: none)
      #let main-hand = get(equipped, "main_hand", default: none)
      #let off-hand = get(equipped, "off_hand", default: none)

      #if armor != none [
        #inline-labeled("Armor", armor)
        #linebreak()
      ]
      #if shield != none [
        #inline-labeled("Shield", shield)
        #linebreak()
      ]
      #if main-hand != none [
        #inline-labeled("Main Hand", main-hand)
        #linebreak()
      ]
      #if off-hand != none [
        #inline-labeled("Off Hand", off-hand)
        #linebreak()
      ]

      #v(spacing.sm)

      // Currency
      #let currency = get(char-data, "currency", default: ())
      #let cp = get(currency, "copper", default: 0)
      #let sp = get(currency, "silver", default: 0)
      #let ep = get(currency, "electrum", default: 0)
      #let gp = get(currency, "gold", default: 0)
      #let pp = get(currency, "platinum", default: 0)

      #label-text("Currency")
      #linebreak()
      #text(size: sizes.sm)[
        #if pp > 0 [#pp pp ]
        #if gp > 0 [#gp gp ]
        #if ep > 0 [#ep ep ]
        #if sp > 0 [#sp sp ]
        #if cp > 0 [#cp cp]
      ]
    ]

    v(spacing.sm)

    // Inventory summary
    let inventory = get(char-data, "inventory", default: ())
    if inventory.len() > 0 [
      #info-box(title: "Inventory")[
        #for item in inventory.slice(0, calc.min(8, inventory.len())) {
          let name = get(item, "name", default: "?")
          let qty = get(item, "quantity", default: 1)
          text(size: sizes.sm)[#if qty > 1 [#qty x ]#name]
          linebreak()
        }
        #if inventory.len() > 8 [
          #small-text[... and #(inventory.len() - 8) more items]
        ]
      ]
    ]
  }
)

#v(spacing.md)

// =============================================================================
// FEATURES AND SPELLS SUMMARY
// =============================================================================

#grid(
  columns: (1fr, 1fr),
  column-gutter: spacing.md,

  // Features & Traits
  {
    let class-features = get(char-data, "class_features", default: ())
    let feats = get(char-data, "feats", default: ())

    info-box(title: "Features & Traits")[
      #if type(class-features) == array and class-features.len() > 0 [
        #for feature in class-features [
          #let feature-name = if type(feature) == dictionary { get(feature, "name", default: "?") } else { feature }
          #let feature-class = if type(feature) == dictionary { get(feature, "class_name", default: "") } else { "" }
          #let feature-subclass = if type(feature) == dictionary { get(feature, "subclass_name", default: none) } else { none }
          #let feature-level = if type(feature) == dictionary { get(feature, "level", default: 0) } else { 0 }
          #let feature-desc = get-feature-description(feature-name)

          #text(size: sizes.sm, weight: "bold")[#feature-name]
          #if feature-class != "" [
            #text(size: sizes.xs, fill: colors.text-secondary)[ (#if feature-subclass != none [#feature-subclass] else [#feature-class] Lv#feature-level)]
          ]
          #linebreak()
          #if feature-desc != none [
            #text(size: sizes.xs)[#feature-desc]
            #v(spacing.xs)
          ]
        ]
      ] else [
        #text(size: sizes.sm, fill: colors.text-secondary)[No features yet]
      ]
      #if type(feats) == array and feats.len() > 0 [
        #v(spacing.sm)
        #label-text("Feats")
        #linebreak()
        #for feat in feats [
          #text(size: sizes.sm)[- #feat]
          #linebreak()
        ]
      ]
    ]
  },

  // Spells Summary
  {
    let spells = get(char-data, "spells", default: ())
    let cantrips = get(spells, "cantrips", default: ())
    let prepared = get(spells, "prepared_spells", default: ())
    let known = get(spells, "known_spells", default: ())
    let slots = get(spells, "spell_slots", default: (:))

    if cantrips.len() > 0 or prepared.len() > 0 or known.len() > 0 [
      #info-box(title: "Spellcasting")[
        // Spell slots
        #if slots.keys().len() > 0 [
          #label-text("Spell Slots")
          #linebreak()
          #for level in slots.keys().sorted() {
            let slot = slots.at(level)
            let max-slots = get(slot, "max", default: 0)
            let current-slots = get(slot, "current", default: 0)
            text(size: sizes.sm)[Lv#level: #current-slots/#max-slots ]
          }
          #v(spacing.sm)
        ]

        // Cantrips (extract names from SpellReference objects)
        #if cantrips.len() > 0 [
          #label-text("Cantrips")
          #linebreak()
          #text(size: sizes.sm)[#cantrips.map((s) => if type(s) == dictionary { get(s, "name", default: "?") } else { s }).join(", ")]
          #v(spacing.sm)
        ]

        // Prepared/Known spells (extract names from SpellReference objects)
        #let char-spell-list = if prepared.len() > 0 { prepared } else { known }
        #if char-spell-list.len() > 0 [
          #label-text(if prepared.len() > 0 { "Prepared Spells" } else { "Known Spells" })
          #linebreak()
          #text(size: sizes.sm)[#char-spell-list.map((s) => if type(s) == dictionary { get(s, "name", default: "?") } else { s }).join(", ")]
        ]

        #if include-spell-cards and spell-list.len() > 0 [
          #v(spacing.sm)
          #small-text[(See attached spell cards)]
        ]
      ]
    ] else [
      // Personality for non-spellcasters
      #let personality = get(char-data, "personality", default: ())
      #let traits = get(personality, "traits", default: none)
      #let ideals = get(personality, "ideals", default: none)
      #let bonds = get(personality, "bonds", default: none)
      #let flaws = get(personality, "flaws", default: none)

      #info-box(title: "Personality")[
        #if traits != none [
          #label-text("Traits")
          #linebreak()
          #text(size: sizes.sm)[#traits]
          #v(spacing.sm)
        ]
        #if ideals != none [
          #label-text("Ideals")
          #linebreak()
          #text(size: sizes.sm)[#ideals]
          #v(spacing.sm)
        ]
        #if bonds != none [
          #label-text("Bonds")
          #linebreak()
          #text(size: sizes.sm)[#bonds]
          #v(spacing.sm)
        ]
        #if flaws != none [
          #label-text("Flaws")
          #linebreak()
          #text(size: sizes.sm)[#flaws]
        ]
      ]
    ]
  }
)

// =============================================================================
// FOOTER FOR CHARACTER SHEET
// =============================================================================

#v(1fr)
#align(center)[
  #small-text[Generated by Mimir]
]

// =============================================================================
// EQUIPMENT PAGE (PAGE 2)
// =============================================================================

#let eq-inventory = get(char-data, "inventory", default: ())
#let eq-equipped = get(char-data, "equipped", default: (:))
#let eq-currency = get(char-data, "currency", default: (:))
#let eq-item-details = get(data, "item_details", default: ())

// Simple helper to find item in details list
#let find-item-detail(name, source) = {
  let result = none
  for item in eq-item-details {
    if get(item, "name", default: "") == name and get(item, "source", default: "") == source {
      result = item
    }
  }
  result
}

// Item type code to full name mapping
#let item-type-names = (
  "M": "Melee Weapon",
  "R": "Ranged Weapon",
  "A": "Ammunition",
  "LA": "Light Armor",
  "MA": "Medium Armor",
  "HA": "Heavy Armor",
  "S": "Shield",
  "G": "Gear",
  "SCF": "Spellcasting Focus",
  "AT": "Artisan's Tools",
  "GS": "Gaming Set",
  "INS": "Instrument",
  "T": "Tools",
  "P": "Potion",
  "RD": "Rod",
  "RG": "Ring",
  "SC": "Scroll",
  "WD": "Wand",
  "W": "Wondrous Item",
  "$": "Currency/Treasure",
  "TAH": "Tack and Harness",
  "TG": "Trade Good",
  "MNT": "Mount",
  "VEH": "Vehicle",
  "EXP": "Explosive",
  "AF": "Arcane Focus",
  "AIR": "Vehicle (Air)",
  "SHP": "Vehicle (Water)",
)

#let get-item-type-name(type-code) = {
  if type-code == none { none }
  else {
    let code = str(type-code)
    item-type-names.at(code, default: code)
  }
}

#if eq-inventory.len() > 0 [
  #pagebreak()

  // Page header
  #block(
    width: 100%,
    inset: spacing.md,
    stroke: (bottom: 2pt + colors.accent),
  )[
    #title-text[#char-name's Equipment]
    #v(spacing.xs)
    #text(size: sizes.md)[Inventory and Gear]
  ]

  #v(spacing.md)

  // Currency
  #info-box(title: "Currency")[
    #{
      let cp = get(eq-currency, "copper", default: 0)
      let sp = get(eq-currency, "silver", default: 0)
      let ep = get(eq-currency, "electrum", default: 0)
      let gp = get(eq-currency, "gold", default: 0)
      let pp = get(eq-currency, "platinum", default: 0)
      [*PP:* #pp | *GP:* #gp | *EP:* #ep | *SP:* #sp | *CP:* #cp]
    }
  ]

  #v(spacing.sm)

  // Equipped Items
  #info-box(title: "Equipped")[
    #{
      let eq-armor = get(eq-equipped, "armor", default: none)
      let eq-shield = get(eq-equipped, "shield", default: none)
      let eq-main = get(eq-equipped, "main_hand", default: none)
      let eq-off = get(eq-equipped, "off_hand", default: none)

      [*Armor:* #if eq-armor != none [#eq-armor] else [None] |
      *Shield:* #if eq-shield != none [#eq-shield] else [None] |
      *Main Hand:* #if eq-main != none [#eq-main] else [None] |
      *Off Hand:* #if eq-off != none [#eq-off] else [None]]
    }
  ]

  #v(spacing.sm)

  // Inventory with details
  #info-box(title: "Inventory")[
    #set par(leading: 0.4em)
    #for item in eq-inventory [
      #{
        let item-name = get(item, "name", default: "Unknown")
        let item-source = get(item, "source", default: "PHB")
        let item-qty = get(item, "quantity", default: 1)
        let item-weight = get(item, "weight", default: 0)
        let item-notes = get(item, "notes", default: none)

        // Get catalog details
        let detail = find-item-detail(item-name, item-source)
        let item-type-code = if detail != none { get(detail, "type", default: none) } else { none }
        let item-type = get-item-type-name(item-type-code)
        let item-rarity = if detail != none { get(detail, "rarity", default: none) } else { none }
        let item-ac = if detail != none { get(detail, "ac", default: none) } else { none }
        let item-dmg = if detail != none { get(detail, "dmg1", default: none) } else { none }
        let item-entries = if detail != none { get(detail, "entries", default: none) } else { none }
        let item-desc = if item-entries != none and type(item-entries) == array and item-entries.len() > 0 {
          let first = item-entries.at(0)
          if type(first) == str { first } else { none }
        } else { none }

        // Build stats line
        let stats-parts = ()
        if item-type != none { stats-parts.push(item-type) }
        if item-rarity != none and item-rarity != "none" { stats-parts.push(item-rarity) }
        if item-ac != none { stats-parts.push("AC " + str(item-ac)) }
        if item-dmg != none { stats-parts.push(str(item-dmg) + " dmg") }
        if item-weight > 0 { stats-parts.push(str(item-weight) + " lb") }

        block(
          width: 100%,
          inset: (y: 2pt),
          below: 4pt,
        )[
          #text(weight: "bold", size: sizes.sm)[#if item-qty > 1 [#item-qty x ]#item-name]
          #if stats-parts.len() > 0 [
            #text(size: sizes.xs, fill: colors.text-secondary)[ (#stats-parts.join(", "))]
          ]
          #if item-notes != none [
            #linebreak()
            #text(size: sizes.xs, style: "italic")[#item-notes]
          ]
          #if item-desc != none [
            #linebreak()
            #text(size: sizes.xs, fill: colors.text-secondary)[#item-desc]
          ]
        ]
      }
    ]

    // Total weight
    #v(spacing.xs)
    #{
      let total = 0
      for item in eq-inventory {
        let w = get(item, "weight", default: 0)
        let q = get(item, "quantity", default: 1)
        total = total + w * q
      }
      align(right)[
        #text(weight: "bold", size: sizes.sm)[Total Weight: #total lb]
      ]
    }
  ]

  #v(1fr)
  #align(center)[
    #small-text[Generated by Mimir]
  ]
]

// =============================================================================
// SPELL CARDS SECTION (PAGES 2+)
// =============================================================================

#if include-spell-cards and spell-list.len() > 0 [
  // Card dimensions
  #let card-width = 2.5in
  #let card-height = 3.5in
  #let show-cut-lines = true

  // Level string helper
  #let level-str(lvl) = {
    if lvl == 0 { "Cantrip" }
    else if lvl == 1 { "1st-level" }
    else if lvl == 2 { "2nd-level" }
    else if lvl == 3 { "3rd-level" }
    else { str(lvl) + "th-level" }
  }

  // Single-letter code to school name mapping
  #let school-names = (
    a: "abjuration",
    c: "conjuration",
    d: "divination",
    e: "enchantment",
    v: "evocation",
    i: "illusion",
    n: "necromancy",
    t: "transmutation",
  )

  // Get full school name from code or name
  #let get-school-name(school) = {
    let s = str(school)
    if s.len() == 1 {
      school-names.at(lower(s), default: s)
    } else {
      lower(s)
    }
  }

  // Single spell card component
  #let spell-card(spell) = {
    let spell-name = get(spell, "name", default: "Unknown")
    let spell-level = get(spell, "level", default: 0)
    let school = get(spell, "school", default: "Unknown")
    let source = get(spell, "source", default: "")

    // Casting time
    let casting-time = if "casting_time" in spell { spell.casting_time }
      else if "time" in spell and type(spell.time) == array and spell.time.len() > 0 {
        let t = spell.time.at(0)
        str(get(t, "number", default: 1)) + " " + get(t, "unit", default: "action")
      } else { "1 action" }

    // Range
    let spell-range = if "range" in spell and type(spell.range) == str { spell.range }
      else if "range" in spell and type(spell.range) == dictionary {
        if "distance" in spell.range {
          let dist = spell.range.distance
          let amount = get(dist, "amount", default: none)
          if amount != none { str(amount) + " " + get(dist, "distance_type", default: "ft") }
          else { get(dist, "distance_type", default: "Self") }
        } else { get(spell.range, "type", default: "Self") }
      } else { "Self" }

    // Components
    let components-str = if "components" in spell and type(spell.components) == str {
      spell.components
    } else if "components" in spell and type(spell.components) == dictionary {
      let parts = ()
      let v-val = get(spell.components, "v", default: false)
      let s-val = get(spell.components, "s", default: false)
      if v-val == true { parts.push("V") }
      if s-val == true { parts.push("S") }
      if get(spell.components, "m", default: none) != none { parts.push("M") }
      parts.join(", ")
    } else { "V, S" }

    // Duration
    let duration-str = if "duration" in spell and type(spell.duration) == str { spell.duration }
      else if "duration" in spell and type(spell.duration) == array and spell.duration.len() > 0 {
        let d = spell.duration.at(0)
        let dur-type = get(d, "duration_type", default: get(d, "type", default: "instant"))
        if dur-type == "instant" { "Instantaneous" } else { dur-type }
      } else { "Instantaneous" }

    // Flags - ensure boolean values (handle none)
    let conc-val = get(spell, "concentration", default: false)
    let is-concentration = if conc-val == none { false } else { conc-val }

    let ritual-val = get(spell, "ritual", default: false)
    let is-ritual = if ritual-val == none { false } else { ritual-val }

    // Check for meta field for ritual
    let meta = get(spell, "meta", default: none)
    let is-ritual = if meta != none {
      let meta-ritual = get(meta, "ritual", default: false)
      if meta-ritual == none { is-ritual } else { meta-ritual }
    } else { is-ritual }

    // Description
    let description = if "description" in spell { spell.description }
      else if "entries" in spell and type(spell.entries) == array and spell.entries.len() > 0 {
        spell.entries.filter((e) => type(e) == str).join(" ")
      } else { "" }

    // Classes
    let spell-classes = if "classes" in spell and spell.classes != none {
      let from-list = get(spell.classes, "from_class_list", default: none)
      if from-list != none and type(from-list) == array {
        from-list.slice(0, calc.min(3, from-list.len())).map((c) => get(c, "name", default: "?")).join(", ")
      } else { none }
    } else { none }

    // Return the card content
    box(
      width: card-width,
      height: card-height,
      stroke: 0.5pt + colors.border,
      radius: 3pt,
      clip: true,
      inset: 0pt,
    )[
      // Header
      #block(
        width: 100%,
        fill: colors.background-alt,
        inset: (x: 5pt, y: 4pt),
      )[
        #grid(
          columns: (auto, 1fr),
          column-gutter: 4pt,
          spell-school-icon(school, size: sizes.sm),
          [
            #text(size: 9pt, weight: "bold")[#spell-name]
            #if is-ritual [ #text(size: 6pt)[(R)]]
          ]
        )
        #text(size: 6.5pt, fill: colors.text-secondary)[
          #level-str(spell-level) #get-school-name(school)
          #if is-concentration [ (C)]
        ]
      ]

      // Stats
      #block(
        width: 100%,
        inset: 5pt,
        stroke: (bottom: 0.5pt + colors.border-light),
      )[
        #set text(size: 7pt)
        #grid(
          columns: (auto, 1fr),
          row-gutter: 2pt,
          [*Cast:*], [#casting-time],
          [*Range:*], [#spell-range],
          [*Comp:*], [#components-str],
          [*Dur:*], [#duration-str],
        )
      ]

      // Description
      #block(
        width: 100%,
        inset: 5pt,
      )[
        #set align(left)
        #set par(justify: false)
        #text(size: 6.5pt)[
          #description.codepoints().slice(0, calc.min(400, description.codepoints().len())).join("")#if description.codepoints().len() > 400 [...]
        ]
      ]

      // Footer
      #place(
        bottom + left,
        block(
          width: 100%,
          fill: colors.background-alt,
          inset: (x: 5pt, y: 3pt),
        )[
          #text(size: 5pt, fill: colors.text-secondary)[
            #if spell-classes != none [#spell-classes]
            #h(1fr)
            #source
          ]
        ]
      )
    ]
  }

  // Calculate pages needed
  #let cards-per-page = 9
  #let total-pages = if spell-list.len() > 0 { calc.ceil(spell-list.len() / cards-per-page) } else { 0 }

  #for page-num in range(total-pages) [
    #pagebreak()

    // Reset page margins for spell cards
    #set page(margin: 0.25in)

    #let start-idx = page-num * cards-per-page
    #let end-idx = calc.min(start-idx + cards-per-page, spell-list.len())
    #let page-spells = spell-list.slice(start-idx, end-idx)

    // Page header
    #align(center)[
      #text(size: sizes.sm, weight: "bold")[#char-name's Spell Cards #if total-pages > 1 [(Page #(page-num + 1) of #total-pages)]]
    ]
    #v(spacing.sm)

    // Center the card grid
    #align(center)[
      #grid(
        columns: (card-width,) * 3,
        rows: (card-height,) * 3,
        column-gutter: if show-cut-lines { 0pt } else { 4pt },
        row-gutter: if show-cut-lines { 0pt } else { 4pt },

        ..for i in range(9) {
          if i < page-spells.len() {
            (spell-card(page-spells.at(i)),)
          } else {
            (box(width: card-width, height: card-height),)
          }
        }
      )
    ]

    // Cut lines indicator
    #if show-cut-lines and page-spells.len() > 0 [
      #place(
        bottom + center,
        dy: 0.1in,
        text(size: 6pt, fill: colors.text-secondary)[Cut along card borders]
      )
    ]
  ]
]
