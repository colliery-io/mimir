// Test template for shared components
// This verifies all shared components render correctly

#import "/_shared/styles.typ": *
#import "/_shared/components.typ": *
#import "/_shared/icons.typ": *

#show: mimir-doc

= Component Test Document

#subtitle-text[Testing all shared Mimir print components]

#section-divider()

== Typography

#title-text[Title Text]

#heading-text[Heading Text]

#subtitle-text[Subtitle Text]

#label-text[Label Text]

#value-text[Value Text]

#small-text[Small text for fine print]

#mono-text[Monospace: 2d6+3]

#section-divider()

== Ability Scores

=== Grid Layout (2x3)

#ability-scores(str: 18, dex: 14, con: 16, int: 10, wis: 12, cha: 8)

=== Row Layout (6x1)

#ability-scores(str: 18, dex: 14, con: 16, int: 10, wis: 12, cha: 8, layout: "row")

#section-divider()

== Icons

=== Class Icons

#class-icon("fighter") Fighter
#h(spacing.md)
#class-icon("wizard") Wizard
#h(spacing.md)
#class-icon("rogue") Rogue
#h(spacing.md)
#class-icon("cleric") Cleric

=== Spell School Icons

#spell-school-label("Evocation")
#h(spacing.md)
#spell-school-label("Abjuration")
#h(spacing.md)
#spell-school-label("Necromancy")

=== Damage Types

#damage-with-type("2d6", "fire")
#h(spacing.md)
#damage-with-type("1d8", "cold")
#h(spacing.md)
#damage-with-type("3d10", "lightning")

=== Spell Components

#spell-components(verbal: true, somatic: true, material: "a pinch of sulfur")

#section-divider()

== Layout Components

=== Two Columns

#two-columns(
  info-box(title: "Left Column")[
    This is content in the left column.
  ],
  info-box(title: "Right Column")[
    This is content in the right column.
  ]
)

=== Labeled Values

#grid(
  columns: (1fr, 1fr, 1fr, 1fr),
  column-gutter: spacing.md,
  labeled-value("Level", "5"),
  labeled-value("HP", "45"),
  labeled-value("AC", "16"),
  labeled-value("Speed", "30 ft"),
)

=== Highlight Box

#highlight-box[
  This is important information that should stand out.
]

#section-divider()

== Tables

=== Standard Table

#mimir-table(
  columns: (auto, 1fr, auto),
  [*Name*], [*Description*], [*Value*],
  [Strength], [Physical power], [18],
  [Dexterity], [Agility and reflexes], [14],
  [Constitution], [Endurance and health], [16],
)

#section-divider()

== Monster Stat Block

#stat-block(
  name: "Goblin",
  size: "Small",
  type: "humanoid (goblinoid)",
  alignment: "neutral evil",
  ac: "15 (leather armor, shield)",
  hp: "7 (2d6)",
  speed: "30 ft.",
  str: 8,
  dex: 14,
  con: 10,
  int: 10,
  wis: 8,
  cha: 8,
  skills: "Stealth +6",
  senses: "darkvision 60 ft., passive Perception 9",
  languages: "Common, Goblin",
  cr: "1/4",
  traits: (
    (name: "Nimble Escape", description: "The goblin can take the Disengage or Hide action as a bonus action on each of its turns."),
  ),
  actions: (
    (name: "Scimitar", description: [_Melee Weapon Attack:_ +4 to hit, reach 5 ft., one target. _Hit:_ 5 (1d6 + 2) slashing damage.]),
    (name: "Shortbow", description: [_Ranged Weapon Attack:_ +4 to hit, range 80/320 ft., one target. _Hit:_ 5 (1d6 + 2) piercing damage.]),
  ),
)

#pagebreak()

== Card Test

#v(spacing.xl)

// Simulating a card (would normally use card document size)
#box(width: 2.5in, height: 3.5in)[
  #card(
    title: "Fireball",
    subtitle: "3rd-level evocation",
    category: "Wizard, Sorcerer",
    footer: "PHB p. 241",
  )[
    A bright streak flashes from your pointing finger to a point you choose within range and then blossoms with a low roar into an explosion of flame.

    #v(spacing.sm)
    #dice-with-avg("8d6", "28") fire damage

    #v(spacing.sm)
    #spell-components(verbal: true, somatic: true, material: "bat guano and sulfur")
  ]
]
