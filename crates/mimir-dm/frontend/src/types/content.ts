// Content rendering and display type definitions

export interface TooltipPosition {
  x: number
  y: number
}

export interface ModalContent {
  title: string
  content: string
  visible: boolean
}

export interface SchoolAbbreviation {
  [key: string]: string
}

export interface SizeAbbreviation {
  [key: string]: string
}

export interface AlignmentMapping {
  [key: string]: string
}

export interface AttackTypeMapping {
  [key: string]: string
}

export interface DamageTypeColors {
  [key: string]: string
}

export interface ConditionColors {
  [key: string]: string
}

export const SCHOOL_ABBREVIATIONS: SchoolAbbreviation = {
  'A': 'Abjuration',
  'C': 'Conjuration',
  'D': 'Divination',
  'E': 'Enchantment',
  'V': 'Evocation',
  'I': 'Illusion',
  'N': 'Necromancy',
  'T': 'Transmutation'
}

export const SIZE_ABBREVIATIONS: SizeAbbreviation = {
  'T': 'Tiny',
  'S': 'Small',
  'M': 'Medium',
  'L': 'Large',
  'H': 'Huge',
  'G': 'Gargantuan'
}

export const ALIGNMENT_MAPPINGS: AlignmentMapping = {
  'L': 'Lawful',
  'N': 'Neutral',
  'C': 'Chaotic',
  'G': 'Good',
  'E': 'Evil',
  'U': 'Unaligned',
  'A': 'Any'
}

export const ATTACK_TYPES: AttackTypeMapping = {
  'mw': 'Melee Weapon Attack',
  'rw': 'Ranged Weapon Attack',
  'ms': 'Melee Spell Attack',
  'rs': 'Ranged Spell Attack',
  'mw,rw': 'Melee or Ranged Weapon Attack',
  'ms,rs': 'Melee or Ranged Spell Attack'
}