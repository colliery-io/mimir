// Mimir Monster Card Template
// Condensed monster reference card (poker card size 2.5" x 3.5")

#import "/_shared/styles.typ": *
#import "/_shared/components.typ": *
#import "/_shared/icons.typ": *

// Card document setup
#show: mimir-card-doc

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
// DATA EXTRACTION (simplified for card format)
// =============================================================================

#let monster-name = get(data, "name", default: "Unknown")
#let source = get(data, "source", default: "")

// Size
#let size-data = get(data, "size", default: ("M",))
#let monster-size = if type(size-data) == array and size-data.len() > 0 {
  size-data.at(0)
} else if type(size-data) == str {
  size-data.slice(0, 1)
} else { "M" }

// Type
#let type-data = get(data, "creature_type", default: "humanoid")
#let monster-type = if type(type-data) == str { type-data }
  else if type(type-data) == dictionary { get(type-data, "type", default: "humanoid") }
  else { "humanoid" }

// AC
#let ac-data = get(data, "ac", default: 10)
#let monster-ac = if type(ac-data) == int { ac-data }
  else if type(ac-data) == array and ac-data.len() > 0 {
    let first = ac-data.at(0)
    if type(first) == int { first }
    else if type(first) == dictionary { get(first, "ac", default: 10) }
    else { 10 }
  } else { 10 }

// HP
#let hp-data = get(data, "hp", default: none)
#let monster-hp = if hp-data == none { 1 }
  else if type(hp-data) == int { hp-data }
  else if type(hp-data) == dictionary { get(hp-data, "average", default: 1) }
  else { 1 }

// Speed (simplified)
#let speed-data = get(data, "speed", default: none)
#let monster-speed = if speed-data == none { 30 }
  else {
    let walk = get(speed-data, "walk", default: none)
    if walk == none { 30 }
    else if type(walk) == int { walk }
    else if type(walk) == dictionary { get(walk, "number", default: 30) }
    else { 30 }
  }

// Ability scores
#let str-score = get(data, "str", default: 10)
#let dex-score = get(data, "dex", default: 10)
#let con-score = get(data, "con", default: 10)
#let int-score = get(data, "int", default: 10)
#let wis-score = get(data, "wis", default: 10)
#let cha-score = get(data, "cha", default: 10)

// CR
#let cr-data = get(data, "cr", default: "0")
#let monster-cr = if type(cr-data) == str { cr-data }
  else if type(cr-data) == dictionary { get(cr-data, "cr", default: "0") }
  else if type(cr-data) == int { str(cr-data) }
  else { "0" }

// Passive perception
#let passive = get(data, "passive", default: 10)

// Key actions (first 2)
#let action-data = get(data, "action", default: ())
#let key-actions = if action-data.len() > 0 {
  action-data.slice(0, calc.min(2, action-data.len())).map(a => {
    if type(a) == dictionary { get(a, "name", default: "Attack") }
    else { "Attack" }
  })
} else { () }

// =============================================================================
// CARD LAYOUT
// =============================================================================

#box(
  width: 100%,
  height: 100%,
  stroke: 1pt + colors.border,
  radius: 4pt,
  clip: true,
)[
  // Header with name and CR
  #block(
    width: 100%,
    fill: colors.background-alt,
    inset: (x: spacing.sm, y: spacing.xs),
  )[
    #grid(
      columns: (1fr, auto),
      [
        #text(size: sizes.sm, weight: "bold")[#monster-name]
        #linebreak()
        #text(size: 5pt, fill: colors.text-secondary)[#monster-size #monster-type]
      ],
      align(right)[
        #cr-indicator(monster-cr)
      ]
    )
  ]

  // Combat stats row
  #block(
    width: 100%,
    inset: spacing.xs,
    stroke: (bottom: 0.5pt + colors.border-light),
  )[
    #set text(size: 6pt)
    #grid(
      columns: (1fr, 1fr, 1fr, 1fr),
      align(center)[
        #label-text("AC")
        #linebreak()
        #text(weight: "bold")[#monster-ac]
      ],
      align(center)[
        #label-text("HP")
        #linebreak()
        #text(weight: "bold")[#monster-hp]
      ],
      align(center)[
        #label-text("SPD")
        #linebreak()
        #text(weight: "bold")[#monster-speed]
      ],
      align(center)[
        #label-text("PP")
        #linebreak()
        #text(weight: "bold")[#passive]
      ],
    )
  ]

  // Ability scores (compact)
  #block(
    width: 100%,
    inset: (x: spacing.xs, y: 2pt),
    stroke: (bottom: 0.5pt + colors.border-light),
  )[
    #set text(size: 5pt)
    #grid(
      columns: (1fr,) * 6,
      align(center)[
        #text(weight: "bold")[STR]
        #linebreak()
        #fmt-mod(str-score)
      ],
      align(center)[
        #text(weight: "bold")[DEX]
        #linebreak()
        #fmt-mod(dex-score)
      ],
      align(center)[
        #text(weight: "bold")[CON]
        #linebreak()
        #fmt-mod(con-score)
      ],
      align(center)[
        #text(weight: "bold")[INT]
        #linebreak()
        #fmt-mod(int-score)
      ],
      align(center)[
        #text(weight: "bold")[WIS]
        #linebreak()
        #fmt-mod(wis-score)
      ],
      align(center)[
        #text(weight: "bold")[CHA]
        #linebreak()
        #fmt-mod(cha-score)
      ],
    )
  ]

  // Key actions
  #if key-actions.len() > 0 [
    #block(
      width: 100%,
      inset: spacing.xs,
    )[
      #set text(size: 5pt)
      #text(weight: "bold")[Actions: ]
      #key-actions.join(", ")
    ]
  ]

  // Footer
  #place(
    bottom + left,
    block(
      width: 100%,
      fill: colors.background-alt,
      inset: (x: spacing.xs, y: 2pt),
    )[
      #text(size: 4pt, fill: colors.text-secondary)[
        #source
        #h(1fr)
        Mimir
      ]
    ]
  )
]
