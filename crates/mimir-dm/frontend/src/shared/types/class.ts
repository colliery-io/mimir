// Class-related type definitions based on our schema

export interface Class {
  name: string;
  source: string;
  page?: number;
  srd?: boolean;
  basicRules?: boolean;
  hd: HitDice;
  proficiency: number[];
  classFeatures: (string | ClassFeatureReference)[];
  startingProficiencies: StartingProficiencies;
  multiclassing?: Multiclassing;
  subclassTitle?: string;
  casterProgression?: 'full' | '1/2' | '1/3' | 'pact' | null;
  cantripProgression?: number[];
  spellsKnownProgression?: number[];
  preparedSpells?: string;
  spellcastingAbility?: 'int' | 'wis' | 'cha';
  classTableGroups?: ClassTableGroup[];
  startingEquipment?: StartingEquipment;
  optionalfeatureProgression?: OptionalFeatureProgression[];
  hasFluff?: boolean;
  hasFluffImages?: boolean;
}

export interface HitDice {
  number: number;
  faces: number;
}

export interface ClassFeatureReference {
  classFeature: string;
  gainSubclassFeature?: boolean;
  tableDisplayName?: string;
}

export interface StartingProficiencies {
  armor?: string[];
  weapons?: string[];
  tools?: string[];
  toolProficiencies?: ToolProficiencyChoice[];
  skills?: SkillChoice[];
  savingThrows?: string[];
}

export interface ToolProficiencyChoice {
  choose?: {
    from: string[];
    count: number;
  };
  [key: string]: any;
}

export interface SkillChoice {
  choose?: {
    from: string[];
    count: number;
  };
  [key: string]: any;  // Allow any type for dynamic skill properties
}

export interface Multiclassing {
  requirements?: {
    str?: number;
    dex?: number;
    con?: number;
    int?: number;
    wis?: number;
    cha?: number;
    or?: Record<string, number>[];
  };
  proficienciesGained?: {
    armor?: string[];
    weapons?: string[];
    tools?: string[];
    skills?: SkillChoice[];
  };
}

export interface ClassTableGroup {
  title?: string;
  colLabels: string[];
  colStyles?: string[];
  rows?: any[][];
  rowsSpellProgression?: number[][];
}

export interface StartingEquipment {
  additionalFromBackground?: boolean;
  default?: string[];
  goldAlternative?: string;
}

export interface OptionalFeatureProgression {
  name: string;
  featureType: string[];
  progression: Record<string, number>;
}

// Subclass types
export interface Subclass {
  name: string;
  shortName: string;
  source: string;
  className: string;
  classSource: string;
  page?: number;
  subclassFeatures: string[];
  subclassTableGroups?: ClassTableGroup[];
  casterProgression?: 'full' | '1/2' | '1/3';
  spellcastingAbility?: 'int' | 'wis' | 'cha';
  cantripProgression?: number[];
  spellsKnownProgression?: number[];
  additionalSpells?: AdditionalSpells[];
  hasFluff?: boolean;
  hasFluffImages?: boolean;
}

export interface AdditionalSpells {
  prepared?: Record<string, string[]>;
  known?: Record<string, string[]>;
  expanded?: Record<string, string[]>;
}

// Class Feature types
export interface ClassFeature {
  name: string;
  source: string;
  className: string;
  classSource: string;
  level: number;
  entries: any[];
  page?: number;
  srd?: boolean;
}

export interface SubclassFeature {
  name: string;
  source: string;
  className: string;
  classSource: string;
  subclassShortName: string;
  subclassSource: string;
  level: number;
  entries: any[];
  page?: number;
}

// Fluff types
export interface ClassFluff {
  name: string;
  source: string;
  entries: any[];
  images?: ImageEntry[];
}

export interface SubclassFluff {
  name: string;
  shortName: string;
  source: string;
  className: string;
  classSource: string;
  entries: any[];
  images?: ImageEntry[];
}

export interface ImageEntry {
  type: 'image';
  href: {
    type: 'internal' | 'external';
    path: string;
    url?: string;
  };
  title?: string;
  credit?: string;
}

// API response types
export interface ClassData {
  class: Class[];
  subclass?: Subclass[];
}

export interface ClassFeatureData {
  classFeature?: ClassFeature[];
  subclassFeature?: SubclassFeature[];
}

export interface ClassFluffData {
  classFluff?: ClassFluff[];
  subclassFluff?: SubclassFluff[];
}

// Utility type for class with all related data
export interface ClassWithDetails {
  class: Class;
  subclasses: Subclass[];
  features: ClassFeature[];
  fluff?: ClassFluff;
  subclassFluff?: SubclassFluff[];
}