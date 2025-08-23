import { processFormattingTags, formatEntries } from '../utils/textFormatting'
import { invoke } from '@tauri-apps/api/core'
import type { Class, ClassWithDetails, ClassSummary, Subclass, ClassFeature, SubclassFeature, ClassFluff, SubclassFluff } from '../composables/useCatalog'

export async function formatClassDetails(classData: ClassWithDetails | ClassSummary): Promise<string> {
  // Handle both summary and full details
  const isFullDetails = 'class' in classData
  
  if (!isFullDetails) {
    return formatClassSummary(classData as ClassSummary)
  }
  
  return await formatFullClassDetails(classData as ClassWithDetails)
}

function formatClassSummary(classSummary: ClassSummary): string {
  let html = '<div class="class-details">'
  
  // Header section
  html += '<div class="class-header-section">'
  html += `<h2>${classSummary.name}</h2>`
  html += '</div>'
  
  // Basic properties
  html += '<div class="class-properties-grid">'
  html += `<div class="property-item">
    <span class="property-label">Hit Dice:</span>
    <span class="property-value">${classSummary.hitDice}</span>
  </div>`
  html += `<div class="property-item">
    <span class="property-label">Primary Ability:</span>
    <span class="property-value">${classSummary.primaryAbility}</span>
  </div>`
  html += `<div class="property-item">
    <span class="property-label">Saving Throw Proficiencies:</span>
    <span class="property-value">${classSummary.proficiency}</span>
  </div>`
  
  if (classSummary.spellcastingAbility) {
    html += `<div class="property-item">
      <span class="property-label">Spellcasting Ability:</span>
      <span class="property-value">${formatAbilityScore(classSummary.spellcastingAbility)}</span>
    </div>`
  }
  
  if (classSummary.subclassTitle) {
    html += `<div class="property-item">
      <span class="property-label">Subclass Type:</span>
      <span class="property-value">${classSummary.subclassTitle}</span>
    </div>`
  }
  
  html += '</div>'
  
  // Description
  if (classSummary.description) {
    html += '<div class="class-description">'
    html += processFormattingTags(classSummary.description)
    html += '</div>'
  }
  
  // Source
  html += `<div class="source-info">Source: ${classSummary.source}${classSummary.page ? `, p. ${classSummary.page}` : ''}</div>`
  
  html += '</div>'
  return html
}

