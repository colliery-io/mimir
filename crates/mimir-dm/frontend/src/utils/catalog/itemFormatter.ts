import type { ItemSummary } from '@/composables/catalog/useCatalog'

export function formatItemDetails(item: ItemSummary): string {
  let html = '<div class="item-details">'
  
  // Header with item type
  html += '<div class="item-header">'
  html += `<div class="item-type">${item.typeName}</div>`
  
  // Show rarity for magic items
  if (item.rarity && item.rarity !== 'none') {
    html += `<div class="item-rarity">${formatRarity(item.rarity)}</div>`
  }
  html += '</div>'
  
  // Item properties
  html += '<div class="item-properties">'
  
  // Cost and weight
  if (item.value) {
    html += `<div><strong>Cost:</strong> ${formatCost(item.value)}</div>`
  }
  if (item.weight) {
    html += `<div><strong>Weight:</strong> ${item.weight} lb</div>`
  }
  
  // Armor class for armor
  if (item.ac) {
    html += `<div><strong>Armor Class:</strong> ${item.ac}</div>`
  }
  
  // Damage for weapons
  if (item.damage) {
    html += `<div><strong>Damage:</strong> ${item.damage}</div>`
  }
  
  // Attunement for magic items
  if (item.reqAttune) {
    html += `<div><strong>Requires Attunement:</strong> ${typeof item.reqAttune === 'string' ? item.reqAttune : 'Yes'}</div>`
  }
  
  html += '</div>'
  
  // Description
  if (item.description) {
    html += '<div class="item-description">'
    html += `<h4>Description:</h4>`
    html += `<p>${item.description}</p>`
    html += '</div>'
  }
  
  // Source
  html += `<div class="item-source">Source: ${item.source}</div>`
  html += '</div>'
  
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
  const rarityMap: Record<string, string> = {
    'common': 'Common',
    'uncommon': 'Uncommon',
    'rare': 'Rare',
    'very rare': 'Very Rare',
    'legendary': 'Legendary',
    'artifact': 'Artifact'
  }
  return rarityMap[rarity] || rarity
}