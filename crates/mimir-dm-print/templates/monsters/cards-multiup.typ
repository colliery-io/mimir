// Mimir Monster Cards Multi-up Template
// 9 cards per US Letter page (3x3 grid)

#import "/_shared/styles.typ": *
#import "/_shared/components.typ": *
#import "/_shared/icons.typ": *

// Page setup for multi-up cards
#set page(
  paper: "us-letter",
  margin: 0.25in,
)

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

#let monsters = get(data, "monsters", default: ())
#let show-cut-lines = get(data, "show_cut_lines", default: true)

// Card dimensions
#let card-width = 2.5in
#let card-height = 3.5in

// =============================================================================
// SINGLE CARD COMPONENT
// =============================================================================

#let monster-card(monster) = {
  let monster-name = get(monster, "name", default: "Unknown")
  let source = get(monster, "source", default: "")

  // Size
  let size-data = get(monster, "size", default: ("M",))
  let monster-size = if type(size-data) == array and size-data.len() > 0 {
    size-data.at(0)
  } else { "M" }

  // Type
  let type-data = get(monster, "creature_type", default: "humanoid")
  let monster-type = if type(type-data) == str { type-data }
    else if type(type-data) == dictionary { get(type-data, "type", default: "humanoid") }
    else { "humanoid" }

  // AC
  let ac-data = get(monster, "ac", default: 10)
  let monster-ac = if type(ac-data) == int { ac-data }
    else if type(ac-data) == array and ac-data.len() > 0 {
      let first = ac-data.at(0)
      if type(first) == int { first }
      else if type(first) == dictionary { get(first, "ac", default: 10) }
      else { 10 }
    } else { 10 }

  // HP
  let hp-data = get(monster, "hp", default: none)
  let monster-hp = if hp-data == none { 1 }
    else if type(hp-data) == int { hp-data }
    else if type(hp-data) == dictionary { get(hp-data, "average", default: 1) }
    else { 1 }

  // Speed
  let speed-data = get(monster, "speed", default: none)
  let monster-speed = if speed-data == none { 30 }
    else {
      let walk = get(speed-data, "walk", default: none)
      if walk == none { 30 }
      else if type(walk) == int { walk }
      else { 30 }
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
      inset: (x: 4pt, y: 3pt),
    )[
      #grid(
        columns: (1fr, auto),
        [
          #text(size: 7pt, weight: "bold")[#monster-name]
          #linebreak()
          #text(size: 5pt, fill: colors.text-secondary)[#monster-size #monster-type]
        ],
        align(right)[
          #text(size: 6pt, weight: "bold")[CR #monster-cr]
        ]
      )
    ]

    // Combat stats
    #block(
      width: 100%,
      inset: 4pt,
      stroke: (bottom: 0.5pt + colors.border-light),
    )[
      #set text(size: 6pt)
      #grid(
        columns: (1fr, 1fr, 1fr, 1fr),
        align(center)[
          #text(size: 5pt, fill: colors.text-secondary)[AC]
          #linebreak()
          #text(weight: "bold")[#monster-ac]
        ],
        align(center)[
          #text(size: 5pt, fill: colors.text-secondary)[HP]
          #linebreak()
          #text(weight: "bold")[#monster-hp]
        ],
        align(center)[
          #text(size: 5pt, fill: colors.text-secondary)[SPD]
          #linebreak()
          #text(weight: "bold")[#monster-speed]
        ],
        align(center)[
          #text(size: 5pt, fill: colors.text-secondary)[PP]
          #linebreak()
          #text(weight: "bold")[#passive]
        ],
      )
    ]

    // Abilities
    #block(
      width: 100%,
      inset: (x: 4pt, y: 2pt),
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

    // Actions
    #if key-actions.len() > 0 [
      #block(
        width: 100%,
        inset: 4pt,
      )[
        #text(size: 5pt)[
          #text(weight: "bold")[Actions: ]
          #key-actions.join(", ")
        ]
      ]
    ]

    // Footer
    #place(
      bottom + left,
      block(
        width: 100%,
        fill: colors.background-alt,
        inset: (x: 4pt, y: 2pt),
      )[
        #text(size: 4pt, fill: colors.text-secondary)[
          #source
          #h(1fr)
          Mimir
        ]
      ]
    )
  ]
}

// =============================================================================
// MULTI-UP LAYOUT
// =============================================================================

// Calculate pages needed
#let cards-per-page = 9
#let total-pages = if monsters.len() > 0 { calc.ceil(monsters.len() / cards-per-page) } else { 0 }

#for page-num in range(total-pages) [
  #let start-idx = page-num * cards-per-page
  #let end-idx = calc.min(start-idx + cards-per-page, monsters.len())
  #let page-monsters = monsters.slice(start-idx, end-idx)

  // Add page break after first page
  #if page-num > 0 [
    #pagebreak()
  ]

  // Center the card grid
  #align(center)[
    #grid(
      columns: (card-width,) * 3,
      rows: (card-height,) * 3,
      column-gutter: if show-cut-lines { 0pt } else { 4pt },
      row-gutter: if show-cut-lines { 0pt } else { 4pt },

      ..for i in range(9) {
        if i < page-monsters.len() {
          (monster-card(page-monsters.at(i)),)
        } else {
          (box(width: card-width, height: card-height),)
        }
      }
    )
  ]

  // Cut lines indicator
  #if show-cut-lines and page-monsters.len() > 0 [
    #place(
      bottom + center,
      dy: 0.1in,
      text(size: 6pt, fill: colors.text-secondary)[Cut along card borders]
    )
  ]
]

// Empty state
#if monsters.len() == 0 [
  #align(center + horizon)[
    #text(fill: colors.text-secondary)[No monsters to display]
  ]
]
