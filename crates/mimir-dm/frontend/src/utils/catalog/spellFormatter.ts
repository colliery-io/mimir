import type { SpellSummary } from '@/composables/catalog/useCatalog'

export function formatSpellDetails(spell: SpellSummary): string {
  let html = '<div class="spell-details">'
  
  // Header with level and school
  html += '<div class="spell-header">'
  html += `<div class="spell-level">${formatLevel(spell.level)} ${spell.school}</div>`
  if (spell.ritual) {
    html += '<div class="spell-ritual">(Ritual)</div>'
  }
  html += '</div>'
  
  // Casting details
  html += '<div class="spell-properties">'
  html += `<div><strong>Casting Time:</strong> ${spell.casting_time}</div>`
  html += `<div><strong>Range:</strong> ${spell.range}</div>`
  html += `<div><strong>Components:</strong> ${spell.components}</div>`
  html += `<div><strong>Duration:</strong> `
  if (spell.concentration) {
    html += 'Concentration'
  } else {
    html += 'Instantaneous'
  }
  html += '</div>'
  
  // Classes that can cast this spell
  if (spell.classes && spell.classes.length > 0) {
    html += `<div><strong>Classes:</strong> ${spell.classes.join(', ')}</div>`
  }
  
  // Description
  if (spell.description) {
    html += '<div class="spell-description">'
    html += `<h4>Description:</h4>`
    html += `<p>${spell.description}</p>`
    html += '</div>'
  }
  
  html += '</div>'
  
  // Source
  html += `<div class="spell-source">Source: ${spell.source}</div>`
  html += '</div>'
  
  return html
}

function formatLevel(level: number): string {
  if (level === 0) return 'Cantrip'
  if (level === 1) return '1st-level'
  if (level === 2) return '2nd-level'
  if (level === 3) return '3rd-level'
  return `${level}th-level`
}