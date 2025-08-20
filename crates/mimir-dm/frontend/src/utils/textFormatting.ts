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