// Mimir Session Prep Template
// DM reference sheet for running a session

#import "/_shared/styles.typ": *
#import "/_shared/components.typ": *
#import "/_shared/icons.typ": *

// Document setup
#show: mimir-doc.with(margin: 0.4in)

// Helper to safely get nested data
#let get(obj, key, default: none) = {
  if obj != none and key in obj { obj.at(key) } else { default }
}

// =============================================================================
// DATA EXTRACTION
// =============================================================================

#let session-title = get(data, "title", default: "Session Prep")
#let module-name = get(data, "module", default: none)
#let session-number = get(data, "session_number", default: none)
#let date = get(data, "date", default: none)
#let summary = get(data, "summary", default: none)

#let npcs = get(data, "npcs", default: ())
#let locations = get(data, "locations", default: ())
#let encounters = get(data, "encounters", default: ())
#let items = get(data, "items", default: ())
#let notes = get(data, "notes", default: ())
#let secrets = get(data, "secrets", default: ())
#let hooks = get(data, "hooks", default: ())

// =============================================================================
// HEADER
// =============================================================================

#block(
  width: 100%,
  fill: colors.background-alt,
  inset: spacing.md,
  radius: 2pt,
)[
  #grid(
    columns: (1fr, auto),
    [
      #title-text(session-title)
      #if module-name != none [
        #linebreak()
        #text(style: "italic")[#module-name]
      ]
    ],
    align(right)[
      #if session-number != none [
        #text(size: sizes.lg, weight: "bold")[Session #session-number]
        #linebreak()
      ]
      #if date != none [
        #text(size: sizes.sm, fill: colors.text-secondary)[#date]
      ]
    ]
  )
]

#v(spacing.md)

// =============================================================================
// SUMMARY
// =============================================================================

#if summary != none [
  #block(
    width: 100%,
    stroke: 1pt + colors.border,
    inset: spacing.md,
    radius: 2pt,
  )[
    #text(weight: "bold")[Session Summary]
    #v(spacing.sm)
    #summary
  ]
  #v(spacing.md)
]

// =============================================================================
// TWO-COLUMN LAYOUT: NPCs and Locations
// =============================================================================

#grid(
  columns: (1fr, 1fr),
  column-gutter: spacing.md,

  // NPCs Column
  [
    #block(
      width: 100%,
      fill: colors.background-alt,
      inset: spacing.sm,
      radius: 2pt,
    )[
      #heading-text("Key NPCs")
    ]
    #v(spacing.xs)
    #if npcs.len() > 0 [
      #for npc in npcs [
        #let npc-name = get(npc, "name", default: "Unknown")
        #let npc-role = get(npc, "role", default: none)
        #let npc-notes = get(npc, "notes", default: none)

        #block(
          width: 100%,
          inset: (x: spacing.sm, y: spacing.xs),
          stroke: (left: 2pt + colors.accent),
        )[
          #text(weight: "bold")[#npc-name]
          #if npc-role != none [
            #text(size: sizes.sm, fill: colors.text-secondary)[ - #npc-role]
          ]
          #if npc-notes != none [
            #linebreak()
            #text(size: sizes.sm)[#npc-notes]
          ]
        ]
        #v(spacing.xs)
      ]
    ] else [
      #text(fill: colors.text-secondary, style: "italic")[No NPCs listed]
    ]
  ],

  // Locations Column
  [
    #block(
      width: 100%,
      fill: colors.background-alt,
      inset: spacing.sm,
      radius: 2pt,
    )[
      #heading-text("Locations")
    ]
    #v(spacing.xs)
    #if locations.len() > 0 [
      #for loc in locations [
        #let loc-name = get(loc, "name", default: "Unknown")
        #let loc-type = get(loc, "type", default: none)
        #let loc-notes = get(loc, "notes", default: none)

        #block(
          width: 100%,
          inset: (x: spacing.sm, y: spacing.xs),
          stroke: (left: 2pt + colors.accent),
        )[
          #text(weight: "bold")[#loc-name]
          #if loc-type != none [
            #text(size: sizes.sm, fill: colors.text-secondary)[ - #loc-type]
          ]
          #if loc-notes != none [
            #linebreak()
            #text(size: sizes.sm)[#loc-notes]
          ]
        ]
        #v(spacing.xs)
      ]
    ] else [
      #text(fill: colors.text-secondary, style: "italic")[No locations listed]
    ]
  ],
)

#v(spacing.md)

// =============================================================================
// ENCOUNTERS
// =============================================================================

#block(
  width: 100%,
  fill: colors.background-alt,
  inset: spacing.sm,
  radius: 2pt,
)[
  #heading-text("Planned Encounters")
]
#v(spacing.xs)

