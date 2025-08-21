// Modal content formatters

import type { ReferenceData } from '../../../types/reference'
import { processFormattingTags } from '../utils/textFormatting'

export function renderModalContent(referenceData: ReferenceData): string {
  if (!referenceData || !referenceData.success || !referenceData.data) return ''
  
  const data = referenceData.data
  let html = '<div class="modal-content">'
  
  // Title
  if (data.name) {
    html += `<h3>${data.name}</h3>`
  }
  
  // Type-specific rendering
  switch (data.type) {
    case 'spell':
      html += renderSpellContent(data)
      break
    case 'item':
      html += renderItemContent(data)
      break
    case 'monster':
      html += renderMonsterContent(data)
      break
    case 'condition':
      html += renderConditionContent(data)
      break
    case 'action':
      html += renderActionContent(data)
      break
    default:
      html += renderGenericContent(data)
  }
  
  html += '</div>'
  return html
}

function renderSpellContent(data: any): string {
  let html = '<div class="spell-content">'
  
  if (data.level !== undefined) {
    const levelStr = data.level === 0 ? 'Cantrip' : `${data.level}${getOrdinalSuffix(data.level)}-level`
    html += `<p><strong>${levelStr} ${data.school || ''}</strong></p>`
  }
  
  if (data.time) {
    html += `<p><strong>Casting Time:</strong> ${data.time}</p>`
  }
  
  if (data.range) {
    html += `<p><strong>Range:</strong> ${data.range}</p>`
  }
  
  if (data.components) {
    html += `<p><strong>Components:</strong> ${data.components}</p>`
  }
  
  if (data.duration) {
    html += `<p><strong>Duration:</strong> ${data.duration}</p>`
  }
  
  if (data.entries && Array.isArray(data.entries)) {
    html += '<div class="spell-description">'
    for (const entry of data.entries) {
      if (typeof entry === 'string') {
        html += `<p>${processFormattingTags(entry)}</p>`
      }
    }
    html += '</div>'
  }
  
  html += '</div>'
  return html
}

function renderItemContent(data: any): string {
  let html = '<div class="item-content">'
  
  if (data.type) {
    html += `<p><strong>Type:</strong> ${data.type}</p>`
  }
  
  if (data.rarity && data.rarity !== 'none') {
    html += `<p><strong>Rarity:</strong> ${data.rarity}</p>`
  }
  
  if (data.value) {
    html += `<p><strong>Value:</strong> ${data.value} gp</p>`
  }
  
  if (data.weight) {
    html += `<p><strong>Weight:</strong> ${data.weight} lb.</p>`
  }
  
  if (data.entries && Array.isArray(data.entries)) {
    html += '<div class="item-description">'
    for (const entry of data.entries) {
      if (typeof entry === 'string') {
        html += `<p>${processFormattingTags(entry)}</p>`
      }
    }
    html += '</div>'
  }
  
  html += '</div>'
  return html
}

function renderMonsterContent(data: any): string {
  let html = '<div class="monster-content">'
  
  if (data.size && data.type) {
    html += `<p><em>${data.size} ${data.type}, ${data.alignment || 'unaligned'}</em></p>`
  }
  
  if (data.ac) {
    html += `<p><strong>Armor Class:</strong> ${data.ac}</p>`
  }
  
  if (data.hp) {
    html += `<p><strong>Hit Points:</strong> ${data.hp}</p>`
  }
  
  if (data.speed) {
    html += `<p><strong>Speed:</strong> ${data.speed}</p>`
  }
  
  // Ability scores
  if (data.str || data.dex || data.con || data.int || data.wis || data.cha) {
    html += '<table class="ability-scores">'
    html += '<tr>'
    html += `<th>STR</th><th>DEX</th><th>CON</th><th>INT</th><th>WIS</th><th>CHA</th>`
    html += '</tr>'
    html += '<tr>'
    html += `<td>${data.str || 10}</td>`
    html += `<td>${data.dex || 10}</td>`
    html += `<td>${data.con || 10}</td>`
    html += `<td>${data.int || 10}</td>`
    html += `<td>${data.wis || 10}</td>`
    html += `<td>${data.cha || 10}</td>`
    html += '</tr>'
    html += '</table>'
  }
  
  if (data.cr) {
    html += `<p><strong>Challenge:</strong> ${data.cr}</p>`
  }
  
  html += '</div>'
  return html
}

function renderConditionContent(data: any): string {
  let html = '<div class="condition-content">'
  
  if (data.entries && Array.isArray(data.entries)) {
    for (const entry of data.entries) {
      if (typeof entry === 'string') {
        html += `<p>${processFormattingTags(entry)}</p>`
      } else if (entry.type === 'list' && entry.items) {
        html += '<ul>'
        for (const item of entry.items) {
          html += `<li>${processFormattingTags(item)}</li>`
        }
        html += '</ul>'
      }
    }
  }
  
  html += '</div>'
  return html
}

function renderActionContent(data: any): string {
  let html = '<div class="action-content">'
  
  if (data.time) {
    html += `<p><strong>Time:</strong> ${data.time}</p>`
  }
  
  if (data.entries && Array.isArray(data.entries)) {
    for (const entry of data.entries) {
      if (typeof entry === 'string') {
        html += `<p>${processFormattingTags(entry)}</p>`
      }
    }
  }
  
  html += '</div>'
  return html
}

function renderGenericContent(data: any): string {
  let html = '<div class="generic-content">'
  
  if (data.entries && Array.isArray(data.entries)) {
    for (const entry of data.entries) {
      if (typeof entry === 'string') {
        html += `<p>${processFormattingTags(entry)}</p>`
      }
    }
  } else if (data.text) {
    html += `<p>${processFormattingTags(data.text)}</p>`
  } else if (data.description) {
    html += `<p>${processFormattingTags(data.description)}</p>`
  }
  
  html += '</div>'
  return html
}

function getOrdinalSuffix(num: number): string {
  const j = num % 10
  const k = num % 100
  
  if (j === 1 && k !== 11) {
    return 'st'
  }
  if (j === 2 && k !== 12) {
    return 'nd'
  }
  if (j === 3 && k !== 13) {
    return 'rd'
  }
  return 'th'
}