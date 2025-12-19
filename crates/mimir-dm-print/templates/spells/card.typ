// Mimir Spell Card Template
// Individual spell card sized for poker cards (2.5" x 3.5")

#import "/_shared/styles.typ": *
#import "/_shared/components.typ": *
#import "/_shared/icons.typ": *

// Card document setup
#show: mimir-card-doc

// Helper to safely get nested data
#let get(obj, key, default: none) = {
  if obj != none and key in obj { obj.at(key) } else { default }
}

// =============================================================================
// DATA EXTRACTION
// =============================================================================

#let spell-name = get(data, "name", default: "Unknown Spell")
#let spell-level = get(data, "level", default: 0)
#let school = get(data, "school", default: "Unknown")
#let source = get(data, "source", default: "")

// Casting time (handle both summary and full format)
#let casting-time = if "casting_time" in data {
  data.casting_time
} else if "time" in data and data.time.len() > 0 {
  let t = data.time.at(0)
  let num = get(t, "number", default: 1)
  let unit = get(t, "unit", default: "action")
  str(num) + " " + unit
} else {
  "1 action"
}

// Range
#let spell-range = if "range" in data and type(data.range) == str {
  data.range
} else if "range" in data and type(data.range) == dictionary {
  if "distance" in data.range {
    let dist = data.range.distance
    let amount = get(dist, "amount", default: none)
    let dist-type = get(dist, "distance_type", default: get(dist, "type", default: "feet"))
    if amount != none { str(amount) + " " + dist-type } else { dist-type }
  } else {
    get(data.range, "type", default: "Self")
  }
} else {
  "Self"
}

// Components
#let components-str = if "components" in data and type(data.components) == str {
  data.components
} else if "components" in data and type(data.components) == dictionary {
  let parts = ()
  if get(data.components, "v", default: false) { parts.push("V") }
  if get(data.components, "s", default: false) { parts.push("S") }
  let mat = get(data.components, "m", default: none)
  if mat != none { parts.push("M") }
  parts.join(", ")
} else {
  "V, S"
}

// Material component text
#let material-text = if "components" in data and type(data.components) == dictionary {
  let mat = get(data.components, "m", default: none)
  if mat != none and type(mat) == str { mat }
  else if mat != none and type(mat) == dictionary { get(mat, "text", default: none) }
  else { none }
} else { none }

// Duration
#let duration-str = if "duration" in data and type(data.duration) == str {
  data.duration
} else if "duration" in data and type(data.duration) == array and data.duration.len() > 0 {
  let d = data.duration.at(0)
  let dur-type = get(d, "duration_type", default: get(d, "type", default: "instant"))
  let conc = get(d, "concentration", default: false)
  let dur-val = get(d, "duration", default: none)

  if dur-type == "instant" { "Instantaneous" }
  else if dur-type == "permanent" { "Permanent" }
  else if dur-val != none {
    let amount = get(dur-val, "amount", default: 1)
    let val-type = get(dur-val, "value_type", default: get(dur-val, "type", default: "minute"))
    let up-to = get(dur-val, "up_to", default: false)
    let prefix = if conc { "Conc., " } else { "" }
    let up-to-str = if up-to { "up to " } else { "" }
    prefix + up-to-str + str(amount) + " " + val-type + if amount > 1 { "s" } else { "" }
  }
  else if conc { "Concentration" }
  else { dur-type }
} else {
  "Instantaneous"
}

// Check concentration and ritual
#let is-concentration = if "concentration" in data { data.concentration }
  else if "duration" in data and type(data.duration) == array {
    data.duration.any(d => get(d, "concentration", default: false))
  } else { false }

#let is-ritual = if "ritual" in data { data.ritual }
  else if "meta" in data { get(data.meta, "ritual", default: false) }
  else { false }

// Description
#let description = if "description" in data { data.description }
  else if "entries" in data and data.entries.len() > 0 {
    // Join all string entries
    data.entries.filter(e => type(e) == str).join(" ")
  } else { "" }

// Level string
#let level-str = if spell-level == 0 { "Cantrip" }
  else if spell-level == 1 { "1st-level" }
  else if spell-level == 2 { "2nd-level" }
  else if spell-level == 3 { "3rd-level" }
  else { str(spell-level) + "th-level" }

// Classes (if available)
#let spell-classes = if "classes" in data and type(data.classes) == array {
  data.classes.join(", ")
} else { none }

// =============================================================================
// CARD LAYOUT
// =============================================================================

// Card border
#box(
  width: 100%,
  height: 100%,
  stroke: 1pt + colors.border,
  radius: 4pt,
  clip: true,
)[
  // Header
  #block(
    width: 100%,
    fill: colors.background-alt,
    inset: (x: spacing.sm, y: spacing.xs),
  )[
    #grid(
      columns: (auto, 1fr),
      column-gutter: spacing.xs,
      spell-school-icon(school, size: sizes.sm),
      [
        #text(size: sizes.sm, weight: "bold")[#spell-name]
        #if is-ritual [ #text(size: sizes.xs, style: "italic")[(R)]]
      ]
    )
    #text(size: sizes.xs, fill: colors.text-secondary)[
      #level-str #lower(school)
      #if is-concentration [ (concentration)]
    ]
  ]

  // Stats
  #block(
    width: 100%,
    inset: spacing.xs,
    stroke: (bottom: 0.5pt + colors.border-light),
  )[
    #set text(size: sizes.xs)
    #grid(
      columns: (auto, 1fr),
      row-gutter: 1pt,
      [*Cast:*], [#casting-time],
      [*Range:*], [#spell-range],
      [*Comp:*], [#components-str],
      [*Dur:*], [#duration-str],
    )
  ]

  // Material component detail (if present)
  #if material-text != none [
    #block(
      width: 100%,
      inset: (x: spacing.xs, y: 2pt),
      fill: colors.background-alt,
    )[
      #text(size: 5pt, fill: colors.text-secondary)[M: #material-text]
    ]
  ]

  // Description
  #block(
    width: 100%,
    inset: spacing.xs,
    // Limit description to fit card
    text(size: 6pt)[
      #description.codepoints().slice(0, calc.min(600, description.codepoints().len())).join("")#if description.codepoints().len() > 600 [...]
    ]
  )

  // Footer
  #place(
    bottom + left,
    block(
      width: 100%,
      fill: colors.background-alt,
      inset: (x: spacing.xs, y: 2pt),
    )[
      #text(size: 5pt, fill: colors.text-secondary)[
        #if spell-classes != none [#spell-classes]
        #h(1fr)
        #source
      ]
    ]
  )
]
