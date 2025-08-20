import { processFormattingTags } from '../textFormatting'

interface ItemDetails {
  name: string
  type: string
  typeName?: string
  rarity?: string
  value?: number
  weight?: number
  weightNote?: string
  ac?: number
  strength?: string
  stealth?: boolean
  dmg1?: string
  dmg2?: string
  dmgType?: string
  property?: string[]
  range?: string
  reqAttune?: boolean | string
  reqAttuneTags?: any[]
  source: string
  page?: number
  entries?: (string | object)[]
  additionalEntries?: (string | object)[]
  modifySpeed?: object
  resist?: string[]
  immune?: string[]
  conditionImmune?: string[]
  bonusSpellAttack?: string
  bonusSpellSaveDc?: string
  bonusWeaponAttack?: number
  bonusAc?: number
  grantsProficiency?: boolean
  // Equipment-specific fields
  weapon?: boolean
  weaponCategory?: string
  armor?: boolean
  ammoType?: string
  scfType?: string
  group?: string[]
  light?: any[]
  containerCapacity?: any
  carryingCapacity?: number
  speed?: number
  packContents?: any[]
  miscTags?: string[]
  tier?: string
  lootTables?: string[]
  // Tool categories
  hasFluffImages?: boolean
  poison?: boolean
}

export function formatItemDetails(item: any): string {
  // Handle both summary and full details
  const isFullDetails = item.entries !== undefined || item.additionalEntries !== undefined
  
  if (!isFullDetails) {
    return formatItemSummary(item)
  }
  
  return formatFullItemDetails(item as ItemDetails)
}

function formatItemSummary(item: any): string {
  let html = '<div class="item-details">'
  
  // Header with type and rarity
  html += '<div class="item-header-section">'
  html += `<div class="item-type-rarity">${item.typeName || item.type || 'Item'}`
  if (item.rarity && item.rarity !== 'none') {
    html += ` (${formatRarity(item.rarity)})`
  }
  html += '</div>'
  
  if (item.reqAttune) {
    html += '<div class="item-tag attunement">Requires Attunement</div>'
  }
  html += '</div>'
  
  // Properties section
  html += '<div class="item-properties-grid">'
  
  if (item.value) {
    html += `<div class="property-item">
      <span class="property-label">Cost:</span>
      <span class="property-value">${formatCost(item.value)}</span>
    </div>`
  }
  
  if (item.weight) {
    html += `<div class="property-item">
      <span class="property-label">Weight:</span>
      <span class="property-value">${item.weight} lb</span>
    </div>`
  }
  
  if (item.ac !== undefined) {
    html += `<div class="property-item">
      <span class="property-label">AC:</span>
      <span class="property-value">${item.ac}</span>
    </div>`
  }
  
  if (item.dmg1) {
    html += `<div class="property-item">
      <span class="property-label">Damage:</span>
      <span class="property-value">${item.dmg1} ${item.dmgType || ''}</span>
    </div>`
  }
  
  html += '</div>'
  
  // Basic description if available
  if (item.description) {
    html += '<div class="item-description-section">'
    html += '<h4>Description</h4>'
    html += `<div class="description-text">${processFormattingTags(item.description)}</div>`
    html += '</div>'
  }
  
  // Footer
  html += `<div class="item-footer">
    <span class="source-info">Source: ${item.source}</span>
  </div>`
  html += '</div>'
  
  return html
}

