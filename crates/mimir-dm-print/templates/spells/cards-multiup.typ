// Mimir Spell Cards Multi-up Template
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

// =============================================================================
// DATA EXTRACTION
// =============================================================================

#let spells = get(data, "spells", default: ())
#let show-cut-lines = get(data, "show_cut_lines", default: true)

// Card dimensions
#let card-width = 2.5in
#let card-height = 3.5in

// Level string helper
#let level-str(level) = {
  if level == 0 { "Cantrip" }
  else if level == 1 { "1st-level" }
  else if level == 2 { "2nd-level" }
  else if level == 3 { "3rd-level" }
  else { str(level) + "th-level" }
}

// =============================================================================
// SINGLE CARD COMPONENT
// =============================================================================

#let spell-card(spell) = {
  let spell-name = get(spell, "name", default: "Unknown")
  let spell-level = get(spell, "level", default: 0)
  let school = get(spell, "school", default: "Unknown")
  let source = get(spell, "source", default: "")

  // Casting time
  let casting-time = if "casting_time" in spell { spell.casting_time }
    else if "time" in spell and spell.time.len() > 0 {
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
    if get(spell.components, "v", default: false) { parts.push("V") }
    if get(spell.components, "s", default: false) { parts.push("S") }
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

  // Flags
  let is-concentration = get(spell, "concentration", default: false)
  let is-ritual = get(spell, "ritual", default: false)

  // Description
  let description = if "description" in spell { spell.description }
    else if "entries" in spell and spell.entries.len() > 0 {
      spell.entries.filter(e => type(e) == str).join(" ")
    } else { "" }

  // Classes
  let spell-classes = if "classes" in spell and type(spell.classes) == array {
    spell.classes.slice(0, calc.min(3, spell.classes.len())).join(", ")
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
      inset: (x: 4pt, y: 3pt),
    )[
      #grid(
        columns: (auto, 1fr),
        column-gutter: 3pt,
        spell-school-icon(school, size: sizes.xs),
        [
          #text(size: 7pt, weight: "bold")[#spell-name]
          #if is-ritual [ #text(size: 5pt)[(R)]]
        ]
      )
      #text(size: 5pt, fill: colors.text-secondary)[
        #level-str(spell-level) #lower(school)
        #if is-concentration [ (C)]
      ]
    ]

    // Stats
    #block(
      width: 100%,
      inset: 4pt,
      stroke: (bottom: 0.5pt + colors.border-light),
    )[
      #set text(size: 6pt)
      #grid(
        columns: (auto, 1fr),
        row-gutter: 1pt,
        [*Cast:*], [#casting-time],
        [*Range:*], [#spell-range],
        [*Comp:*], [#components-str],
        [*Dur:*], [#duration-str],
      )
    ]

    // Description
    #block(
      width: 100%,
      inset: 4pt,
    )[
      #text(size: 5.5pt)[
        #description.codepoints().slice(0, calc.min(500, description.codepoints().len())).join("")#if description.codepoints().len() > 500 [...]
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
          #if spell-classes != none [#spell-classes]
          #h(1fr)
          #source
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
#let total-pages = if spells.len() > 0 { calc.ceil(spells.len() / cards-per-page) } else { 0 }

#for page-num in range(total-pages) [
  #let start-idx = page-num * cards-per-page
  #let end-idx = calc.min(start-idx + cards-per-page, spells.len())
  #let page-spells = spells.slice(start-idx, end-idx)

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

// Empty state
#if spells.len() == 0 [
  #align(center + horizon)[
    #text(fill: colors.text-secondary)[No spells to display]
  ]
]
