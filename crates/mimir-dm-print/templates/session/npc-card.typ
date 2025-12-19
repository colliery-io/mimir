// Mimir NPC Card Template
// Index card size (3" x 5") for quick reference during roleplay

#import "/_shared/styles.typ": *
#import "/_shared/components.typ": *
#import "/_shared/icons.typ": *

// Index card document setup (3" x 5")
#set page(
  width: 5in,
  height: 3in,
  margin: 0.2in,
)
#set text(font: font-body, size: sizes.sm)

// Helper to safely get nested data
#let get(obj, key, default: none) = {
  if obj != none and key in obj { obj.at(key) } else { default }
}

// =============================================================================
// DATA EXTRACTION
// =============================================================================

#let npc-name = get(data, "name", default: "Unknown NPC")
#let race = get(data, "race", default: none)
#let role = get(data, "role", default: none)
#let occupation = get(data, "occupation", default: none)
#let alignment = get(data, "alignment", default: none)
#let location = get(data, "location", default: none)

#let appearance = get(data, "appearance", default: none)
#let personality = get(data, "personality", default: none)
#let mannerisms = get(data, "mannerisms", default: none)
#let voice = get(data, "voice", default: none)

#let goal = get(data, "goal", default: none)
#let motivation = get(data, "motivation", default: none)
#let bond = get(data, "bond", default: none)
#let flaw = get(data, "flaw", default: none)

#let secret = get(data, "secret", default: none)
#let key-info = get(data, "key_info", default: none)
#let notes = get(data, "notes", default: none)

// Build subtitle
#let subtitle-parts = ()
#if race != none { subtitle-parts.push(race) }
#if occupation != none { subtitle-parts.push(occupation) }
#if role != none { subtitle-parts.push(role) }

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
  // Header
  #block(
    width: 100%,
    fill: colors.background-alt,
    inset: (x: spacing.sm, y: spacing.xs),
  )[
    #grid(
      columns: (1fr, auto),
      [
        #text(size: sizes.md, weight: "bold")[#npc-name]
        #if subtitle-parts.len() > 0 [
          #linebreak()
          #text(size: sizes.xs, fill: colors.text-secondary)[#subtitle-parts.join(", ")]
        ]
      ],
      align(right)[
        #if alignment != none [
          #text(size: sizes.xs, fill: colors.text-secondary)[#alignment]
        ]
      ]
    )
  ]

  // Main content - two columns
  #block(
    width: 100%,
    inset: spacing.xs,
  )[
    #grid(
      columns: (1fr, 1fr),
      column-gutter: spacing.sm,

      // Left column: Appearance & Personality
      [
        #if appearance != none [
          #text(size: 6pt)[
            #text(weight: "bold")[Appearance: ]
            #appearance
          ]
          #v(2pt)
        ]

        #if personality != none [
          #text(size: 6pt)[
            #text(weight: "bold")[Personality: ]
            #personality
          ]
          #v(2pt)
        ]

        #if mannerisms != none [
          #text(size: 6pt)[
            #text(weight: "bold")[Mannerisms: ]
            #mannerisms
          ]
          #v(2pt)
        ]

        #if voice != none [
          #text(size: 6pt)[
            #text(weight: "bold")[Voice: ]
            #voice
          ]
        ]
      ],

      // Right column: Goals & Info
      [
        #if goal != none [
          #text(size: 6pt)[
            #text(weight: "bold")[Goal: ]
            #goal
          ]
          #v(2pt)
        ]

        #if motivation != none [
          #text(size: 6pt)[
            #text(weight: "bold")[Motivation: ]
            #motivation
          ]
          #v(2pt)
        ]

        #if bond != none [
          #text(size: 6pt)[
            #text(weight: "bold")[Bond: ]
            #bond
          ]
          #v(2pt)
        ]

        #if flaw != none [
          #text(size: 6pt)[
            #text(weight: "bold")[Flaw: ]
            #flaw
          ]
        ]
      ],
    )
  ]

  // Key info section
  #if key-info != none [
    #block(
      width: 100%,
      inset: (x: spacing.xs, y: 2pt),
      stroke: (top: 0.5pt + colors.border-light),
    )[
      #text(size: 6pt)[
        #text(weight: "bold")[Key Info: ]
        #key-info
      ]
    ]
  ]

  // Secret section (DM only)
  #if secret != none [
    #place(
      bottom + left,
      block(
        width: 100%,
        fill: rgb("#fef2f2"),
        stroke: (top: 1pt + rgb("#dc2626")),
        inset: (x: spacing.xs, y: 2pt),
      )[
        #text(size: 5pt, fill: rgb("#dc2626"))[
          #text(weight: "bold")[SECRET: ]
          #secret
        ]
      ]
    )
  ]

  // Location footer (if no secret)
  #if secret == none and location != none [
    #place(
      bottom + left,
      block(
        width: 100%,
        fill: colors.background-alt,
        inset: (x: spacing.xs, y: 2pt),
      )[
        #text(size: 5pt, fill: colors.text-secondary)[
          #text(weight: "bold")[Location: ]
          #location
        ]
      ]
    )
  ]
]