function formatFullItemDetails(item: ItemDetails): string {
  let html = '<div class="item-details enhanced">'
  
  // Header section
  html += '<div class="item-header-section">'
  const typeInfo = formatItemTypeInfo(item)
  html += `<div class="item-type-rarity">${typeInfo}`
  if (item.rarity && item.rarity !== 'none') {
    html += ` (${formatRarity(item.rarity)})`
  }
  html += '</div>'
  
  const tags = []
  if (item.reqAttune) {
    const attunementText = typeof item.reqAttune === 'string' 
      ? `Requires Attunement (${item.reqAttune})`
      : 'Requires Attunement'
    tags.push(`<span class="item-tag attunement">${attunementText}</span>`)
  }
  if (item.tier) {
    tags.push(`<span class="item-tag tier">${formatTier(item.tier)}</span>`)
  }
  if (item.miscTags) {
    item.miscTags.forEach((tag: string) => {
      tags.push(`<span class="item-tag misc">${formatMiscTag(tag)}</span>`)
    })
  }
  if (tags.length > 0) {
    html += `<div class="item-tags">${tags.join(' ')}</div>`
  }
  html += '</div>'
  
  // Properties grid
  html += '<div class="item-properties-grid">'
  
  // Basic properties
  if (item.value) {
    html += `<div class="property-item">
      <span class="property-label">Cost</span>
      <span class="property-value">${formatCost(item.value)}</span>
    </div>`
  }
  
  if (item.weight) {
    const weightText = item.weightNote ? `${item.weight} lb ${item.weightNote}` : `${item.weight} lb`
    html += `<div class="property-item">
      <span class="property-label">Weight</span>
      <span class="property-value">${weightText}</span>
    </div>`
  }
  
  // Armor properties
  if (item.ac !== undefined) {
    html += `<div class="property-item">
      <span class="property-label">Armor Class</span>
      <span class="property-value">${item.ac}</span>
    </div>`
  }
  
  if (item.strength) {
    html += `<div class="property-item">
      <span class="property-label">Strength Req.</span>
      <span class="property-value">${item.strength}</span>
    </div>`
  }
  
  if (item.stealth === true) {
    html += `<div class="property-item">
      <span class="property-label">Stealth</span>
      <span class="property-value">Disadvantage</span>
    </div>`
  }
  
  // Weapon properties
  if (item.dmg1) {
    const damageType = formatDamageType(item.dmgType)
    html += `<div class="property-item">
      <span class="property-label">Damage</span>
      <span class="property-value">
        <span class="damage-dice">${item.dmg1}</span>
        ${damageType ? `<span class="damage-type">${damageType}</span>` : ''}
      </span>
    </div>`
  }
  
  if (item.dmg2) {
    const damageType = formatDamageType(item.dmgType)
    html += `<div class="property-item">
      <span class="property-label">Versatile</span>
      <span class="property-value">
        <span class="damage-dice">${item.dmg2}</span>
        ${damageType ? `<span class="damage-type">${damageType}</span>` : ''}
      </span>
    </div>`
  }
  
  if (item.range) {
    html += `<div class="property-item">
      <span class="property-label">Range</span>
      <span class="property-value">${item.range}</span>
    </div>`
  }
  
  if (item.property && item.property.length > 0) {
    html += `<div class="property-item full-width">
      <span class="property-label">Properties</span>
      <span class="property-value">${formatWeaponProperties(item.property)}</span>
    </div>`
  }
  
  html += '</div>'
  
  // Combat mechanics section for weapons
  if (item.weapon || item.dmg1 || item.property) {
    html += formatWeaponCombatSection(item)
  }
  
  // Magic item properties section
  if (item.bonusSpellAttack || item.bonusSpellSaveDc || item.bonusAc || item.lootTables) {
    html += formatMagicItemSection(item)
  }
  
  // Container/transport section
  if (item.containerCapacity || item.carryingCapacity || item.speed || item.packContents) {
    html += formatContainerSection(item)
  }
  
  // Light source section
  if (item.light) {
    html += formatLightSection(item)
  }
  
  // Main description
  if (item.entries && item.entries.length > 0) {
    html += '<div class="item-description-section">'
    html += '<h4>Description</h4>'
    html += '<div class="description-text">'
    for (const entry of item.entries) {
      if (typeof entry === 'string') {
        html += `<p>${processFormattingTags(entry)}</p>`
      } else if (typeof entry === 'object' && entry !== null) {
        const entryObj = entry as any
        if (entryObj.type === 'list') {
          html += formatList(entryObj)
        } else if (entryObj.type === 'entries' && entryObj.entries) {
          if (entryObj.name) {
            html += `<h5>${entryObj.name}</h5>`
          }
          for (const subEntry of entryObj.entries) {
            if (typeof subEntry === 'string') {
              html += `<p>${processFormattingTags(subEntry)}</p>`
            }
          }
        }
      }
    }
    html += '</div>'
    html += '</div>'
  }
  
  // Additional entries
  if (item.additionalEntries && item.additionalEntries.length > 0) {
    html += '<div class="item-additional-section">'
    html += '<div class="description-text">'
    for (const entry of item.additionalEntries) {
      if (typeof entry === 'string') {
        html += `<p>${processFormattingTags(entry)}</p>`
      }
    }
    html += '</div>'
    html += '</div>'
  }
  
  // Footer
  html += '<div class="item-footer">'
  html += `<span class="source-info">Source: ${item.source}`
  if (item.page) html += `, p. ${item.page}`
  html += '</span>'
  html += '</div>'
  
  html += '</div>'
  
  return html
}

