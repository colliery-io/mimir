// Mimir NPC Cards Multi-up Template
// 4 index cards per US Letter page (2x2 grid, 5" x 3" each)

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

#let npcs = get(data, "npcs", default: ())
#let show-cut-lines = get(data, "show_cut_lines", default: true)

// Card dimensions (index card: 5" x 3")
#let card-width = 5in
#let card-height = 3in

// =============================================================================
// SINGLE NPC CARD COMPONENT
// =============================================================================

#let npc-card(npc) = {
  let npc-name = get(npc, "name", default: "Unknown NPC")
  let race = get(npc, "race", default: none)
  let role = get(npc, "role", default: none)
  let occupation = get(npc, "occupation", default: none)
  let alignment = get(npc, "alignment", default: none)
  let location = get(npc, "location", default: none)

  let appearance = get(npc, "appearance", default: none)
  let personality = get(npc, "personality", default: none)
  let mannerisms = get(npc, "mannerisms", default: none)
  let voice = get(npc, "voice", default: none)

  let goal = get(npc, "goal", default: none)
  let motivation = get(npc, "motivation", default: none)
  let bond = get(npc, "bond", default: none)
  let flaw = get(npc, "flaw", default: none)

  let secret = get(npc, "secret", default: none)
  let key-info = get(npc, "key_info", default: none)

  // Build subtitle
  let subtitle-parts = ()
  if race != none { subtitle-parts.push(race) }
  if occupation != none { subtitle-parts.push(occupation) }
  if role != none { subtitle-parts.push(role) }

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
      inset: (x: 6pt, y: 4pt),
    )[
      #grid(
        columns: (1fr, auto),
        [
          #text(size: 8pt, weight: "bold")[#npc-name]
          #if subtitle-parts.len() > 0 [
            #linebreak()
            #text(size: 6pt, fill: colors.text-secondary)[#subtitle-parts.join(", ")]
          ]
        ],
        align(right)[
          #if alignment != none [
            #text(size: 5pt, fill: colors.text-secondary)[#alignment]
          ]
        ]
      )
    ]

    // Main content
    #block(
      width: 100%,
      inset: 4pt,
    )[
      #grid(
        columns: (1fr, 1fr),
        column-gutter: 4pt,

        // Left column
        [
          #if appearance != none [
            #text(size: 5pt)[#text(weight: "bold")[Look: ]#appearance]
            #v(1pt)
          ]
          #if personality != none [
            #text(size: 5pt)[#text(weight: "bold")[Personality: ]#personality]
            #v(1pt)
          ]
          #if mannerisms != none [
            #text(size: 5pt)[#text(weight: "bold")[Mannerism: ]#mannerisms]
          ]
        ],

        // Right column
        [
          #if goal != none [
            #text(size: 5pt)[#text(weight: "bold")[Goal: ]#goal]
            #v(1pt)
          ]
          #if motivation != none [
            #text(size: 5pt)[#text(weight: "bold")[Motivation: ]#motivation]
            #v(1pt)
          ]
          #if flaw != none [
            #text(size: 5pt)[#text(weight: "bold")[Flaw: ]#flaw]
          ]
        ],
      )
    ]

    // Key info
    #if key-info != none [
      #block(
        width: 100%,
        inset: (x: 4pt, y: 2pt),
        stroke: (top: 0.5pt + colors.border-light),
      )[
        #text(size: 5pt)[#text(weight: "bold")[Info: ]#key-info]
      ]
    ]

    // Secret (bottom)
    #if secret != none [
      #place(
        bottom + left,
        block(
          width: 100%,
          fill: rgb("#fef2f2"),
          stroke: (top: 0.5pt + rgb("#dc2626")),
          inset: (x: 4pt, y: 2pt),
        )[
          #text(size: 4pt, fill: rgb("#dc2626"))[#text(weight: "bold")[SECRET: ]#secret]
        ]
      )
    ] else if location != none [
      #place(
        bottom + left,
        block(
          width: 100%,
          fill: colors.background-alt,
          inset: (x: 4pt, y: 2pt),
        )[
          #text(size: 4pt, fill: colors.text-secondary)[#location]
        ]
      )
    ]
  ]
}

// =============================================================================
// MULTI-UP LAYOUT
// =============================================================================

// Calculate pages needed (4 cards per page)
#let cards-per-page = 4
#let total-pages = if npcs.len() > 0 { calc.ceil(npcs.len() / cards-per-page) } else { 0 }

#for page-num in range(total-pages) [
  #let start-idx = page-num * cards-per-page
  #let end-idx = calc.min(start-idx + cards-per-page, npcs.len())
  #let page-npcs = npcs.slice(start-idx, end-idx)

  // Add page break after first page
  #if page-num > 0 [
    #pagebreak()
  ]

  // Center the card grid
  #align(center + horizon)[
    #grid(
      columns: (card-width,) * 2,
      rows: (card-height,) * 2,
      column-gutter: if show-cut-lines { 0pt } else { 4pt },
      row-gutter: if show-cut-lines { 0pt } else { 4pt },

      ..for i in range(4) {
        if i < page-npcs.len() {
          (npc-card(page-npcs.at(i)),)
        } else {
          (box(width: card-width, height: card-height),)
        }
      }
    )
  ]

  // Cut lines indicator
  #if show-cut-lines and page-npcs.len() > 0 [
    #place(
      bottom + center,
      dy: 0.1in,
      text(size: 6pt, fill: colors.text-secondary)[Cut along card borders]
    )
  ]
]

// Empty state
#if npcs.len() == 0 [
  #align(center + horizon)[
    #text(fill: colors.text-secondary)[No NPCs to display]
  ]
]
