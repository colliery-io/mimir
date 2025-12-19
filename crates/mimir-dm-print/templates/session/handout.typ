// Mimir Handout Template
// Generic player-facing document template

#import "/_shared/styles.typ": *
#import "/_shared/components.typ": *
#import "/_shared/icons.typ": *

// Document setup
#show: mimir-doc.with(margin: 0.75in)

// Helper to safely get nested data
#let get(obj, key, default: none) = {
  if obj != none and key in obj { obj.at(key) } else { default }
}

// =============================================================================
// DATA EXTRACTION
// =============================================================================

#let handout-title = get(data, "title", default: "Handout")
#let subtitle = get(data, "subtitle", default: none)
#let handout-type = get(data, "type", default: none)
#let author = get(data, "author", default: none)
#let date-written = get(data, "date", default: none)
#let body-text = get(data, "body", default: none)
#let sections = get(data, "sections", default: ())
#let footer-text = get(data, "footer", default: none)
#let style = get(data, "style", default: "default")

// Define colors for styles
#let color-aged = rgb("8b7355")
#let color-aged-bg = rgb("faf3e0")
#let color-magic = rgb("6366f1")
#let color-magic-bg = rgb("f5f3ff")

// Style settings
#let is-aged = style == "aged"
#let is-formal = style == "formal"
#let is-magical = style == "magical"

#let border-style = if is-aged { 2pt + color-aged } else if is-formal { 1pt + black } else if is-magical { 2pt + color-magic } else { 1pt + colors.border }
#let bg-color = if is-aged { color-aged-bg } else if is-magical { color-magic-bg } else { white }
#let use-italic = is-aged

// =============================================================================
// HANDOUT LAYOUT
// =============================================================================

#set page(fill: bg-color)

// Main content box
#block(
  width: 100%,
  stroke: border-style,
  inset: spacing.lg,
  radius: if is-formal { 0pt } else { 4pt },
)[
  // Title
  #align(center)[
    #if handout-type != none [
      #text(size: sizes.sm, fill: colors.text-secondary, weight: "bold")[#upper(handout-type)]
      #v(spacing.xs)
    ]

    #if use-italic [
      #text(size: sizes.xl, weight: "bold", style: "italic")[#handout-title]
    ] else [
      #text(size: sizes.xl, weight: "bold")[#handout-title]
    ]

    #if subtitle != none [
      #v(spacing.xs)
      #text(size: sizes.md, style: "italic")[#subtitle]
    ]

    #if author != none or date-written != none [
      #v(spacing.sm)
      #text(size: sizes.sm, fill: colors.text-secondary)[
        #if author != none [#author]
        #if author != none and date-written != none [ - ]
        #if date-written != none [#date-written]
      ]
    ]
  ]

  #v(spacing.lg)

  #if is-formal [
    #line(length: 100%, stroke: 0.5pt + colors.border)
  ] else [
    #divider()
  ]
  #v(spacing.md)

  // Body text
  #if body-text != none [
    #set par(justify: true, leading: 0.8em)

    #if type(body-text) == array [
      #for paragraph in body-text [
        #if use-italic [
          #text(style: "italic")[#paragraph]
        ] else [
          #paragraph
        ]
        #v(spacing.md)
      ]
    ] else [
      #if use-italic [
        #text(style: "italic")[#body-text]
      ] else [
        #body-text
      ]
    ]
  ]

  // Sections
  #for section in sections [
    #let section-title = get(section, "title", default: none)
    #let section-content = get(section, "content", default: "")

    #if section-title != none [
      #v(spacing.md)
      #if use-italic [
        #text(size: sizes.md, weight: "bold", style: "italic")[#section-title]
      ] else [
        #text(size: sizes.md, weight: "bold")[#section-title]
      ]
      #v(spacing.sm)
    ]

    #set par(justify: true)
    #if use-italic [
      #text(style: "italic")[#section-content]
    ] else [
      #section-content
    ]
  ]

  // Footer
  #if footer-text != none [
    #v(spacing.lg)
    #if is-formal [
      #line(length: 100%, stroke: 0.5pt + colors.border)
    ] else [
      #divider()
    ]
    #v(spacing.sm)
    #align(center)[
      #text(size: sizes.sm, style: "italic", fill: colors.text-secondary)[#footer-text]
    ]
  ]
]

// Magical style decoration
#if is-magical [
  #let star-color = color-magic.lighten(60%)
  #place(top + left, dx: 0.5in, dy: 0.5in)[#text(size: 24pt, fill: star-color)[+]]
  #place(top + right, dx: -0.5in, dy: 0.5in)[#text(size: 24pt, fill: star-color)[+]]
  #place(bottom + left, dx: 0.5in, dy: -0.5in)[#text(size: 24pt, fill: star-color)[+]]
  #place(bottom + right, dx: -0.5in, dy: -0.5in)[#text(size: 24pt, fill: star-color)[+]]
]
