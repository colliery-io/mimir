import type { ClassSummary } from '@/composables/catalog/useCatalog'

export function formatClassDetails(cls: ClassSummary, parentClass?: ClassSummary): string {
  let html = '<div class="class-details">'
  
  // Check if it's a subclass (name contains ':')
  const isSubclass = cls.name.includes(':')
  
  if (isSubclass) {
    const [parentClassName, subclassName] = cls.name.split(': ')
    
    // Header for subclass
    html += '<div class="class-header-info">'
    html += `<div class="class-type">Subclass of ${parentClassName}</div>`
    html += `<h3>${subclassName}</h3>`
    html += '</div>'
    
    // Subclass description
    html += '<div class="class-properties">'
    html += `<div>${cls.description}</div>`
    
    // Parent class info if provided
    if (parentClass) {
      html += '<div class="parent-class-info">'
      html += '<h4>Parent Class Info:</h4>'
      html += `<div><strong>Hit Die:</strong> ${parentClass.hitDie}</div>`
      html += `<div><strong>Primary Ability:</strong> ${parentClass.primaryAbility}</div>`
      html += `<div><strong>Saving Throw Proficiencies:</strong> ${parentClass.saves}</div>`
      if (parentClass.spellcaster) {
        html += `<div><strong>Spellcasting:</strong> ${parentClass.description}</div>`
      }
      html += '</div>'
    }
    html += '</div>'
  } else {
    // Header for main class
    html += '<div class="class-header-info">'
    html += `<div class="class-type">${cls.spellcaster ? 'Spellcaster' : 'Martial'} Class</div>`
    html += '</div>'
    
    // Core class info
    html += '<div class="class-properties">'
    html += `<div><strong>Hit Die:</strong> ${cls.hitDie}</div>`
    html += `<div><strong>Primary Ability:</strong> ${cls.primaryAbility}</div>`
    html += `<div><strong>Saving Throw Proficiencies:</strong> ${cls.saves}</div>`
    if (cls.spellcaster) {
      html += `<div><strong>Spellcasting:</strong> ${cls.description}</div>`
    }
    html += '</div>'
  }
  
  // Source
  html += `<div class="item-source">Source: ${cls.source}</div>`
  html += '</div>'
  
  return html
}

export function formatEnhancedClassDetails(cls: ClassSummary, parentClass?: ClassSummary): string {
  let html = '<div class="class-details">'
  
  const [parentClassName, subclassName] = cls.name.split(': ')
  
  // Header for subclass
  html += '<div class="class-header-info">'
  html += `<div class="class-type">Subclass of ${parentClassName}</div>`
  html += `<h3>${subclassName}</h3>`
  html += '</div>'
  
  // Subclass description
  html += '<div class="class-properties">'
  html += `<div>${cls.description}</div>`
  
  // Show subclass-specific features if available
  if (cls.subclassFeatures && cls.subclassFeatures.length > 0) {
    html += '<div class="subclass-section">'
    html += '<h4>Subclass Features:</h4>'
    html += '<ul class="feature-list">'
    for (const feature of cls.subclassFeatures) {
      html += `<li>${feature}</li>`
    }
    html += '</ul>'
    html += '</div>'
  }
  
  // Show additional spells (domain spells, oath spells, etc.)
  if (cls.additionalSpells && cls.additionalSpells.length > 0) {
    html += '<div class="subclass-section">'
    html += '<h4>Additional Spells:</h4>'
    html += '<ul class="spell-list">'
    for (const spell of cls.additionalSpells) {
      html += `<li class="spell-item">${spell}</li>`
    }
    html += '</ul>'
    html += '</div>'
  }
  
  // Show spellcasting ability if this subclass grants it
  if (cls.spellcastingAbility) {
    html += '<div class="subclass-section">'
    html += '<h4>Spellcasting:</h4>'
    html += `<div><strong>Spellcasting Ability:</strong> ${cls.spellcastingAbility}</div>`
    
    // Show cantrip progression if available
    if (cls.cantripProgression && cls.cantripProgression.length > 0) {
      html += '<div><strong>Cantrips Known:</strong> '
      const levels = [1, 4, 10]  // Common cantrip progression levels
      const progression = []
      for (let i = 0; i < Math.min(3, cls.cantripProgression.length); i++) {
        if (cls.cantripProgression[i] > 0) {
          progression.push(`Level ${levels[i]}: ${cls.cantripProgression[i]}`)
        }
      }
      html += progression.join(', ')
      html += '</div>'
    }
    html += '</div>'
  }
  
  // Parent class info if provided
  if (parentClass) {
    html += '<div class="parent-class-info">'
    html += '<h4>Base Class Info:</h4>'
    html += `<div><strong>Hit Die:</strong> ${parentClass.hitDie}</div>`
    html += `<div><strong>Primary Ability:</strong> ${parentClass.primaryAbility}</div>`
    html += `<div><strong>Saving Throw Proficiencies:</strong> ${parentClass.saves}</div>`
    html += '</div>'
  }
  
  html += '</div>'
  
  // Source
  html += `<div class="item-source">Source: ${cls.source}</div>`
  html += '</div>'
  
  return html
}