async function formatFullClassDetails(classDetails: ClassWithDetails): Promise<string> {
  const classData = classDetails.class
  let html = '<div class="class-details">'
  
  // Header section
  html += '<div class="class-header-section">'
  html += `<h2>${classData.name}</h2>`
  html += '</div>'
  
  // Basic properties
  html += '<div class="class-properties-grid">'
  
  // Format hit dice
  if (classData.hd) {
    const hdText = typeof classData.hd === 'object' 
      ? `${classData.hd.number || 1}d${classData.hd.faces || 6}`
      : '1d6'
    html += `<div class="property-item">
      <span class="property-label">Hit Dice:</span>
      <span class="property-value">${hdText}</span>
    </div>`
  }
  
  // Format proficiencies
  if (classData.startingProficiencies?.savingThrows) {
    const saves = classData.startingProficiencies.savingThrows
      .map((s: string) => s.toUpperCase())
      .join(', ')
    html += `<div class="property-item">
      <span class="property-label">Saving Throw Proficiencies:</span>
      <span class="property-value">${saves}</span>
    </div>`
  }
  
  if (classData.spellcastingAbility) {
    html += `<div class="property-item">
      <span class="property-label">Spellcasting Ability:</span>
      <span class="property-value">${formatAbilityScore(classData.spellcastingAbility)}</span>
    </div>`
  }
  
  if (classData.subclassTitle) {
    html += `<div class="property-item">
      <span class="property-label">Subclass Type:</span>
      <span class="property-value">${classData.subclassTitle}</span>
    </div>`
  }
  
  html += '</div>'
  
  // Add fluff description if available
  if (classDetails.fluff) {
    html += '<div class="class-fluff-section">'
    html += '<h3>Description</h3>'
    
    // Add images if present
    if (classDetails.fluff.images && classDetails.fluff.images.length > 0) {
      html += '<div class="class-images">'
      for (const image of classDetails.fluff.images) {
        if (typeof image === 'object' && image.href && image.href.path) {
          try {
            const response = await invoke<any>('serve_book_image', {
              bookId: classData.source,
              imagePath: image.href.path
            })
            if (response && response.success && response.data) {
              html += `<img src="${response.data}" alt="${classData.name}" class="class-image" style="max-width: 400px; max-height: 400px; width: auto; height: auto; object-fit: contain; display: block; margin: 1rem auto;" />`
            }
          } catch (e) {
          }
        }
      }
      html += '</div>'
    }
    
    // Add fluff entries
    if (classDetails.fluff.entries && classDetails.fluff.entries.length > 0) {
      html += '<div class="fluff-entries">'
      html += formatEntries(classDetails.fluff.entries)
      html += '</div>'
    }
    html += '</div>'
  }
  
  // Class features sections from entries
  if (classData.entries && classData.entries.length > 0) {
    html += '<div class="class-entries">'
    html += '<h3>Class Features</h3>'
    for (const entry of classData.entries) {
      html += formatEntry(entry)
    }
    html += '</div>'
  }
  
  // Subclasses
  if (classDetails.subclasses && classDetails.subclasses.length > 0) {
    html += '<div class="subclasses-section">'
    html += `<h3>${classData.subclassTitle || 'Subclasses'}</h3>`
    
    for (const subclass of classDetails.subclasses) {
      // Find matching fluff for this subclass
      const subclassFluff = classDetails.subclass_fluff?.find(f => 
        f.name === subclass.name && f.source === subclass.source
      )
      
      // Find the introductory feature for this subclass with more flexible matching
      const subclassIntroFeature = classDetails.subclass_features?.find(f => {
        // Try both snake_case and camelCase field names
        const shortName = (f as any).subclassShortName || f.subclass_short_name
        const subclassSource = (f as any).subclassSource || f.subclass_source
        
        // Check if this feature belongs to this subclass
        const nameMatch = shortName === subclass.short_name || 
                         shortName === subclass.name ||
                         (subclass.short_name && shortName?.toLowerCase() === subclass.short_name.toLowerCase()) ||
                         // Also try matching against the feature name if it's the main path description
                         (f.name === subclass.name && (f.level === 3 || f.level === 1 || f.level === 2))
        
        const sourceMatch = subclassSource === subclass.source
        const levelMatch = f.level === 3 || f.level === 1 || f.level === 2
        
        if (nameMatch && sourceMatch && levelMatch) {
          return true
        }
        return false
      })
      
      // Get all features for this subclass
      const subclassFeatures = classDetails.subclass_features?.filter(f => {
        const shortName = (f as any).subclassShortName || f.subclass_short_name
        const subclassSource = (f as any).subclassSource || f.subclass_source
        
        return (shortName === subclass.short_name || 
                shortName === subclass.name ||
                // For Path of the Berserker, shortName might be "Berserker"
                (subclass.name.includes(shortName) && shortName)) &&
               subclassSource === subclass.source
      })
      
      html += await formatSubclass(subclass, subclassFluff, subclassIntroFeature, subclassFeatures)
    }
    html += '</div>'
  }
  
  // Class Features Table
  if (classDetails.features && classDetails.features.length > 0) {
    html += '<div class="features-section">'
    html += '<h3>Features by Level</h3>'
    html += formatFeaturesTable(classDetails.features)
    html += '</div>'
  }
  
  // Class Table Groups (includes spell slots for casters)
  if (classData.classTableGroups && classData.classTableGroups.length > 0) {
    html += '<div class="class-tables-section">'
    html += '<h3>Class Progression</h3>'
    for (const tableGroup of classData.classTableGroups) {
      html += formatTable(tableGroup)
    }
    html += '</div>'
  }
  
  // Source
  html += `<div class="source-info">Source: ${classData.source}${classData.page ? `, p. ${classData.page}` : ''}</div>`
  
  html += '</div>'
  return html
}

function formatEntry(entry: any): string {
  if (typeof entry === 'string') {
    return `<p>${processFormattingTags(entry)}</p>`
  }
  
  if (entry.type === 'entries') {
    let html = ''
    if (entry.name) {
      html += `<h4>${entry.name}</h4>`
    }
    if (entry.entries) {
      for (const subEntry of entry.entries) {
        html += formatEntry(subEntry)
      }
    }
    return html
  }
  
  if (entry.type === 'list') {
    let html = '<ul>'
    for (const item of entry.items) {
      html += `<li>${processFormattingTags(item)}</li>`
    }
    html += '</ul>'
    return html
  }
  
  if (entry.type === 'table') {
    return formatTable(entry)
  }
  
  return ''
}