#if encounters.len() > 0 [
  #for (i, enc) in encounters.enumerate() [
    #let enc-name = get(enc, "name", default: "Encounter")
    #let enc-type = get(enc, "type", default: none)
    #let enc-difficulty = get(enc, "difficulty", default: none)
    #let enc-monsters = get(enc, "monsters", default: ())
    #let enc-notes = get(enc, "notes", default: none)

    #block(
      width: 100%,
      stroke: 0.5pt + colors.border,
      inset: spacing.sm,
      radius: 2pt,
    )[
      #grid(
        columns: (auto, 1fr, auto),
        column-gutter: spacing.sm,
        [
          #box(
            width: 1.5em,
            height: 1.5em,
            stroke: 1pt + colors.border,
            radius: 2pt,
            align(center + horizon)[#text(size: sizes.sm)[#(i + 1)]]
          )
        ],
        [
          #text(weight: "bold")[#enc-name]
          #if enc-type != none [
            #text(size: sizes.sm, fill: colors.text-secondary)[ (#enc-type)]
          ]
        ],
        {
          if enc-difficulty != none {
            let color-easy = rgb("#22c55e")
            let color-medium = rgb("#f59e0b")
            let color-hard = rgb("#ef4444")
            let color-deadly = rgb("#dc2626")
            let diff-color = if enc-difficulty == "easy" { color-easy }
              else if enc-difficulty == "medium" { color-medium }
              else if enc-difficulty == "hard" { color-hard }
              else if enc-difficulty == "deadly" { color-deadly }
              else { colors.text-secondary }
            box(
              fill: diff-color.lighten(80%),
              inset: (x: spacing.sm, y: 2pt),
              radius: 2pt,
            )[
              #text(size: sizes.xs, fill: diff-color.darken(20%), weight: "bold")[#upper(enc-difficulty)]
            ]
          }
        }
      )

      #if enc-monsters.len() > 0 [
        #v(spacing.xs)
        #text(size: sizes.sm)[
          *Creatures:* #enc-monsters.map(m => {
            let name = get(m, "name", default: "?")
            let count = get(m, "count", default: 1)
            if count > 1 { str(count) + "x " + name } else { name }
          }).join(", ")
        ]
      ]

      #if enc-notes != none [
        #v(spacing.xs)
        #text(size: sizes.sm, fill: colors.text-secondary)[#enc-notes]
      ]
    ]
    #v(spacing.xs)
  ]
] else [
  #text(fill: colors.text-secondary, style: "italic")[No encounters planned]
]

#v(spacing.md)

// =============================================================================
// TWO-COLUMN: Items and Hooks
// =============================================================================

#grid(
  columns: (1fr, 1fr),
  column-gutter: spacing.md,

  // Key Items
  [
    #block(
      width: 100%,
      fill: colors.background-alt,
      inset: spacing.sm,
      radius: 2pt,
    )[
      #heading-text("Key Items")
    ]
    #v(spacing.xs)
    #if items.len() > 0 [
      #for item in items [
        #let item-name = get(item, "name", default: "Item")
        #let item-desc = get(item, "description", default: none)

        - #text(weight: "bold")[#item-name]#if item-desc != none [ - #item-desc]
      ]
    ] else [
      #text(fill: colors.text-secondary, style: "italic")[No key items]
    ]
  ],

  // Plot Hooks
  [
    #block(
      width: 100%,
      fill: colors.background-alt,
      inset: spacing.sm,
      radius: 2pt,
    )[
      #heading-text("Plot Hooks")
    ]
    #v(spacing.xs)
    #if hooks.len() > 0 [
      #for hook in hooks [
        - #hook
      ]
    ] else [
      #text(fill: colors.text-secondary, style: "italic")[No plot hooks]
    ]
  ],
)

#v(spacing.md)

// =============================================================================
// SECRETS (DM Eyes Only)
// =============================================================================

#if secrets.len() > 0 [
  #block(
    width: 100%,
    stroke: 2pt + rgb("#dc2626"),
    fill: rgb("#fef2f2"),
    inset: spacing.md,
    radius: 2pt,
  )[
    #text(weight: "bold", fill: rgb("#dc2626"))[DM SECRETS - DO NOT SHARE]
    #v(spacing.sm)
    #for secret in secrets [
      - #secret
    ]
  ]
  #v(spacing.md)
]

// =============================================================================
// NOTES
// =============================================================================

#block(
  width: 100%,
  fill: colors.background-alt,
  inset: spacing.sm,
  radius: 2pt,
)[
  #heading-text("Session Notes")
]
#v(spacing.xs)

#if type(notes) == array and notes.len() > 0 [
  #for note in notes [
    - #note
  ]
] else if type(notes) == str and notes != "" [
  #notes
] else [
  #block(
    width: 100%,
    height: 2in,
    stroke: 0.5pt + colors.border-light,
    inset: spacing.sm,
    radius: 2pt,
  )[
    #text(fill: colors.text-secondary, style: "italic")[Space for session notes...]
  ]
]

// =============================================================================
// FOOTER
// =============================================================================

#v(1fr)
#align(center)[
  #small-text[#session-title - Generated by Mimir]
]
