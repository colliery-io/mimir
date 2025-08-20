import { processFormattingTags } from '../textFormatting'

interface ItemDetails {
  name: string
  type: string
  typeName?: string
  rarity?: string
  value?: number
  weight?: number
  ac?: number
  strength?: number
  stealth?: boolean
  dmg1?: string
  dmg2?: string
  dmgType?: string
  property?: string[]
  range?: string
  reqAttune?: boolean
  source: string
  page?: number
  entries?: (string | object)[]
  additionalEntries?: (string | object)[]
  modifySpeed?: object
  resist?: string[]
  immune?: string[]
  conditionImmune?: string[]
  bonusSpellAttack?: number
  bonusWeaponAttack?: number
  bonusAc?: number
  grantsProficiency?: boolean
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
  const typeRarity = `${item.typeName || item.type || 'Item'}`
  html += `<div class="item-type-rarity">${typeRarity}`
  if (item.rarity && item.rarity !== 'none') {
    html += ` (${formatRarity(item.rarity)})`
  }
  html += '</div>'
  
  const tags = []
  if (item.reqAttune) tags.push('<span class="item-tag attunement">Requires Attunement</span>')
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
    html += `<div class="property-item">
      <span class="property-label">Weight</span>
      <span class="property-value">${item.weight} lb</span>
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