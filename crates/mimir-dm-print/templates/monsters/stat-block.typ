// Mimir Monster Stat Block Template
// Full-page monster stat block with all details

#import "/_shared/styles.typ": *
#import "/_shared/components.typ": *
#import "/_shared/icons.typ": *

// Document setup
#show: mimir-doc.with(margin: 0.5in)

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

#let monster-name = get(data, "name", default: "Unknown Monster")
#let source = get(data, "source", default: "")

// Size - can be array or string
#let size-data = get(data, "size", default: ("M",))
#let monster-size = if type(size-data) == array and size-data.len() > 0 {
  let s = size-data.at(0)
  if s == "T" { "Tiny" }
  else if s == "S" { "Small" }
  else if s == "M" { "Medium" }
  else if s == "L" { "Large" }
  else if s == "H" { "Huge" }
  else if s == "G" { "Gargantuan" }
  else { s }
} else if type(size-data) == str {
  size-data
} else { "Medium" }

// Creature type - can be string or object
#let type-data = get(data, "creature_type", default: "humanoid")
#let monster-type = if type(type-data) == str {
  type-data
} else if type(type-data) == dictionary {
  let base = get(type-data, "type", default: "humanoid")
  let tags = get(type-data, "tags", default: ())
  if tags.len() > 0 {
    base + " (" + tags.join(", ") + ")"
  } else { base }
} else { "humanoid" }

// Alignment - can be array or string
#let align-data = get(data, "alignment", default: none)
#let monster-alignment = if align-data == none { "unaligned" }
  else if type(align-data) == array {
    let abbrevs = align-data.map(a => {
      if type(a) == str {
        if a == "L" { "lawful" }
        else if a == "N" { "neutral" }
        else if a == "NX" { "neutral" }
        else if a == "NY" { "neutral" }
        else if a == "C" { "chaotic" }
        else if a == "G" { "good" }
        else if a == "E" { "evil" }
        else if a == "U" { "unaligned" }
        else if a == "A" { "any alignment" }
        else { a }
      } else if type(a) == dictionary {
        get(a, "alignment", default: "unaligned")
      } else { str(a) }
    })
    abbrevs.join(" ")
  }
  else if type(align-data) == str { align-data }
  else { "unaligned" }

// AC - can be number or array of objects
#let ac-data = get(data, "ac", default: 10)
#let monster-ac = if type(ac-data) == int { str(ac-data) }
  else if type(ac-data) == array and ac-data.len() > 0 {
    let first = ac-data.at(0)
    if type(first) == int { str(first) }
    else if type(first) == dictionary {
      let ac-val = get(first, "ac", default: 10)
      let ac-from = get(first, "from", default: ())
      if ac-from.len() > 0 {
        str(ac-val) + " (" + ac-from.join(", ") + ")"
      } else { str(ac-val) }
    } else { "10" }
  } else { "10" }

// HP - can be number or object
#let hp-data = get(data, "hp", default: none)
#let monster-hp = if hp-data == none { "1" }
  else if type(hp-data) == int { str(hp-data) }
  else if type(hp-data) == dictionary {
    let avg = get(hp-data, "average", default: 1)
    let formula = get(hp-data, "formula", default: none)
    if formula != none { str(avg) + " (" + formula + ")" }
    else { str(avg) }
  } else { "1" }

// Speed
#let speed-data = get(data, "speed", default: none)
#let monster-speed = if speed-data == none { "30 ft." }
  else {
    let parts = ()
    let walk = get(speed-data, "walk", default: none)
    if walk != none {
      if type(walk) == int { parts.push(str(walk) + " ft.") }
      else if type(walk) == dictionary {
        let amount = get(walk, "number", default: 30)
        parts.push(str(amount) + " ft.")
      } else { parts.push("30 ft.") }
    }
    let burrow = get(speed-data, "burrow", default: none)
    if burrow != none {
      if type(burrow) == int { parts.push("burrow " + str(burrow) + " ft.") }
    }
    let climb = get(speed-data, "climb", default: none)
    if climb != none {
      if type(climb) == int { parts.push("climb " + str(climb) + " ft.") }
    }
    let fly = get(speed-data, "fly", default: none)
    if fly != none {
      let hover = get(speed-data, "hover", default: false) or get(speed-data, "canHover", default: false)
      if type(fly) == int {
        let fly-str = "fly " + str(fly) + " ft."
        if hover { fly-str = fly-str + " (hover)" }
        parts.push(fly-str)
      } else if type(fly) == dictionary {
        let amount = get(fly, "number", default: 30)
        let fly-str = "fly " + str(amount) + " ft."
        if hover { fly-str = fly-str + " (hover)" }
        parts.push(fly-str)
      }
    }
    let swim = get(speed-data, "swim", default: none)
    if swim != none {
      if type(swim) == int { parts.push("swim " + str(swim) + " ft.") }
    }
    if parts.len() > 0 { parts.join(", ") } else { "30 ft." }
  }