async function formatSubclass(subclass: Subclass, fluff?: SubclassFluff, introFeature?: SubclassFeature, allSubclassFeatures?: SubclassFeature[]): Promise<string> {
  let html = '<div class="subclass-item">'
  html += `<h4>${subclass.name}</h4>`
  
  // Only show short_name if it exists and is different from name
  if (subclass.short_name && subclass.short_name !== subclass.name) {
    html += `<p class="subclass-short-name">Also known as: ${subclass.short_name}</p>`
  }
  
  // Add intro feature description if available
  if (introFeature && introFeature.entries && introFeature.entries.length > 0) {
    html += '<div class="subclass-description">'
    // Show just the first entry as the main description
    const firstEntry = introFeature.entries[0]
    if (typeof firstEntry === 'string') {
      html += `<p>${processFormattingTags(firstEntry)}</p>`
    } else {
      html += formatEntry(firstEntry)
    }
    html += '</div>'
  }
  
  // Add spellcasting info if present
  if (subclass.spellcasting_ability || subclass.caster_progression) {
    html += '<div class="subclass-spellcasting">'
    if (subclass.spellcasting_ability) {
      html += `<span>Spellcasting: ${formatAbilityScore(subclass.spellcasting_ability)}</span>`
    }
    if (subclass.caster_progression) {
      html += `<span> (${subclass.caster_progression} caster)</span>`
    }
    html += '</div>'
  }
  
  // Add fluff description if available
  if (fluff) {
    // Add images if present
    if (fluff.images && fluff.images.length > 0) {
      html += '<div class="subclass-images">'
      for (const image of fluff.images) {
        if (typeof image === 'object' && image.href && image.href.path) {
          try {
            const response = await invoke<any>('serve_book_image', {
              bookId: subclass.source,
              imagePath: image.href.path
            })
            if (response && response.success && response.data) {
              html += `<img src="${response.data}" alt="${subclass.name}" class="subclass-image" style="max-width: 300px; max-height: 300px; width: auto; height: auto; object-fit: contain; display: block; margin: 0.5rem auto;" />`
            }
          } catch (e) {
          }
        }
      }
      html += '</div>'
    }
    
    // Add fluff entries
    if (fluff.entries && fluff.entries.length > 0) {
      html += '<div class="subclass-fluff">'
      // Only show first 2 entries as preview to keep it concise
      const entriesToShow = fluff.entries.slice(0, 2)
      html += formatEntries(entriesToShow)
      if (fluff.entries.length > 2) {
        html += '<p class="more-content">...</p>'
      }
      html += '</div>'
    }
  }
  
  // Add list of subclass features if available
  if (allSubclassFeatures && allSubclassFeatures.length > 0) {
    // Group features by level
    const featuresByLevel = new Map<number, SubclassFeature[]>()
    
    for (const feature of allSubclassFeatures) {
      // Skip the intro feature we already displayed
      if (feature === introFeature) continue
      
      if (!featuresByLevel.has(feature.level)) {
        featuresByLevel.set(feature.level, [])
      }
      featuresByLevel.get(feature.level)!.push(feature)
    }
    
    // Sort levels and display features
    const levels = Array.from(featuresByLevel.keys()).sort((a, b) => a - b)
    
    if (levels.length > 0) {
      html += '<div class="subclass-features">'
      html += '<h5>Subclass Features</h5>'
      html += '<ul class="feature-list">'
      
      for (const level of levels) {
        const levelFeatures = featuresByLevel.get(level)!
        for (const feature of levelFeatures) {
          html += `<li><strong>${formatOrdinal(level)} Level:</strong> ${feature.name}</li>`
        }
      }
      
      html += '</ul>'
      html += '</div>'
    }
  }
  
  // Add subclass features summary if present (legacy)
  if (subclass.subclass_features && !allSubclassFeatures) {
    html += '<div class="subclass-features-preview">'
    if (Array.isArray(subclass.subclass_features)) {
      const featureCount = subclass.subclass_features.length
      html += `<p class="feature-count">${featureCount} unique feature${featureCount !== 1 ? 's' : ''}</p>`
    }
    html += '</div>'
  }
  
  html += `<p class="source-info">Source: ${subclass.source}${subclass.page ? `, p. ${subclass.page}` : ''}</p>`
  html += '</div>'
  return html
}

