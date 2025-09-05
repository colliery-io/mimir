// Spell-specific formatters
export function formatSpellLevel(level: number): string {
  if (level === 0) return 'Cantrip'
  if (level === 1) return '1st'
  if (level === 2) return '2nd'
  if (level === 3) return '3rd'
  return `${level}th`
}

export function formatSpellTags(spell: any): string[] {
  const tags = []
  if (spell.concentration) tags.push('Concentration')
  if (spell.ritual) tags.push('Ritual')
  return tags
}

// Generic formatters for other catalog types
export function formatCR(cr: string): string {
  if (cr === '0') return '0'
  if (cr.includes('/')) return cr
  return cr
}

export function formatGold(value: number): string {
  if (value < 1) {
    const silver = Math.floor(value * 100)
    if (silver < 1) {
      const copper = Math.floor(value * 10000)
      return `${copper} cp`
    }
    return `${silver} sp`
  }
  return `${value} gp`
}

export function formatWeight(weight: number): string {
  if (weight === 0) return '—'
  if (weight < 1) return `${weight} lb.`
  return `${weight} lb.`
}

export function formatArray(arr: string[] | undefined): string {
  if (!arr || arr.length === 0) return '—'
  return arr.join(', ')
}