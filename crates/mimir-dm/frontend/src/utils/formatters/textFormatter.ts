// Text formatting utilities for 5etools content

import { ATTACK_TYPES } from '../../types/content'

/**
 * Process 5etools formatting tags in text
 * Converts tags like {@b bold}, {@spell fireball}, etc. to HTML
 */
export function processFormatting(text: string): string {
  if (!text) return ''
  
  let processed = text
  
  // Basic text formatting
  processed = processBasicFormatting(processed)
  
  // Game mechanics formatting
  processed = processGameMechanics(processed)
  
  // Cross-reference formatting
  processed = processCrossReferences(processed)
  
  // Other special formatting
  processed = processSpecialFormatting(processed)
  
  return processed
}

/**
 * Process basic text formatting tags
 */
function processBasicFormatting(text: string): string {
  let processed = text
  
  // Bold text {@b text} or {@bold text}
  processed = processed.replace(/{@(?:b|bold)\s+([^}]+)}/g, '<strong>$1</strong>')
  
  // Italic text {@i text} or {@italic text}
  processed = processed.replace(/{@(?:i|italic)\s+([^}]+)}/g, '<em>$1</em>')
  
  // Strike-through {@s text} or {@strike text}
  processed = processed.replace(/{@(?:s|strike)\s+([^}]+)}/g, '<s>$1</s>')
  
  // Underline {@u text} or {@underline text}
  processed = processed.replace(/{@(?:u|underline)\s+([^}]+)}/g, '<u>$1</u>')
  
  // Code {@code text}
  processed = processed.replace(/{@code\s+([^}]+)}/g, '<code>$1</code>')
  
  // Note {@note text}
  processed = processed.replace(/{@note\s+([^}]+)}/g, '<span class="note">$1</span>')
  
  return processed
}

/**
 * Process game mechanics formatting
 */
function processGameMechanics(text: string): string {
  let processed = text
  
  // Dice rolls {@dice 1d20+5}
  processed = processed.replace(/{@dice\s+([^}]+)}/g, '<span class="dice-roll">$1</span>')
  
  // Damage rolls {@damage 2d6}
  processed = processed.replace(/{@damage\s+([^}]+)}/g, '<span class="damage-roll">$1</span>')
  
  // d20 rolls {@d20 15}
  processed = processed.replace(/{@d20\s+(\d+)}/g, '<span class="d20-check">d20 ≥ $1</span>')
  
  // DC checks {@dc 15}
  processed = processed.replace(/{@dc\s+(\d+)}/g, '<span class="dc-check">DC $1</span>')
  
  // Scaled damage {@scaledamage 2d6|1-9|1d6}
  processed = processed.replace(/{@scaledamage\s+([^|}\s]+)(?:\|([^|}]+))?(?:\|([^}]+))?}/g, 
    (match, baseDamage) => `<span class="scaled-value">${baseDamage}</span>`)
  
  // Scaled dice {@scaledice 1d6|5-9|1d6}
  processed = processed.replace(/{@scaledice\s+([^|}\s]+)(?:\|([^|}]+))?(?:\|([^}]+))?}/g, 
    (match, baseDice) => `<span class="scaled-value">${baseDice}</span>`)
  
  // Skill checks {@skill Athletics}
  processed = processed.replace(/{@skill\s+([^}]+)}/g, '<span class="skill-check">$1</span>')
  
  // Actions {@action Attack}
  processed = processed.replace(/{@action\s+([^}]+)}/g, '<span class="action-name">$1</span>')
  
  // Conditions {@condition poisoned}
  processed = processed.replace(/{@condition\s+([^}]+)}/g, 
    (match, condition) => `<span class="condition" title="${condition}">${condition}</span>`)
  
  // Status {@status prone}
  processed = processed.replace(/{@status\s+([^}]+)}/g, 
    (match, status) => `<span class="status" title="${status}">${status}</span>`)
  
  // Hit bonus {@hit +5} or {@hit 5}
  processed = processed.replace(/{@hit\s+([+-]?\d+)}/g, (match, bonus) => {
    const formattedBonus = bonus.startsWith('+') || bonus.startsWith('-') ? bonus : `+${bonus}`
    return `<span class="hit-bonus">${formattedBonus}</span>`
  })
  
  // Attack type {@atk mw}
  processed = processed.replace(/{@atk\s+([^}]+)}/g, (match, type) => {
    return `<em>${ATTACK_TYPES[type] || type}:</em>`
  })
  
  // Hit indicator {@h}
  processed = processed.replace(/{@h}/g, '<em>Hit:</em>')
  
  // Recharge {@recharge} or {@recharge 5}
  processed = processed.replace(/{@recharge\s*(\d+)?}/g, (match, num) => {
    return num ? `<span class="recharge">Recharge ${num}-6</span>` : '<span class="recharge">Recharge</span>'
  })
  
  return processed
}

