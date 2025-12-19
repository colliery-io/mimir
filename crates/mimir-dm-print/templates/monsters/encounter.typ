// Mimir Encounter Template
// Multiple monsters for encounter planning

#import "/_shared/styles.typ": *
#import "/_shared/components.typ": *
#import "/_shared/icons.typ": *

// Document setup
#show: mimir-doc.with(margin: 0.4in)

// Helper to safely get nested data
#let get(obj, key, default: none) = {
  if obj != none and key in obj { obj.at(key) } else { default }
}

// Helper to format ability modifier
#let fmt-mod(score) = {
  let mod = calc.floor((score - 10) / 2)
  if mod >= 0 { "+" + str(mod) } else { str(mod) }
}

// =============================================================================
// DATA EXTRACTION
// =============================================================================

#let encounter-title = get(data, "title", default: "Encounter")
#let monsters = get(data, "monsters", default: ())
#let notes = get(data, "notes", default: none)

// =============================================================================
// COMPACT MONSTER STAT BLOCK
// =============================================================================

#let compact-stat-block(monster) = {
  let monster-name = get(monster, "name", default: "Unknown")
  let source = get(monster, "source", default: "")

  // Size
  let size-data = get(monster, "size", default: ("M",))
  let monster-size = if type(size-data) == array and size-data.len() > 0 {
    let s = size-data.at(0)
    if s == "T" { "Tiny" }
    else if s == "S" { "Small" }
    else if s == "M" { "Medium" }
    else if s == "L" { "Large" }
    else if s == "H" { "Huge" }
    else if s == "G" { "Gargantuan" }
    else { s }
  } else { "Medium" }

  // Type
  let type-data = get(monster, "creature_type", default: "humanoid")
  let monster-type = if type(type-data) == str { type-data }
    else if type(type-data) == dictionary { get(type-data, "type", default: "humanoid") }
    else { "humanoid" }

  // Alignment
  let align-data = get(monster, "alignment", default: none)
  let monster-alignment = if align-data == none { "unaligned" }
    else if type(align-data) == array and align-data.len() > 0 {
      let first = align-data.at(0)
      if type(first) == str {
        if first == "L" { "lawful" }
        else if first == "N" or first == "NX" or first == "NY" { "neutral" }
        else if first == "C" { "chaotic" }
        else if first == "G" { "good" }
        else if first == "E" { "evil" }
        else if first == "U" { "unaligned" }
        else { first }
      } else { "unaligned" }
    }
    else if type(align-data) == str { align-data }
    else { "unaligned" }

  // AC
  let ac-data = get(monster, "ac", default: 10)
  let monster-ac = if type(ac-data) == int { str(ac-data) }
    else if type(ac-data) == array and ac-data.len() > 0 {
      let first = ac-data.at(0)
      if type(first) == int { str(first) }
      else if type(first) == dictionary {
        let ac-val = get(first, "ac", default: 10)
        let ac-from = get(first, "from", default: ())
        if ac-from.len() > 0 { str(ac-val) + " (" + ac-from.join(", ") + ")" }
        else { str(ac-val) }
      } else { "10" }
    } else { "10" }

  // HP
  let hp-data = get(monster, "hp", default: none)
  let monster-hp = if hp-data == none { "1" }
    else if type(hp-data) == int { str(hp-data) }
    else if type(hp-data) == dictionary {
      let avg = get(hp-data, "average", default: 1)
      let formula = get(hp-data, "formula", default: none)
      if formula != none { str(avg) + " (" + formula + ")" }
      else { str(avg) }
    } else { "1" }

  // Speed (simplified)
  let speed-data = get(monster, "speed", default: none)
  let monster-speed = if speed-data == none { "30 ft." }
    else {
      let walk = get(speed-data, "walk", default: none)
      if walk == none { "30 ft." }
      else if type(walk) == int { str(walk) + " ft." }
      else { "30 ft." }
    }

  // Ability scores
  let str-score = get(monster, "str", default: 10)
  let dex-score = get(monster, "dex", default: 10)
  let con-score = get(monster, "con", default: 10)
  let int-score = get(monster, "int", default: 10)
  let wis-score = get(monster, "wis", default: 10)
  let cha-score = get(monster, "cha", default: 10)

  // CR
  let cr-data = get(monster, "cr", default: "0")
  let monster-cr = if type(cr-data) == str { cr-data }
    else if type(cr-data) == dictionary { get(cr-data, "cr", default: "0") }
    else if type(cr-data) == int { str(cr-data) }
    else { "0" }

  // Passive perception
  let passive = get(monster, "passive", default: 10)

  // Key actions
  let action-data = get(monster, "action", default: ())
  let key-actions = if action-data.len() > 0 {
    action-data.slice(0, calc.min(3, action-data.len())).map(a => {
      if type(a) == dictionary { get(a, "name", default: "Attack") }
      else { "Attack" }
    })
  } else { () }

  // Saves (simplified)
  let save-data = get(monster, "save", default: none)
  let monster-saves = if save-data == none { none }
    else {
      let parts = ()
      for (key, label) in (("str", "Str"), ("dex", "Dex"), ("con", "Con"), ("int", "Int"), ("wis", "Wis"), ("cha", "Cha")) {
        let val = get(save-data, key, default: none)
        if val != none { parts.push(label + " " + val) }
      }
      if parts.len() > 0 { parts.join(", ") } else { none }
    }

  // Render compact stat block
  box(
    width: 100%,
    stroke: (
      top: 2pt + colors.accent,
      bottom: 2pt + colors.accent,
    ),
    inset: 0pt,
  )[
    // Header
    #block(
      width: 100%,
      fill: colors.background-alt,
      inset: spacing.sm,
    )[
      #grid(
        columns: (1fr, auto),
        [
          #text(size: sizes.md, weight: "bold")[#monster-name]
          #linebreak()
          #text(size: sizes.xs, style: "italic")[#monster-size #monster-type, #monster-alignment]
        ],
        align(right + horizon)[
          #cr-indicator(monster-cr)
        ]
      )
    ]

    // Stats row
    #block(
      width: 100%,
      inset: spacing.sm,
      stroke: (bottom: 0.5pt + colors.border-light),
    )[
      #set text(size: sizes.xs)
      #grid(
        columns: (auto, auto, auto, auto, 1fr),
        column-gutter: spacing.md,
        [*AC* #monster-ac],
        [*HP* #monster-hp],
        [*Speed* #monster-speed],
        [*PP* #passive],
        if monster-saves != none [*Saves* #monster-saves] else [],
      )
    ]

    // Abilities
    #block(
      width: 100%,
      inset: spacing.sm,
      stroke: (bottom: 0.5pt + colors.border-light),
    )[
      #set text(size: sizes.xs)
      #grid(
        columns: (1fr,) * 6,
        align(center)[*STR* #str-score (#fmt-mod(str-score))],
        align(center)[*DEX* #dex-score (#fmt-mod(dex-score))],
        align(center)[*CON* #con-score (#fmt-mod(con-score))],
        align(center)[*INT* #int-score (#fmt-mod(int-score))],
        align(center)[*WIS* #wis-score (#fmt-mod(wis-score))],
        align(center)[*CHA* #cha-score (#fmt-mod(cha-score))],
      )
    ]

    // Key actions
    #if key-actions.len() > 0 [
      #block(
        width: 100%,
        inset: spacing.sm,
      )[
        #set text(size: sizes.xs)
        *Key Actions:* #key-actions.join(", ")
      ]
    ]
  ]
}

// =============================================================================
// ENCOUNTER LAYOUT
// =============================================================================

// Header
#align(center)[
  #title-text(encounter-title)
]

#v(spacing.sm)

// Encounter summary
#if monsters.len() > 0 [
  #block(
    width: 100%,
    fill: colors.background-alt,
    inset: spacing.md,
    radius: 2pt,
  )[
    #text(weight: "bold")[Encounter Summary]
    #h(1fr)
    #text(size: sizes.sm)[#monsters.len() creature#if monsters.len() != 1 [s]]
  ]
]

#v(spacing.md)

// Monster stat blocks
#for (i, monster) in monsters.enumerate() [
  #compact-stat-block(monster)
  #if i < monsters.len() - 1 [
    #v(spacing.md)
  ]
]

// Notes section
#if notes != none [
  #v(spacing.lg)
  #block(
    width: 100%,
    stroke: 1pt + colors.border,
    inset: spacing.md,
    radius: 2pt,
  )[
    #text(weight: "bold")[DM Notes]
    #v(spacing.sm)
    #notes
  ]
]

// Empty state
#if monsters.len() == 0 [
  #align(center + horizon)[
    #text(fill: colors.text-secondary)[No monsters in this encounter]
  ]
]

// Footer
#v(1fr)
#align(center)[
  #small-text[#encounter-title - Generated by Mimir]
]
