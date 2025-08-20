/**
 * Process D&D 5e tools formatting tags and convert them to HTML
 */
export function processFormattingTags(text: string): string {
  if (!text) return ''
  
  let processed = text
  
  // Basic formatting
  processed = processed
    // Bold
    .replace(/{@b ([^}]+)}/g, '<strong>$1</strong>')
    // Italic
    .replace(/{@i ([^}]+)}/g, '<em>$1</em>')
    // Bold-Italic
    .replace(/{@bi ([^}]+)}/g, '<strong><em>$1</em></strong>')
    
  // Dice rolls
  processed = processed
    .replace(/{@dice ([^}]+)}/g, '<span class="dice-roll">$1</span>')
    .replace(/{@damage ([^}]+)}/g, '<span class="damage-roll">$1</span>')
    .replace(/{@d20 ([^}]+)}/g, '<span class="d20-roll">d20$1</span>')
    .replace(/{@hit ([^}]+)}/g, '<span class="hit-bonus">$1</span>')
    
  // Conditions and status
  processed = processed
    .replace(/{@condition ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="condition">$1</span>')
    .replace(/{@status ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="status">$1</span>')
    
  // Spells
  processed = processed
    .replace(/{@spell ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="spell-ref">$1</span>')
    
  // Items
  processed = processed
    .replace(/{@item ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="item-ref">$1</span>')
    
  // Item entry references - these reference the description of another item
  // Special handling for known item groups
  processed = processed
    .replace(/{#itemEntry Armor of Resistance(?:\|[^}]*)?}/gi, 
      '<span class="item-description">You have resistance to one type of damage while you wear this armor.</span>')
    .replace(/{#itemEntry Potion of Resistance(?:\|[^}]*)?}/gi,
      '<span class="item-description">When you drink this potion, you gain resistance to one type of damage for 1 hour.</span>')
    .replace(/{#itemEntry Grenade(?:\|[^}]*)?}/gi,
      '<span class="item-description">As an action, a character can throw a grenade at a point up to 60 feet away. Each creature within 20 feet of an exploding fragmentation grenade must make a DC 15 Dexterity saving throw, taking 5d6 piercing damage on a failed save, or half as much damage on a successful one.</span>')
    // Generic fallback for other item entry references
    .replace(/{#itemEntry ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="item-entry-ref">[See base item: $1]</span>')
    
  // Creatures
  processed = processed
    .replace(/{@creature ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="creature-ref">$1</span>')
    
  // Classes and features
  processed = processed
    .replace(/{@class ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="class-ref">$1</span>')
    .replace(/{@classFeature ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="feature-ref">$1</span>')
    
  // Skills and abilities
  processed = processed
    .replace(/{@skill ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="skill">$1</span>')
    .replace(/{@sense ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="sense">$1</span>')
    
  // Actions
  processed = processed
    .replace(/{@action ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="action">$1</span>')
    
  // Attack types
  processed = processed
    .replace(/{@atk mw}/gi, '<em>Melee Weapon Attack:</em>')
    .replace(/{@atk rw}/gi, '<em>Ranged Weapon Attack:</em>')
    .replace(/{@atk mw,rw}/gi, '<em>Melee or Ranged Weapon Attack:</em>')
    .replace(/{@atk ms}/gi, '<em>Melee Spell Attack:</em>')
    .replace(/{@atk rs}/gi, '<em>Ranged Spell Attack:</em>')
    
  // Hit bonus (the {@h} tag)
  processed = processed
    .replace(/{@h}/gi, '<em>Hit:</em>')
    
  // DC checks
  processed = processed
    .replace(/{@dc (\d+)(?:\|([^}]+))?}/gi, (match, dc, ability) => {
      return `<span class="dc-check">DC ${dc}${ability ? ' ' + ability : ''}</span>`
    })
    
  // Filters (complex references)
  processed = processed
    .replace(/{@filter ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="filter-ref">$1</span>')
    
  // Books and sources
  processed = processed
    .replace(/{@book ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="book-ref">$1</span>')
    
  // Chance
  processed = processed
    .replace(/{@chance (\d+)(?:\|([^}]+))?}/gi, (match, num, text) => {
      return `<span class="chance">${text || num + '% chance'}</span>`
    })
    
  // Recharge
  processed = processed
    .replace(/{@recharge\s*(\d+)?}/gi, (match, num) => {
      return `<span class="recharge">(Recharge${num ? ' ' + num + '–6' : ''})</span>`
    })
    
  // Note blocks
  processed = processed
    .replace(/{@note ([^}]+)}/gi, '<span class="note">Note: $1</span>')
    
  // Catch-all for any remaining tags we haven't handled
  processed = processed
    .replace(/{@\w+ ([^|}]+)(?:\|[^}]*)?}/gi, '<span class="tagged">$1</span>')
  
  return processed
}

/**
 * Format 5etools entries (nested content structures)
 */
export function formatEntries(entries: any[]): string {
  if (!entries || !Array.isArray(entries)) return ''
  
  let html = ''
  for (const entry of entries) {
    if (typeof entry === 'string') {
      html += `<p>${processFormattingTags(entry)}</p>`
    } else if (entry && typeof entry === 'object') {
      if (entry.type === 'entries') {
        if (entry.name) {
          html += `<h5>${entry.name}</h5>`
        }
        if (entry.entries) {
          html += formatEntries(entry.entries)
        }
      } else if (entry.type === 'list') {
        html += '<ul>'
        if (entry.items) {
          for (const item of entry.items) {
            if (typeof item === 'string') {
              html += `<li>${processFormattingTags(item)}</li>`
            } else {
              html += `<li>${formatEntries([item])}</li>`
            }
          }
        }
        html += '</ul>'
      } else if (entry.type === 'table') {
        html += formatTable(entry)
      } else if (entry.type === 'inset' || entry.type === 'insetReadaloud') {
        // In modals, treat all insets as read-aloud for consistent styling
        html += `<div class="inset-readaloud">`
        if (entry.name) {
          html += `<h5>${entry.name}</h5>`
        }
        if (entry.entries) {
          html += formatEntries(entry.entries)
        }
        html += '</div>'
      } else if (entry.entries) {
        html += formatEntries(entry.entries)
      } else if (entry.text) {
        html += `<p>${processFormattingTags(entry.text)}</p>`
      }
    }
  }
  return html
}

function formatTable(table: any): string {
  let html = '<table class="entry-table">'
  
  // Table caption
  if (table.caption) {
    html += `<caption>${table.caption}</caption>`
  }
  
  // Table headers
  if (table.colLabels) {
    html += '<thead><tr>'
    for (const label of table.colLabels) {
      html += `<th>${label}</th>`
    }
    html += '</tr></thead>'
  }
  
  // Table rows
  if (table.rows) {
    html += '<tbody>'
    for (const row of table.rows) {
      html += '<tr>'
      if (Array.isArray(row)) {
        for (const cell of row) {
          html += `<td>${typeof cell === 'string' ? processFormattingTags(cell) : formatEntries([cell])}</td>`
        }
      } else {
        html += `<td>${processFormattingTags(row)}</td>`
      }
      html += '</tr>'
    }
    html += '</tbody>'
  }
  
  html += '</table>'
  return html
}