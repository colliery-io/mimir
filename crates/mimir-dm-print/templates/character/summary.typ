// Mimir Character Summary Template
// Half-page quick reference card

#import "/_shared/styles.typ": *
#import "/_shared/components.typ": *
#import "/_shared/icons.typ": *

// Document setup - half letter page
#set page(
  width: 8.5in,
  height: 5.5in,
  margin: 0.3in,
)
#set text(font: font-body, size: sizes.sm)

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

#let char-name = get(data, "character_name", default: "Unknown")
#let char-level = get(data, "level", default: 1)
#let race = get(data, "race", default: "Unknown")
#let classes = get(data, "classes", default: ())
#let abilities = get(data, "abilities", default: ())
#let max-hp = get(data, "max_hp", default: 10)
#let current-hp = get(data, "current_hp", default: 10)
#let speed = get(data, "speed", default: 30)
#let proficiencies = get(data, "proficiencies", default: ())
#let equipped = get(data, "equipped", default: ())
#let char-spells = get(data, "spells", default: ())

#let str-score = get(abilities, "strength", default: 10)
#let dex-score = get(abilities, "dexterity", default: 10)
#let con-score = get(abilities, "constitution", default: 10)
#let int-score = get(abilities, "intelligence", default: 10)
#let wis-score = get(abilities, "wisdom", default: 10)
#let cha-score = get(abilities, "charisma", default: 10)

// Class string
#let class-str = if classes.len() > 0 {
  classes.map(c => {
    let name = get(c, "class_name", default: "?")
    let lvl = get(c, "level", default: 0)
    str(name) + " " + str(lvl)
  }).join(" / ")
} else { "No Class" }

// AC calculation
#let has-shield = get(equipped, "shield", default: none) != none
#let ac = 10 + calc.floor((dex-score - 10) / 2) + if has-shield { 2 } else { 0 }

// Proficiency bonus
#let prof = prof-bonus(char-level)

// =============================================================================
// LAYOUT
// =============================================================================

// Header
#grid(
  columns: (1fr, auto),
  [
    #text(size: sizes.xl, weight: "bold")[#char-name]
    #linebreak()
    #text(size: sizes.md)[Level #char-level #race #class-str]
  ],
  grid(
    columns: (auto, auto, auto, auto),
    column-gutter: spacing.md,
    box(
      stroke: 1pt + colors.border,
      inset: spacing.sm,
      radius: 2pt,
      align(center)[
        #label-text("HP")
        #linebreak()
        #text(size: sizes.lg, weight: "bold")[#current-hp/#max-hp]
      ]
    ),
    box(
      stroke: 1pt + colors.border,
      inset: spacing.sm,
      radius: 2pt,
      align(center)[
        #label-text("AC")
        #linebreak()
        #text(size: sizes.lg, weight: "bold")[#ac]
      ]
    ),
    box(
      stroke: 1pt + colors.border,
      inset: spacing.sm,
      radius: 2pt,
      align(center)[
        #label-text("Speed")
        #linebreak()
        #text(size: sizes.lg, weight: "bold")[#speed]
      ]
    ),
    box(
      stroke: 1pt + colors.border,
      inset: spacing.sm,
      radius: 2pt,
      align(center)[
        #label-text("Prof")
        #linebreak()
        #text(size: sizes.lg, weight: "bold")[+#prof]
      ]
    ),
  )
)

#divider-heavy()

// Abilities list
#let ability-list = (
  ("STR", str-score),
  ("DEX", dex-score),
  ("CON", con-score),
  ("INT", int-score),
  ("WIS", wis-score),
  ("CHA", cha-score),
)

// Saves list
#let prof-saves = get(proficiencies, "saves", default: ())
#let save-list = (
  ("STR", str-score, prof-saves.contains("Strength")),
  ("DEX", dex-score, prof-saves.contains("Dexterity")),
  ("CON", con-score, prof-saves.contains("Constitution")),
  ("INT", int-score, prof-saves.contains("Intelligence")),
  ("WIS", wis-score, prof-saves.contains("Wisdom")),
  ("CHA", cha-score, prof-saves.contains("Charisma")),
)

// Skills and equipment
#let prof-skills = get(proficiencies, "skills", default: ())
#let armor = get(equipped, "armor", default: none)
#let shield = get(equipped, "shield", default: none)
#let main-hand = get(equipped, "main_hand", default: none)

// Spells
#let cantrips = get(char-spells, "cantrips", default: ())
#let prepared = get(char-spells, "prepared_spells", default: ())
#let known = get(char-spells, "known_spells", default: ())
#let slots = get(char-spells, "spell_slots", default: (:))
#let has-spells = cantrips.len() > 0 or prepared.len() > 0 or known.len() > 0

// Features
#let features = get(data, "class_features", default: ())

// Main content
#grid(
  columns: (1fr, 1fr, 1fr),
  column-gutter: spacing.md,

  // Column 1: Abilities
  [
    #text(weight: "bold")[Abilities]
    #v(spacing.xs)
    #for (name, score) in ability-list [
      #name #h(1fr) #score (#fmt-mod(score))
      #linebreak()
    ]

    #v(spacing.md)

    #text(weight: "bold")[Saves]
    #v(spacing.xs)
    #for (name, score, is-prof) in save-list [
      #let mod = calc.floor((score - 10) / 2)
      #let total = mod + if is-prof { prof } else { 0 }
      #let sign = if total >= 0 { "+" } else { "" }
      #if is-prof [#text(weight: "bold")[#name]] else [#name] #h(1fr) #sign#total
      #linebreak()
    ]
  ],

  // Column 2: Skills & Equipment
  [
    #text(weight: "bold")[Skills]
    #v(spacing.xs)
    #if prof-skills.len() > 0 [
      #for skill in prof-skills [
        - #skill
        #linebreak()
      ]
    ] else [
      #text(fill: colors.text-secondary)[None]
    ]

    #v(spacing.md)

    #text(weight: "bold")[Equipment]
    #v(spacing.xs)
    #if armor != none [Armor: #armor #linebreak()]
    #if shield != none [Shield: #shield #linebreak()]
    #if main-hand != none [Weapon: #main-hand #linebreak()]
  ],

  // Column 3: Spells or Features
  [
    #if has-spells [
      #text(weight: "bold")[Spellcasting]
      #v(spacing.xs)

      #if slots.keys().len() > 0 [
        #for lvl in slots.keys().sorted() [
          #let slot = slots.at(lvl)
          L#lvl: #get(slot, "current", default: 0)/#get(slot, "max", default: 0)
        ]
        #v(spacing.xs)
      ]

      #if cantrips.len() > 0 [
        Cantrips: #cantrips.slice(0, calc.min(4, cantrips.len())).join(", ")
        #if cantrips.len() > 4 [...]
        #linebreak()
      ]

      #let spell-list = if prepared.len() > 0 { prepared } else { known }
      #if spell-list.len() > 0 [
        Spells: #spell-list.slice(0, calc.min(6, spell-list.len())).join(", ")
        #if spell-list.len() > 6 [...]
      ]
    ] else [
      #text(weight: "bold")[Features]
      #v(spacing.xs)
      #if features.len() > 0 [
        #for feature in features.slice(0, calc.min(8, features.len())) [
          - #feature
          #linebreak()
        ]
        #if features.len() > 8 [
          #text(fill: colors.text-secondary)[+ #(features.len() - 8) more]
        ]
      ] else [
        #text(fill: colors.text-secondary)[None]
      ]
    ]
  ]
)

// Footer
#v(1fr)
#align(center)[
  #small-text[#char-name - Level #char-level #class-str - Generated by Mimir]
]
