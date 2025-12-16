// Mimir Character Sheet Template
// Full character sheet for US Letter paper

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
// HEADER
// =============================================================================

#let char-name = get(data, "character_name", default: "Unknown Character")
#let level = get(data, "level", default: 1)
#let race = get(data, "race", default: "Unknown")
#let subrace = get(data, "subrace", default: none)
#let background = get(data, "background", default: "Unknown")
#let alignment = get(data, "alignment", default: none)
#let classes = get(data, "classes", default: ())

// Build class string
#let class-str = if classes.len() > 0 {
  classes.map(c => {
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

#let abilities = get(data, "abilities", default: (
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

#let max-hp = get(data, "max_hp", default: 10)
#let current-hp = get(data, "current_hp", default: 10)
#let speed = get(data, "speed", default: 30)

#let proficiencies = get(data, "proficiencies", default: ())
#let prof-saves = get(proficiencies, "saves", default: ())

// Calculate AC (basic formula: 10 + dex mod, or from equipped armor)
#let equipped = get(data, "equipped", default: ())
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
      #let currency = get(data, "currency", default: ())
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
    let inventory = get(data, "inventory", default: ())
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
// FEATURES AND SPELLS
// =============================================================================

#grid(
  columns: (1fr, 1fr),
  column-gutter: spacing.md,

  // Features & Traits
  {
    let class-features = get(data, "class_features", default: ())
    let feats = get(data, "feats", default: ())

    info-box(title: "Features & Traits")[
      #for feature in class-features {
        text(size: sizes.sm)[- #feature]
        linebreak()
      }
      #if feats.len() > 0 [
        #v(spacing.sm)
        #label-text("Feats")
        #linebreak()
        #for feat in feats {
          text(size: sizes.sm)[- #feat]
          linebreak()
        }
      ]
    ]
  },

  // Spells
  {
    let spells = get(data, "spells", default: ())
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
          #text(size: sizes.sm)[#cantrips.map(s => if type(s) == dictionary { get(s, "name", default: "?") } else { s }).join(", ")]
          #v(spacing.sm)
        ]

        // Prepared/Known spells (extract names from SpellReference objects)
        #let spell-list = if prepared.len() > 0 { prepared } else { known }
        #if spell-list.len() > 0 [
          #label-text(if prepared.len() > 0 { "Prepared Spells" } else { "Known Spells" })
          #linebreak()
          #text(size: sizes.sm)[#spell-list.map(s => if type(s) == dictionary { get(s, "name", default: "?") } else { s }).join(", ")]
        ]
      ]
    ] else [
      // Personality for non-spellcasters
      #let personality = get(data, "personality", default: ())
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
// FOOTER
// =============================================================================

#v(1fr)
#align(center)[
  #small-text[Generated by Mimir]
]