// Ability scores
#let str-score = get(data, "str", default: 10)
#let dex-score = get(data, "dex", default: 10)
#let con-score = get(data, "con", default: 10)
#let int-score = get(data, "int", default: 10)
#let wis-score = get(data, "wis", default: 10)
#let cha-score = get(data, "cha", default: 10)

// Saves
#let save-data = get(data, "save", default: none)
#let monster-saves = if save-data == none { none }
  else {
    let parts = ()
    let str-save = get(save-data, "str", default: none)
    if str-save != none { parts.push("Str " + str-save) }
    let dex-save = get(save-data, "dex", default: none)
    if dex-save != none { parts.push("Dex " + dex-save) }
    let con-save = get(save-data, "con", default: none)
    if con-save != none { parts.push("Con " + con-save) }
    let int-save = get(save-data, "int", default: none)
    if int-save != none { parts.push("Int " + int-save) }
    let wis-save = get(save-data, "wis", default: none)
    if wis-save != none { parts.push("Wis " + wis-save) }
    let cha-save = get(save-data, "cha", default: none)
    if cha-save != none { parts.push("Cha " + cha-save) }
    if parts.len() > 0 { parts.join(", ") } else { none }
  }

// Skills
#let skill-data = get(data, "skill", default: none)
#let monster-skills = if skill-data == none { none }
  else {
    let parts = ()
    let skill-names = (
      ("acrobatics", "Acrobatics"),
      ("arcana", "Arcana"),
      ("athletics", "Athletics"),
      ("deception", "Deception"),
      ("history", "History"),
      ("insight", "Insight"),
      ("intimidation", "Intimidation"),
      ("investigation", "Investigation"),
      ("medicine", "Medicine"),
      ("nature", "Nature"),
      ("perception", "Perception"),
      ("performance", "Performance"),
      ("persuasion", "Persuasion"),
      ("religion", "Religion"),
      ("sleight_of_hand", "Sleight of Hand"),
      ("stealth", "Stealth"),
      ("survival", "Survival"),
    )
    for (key, name) in skill-names {
      let val = get(skill-data, key, default: none)
      if val != none { parts.push(name + " " + val) }
    }
    if parts.len() > 0 { parts.join(", ") } else { none }
  }

// Damage vulnerabilities, resistances, immunities
#let vuln-data = get(data, "damage_vulnerabilities", default: ())
#let monster-vuln = if vuln-data.len() > 0 { vuln-data.join(", ") } else { none }

#let resist-data = get(data, "damage_resistances", default: ())
#let monster-resist = if resist-data.len() > 0 { resist-data.join(", ") } else { none }

#let immune-data = get(data, "damage_immunities", default: ())
#let monster-immune = if immune-data.len() > 0 { immune-data.join(", ") } else { none }

#let cond-immune-data = get(data, "condition_immunities", default: ())
#let monster-cond-immune = if cond-immune-data.len() > 0 { cond-immune-data.join(", ") } else { none }

// Senses
#let senses-data = get(data, "senses", default: ())
#let passive = get(data, "passive", default: 10)
#let monster-senses = {
  let parts = if type(senses-data) == array { senses-data } else { () }
  parts.push("passive Perception " + str(passive))
  parts.join(", ")
}

// Languages
#let lang-data = get(data, "languages", default: ())
#let monster-languages = if lang-data.len() > 0 { lang-data.join(", ") }
  else { "--" }

