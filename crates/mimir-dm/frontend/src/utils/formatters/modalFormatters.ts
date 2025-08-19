// Modal content formatters from original BookApp.vue
import { processFormatting } from './textFormatter'

// Get school name for spells
export function getSchoolName(school: string): string {
  const schools: Record<string, string> = {
    'A': 'Abjuration',
    'C': 'Conjuration',
    'D': 'Divination',
    'E': 'Enchantment',
    'V': 'Evocation',
    'I': 'Illusion',
    'N': 'Necromancy',
    'T': 'Transmutation'
  }
  return schools[school] || school || 'Unknown'
}

// Get size name for creatures
export function getSizeName(size: string): string {
  const sizes: Record<string, string> = {
    'T': 'Tiny',
    'S': 'Small',
    'M': 'Medium',
    'L': 'Large',
    'H': 'Huge',
    'G': 'Gargantuan'
  }
  return sizes[size] || size || 'Medium'
}

// Format range for spells
export function formatRange(range: any): string {
  if (!range) return 'Self'
  if (range.type === 'point') {
    return `${range.distance?.amount || 0} ${range.distance?.type || 'feet'}`
  }
  return range.type || 'Unknown'
}

// Format spell components
export function formatComponents(components: any): string {
  if (!components) return 'None'
  const parts = []
  if (components.v) parts.push('V')
  if (components.s) parts.push('S')
  if (components.m) parts.push(`M (${typeof components.m === 'string' ? components.m : 'materials'})`)
  return parts.join(', ') || 'None'
}

// Format duration
export function formatDuration(duration: any): string {
  if (!duration || !duration[0]) return 'Instantaneous'
  const d = duration[0]
  if (d.type === 'instant') return 'Instantaneous'
  if (d.type === 'permanent') return 'Until dispelled'
  if (d.concentration) return `Concentration, up to ${d.duration?.amount || 1} ${d.duration?.type || 'minute'}`
  return `${d.duration?.amount || 1} ${d.duration?.type || 'minute'}`
}

// Format AC
export function formatAC(ac: any): string {
  if (!ac) return '10'
  if (typeof ac === 'number') return String(ac)
  if (Array.isArray(ac) && ac.length > 0) {
    const first = ac[0]
    if (typeof first === 'number') return String(first)
    if (first.ac) return String(first.ac) + (first.from ? ` (${first.from.join(', ')})` : '')
  }
  return '10'
}

// Format HP
export function formatHP(hp: any): string {
  if (!hp) return '1'
  if (hp.average) return `${hp.average} (${hp.formula || '1d4'})`
  return '1'
}

// Format speed
export function formatSpeed(speed: any): string {
  if (!speed) return '30 ft.'
  const parts = []
  if (speed.walk) parts.push(`${speed.walk} ft.`)
  if (speed.fly) parts.push(`fly ${speed.fly} ft.`)
  if (speed.swim) parts.push(`swim ${speed.swim} ft.`)
  if (speed.climb) parts.push(`climb ${speed.climb} ft.`)
  if (speed.burrow) parts.push(`burrow ${speed.burrow} ft.`)
  return parts.join(', ') || '30 ft.'
}

// Format alignment
export function formatAlignment(alignment: any): string {
  if (!alignment || !Array.isArray(alignment)) return ''
  const alignMap: Record<string, string> = {
    'L': 'lawful',
    'N': 'neutral',
    'C': 'chaotic',
    'G': 'good',
    'E': 'evil',
    'U': 'unaligned',
    'A': 'any alignment'
  }
  
  return alignment.map(a => {
    if (typeof a === 'string') return alignMap[a] || a
    if (a.alignment) return a.alignment.map((al: string) => alignMap[al] || al).join(' ')
    return ''
  }).filter(a => a).join(' ') || ''
}

// Format creature skills
export function formatSkills(skills: any): string {
  if (!skills) return ''
  return Object.entries(skills).map(([skill, bonus]) => `${skill} ${bonus}`).join(', ')
}

// Calculate ability modifier
export function getModifier(score: number): string {
  const mod = Math.floor((score - 10) / 2)
  return mod >= 0 ? `+${mod}` : `${mod}`
}