function formatList(listObj: any): string {
  let html = '<ul class="item-list">'
  if (listObj.items) {
    for (const item of listObj.items) {
      if (typeof item === 'string') {
        html += `<li>${processFormattingTags(item)}</li>`
      } else if (typeof item === 'object' && item.name) {
        html += `<li><strong>${item.name}:</strong> ${processFormattingTags(item.entry || '')}</li>`
      }
    }
  }
  html += '</ul>'
  return html
}

function formatCost(value: number): string {
  if (value >= 100) {
    return `${value / 100} gp`
  } else if (value >= 10) {
    return `${value / 10} sp`
  } else {
    return `${value} cp`
  }
}

function formatRarity(rarity: string): string {
  return rarity.charAt(0).toUpperCase() + rarity.slice(1)
}

function formatDamageType(damageType?: string): string {
  if (!damageType) return ''
  
  const damageTypeMap: Record<string, string> = {
    'A': 'acid',
    'B': 'bludgeoning', 
    'C': 'cold',
    'F': 'fire',
    'O': 'force',
    'L': 'lightning',
    'N': 'necrotic',
    'P': 'piercing',
    'I': 'poison',
    'Y': 'psychic',
    'R': 'radiant',
    'S': 'slashing',
    'T': 'thunder'
  }
  
  return damageTypeMap[damageType] || damageType.toLowerCase()
}

function formatWeaponProperties(properties: string[]): string {
  const propertyDescriptions: Record<string, string> = {
    'A': 'Ammunition',
    'F': 'Finesse',
    'H': 'Heavy',
    'L': 'Light',
    'LD': 'Loading',
    'R': 'Reach',
    'RLD': 'Reload',
    'S': 'Special',
    'T': 'Thrown',
    'TH': 'Two-Handed',
    'V': 'Versatile',
    '2H': 'Two-Handed',
    'AF': 'Ammunition, Finesse',
    'RN': 'Range',
    'BF': 'Burst Fire',
    'REL': 'Reload'
  }
  
  return properties.map(prop => {
    // Handle range properties like "RN|20/60"
    if (prop.startsWith('RN|')) {
      const range = prop.split('|')[1]
      return `<span class="weapon-property range">Range (${range} ft.)</span>`
    }
    
    // Handle thrown properties like "T|20/60"  
    if (prop.startsWith('T|')) {
      const range = prop.split('|')[1]
      return `<span class="weapon-property thrown">Thrown (range ${range} ft.)</span>`
    }
    
    // Handle versatile properties like "V|1d8"
    if (prop.startsWith('V|')) {
      const damage = prop.split('|')[1]
      return `<span class="weapon-property versatile">Versatile (${damage})</span>`
    }
    
    // Handle ammunition with damage like "AF|DMG"
    if (prop.startsWith('AF|')) {
      const details = prop.split('|')[1]
      if (details === 'DMG') {
        return `<span class="weapon-property ammunition">Ammunition (deals damage)</span>`
      }
      return `<span class="weapon-property ammunition">Ammunition, Finesse (${details})</span>`
    }
    
    // Handle burst fire with damage like "BF|DMG"
    if (prop.startsWith('BF|')) {
      const details = prop.split('|')[1]
      if (details === 'DMG') {
        return `<span class="weapon-property burstfire">Burst Fire (extra damage)</span>`
      }
      return `<span class="weapon-property burstfire">Burst Fire (${details})</span>`
    }
    
    // Handle reload properties like "RLD|6"
    if (prop.startsWith('RLD|')) {
      const shots = prop.split('|')[1]
      return `<span class="weapon-property reload">Reload (${shots} shots)</span>`
    }
    
    // Handle loading properties with details
    if (prop.startsWith('LD|')) {
      const details = prop.split('|')[1]
      return `<span class="weapon-property loading">Loading (${details})</span>`
    }
    
    // Standard properties
    const description = propertyDescriptions[prop] || prop
    const className = prop.toLowerCase().replace(/[^a-z]/g, '')
    return `<span class="weapon-property ${className}">${description}</span>`
  }).join(' ')
}