// Challenge rating
#let cr-data = get(data, "cr", default: "0")
#let monster-cr = if type(cr-data) == str { cr-data }
  else if type(cr-data) == dictionary { get(cr-data, "cr", default: "0") }
  else if type(cr-data) == int { str(cr-data) }
  else { "0" }

// XP table
#let xp-table = (
  "0": "0 or 10",
  "1/8": "25",
  "1/4": "50",
  "1/2": "100",
  "1": "200",
  "2": "450",
  "3": "700",
  "4": "1,100",
  "5": "1,800",
  "6": "2,300",
  "7": "2,900",
  "8": "3,900",
  "9": "5,000",
  "10": "5,900",
  "11": "7,200",
  "12": "8,400",
  "13": "10,000",
  "14": "11,500",
  "15": "13,000",
  "16": "15,000",
  "17": "18,000",
  "18": "20,000",
  "19": "22,000",
  "20": "25,000",
  "21": "33,000",
  "22": "41,000",
  "23": "50,000",
  "24": "62,000",
  "25": "75,000",
  "26": "90,000",
  "27": "105,000",
  "28": "120,000",
  "29": "135,000",
  "30": "155,000",
)

#let monster-xp = if monster-cr in xp-table { xp-table.at(monster-cr) } else { "0" }

// =============================================================================
// ENTRY PARSING
// =============================================================================

// Parse entries (traits, actions, etc.) from 5etools format
#let parse-entries(entries) = {
  if entries == none { return () }
  entries.map(entry => {
    if type(entry) == str {
      (name: none, description: entry)
    } else if type(entry) == dictionary {
      let name = get(entry, "name", default: none)
      let entry-entries = get(entry, "entries", default: ())
      let desc = if entry-entries.len() > 0 {
        entry-entries.filter(e => type(e) == str).join(" ")
      } else { "" }
      (name: name, description: desc)
    } else {
      (name: none, description: str(entry))
    }
  }).filter(e => e.description != "" or e.name != none)
}

#let traits = parse-entries(get(data, "trait_entries", default: ()))
#let actions = parse-entries(get(data, "action", default: ()))
#let bonus-actions = parse-entries(get(data, "bonus", default: ()))
#let reactions = parse-entries(get(data, "reaction", default: ()))
#let legendary = parse-entries(get(data, "legendary", default: ()))
#let mythic = parse-entries(get(data, "mythic", default: ()))

// =============================================================================
// STAT BLOCK RENDERING
// =============================================================================

stat-block(
  name: monster-name,
  size: monster-size,
  type: monster-type,
  alignment: monster-alignment,
  ac: monster-ac,
  hp: monster-hp,
  speed: monster-speed,
  str: str-score,
  dex: dex-score,
  con: con-score,
  int: int-score,
  wis: wis-score,
  cha: cha-score,
  saves: monster-saves,
  skills: monster-skills,
  damage-vulnerabilities: monster-vuln,
  damage-resistances: monster-resist,
  damage-immunities: monster-immune,
  condition-immunities: monster-cond-immune,
  senses: monster-senses,
  languages: monster-languages,
  cr: monster-cr + " (" + monster-xp + " XP)",
  traits: traits,
  actions: actions,
  reactions: reactions,
  legendary-actions: legendary,
)

// Bonus actions (not in standard stat-block component)
#if bonus-actions.len() > 0 [
  #block(inset: spacing.md)[
    #heading-text("Bonus Actions")
    #divider()
    #v(spacing.sm)
    #for action in bonus-actions [
      #if action.name != none [
        #text(weight: "bold", style: "italic")[#action.name. ]
      ]
      #action.description
      #v(spacing.sm)
    ]
  ]
]

// Mythic actions (not in standard stat-block component)
#if mythic.len() > 0 [
  #block(inset: spacing.md)[
    #heading-text("Mythic Actions")
    #divider()
    #v(spacing.sm)
    #for action in mythic [
      #if action.name != none [
        #text(weight: "bold", style: "italic")[#action.name. ]
      ]
      #action.description
      #v(spacing.sm)
    ]
  ]
]

// Footer
#v(1fr)
#align(center)[
  #small-text[#monster-name - #source - Generated by Mimir]
]