// Main modal content renderer
export function renderModalContent(content: any): string {
  if (!content) return ''
  
  let html = '<div class="reference-details">'
  
  const refType = content.ref_type || content.type || 'unknown'
  
  if (refType === 'spell' || content.level !== undefined) {
    const level = content.level || 0
    const school = content.school || 'Unknown'
    const castingTime = content.time?.[0] ? 
      `${content.time[0].number} ${content.time[0].unit}` : '1 action'
    const range = formatRange(content.range)
    const components = formatComponents(content.components)
    const duration = formatDuration(content.duration)
    const description = content.entries?.map((e: any) => {
      if (typeof e === 'string') {
        return processFormatting(e)
      } else if (e && typeof e === 'object') {
        // Handle complex entry objects
        if (e.type === 'entries' && e.entries) {
          const header = e.name ? `<strong>${e.name}:</strong> ` : ''
          const entryContent = e.entries.map((subEntry: any) => 
            typeof subEntry === 'string' ? processFormatting(subEntry) : ''
          ).join(' ')
          return header + entryContent
        }
        return '' // Skip other complex types for now
      }
      return ''
    }).filter((e: string) => e).join('<br/><br/>') || ''
    
    html += `
      <div class="spell-details">
        <p><strong>Level:</strong> ${level === 0 ? 'Cantrip' : level}</p>
        <p><strong>School:</strong> ${getSchoolName(school)}</p>
        <p><strong>Casting Time:</strong> ${castingTime}</p>
        <p><strong>Range:</strong> ${range}</p>
        <p><strong>Components:</strong> ${components}</p>
        <p><strong>Duration:</strong> ${duration}</p>
        <div class="description">${description}</div>
      </div>
    `
  } else if (refType === 'item') {
    const itemType = content.type || 'Item'
    const rarity = content.rarity || ''
    const value = content.value || ''
    const weight = content.weight || ''
    const description = content.entries?.map((e: any) => {
      if (typeof e === 'string') {
        return processFormatting(e)
      } else if (e && typeof e === 'object') {
        // Handle complex entry objects
        if (e.type === 'entries' && e.entries) {
          const header = e.name ? `<strong>${e.name}:</strong> ` : ''
          const entryContent = e.entries.map((subEntry: any) => 
            typeof subEntry === 'string' ? processFormatting(subEntry) : ''
          ).join(' ')
          return header + entryContent
        }
        return '' // Skip other complex types for now
      }
      return ''
    }).filter((e: string) => e).join('<br/><br/>') || ''
    
    html += `
      <div class="item-details">
        <p><strong>Type:</strong> ${itemType}</p>
        ${rarity ? `<p><strong>Rarity:</strong> ${rarity}</p>` : ''}
        ${value ? `<p><strong>Value:</strong> ${value} gp</p>` : ''}
        ${weight ? `<p><strong>Weight:</strong> ${weight} lb</p>` : ''}
        <div class="description">${description}</div>
      </div>
    `
  } else if (refType === 'creature' || refType === 'monster') {
    const size = content.size || 'Medium'
    const type = content.type || 'creature'
    const alignment = formatAlignment(content.alignment)
    const ac = formatAC(content.ac)
    const hp = formatHP(content.hp)
    const speed = formatSpeed(content.speed)
    const cr = content.cr || '0'
    
    // Ability scores
    const str = content.str || 10
    const dex = content.dex || 10
    const con = content.con || 10
    const int = content.int || 10
    const wis = content.wis || 10
    const cha = content.cha || 10
    
    html += `
      <div class="creature-details">
        <div class="creature-type">${getSizeName(size)} ${type}${alignment ? `, ${alignment}` : ''}</div>
        <div class="creature-stats">
          <p><strong>Armor Class:</strong> ${ac}</p>
          <p><strong>Hit Points:</strong> ${hp}</p>
          <p><strong>Speed:</strong> ${speed}</p>
        </div>
        <div class="ability-scores">
          <table>
            <tr>
              <th>STR</th><th>DEX</th><th>CON</th><th>INT</th><th>WIS</th><th>CHA</th>
            </tr>
            <tr>
              <td>${str} (${getModifier(str)})</td>
              <td>${dex} (${getModifier(dex)})</td>
              <td>${con} (${getModifier(con)})</td>
              <td>${int} (${getModifier(int)})</td>
              <td>${wis} (${getModifier(wis)})</td>
              <td>${cha} (${getModifier(cha)})</td>
            </tr>
          </table>
        </div>
        ${content.skill ? `<p><strong>Skills:</strong> ${formatSkills(content.skill)}</p>` : ''}
        ${content.senses ? `<p><strong>Senses:</strong> ${content.senses.join(', ')}</p>` : ''}
        ${content.languages ? `<p><strong>Languages:</strong> ${content.languages.join(', ')}</p>` : ''}
        <p><strong>Challenge Rating:</strong> ${cr}</p>
        ${content.trait ? `
          <div class="creature-traits">
            <h4>Traits</h4>
            ${content.trait.map((t: any) => `
              <div class="trait">
                <strong>${t.name}.</strong> ${t.entries ? t.entries.map((e: any) => 
                  typeof e === 'string' ? processFormatting(e) : ''
                ).join(' ') : ''}
              </div>
            `).join('')}
          </div>
        ` : ''}
        ${content.action ? `
          <div class="creature-actions">
            <h4>Actions</h4>
            ${content.action.map((a: any) => `
              <div class="action">
                <strong>${a.name}.</strong> ${a.entries ? a.entries.map((e: any) => 
                  typeof e === 'string' ? processFormatting(e) : ''
                ).join(' ') : ''}
              </div>
            `).join('')}
          </div>
        ` : ''}
      </div>
    `
  } else if (refType === 'race') {
    const description = content.entries?.map((e: any) => {
      if (typeof e === 'string') {
        return processFormatting(e)
      } else if (e && typeof e === 'object') {
        // Handle complex entry objects
        if (e.type === 'entries' && e.entries) {
          const header = e.name ? `<strong>${e.name}:</strong> ` : ''
          const entryContent = e.entries.map((subEntry: any) => 
            typeof subEntry === 'string' ? processFormatting(subEntry) : ''
          ).join(' ')
          return header + entryContent
        }
        return '' // Skip other complex types for now
      }
      return ''
    }).filter((e: string) => e).join('<br/><br/>') || 
                       content.fluff?.[0]?.entries?.map((e: any) => {
                         if (typeof e === 'string') {
                           return processFormatting(e)
                         }
                         return ''
                       }).join('<br/>') || ''
    
    html += `
      <div class="race-details">
        ${content.size ? `<p><strong>Size:</strong> ${getSizeName(content.size)}</p>` : ''}
        ${content.speed ? `<p><strong>Speed:</strong> ${content.speed} feet</p>` : ''}
        <div class="description">${description}</div>
      </div>
    `
  } else if (refType === 'class') {
    const description = content.entries?.map((e: any) => {
      if (typeof e === 'string') {
        return processFormatting(e)
      } else if (e && typeof e === 'object') {
        // Handle complex entry objects
        if (e.type === 'entries' && e.entries) {
          const header = e.name ? `<strong>${e.name}:</strong> ` : ''
          const entryContent = e.entries.map((subEntry: any) => 
            typeof subEntry === 'string' ? processFormatting(subEntry) : ''
          ).join(' ')
          return header + entryContent
        }
        return '' // Skip other complex types for now
      }
      return ''
    }).filter((e: string) => e).join('<br/><br/>') || ''
    
    html += `
      <div class="class-details">
        ${content.hd ? `<p><strong>Hit Dice:</strong> 1d${content.hd.faces}</p>` : ''}
        <div class="description">${description}</div>
      </div>
    `
  } else if (refType === 'background') {
    const description = content.entries?.map((e: any) => {
      if (typeof e === 'string') {
        return processFormatting(e)
      } else if (e && typeof e === 'object') {
        // Handle complex entry objects
        if (e.type === 'entries' && e.entries) {
          const header = e.name ? `<strong>${e.name}:</strong> ` : ''
          const entryContent = e.entries.map((subEntry: any) => 
            typeof subEntry === 'string' ? processFormatting(subEntry) : ''
          ).join(' ')
          return header + entryContent
        }
        return '' // Skip other complex types for now
      }
      return ''
    }).filter((e: string) => e).join('<br/><br/>') || ''
    
    html += `
      <div class="background-details">
        <div class="description">${description}</div>
      </div>
    `
  } else if (refType === 'feat') {
    const description = content.entries?.map((e: any) => {
      if (typeof e === 'string') {
        return processFormatting(e)
      } else if (e && typeof e === 'object') {
        // Handle complex entry objects
        if (e.type === 'entries' && e.entries) {
          const header = e.name ? `<strong>${e.name}:</strong> ` : ''
          const entryContent = e.entries.map((subEntry: any) => 
            typeof subEntry === 'string' ? processFormatting(subEntry) : ''
          ).join(' ')
          return header + entryContent
        }
        return '' // Skip other complex types for now
      }
      return ''
    }).filter((e: string) => e).join('<br/><br/>') || ''
    
    const prereq = content.prerequisite?.map((p: any) => {
      if (p.ability) {
        const abilities = []
        for (const [key, value] of Object.entries(p.ability)) {
          abilities.push(`${key.toUpperCase()} ${value}`)
        }
        return abilities.join(' or ')
      }
      return ''
    }).filter((p: string) => p).join(', ')
    
    html += `
      <div class="feat-details">
        ${prereq ? `<p><strong>Prerequisite:</strong> ${prereq}</p>` : ''}
        <div class="description">${description}</div>
      </div>
    `
  } else if (refType === 'condition') {
    const description = content.entries?.map((e: any) => processFormatting(e)).join('<br/><br/>') || 
                       (content.description ? processFormatting(content.description) : '') || 
                       JSON.stringify(content, null, 2)
    
    html += `
      <div class="condition-details">
        <div class="description">${description}</div>
      </div>
    `
  } else {
    // Generic handler for unknown types
    html += `<pre>${JSON.stringify(content, null, 2)}</pre>`
  }
  
  html += '</div>'
  return html
}