function formatItemTypeInfo(item: ItemDetails): string {
  if (item.weapon && item.weaponCategory) {
    return `${formatWeaponCategory(item.weaponCategory)} Weapon`
  }
  if (item.armor) {
    return 'Armor'
  }
  if (item.scfType) {
    return `${formatSpellcastingFocus(item.scfType)} Focus`
  }
  return item.typeName || formatItemType(item.type) || 'Item'
}

function formatWeaponCategory(category: string): string {
  return category.charAt(0).toUpperCase() + category.slice(1)
}

function formatSpellcastingFocus(focusType: string): string {
  const focusMap: Record<string, string> = {
    'holy': 'Holy Symbol',
    'druidic': 'Druidic Focus',
    'arcane': 'Arcane Focus'
  }
  return focusMap[focusType] || focusType
}

function formatItemType(type: string): string {
  const typeMap: Record<string, string> = {
    'G': 'Adventuring Gear',
    'M': 'Melee Weapon',
    'R': 'Ranged Weapon',
    'A': 'Ammunition',
    'LA': 'Light Armor',
    'MA': 'Medium Armor', 
    'HA': 'Heavy Armor',
    'S': 'Shield',
    'AT': "Artisan's Tools",
    'TG': 'Tool',
    'INS': 'Musical Instrument',
    'T': "Thieves' Tools",
    'TAH': 'Trade Goods',
    'FD': 'Food & Drink',
    'SCF': 'Spellcasting Focus',
    'GS': 'Gaming Set',
    'MNT': 'Mount',
    'VEH': 'Vehicle',
    '$C': 'Currency'
  }
  return typeMap[type] || type
}

function formatTier(tier: string): string {
  return tier.charAt(0).toUpperCase() + tier.slice(1)
}

function formatMiscTag(tag: string): string {
  const tagMap: Record<string, string> = {
    'CNS': 'Consumable'
  }
  return tagMap[tag] || tag
}

function formatWeaponCombatSection(item: ItemDetails): string {
  let html = '<div class="item-combat-section">'
  html += '<h4>Combat Properties</h4>'
  html += '<div class="combat-properties-grid">'
  
  if (item.weaponCategory) {
    html += `<div class="combat-item">
      <span class="combat-label">Category</span>
      <span class="combat-value">${formatWeaponCategory(item.weaponCategory)}</span>
    </div>`
  }
  
  if (item.dmg1 && item.dmgType) {
    html += `<div class="combat-item">
      <span class="combat-label">Damage</span>
      <span class="combat-value">
        <span class="damage-dice">${item.dmg1}</span>
        <span class="damage-type ${item.dmgType.toLowerCase()}">${formatDamageType(item.dmgType)}</span>
      </span>
    </div>`
  }
  
  if (item.dmg2) {
    html += `<div class="combat-item">
      <span class="combat-label">Versatile</span>
      <span class="combat-value">
        <span class="damage-dice">${item.dmg2}</span>
        <span class="damage-type ${item.dmgType?.toLowerCase() || ''}">${formatDamageType(item.dmgType)}</span>
      </span>
    </div>`
  }
  
  if (item.range) {
    html += `<div class="combat-item">
      <span class="combat-label">Range</span>
      <span class="combat-value">${item.range} ft.</span>
    </div>`
  }
  
  if (item.ammoType) {
    html += `<div class="combat-item">
      <span class="combat-label">Ammunition</span>
      <span class="combat-value">${formatAmmoType(item.ammoType)}</span>
    </div>`
  }
  
  html += '</div>'
  html += '</div>'
  return html
}