/**
 * Process cross-reference formatting
 */
function processCrossReferences(text: string): string {
  let processed = text
  
  // Spells {@spell fireball} or {@spell fireball|phb|PHB}
  processed = processed.replace(/{@spell\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, 
    (match, spell, source, display) => {
      const spellName = display || spell
      const title = source ? `${spell} (${source})` : spell
      const dataAttrs = `data-ref-type="spell" data-ref-name="${spell}" data-ref-source="${source || ''}"` 
      return `<span class="spell-ref cross-ref-link" title="${title}" ${dataAttrs}>${spellName}</span>`
    })
  
  // Items {@item longsword}
  processed = processed.replace(/{@item\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, 
    (match, item, source, display) => {
      const itemName = display || item
      const title = source ? `${item} (${source})` : item
      const dataAttrs = `data-ref-type="item" data-ref-name="${item}" data-ref-source="${source || ''}"` 
      return `<span class="item-ref cross-ref-link" title="${title}" ${dataAttrs}>${itemName}</span>`
    })
  
  // Creatures {@creature goblin}
  processed = processed.replace(/{@creature\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, 
    (match, creature, source, display) => {
      const creatureName = display || creature
      const title = source ? `${creature} (${source})` : creature
      const dataAttrs = `data-ref-type="creature" data-ref-name="${creature}" data-ref-source="${source || ''}"` 
      return `<span class="creature-ref cross-ref-link" title="${title}" ${dataAttrs}>${creatureName}</span>`
    })
  
  // Races {@race dragonborn}
  processed = processed.replace(/{@race\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, 
    (match, race, source, display) => {
      const raceName = display || race
      const title = source ? `${race} (${source})` : race
      const dataAttrs = `data-ref-type="race" data-ref-name="${race}" data-ref-source="${source || ''}"` 
      return `<span class="race-ref cross-ref-link" title="${title}" ${dataAttrs}>${raceName}</span>`
    })
  
  // Classes {@class fighter}
  processed = processed.replace(/{@class\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}|]+))?(?:\|([^}]+))?}/g, 
    (match, className, source, subclass, display) => {
      const displayName = display || className
      const title = subclass ? `${className} (${subclass})` : className
      const dataAttrs = `data-ref-type="class" data-ref-name="${className}" data-ref-source="${source || ''}" data-ref-subclass="${subclass || ''}"` 
      return `<span class="class-ref cross-ref-link" title="${title}" ${dataAttrs}>${displayName}</span>`
    })
  
  // Backgrounds {@background soldier}
  processed = processed.replace(/{@background\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, 
    (match, background, source, display) => {
      const backgroundName = display || background
      const title = source ? `${background} (${source})` : background
      const dataAttrs = `data-ref-type="background" data-ref-name="${background}" data-ref-source="${source || ''}"` 
      return `<span class="background-ref cross-ref-link" title="${title}" ${dataAttrs}>${backgroundName}</span>`
    })
  
  // Feats {@feat alert}
  processed = processed.replace(/{@feat\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}]+))?}/g, 
    (match, feat, source, display) => {
      const featName = display || feat
      const title = source ? `${feat} (${source})` : feat
      const dataAttrs = `data-ref-type="feat" data-ref-name="${feat}" data-ref-source="${source || ''}"` 
      return `<span class="feat-ref cross-ref-link" title="${title}" ${dataAttrs}>${featName}</span>`
    })
  
  return processed
}

/**
 * Process special formatting
 */
function processSpecialFormatting(text: string): string {
  let processed = text
  
  // Books {@book Player's Handbook|PHB}
  processed = processed.replace(/{@book\s+([^}|]+)(?:\|([^}|]+))?(?:\|([^}|]+))?(?:\|([^}]+))?}/g, 
    (match, bookName, source, page, display) => {
      const displayText = display || bookName
      const title = page ? `${bookName}, page ${page}` : bookName
      return `<span class="book-ref" title="${title}">${displayText}</span>`
    })
  
  // Links {@link text|url}
  processed = processed.replace(/{@link\s+([^}|]+)\|([^}]+)}/g, '<a href="$2" target="_blank">$1</a>')
  
  // Chance/percentage {@chance 25}
  processed = processed.replace(/{@chance\s+(\d+)(?:\|([^}]+))?}/g, (match, percent, display) => {
    return `<span class="chance">${percent}%${display ? ` ${display}` : ' chance'}</span>`
  })
  
  return processed
}

// Alias for backward compatibility
export const formatText = processFormatting