function formatFeaturesTable(features: ClassFeature[]): string {
  // Group features by level
  const featuresByLevel = new Map<number, ClassFeature[]>()
  for (const feature of features) {
    if (!featuresByLevel.has(feature.level)) {
      featuresByLevel.set(feature.level, [])
    }
    featuresByLevel.get(feature.level)!.push(feature)
  }
  
  // Sort levels
  const levels = Array.from(featuresByLevel.keys()).sort((a, b) => a - b)
  
  let html = '<table class="entry-table">'
  html += '<thead><tr><th>Level</th><th>Features</th></tr></thead>'
  html += '<tbody>'
  
  for (const level of levels) {
    const levelFeatures = featuresByLevel.get(level)!
    html += '<tr>'
    html += `<td>${formatOrdinal(level)}</td>`
    html += '<td>'
    html += levelFeatures.map(f => f.name).join(', ')
    html += '</td>'
    html += '</tr>'
  }
  
  html += '</tbody></table>'
  return html
}
function formatTable(table: any): string {
  let html = '<table class="entry-table">'
  
  // Caption
  if (table.caption) {
    html += `<caption>${table.caption}</caption>`
  }
  
  // Headers - Add "Level" as first column if not present
  if (table.colLabels) {
    html += '<thead><tr>'
    // Check if first label is for level column
    const hasLevelColumn = table.colLabels[0]?.toLowerCase().includes('level')
    if (!hasLevelColumn && table.rows && table.rows.length === 20) {
      // This is likely a class progression table, add Level header
      html += '<th>Level</th>'
    }
    for (const label of table.colLabels) {
      // Process formatting tags in header labels
      html += `<th>${processFormattingTags(label)}</th>`
    }
    html += '</tr></thead>'
  }
  
  // Body
  if (table.rows) {
    html += '<tbody>'
    const hasLevelColumn = table.colLabels && table.colLabels[0]?.toLowerCase().includes('level')
    const isProgressionTable = !hasLevelColumn && table.rows.length === 20
    
    // Check if first column is a dice roll column
    const isDiceColumn = table.colLabels && table.colLabels[0]?.includes('{@dice')
    
    for (let i = 0; i < table.rows.length; i++) {
      const row = table.rows[i]
      html += '<tr>'
      
      // Add level column if this is a progression table without level column
      if (isProgressionTable) {
        html += `<td>${i + 1}</td>`
      }
      
      for (let j = 0; j < row.length; j++) {
        const cell = row[j]
        // Handle various cell types
        let cellContent = ''
        
        // If this is the first column and it's a dice column, and the cell is empty/undefined,
        // fill it with the row number
        if (j === 0 && isDiceColumn && (!cell || cell === '')) {
          cellContent = String(i + 1)
        } else if (typeof cell === 'object' && cell !== null) {
          // Handle bonus objects like {"type":"bonus","value":2}
          if (cell.type === 'bonus' && cell.value !== undefined) {
            cellContent = `+${cell.value}`
          } 
          // Handle cell objects with roll property {"roll": {"exact": 1}, "type": "cell"}
          else if (cell.type === 'cell' && cell.roll) {
            if (cell.roll.exact !== undefined) {
              cellContent = String(cell.roll.exact)
            } else if (cell.roll.min !== undefined && cell.roll.max !== undefined) {
              cellContent = `${cell.roll.min}-${cell.roll.max}`
            } else {
              cellContent = JSON.stringify(cell.roll)
            }
          }
          // Handle simple roll property
          else if (cell.roll) {
            cellContent = cell.roll
          }
          // Handle other object types
          else {
            cellContent = JSON.stringify(cell)
          }
        } else {
          cellContent = cell || ''
        }
        html += `<td>${processFormattingTags(cellContent)}</td>`
      }
      html += '</tr>'
    }
    html += '</tbody>'
  }
  
  html += '</table>'
  return html
}

function formatAbilityScore(ability: string): string {
  const abilityMap: Record<string, string> = {
    str: 'Strength',
    dex: 'Dexterity',
    con: 'Constitution',
    int: 'Intelligence',
    wis: 'Wisdom',
    cha: 'Charisma'
  }
  return abilityMap[ability.toLowerCase()] || ability.toUpperCase()
}

function formatOrdinal(n: number): string {
  const suffixes = ['th', 'st', 'nd', 'rd']
  const v = n % 100
  return n + (suffixes[(v - 20) % 10] || suffixes[v] || suffixes[0])
}

// Export a function to format class for modal display
export async function formatClassForModal(classData: any): Promise<{ title: string; content: string }> {
  const title = classData.name || classData.class?.name || 'Unknown Class'
  const content = await formatClassDetails(classData)
  
  return { title, content }
}