function formatMagicItemSection(item: ItemDetails): string {
  let html = '<div class="item-magic-section">'
  html += '<h4>Magical Properties</h4>'
  html += '<div class="magic-properties-grid">'
  
  if (item.bonusSpellAttack) {
    html += `<div class="magic-item">
      <span class="magic-label">Spell Attack</span>
      <span class="magic-value">${item.bonusSpellAttack} bonus</span>
    </div>`
  }
  
  if (item.bonusSpellSaveDc) {
    html += `<div class="magic-item">
      <span class="magic-label">Spell Save DC</span>
      <span class="magic-value">${item.bonusSpellSaveDc} bonus</span>
    </div>`
  }
  
  if (item.bonusAc) {
    html += `<div class="magic-item">
      <span class="magic-label">AC Bonus</span>
      <span class="magic-value">+${item.bonusAc}</span>
    </div>`
  }
  
  if (item.lootTables) {
    html += `<div class="magic-item full-width">
      <span class="magic-label">Loot Tables</span>
      <span class="magic-value">${item.lootTables.join(', ')}</span>
    </div>`
  }
  
  html += '</div>'
  html += '</div>'
  return html
}

function formatContainerSection(item: ItemDetails): string {
  let html = '<div class="item-container-section">'
  html += '<h4>Capacity & Transport</h4>'
  html += '<div class="container-properties-grid">'
  
  if (item.containerCapacity) {
    if (item.containerCapacity.weight) {
      html += `<div class="container-item">
        <span class="container-label">Capacity</span>
        <span class="container-value">${item.containerCapacity.weight[0]} lb</span>
      </div>`
    }
  }
  
  if (item.carryingCapacity) {
    html += `<div class="container-item">
      <span class="container-label">Carrying Capacity</span>
      <span class="container-value">${item.carryingCapacity} lb</span>
    </div>`
  }
  
  if (item.speed) {
    html += `<div class="container-item">
      <span class="container-label">Speed</span>
      <span class="container-value">${item.speed} ft.</span>
    </div>`
  }
  
  if (item.packContents && item.packContents.length > 0) {
    html += `<div class="container-item full-width">
      <span class="container-label">Contents</span>
      <div class="container-value">
        ${formatPackContents(item.packContents)}
      </div>
    </div>`
  }
  
  html += '</div>'
  html += '</div>'
  return html
}

function formatLightSection(item: ItemDetails): string {
  let html = '<div class="item-light-section">'
  html += '<h4>Light Properties</h4>'
  html += '<div class="light-properties-grid">'
  
  item.light?.forEach((lightSource: any) => {
    if (lightSource.bright) {
      html += `<div class="light-item">
        <span class="light-label">Bright Light</span>
        <span class="light-value">${lightSource.bright} ft.</span>
      </div>`
    }
    
    if (lightSource.dim) {
      html += `<div class="light-item">
        <span class="light-label">Dim Light</span>
        <span class="light-value">${lightSource.dim} ft.</span>
      </div>`
    }
    
    if (lightSource.shape) {
      html += `<div class="light-item">
        <span class="light-label">Shape</span>
        <span class="light-value">${lightSource.shape}</span>
      </div>`
    }
  })
  
  html += '</div>'
  html += '</div>'
  return html
}

function formatPackContents(contents: any[]): string {
  let html = '<ul class="pack-contents-list">'
  
  contents.forEach((item: any) => {
    if (typeof item === 'string') {
      html += `<li>${processFormattingTags(item)}</li>`
    } else if (item.item) {
      const quantity = item.quantity ? `${item.quantity}× ` : ''
      html += `<li>${quantity}${processFormattingTags(item.item)}</li>`
    } else if (item.special) {
      html += `<li>${processFormattingTags(item.special)}</li>`
    }
  })
  
  html += '</ul>'
  return html
}

function formatAmmoType(ammoType: string): string {
  const cleanType = ammoType.replace(/\|.*$/, '')
  return cleanType.split(' ').map(word => 
    word.charAt(0).toUpperCase() + word.slice(1)
  ).join(' ')
}