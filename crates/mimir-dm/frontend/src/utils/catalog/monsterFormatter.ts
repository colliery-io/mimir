import type { MonsterSummary } from '@/composables/catalog/useCatalog'

export function formatMonsterDetails(monster: MonsterSummary): string {
  let html = '<div class="monster-details">'
  
  // Header with type and CR
  html += '<div class="monster-header">'
  html += `<div class="monster-type">${formatSize(monster.size)} ${monster.creature_type}, ${monster.alignment}</div>`
  html += `<div class="monster-cr">Challenge ${monster.cr}</div>`
  html += '</div>'
  
  // Core stats
  html += '<div class="creature-stats">'
  html += '<div class="stat">'
  html += '<div class="stat-label">AC</div>'
  html += `<div class="stat-value">${monster.ac}</div>`
  html += '</div>'
  html += '<div class="stat">'
  html += '<div class="stat-label">HP</div>'
  html += `<div class="stat-value">${monster.hp}</div>`
  html += '</div>'
  html += '<div class="stat">'
  html += '<div class="stat-label">CR</div>'
  html += `<div class="stat-value">${monster.cr}</div>`
  html += '</div>'
  html += '</div>'
  
  // Environment
  if (monster.environment && monster.environment.length > 0) {
    html += '<div class="monster-environment">'
    html += `<h4>Environment:</h4>`
    html += `<p>${monster.environment.join(', ')}</p>`
    html += '</div>'
  }
  
  // Source
  html += `<div class="monster-source">Source: ${monster.source}</div>`
  html += '</div>'
  
  return html
}

function formatSize(size: string): string {
  const sizeMap: Record<string, string> = {
    'T': 'Tiny',
    'S': 'Small',
    'M': 'Medium',
    'L': 'Large',
    'H': 'Huge',
    'G': 'Gargantuan'
  }
  return sizeMap[size] || size
}