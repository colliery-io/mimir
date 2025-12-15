<template>
  <MainLayout>
    <div class="character-sheet-view">
      <div v-if="characterStore.loading" class="loading">
        Loading character...
      </div>

      <div v-else-if="characterStore.error" class="error-message">
        {{ characterStore.error }}
      </div>

      <div v-else-if="!character || !data" class="error-message">
        Character not found
      </div>

      <template v-else-if="data">
        <!-- Header -->
        <div class="sheet-header">
          <div class="header-main">
            <button @click="goBack" class="btn-back">Back</button>
            <!-- Edit mode header -->
            <template v-if="isEditing && editData">
              <input
                v-model="editData.character_name"
                class="edit-input edit-name"
                placeholder="Character Name"
              />
              <div class="character-subtitle">
                Level {{ data.level }} {{ data.race }}{{ data.subrace ? ` (${data.subrace})` : '' }} {{ classString }}
              </div>
              <div class="edit-alignment">
                <span>{{ data.background }} | </span>
                <input
                  v-model="editData.alignment"
                  class="edit-input edit-alignment-input"
                  placeholder="Alignment"
                />
              </div>
            </template>
            <!-- View mode header -->
            <template v-else>
              <h1 class="character-name">{{ data.character_name }}</h1>
              <div class="character-subtitle">
                Level {{ data.level }} {{ data.race }}{{ data.subrace ? ` (${data.subrace})` : '' }} {{ classString }}
              </div>
              <div v-if="data.background" class="character-background">
                {{ data.background }}{{ data.alignment ? ` | ${data.alignment}` : '' }}
              </div>
            </template>
          </div>
          <div class="header-actions">
            <template v-if="isEditing">
              <button @click="saveEdits" class="btn-primary">Save</button>
              <button @click="cancelEditing" class="btn-secondary">Cancel</button>
            </template>
            <template v-else>
              <button @click="startEditing" class="btn-secondary">Edit</button>
              <button @click="printCharacter" class="btn-secondary" :disabled="isPrintingPdf">
                {{ isPrintingPdf ? 'Generating...' : 'Print PDF' }}
              </button>
              <button @click="exportCharacter" class="btn-secondary">Export</button>
              <button @click="showInventoryManager = true" class="btn-secondary">Inventory</button>
              <button @click="levelUp" class="btn-secondary">Level Up</button>
              <button @click="deleteCharacter" class="btn-danger">Delete</button>
            </template>
          </div>
        </div>

        <!-- Tab Navigation -->
        <div class="tab-navigation">
          <button
            @click="activeTab = 'character'"
            :class="['tab-button', { active: activeTab === 'character' }]"
          >
            Character
          </button>
          <button
            v-if="isSpellcaster"
            @click="activeTab = 'spells'"
            :class="['tab-button', { active: activeTab === 'spells' }]"
          >
            Spells
          </button>
        </div>

        <!-- Character Tab Content -->
        <div v-if="activeTab === 'character'" class="sheet-content">
          <!-- Left Column: Abilities & Combat -->
          <div class="sheet-column">
            <!-- Ability Scores -->
            <section class="sheet-section">
              <h2 class="section-title">Ability Scores</h2>
              <div class="ability-grid">
                <div v-for="(score, ability) in data.abilities" :key="ability" class="ability-box">
                  <div class="ability-name">{{ ability.slice(0, 3).toUpperCase() }}</div>
                  <div class="ability-score">{{ score }}</div>
                  <div class="ability-modifier">{{ formatModifier(getModifier(score)) }}</div>
                </div>
              </div>
            </section>

            <!-- Combat Stats -->
            <section class="sheet-section">
              <h2 class="section-title">Combat</h2>
              <div class="combat-grid">
                <div class="combat-stat">
                  <span class="stat-label">Armor Class</span>
                  <span class="stat-value">
                    {{ baseAC }}<span v-if="hasShield" class="ac-shield"> / {{ baseAC + 2 }}</span>
                  </span>
                  <span v-if="data.equipped.armor" class="stat-note">{{ data.equipped.armor }}</span>
                  <span v-if="hasShield" class="stat-note">w/o / w shield</span>
                </div>
                <div class="combat-stat">
                  <span class="stat-label">Initiative</span>
                  <span class="stat-value">{{ formatModifier(getModifier(data.abilities.dexterity)) }}</span>
                </div>
                <div class="combat-stat">
                  <span class="stat-label">Speed</span>
                  <span class="stat-value">{{ data.speed }} ft</span>
                </div>
                <div class="combat-stat">
                  <span class="stat-label">Passive Perception</span>
                  <span class="stat-value">{{ passivePerception }}</span>
                </div>
                <div class="combat-stat hp-stat">
                  <span class="stat-label">Hit Points</span>
                  <span v-if="isEditing && editData" class="stat-value">
                    <input
                      type="number"
                      v-model.number="editData.current_hp"
                      :min="0"
                      :max="data.max_hp"
                      class="edit-input edit-hp"
                    /> / {{ data.max_hp }}
                  </span>
                  <span v-else class="stat-value">{{ data.current_hp }} / {{ data.max_hp }}</span>
                </div>
                <div class="combat-stat">
                  <span class="stat-label">Hit Dice</span>
                  <span class="stat-value">{{ hitDiceString }}</span>
                </div>
                <div class="combat-stat">
                  <span class="stat-label">Proficiency</span>
                  <span class="stat-value">{{ formatModifier(proficiencyBonus) }}</span>
                </div>
              </div>
            </section>

            <!-- Saving Throws -->
            <section class="sheet-section">
              <h2 class="section-title">Saving Throws</h2>
              <div class="saves-list">
                <div v-for="(score, ability) in data.abilities" :key="ability" class="save-item">
                  <span class="save-proficient" :class="{ active: isProficientSave(ability) }">*</span>
                  <span class="save-name">{{ ability }}</span>
                  <span class="save-bonus">{{ formatModifier(getSaveBonus(ability, score)) }}</span>
                </div>
              </div>
            </section>

            <!-- Attacks -->
            <section class="sheet-section" v-if="hasAttacks">
              <h2 class="section-title">Attacks</h2>
              <div class="attacks-list">
                <div v-for="attack in attacks" :key="attack.name" class="attack-item">
                  <span class="attack-name">{{ attack.name }}</span>
                  <span class="attack-bonus">{{ formatModifier(attack.attackBonus) }}</span>
                  <span class="attack-damage">{{ attack.damage }}</span>
                </div>
              </div>
            </section>
          </div>

          <!-- Middle Column: Skills -->
          <div class="sheet-column">
            <section class="sheet-section">
              <h2 class="section-title">Skills</h2>
              <div class="skills-list">
                <div v-for="skill in allSkills" :key="skill.name" class="skill-item">
                  <span class="skill-proficient" :class="{ active: isProficientSkill(skill.name) }">*</span>
                  <span class="skill-name">{{ skill.name }}</span>
                  <span class="skill-ability">({{ skill.ability.slice(0, 3) }})</span>
                  <span class="skill-bonus">{{ formatModifier(getSkillBonus(skill)) }}</span>
                </div>
              </div>
            </section>
          </div>

          <!-- Right Column: Features, Spells, Equipment -->
          <div class="sheet-column">
            <!-- Proficiencies -->
            <section class="sheet-section">
              <h2 class="section-title">Proficiencies</h2>
              <div class="proficiency-group" v-if="data.proficiencies.armor.length">
                <strong>Armor:</strong> {{ data.proficiencies.armor.join(', ') }}
              </div>
              <div class="proficiency-group" v-if="data.proficiencies.weapons.length">
                <strong>Weapons:</strong> {{ data.proficiencies.weapons.join(', ') }}
              </div>
              <div class="proficiency-group" v-if="data.proficiencies.tools.length">
                <strong>Tools:</strong> {{ data.proficiencies.tools.join(', ') }}
              </div>
              <div class="proficiency-group" v-if="data.proficiencies.languages.length">
                <strong>Languages:</strong> {{ data.proficiencies.languages.join(', ') }}
              </div>
            </section>

            <!-- Class Features -->
            <section class="sheet-section" v-if="data.class_features.length">
              <h2 class="section-title">Features & Traits</h2>
              <ul class="feature-list">
                <li v-for="feature in data.class_features" :key="feature">{{ feature }}</li>
              </ul>
            </section>

            <!-- Feats -->
            <section class="sheet-section" v-if="data.feats.length">
              <h2 class="section-title">Feats</h2>
              <ul class="feature-list">
                <li v-for="feat in data.feats" :key="feat">{{ feat }}</li>
              </ul>
            </section>

            <!-- Spellcasting Summary (brief) -->
            <section class="sheet-section" v-if="isSpellcaster">
              <h2 class="section-title">Spellcasting</h2>
              <div class="spell-stats">
                <div class="spell-stat">
                  <span class="stat-label">Spell Save DC</span>
                  <span class="stat-value">{{ spellSaveDC }}</span>
                </div>
                <div class="spell-stat">
                  <span class="stat-label">Spell Attack</span>
                  <span class="stat-value">{{ formatModifier(spellAttackBonus) }}</span>
                </div>
              </div>
              <p class="spell-note">See Spells tab for full spell list</p>
            </section>

            <!-- Equipment -->
            <section class="sheet-section" v-if="hasEquipment || data.inventory.length">
              <h2 class="section-title">Equipment</h2>

              <!-- Equipped Items -->
              <div v-if="hasEquipment" class="equipped-items">
                <div v-if="data.equipped.armor" class="equipped-item">
                  <span class="equipped-slot">Armor:</span> {{ data.equipped.armor }}
                </div>
                <div v-if="data.equipped.shield" class="equipped-item">
                  <span class="equipped-slot">Shield:</span> {{ data.equipped.shield }}
                </div>
                <div v-if="data.equipped.main_hand" class="equipped-item">
                  <span class="equipped-slot">Main Hand:</span> {{ data.equipped.main_hand }}
                </div>
                <div v-if="data.equipped.off_hand" class="equipped-item">
                  <span class="equipped-slot">Off Hand:</span> {{ data.equipped.off_hand }}
                </div>
              </div>

              <!-- Inventory -->
              <div v-if="data.inventory.length" class="inventory-section">
                <strong v-if="hasEquipment" class="inventory-label">Inventory:</strong>
                <div class="item-list">
                  <div
                    v-for="item in data.inventory"
                    :key="item.name"
                    class="item-entry"
                    :class="{ expanded: expandedItems.has(`${item.name}:${item.source || 'PHB'}`) }"
                  >
                    <div class="item-row" @click="toggleItemExpansion(item.name, item.source)">
                      <span class="item-name">{{ item.name }}{{ item.quantity > 1 ? ` (${item.quantity})` : '' }}</span>
                      <span class="expand-icon">{{ expandedItems.has(`${item.name}:${item.source || 'PHB'}`) ? '-' : '+' }}</span>
                    </div>
                    <div v-if="expandedItems.has(`${item.name}:${item.source || 'PHB'}`)" class="item-details">
                      <!-- Custom notes (flavor text) - displayed prominently -->
                      <div v-if="item.notes" class="item-notes">
                        <strong>Notes:</strong> {{ item.notes }}
                      </div>

                      <div v-if="loadingItemDetails.has(`${item.name}:${item.source || 'PHB'}`)" class="loading-details">
                        Loading...
                      </div>
                      <template v-else-if="itemDetails[`${item.name}:${item.source || 'PHB'}`]">
                        <div class="item-meta">
                          <span v-if="itemDetails[`${item.name}:${item.source || 'PHB'}`].type">{{ itemDetails[`${item.name}:${item.source || 'PHB'}`].type }}</span>
                          <span v-if="itemDetails[`${item.name}:${item.source || 'PHB'}`].rarity" class="item-rarity">{{ itemDetails[`${item.name}:${item.source || 'PHB'}`].rarity }}</span>
                        </div>
                        <div class="item-properties">
                          <div v-if="itemDetails[`${item.name}:${item.source || 'PHB'}`].ac"><strong>AC:</strong> {{ itemDetails[`${item.name}:${item.source || 'PHB'}`].ac }}</div>
                          <div v-if="itemDetails[`${item.name}:${item.source || 'PHB'}`].dmg1"><strong>Damage:</strong> {{ itemDetails[`${item.name}:${item.source || 'PHB'}`].dmg1 }} {{ itemDetails[`${item.name}:${item.source || 'PHB'}`].dmgType || '' }}</div>
                          <div v-if="itemDetails[`${item.name}:${item.source || 'PHB'}`].range"><strong>Range:</strong> {{ itemDetails[`${item.name}:${item.source || 'PHB'}`].range }}</div>
                          <div v-if="itemDetails[`${item.name}:${item.source || 'PHB'}`].weight"><strong>Weight:</strong> {{ itemDetails[`${item.name}:${item.source || 'PHB'}`].weight }} lb</div>
                          <div v-if="itemDetails[`${item.name}:${item.source || 'PHB'}`].property?.length"><strong>Properties:</strong> {{ itemDetails[`${item.name}:${item.source || 'PHB'}`].property?.join(', ') }}</div>
                        </div>
                        <div v-if="itemDetails[`${item.name}:${item.source || 'PHB'}`].entries?.length" class="item-description">
                          <p v-for="(entry, idx) in itemDetails[`${item.name}:${item.source || 'PHB'}`].entries" :key="idx">
                            {{ formatItemEntry(entry) }}
                          </p>
                        </div>
                      </template>
                    </div>
                  </div>
                </div>
              </div>
            </section>

            <!-- Currency -->
            <section class="sheet-section">
              <h2 class="section-title">Currency</h2>
              <div class="currency-grid">
                <span class="currency-item"><strong>PP:</strong> {{ data.currency.platinum }}</span>
                <span class="currency-item"><strong>GP:</strong> {{ data.currency.gold }}</span>
                <span class="currency-item"><strong>SP:</strong> {{ data.currency.silver }}</span>
                <span class="currency-item"><strong>CP:</strong> {{ data.currency.copper }}</span>
              </div>
            </section>

            <!-- Personality -->
            <section class="sheet-section" v-if="hasPersonality || isEditing">
              <h2 class="section-title">Personality</h2>
              <!-- Edit mode -->
              <template v-if="isEditing && editData">
                <div class="personality-item">
                  <strong>Traits:</strong>
                  <textarea
                    v-model="editData.personality.traits"
                    class="edit-textarea"
                    placeholder="Personality traits..."
                    rows="2"
                  ></textarea>
                </div>
                <div class="personality-item">
                  <strong>Ideals:</strong>
                  <textarea
                    v-model="editData.personality.ideals"
                    class="edit-textarea"
                    placeholder="Ideals..."
                    rows="2"
                  ></textarea>
                </div>
                <div class="personality-item">
                  <strong>Bonds:</strong>
                  <textarea
                    v-model="editData.personality.bonds"
                    class="edit-textarea"
                    placeholder="Bonds..."
                    rows="2"
                  ></textarea>
                </div>
                <div class="personality-item">
                  <strong>Flaws:</strong>
                  <textarea
                    v-model="editData.personality.flaws"
                    class="edit-textarea"
                    placeholder="Flaws..."
                    rows="2"
                  ></textarea>
                </div>
              </template>
              <!-- View mode -->
              <template v-else>
                <div v-if="data.personality.traits" class="personality-item">
                  <strong>Traits:</strong> {{ data.personality.traits }}
                </div>
                <div v-if="data.personality.ideals" class="personality-item">
                  <strong>Ideals:</strong> {{ data.personality.ideals }}
                </div>
                <div v-if="data.personality.bonds" class="personality-item">
                  <strong>Bonds:</strong> {{ data.personality.bonds }}
                </div>
                <div v-if="data.personality.flaws" class="personality-item">
                  <strong>Flaws:</strong> {{ data.personality.flaws }}
                </div>
              </template>
            </section>
          </div>
        </div>

        <!-- Spells Tab Content -->
        <div v-else-if="activeTab === 'spells'" class="spells-sheet">
          <!-- Spellcasting Header -->
          <div class="spells-header">
            <div class="spellcasting-info">
              <div class="spell-stat-box">
                <span class="stat-label">Spellcasting Class</span>
                <span class="stat-value">{{ spellcastingClass?.class_name || 'None' }}</span>
              </div>
              <div class="spell-stat-box">
                <span class="stat-label">Spellcasting Ability</span>
                <span class="stat-value">{{ spellcastingAbility.slice(0, 3).toUpperCase() }}</span>
              </div>
              <div class="spell-stat-box">
                <span class="stat-label">Spell Save DC</span>
                <span class="stat-value">{{ spellSaveDC }}</span>
              </div>
              <div class="spell-stat-box">
                <span class="stat-label">Spell Attack Bonus</span>
                <span class="stat-value">{{ formatModifier(spellAttackBonus) }}</span>
              </div>
            </div>

            <!-- Spell Slots -->
            <div v-if="Object.keys(calculatedSpellSlots).length" class="spell-slots-row">
              <div v-for="level in Object.keys(calculatedSpellSlots).map(Number).sort((a, b) => a - b)" :key="level" class="spell-slot-box">
                <span class="slot-level-label">{{ level }}</span>
                <div class="slot-circles">
                  <span
                    v-for="i in calculatedSpellSlots[level].max"
                    :key="i"
                    class="slot-circle"
                    :class="{ used: i > calculatedSpellSlots[level].current }"
                  ></span>
                </div>
              </div>
            </div>
          </div>

          <!-- Spell Lists by Level -->
          <div class="spell-levels-grid">
            <!-- Cantrips -->
            <div class="spell-level-section">
              <div class="level-header-box">
                <span class="level-title">Cantrips</span>
              </div>
              <div class="spell-list">
                <div
                  v-for="spellName in (isFullListCaster ? spellsByLevel[0]?.map(s => s.name) : data.spells.cantrips) || []"
                  :key="spellName"
                  class="spell-item"
                  :class="{ expanded: expandedSpells.has(spellName) }"
                >
                  <div class="spell-row" @click="toggleSpellExpansion(spellName)">
                    <span class="spell-name">{{ spellName }}</span>
                    <span class="expand-icon">{{ expandedSpells.has(spellName) ? '-' : '+' }}</span>
                  </div>
                  <div v-if="expandedSpells.has(spellName)" class="spell-details">
                    <div v-if="loadingSpellDetails.has(spellName)" class="loading-details">
                      Loading...
                    </div>
                    <template v-else-if="spellDetails[spellName]">
                      <div class="spell-meta">
                        <span>{{ spellDetails[spellName].school }} cantrip</span>
                      </div>
                      <div class="spell-properties">
                        <div><strong>Casting Time:</strong> {{ spellDetails[spellName].time[0]?.number }} {{ spellDetails[spellName].time[0]?.unit }}</div>
                        <div><strong>Range:</strong> {{ spellDetails[spellName].range.distance?.amount || '' }} {{ spellDetails[spellName].range.distance?.type || spellDetails[spellName].range.type }}</div>
                        <div><strong>Components:</strong>
                          {{ spellDetails[spellName].components.v ? 'V' : '' }}{{ spellDetails[spellName].components.s ? 'S' : '' }}{{ spellDetails[spellName].components.m ? 'M' : '' }}
                        </div>
                        <div><strong>Duration:</strong> {{ spellDetails[spellName].duration[0]?.concentration ? 'Concentration, ' : '' }}{{ spellDetails[spellName].duration[0]?.duration?.amount || '' }} {{ spellDetails[spellName].duration[0]?.duration?.type || spellDetails[spellName].duration[0]?.type }}</div>
                      </div>
                      <div class="spell-description">
                        <p v-for="(entry, idx) in spellDetails[spellName].entries" :key="idx">
                          {{ formatSpellEntry(entry) }}
                        </p>
                      </div>
                    </template>
                  </div>
                </div>
                <div v-if="!(isFullListCaster ? spellsByLevel[0]?.length : data.spells.cantrips.length)" class="no-spells-message">
                  No cantrips
                </div>
              </div>
            </div>

            <!-- Spell Levels 1-9 -->
            <template v-for="level in [1, 2, 3, 4, 5, 6, 7, 8, 9]" :key="level">
              <div v-if="level <= maxSpellLevel" class="spell-level-section">
                <div class="level-header-box">
                  <span class="level-title">Level {{ level }}</span>
                  <div class="level-slots-info">
                    <span v-if="calculatedSpellSlots[level]" class="slots-total">
                      {{ calculatedSpellSlots[level].max }} slots
                    </span>
                    <span v-else class="slots-total">0 slots</span>
                    <div v-if="calculatedSpellSlots[level]" class="slots-circles-header">
                      <span
                        v-for="i in calculatedSpellSlots[level].max"
                        :key="i"
                        class="slot-circle-sm"
                        :class="{ used: i > calculatedSpellSlots[level].current }"
                      ></span>
                    </div>
                  </div>
                </div>
                <div class="spell-list">
                  <div
                    v-for="spellName in (isFullListCaster ? spellsByLevel[level]?.map(s => s.name) : spellsForSheet[level]) || []"
                    :key="spellName"
                    class="spell-item"
                    :class="{ expanded: expandedSpells.has(spellName) }"
                  >
                    <div class="spell-row" @click="toggleSpellExpansion(spellName)">
                      <span class="spell-name">{{ spellName }}</span>
                      <span class="expand-icon">{{ expandedSpells.has(spellName) ? '-' : '+' }}</span>
                    </div>
                    <div v-if="expandedSpells.has(spellName)" class="spell-details">
                      <div v-if="loadingSpellDetails.has(spellName)" class="loading-details">
                        Loading...
                      </div>
                      <template v-else-if="spellDetails[spellName]">
                        <div class="spell-meta">
                          <span>{{ spellDetails[spellName].level === 0 ? 'Cantrip' : `Level ${spellDetails[spellName].level}` }} {{ spellDetails[spellName].school }}</span>
                          <span v-if="spellDetails[spellName].duration[0]?.concentration" class="concentration-tag">Concentration</span>
                        </div>
                        <div class="spell-properties">
                          <div><strong>Casting Time:</strong> {{ spellDetails[spellName].time[0]?.number }} {{ spellDetails[spellName].time[0]?.unit }}</div>
                          <div><strong>Range:</strong> {{ spellDetails[spellName].range.distance?.amount || '' }} {{ spellDetails[spellName].range.distance?.type || spellDetails[spellName].range.type }}</div>
                          <div><strong>Components:</strong>
                            {{ spellDetails[spellName].components.v ? 'V' : '' }}{{ spellDetails[spellName].components.s ? 'S' : '' }}{{ spellDetails[spellName].components.m ? 'M' : '' }}
                          </div>
                          <div><strong>Duration:</strong> {{ spellDetails[spellName].duration[0]?.concentration ? 'Concentration, ' : '' }}{{ spellDetails[spellName].duration[0]?.duration?.amount || '' }} {{ spellDetails[spellName].duration[0]?.duration?.type || spellDetails[spellName].duration[0]?.type }}</div>
                        </div>
                        <div class="spell-description">
                          <p v-for="(entry, idx) in spellDetails[spellName].entries" :key="idx">
                            {{ formatSpellEntry(entry) }}
                          </p>
                        </div>
                      </template>
                    </div>
                  </div>
                  <div v-if="!(isFullListCaster ? spellsByLevel[level]?.length : spellsForSheet[level]?.length)" class="no-spells-message">
                    -
                  </div>
                </div>
              </div>
            </template>
          </div>
        </div>
      </template>
    </div>

    <!-- Level Up Dialog -->
    <LevelUpDialog
      v-if="data"
      :visible="showLevelUpDialog"
      :character-id="characterId"
      :character-data="data"
      @close="showLevelUpDialog = false"
      @completed="handleLevelUpCompleted"
    />

    <!-- Inventory Manager -->
    <InventoryManager
      v-if="data"
      :visible="showInventoryManager"
      :character-id="characterId"
      :character-data="data"
      @close="showInventoryManager = false"
      @updated="handleInventoryUpdated"
    />

    <!-- PDF Preview Modal -->
    <PdfPreviewModal
      ref="pdfPreviewRef"
      :visible="showPdfPreview"
      :title="pdfPreviewTitle"
      :default-file-name="pdfFileName"
      @close="showPdfPreview = false"
      @retry="printCharacter"
    />
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import MainLayout from '../../../shared/components/layout/MainLayout.vue'
import LevelUpDialog from '../components/LevelUpDialog.vue'
import InventoryManager from '../components/InventoryManager.vue'
import { PdfPreviewModal } from '../../../components/print'
import { PrintService } from '../../../services/PrintService'
import { useCharacterStore } from '../../../stores/characters'
import type { CharacterData } from '../../../types/character'

// Spell summary from catalog
interface SpellSummary {
  name: string
  level: number
  school: string
  concentration: boolean
  ritual: boolean
  casting_time: string
  range: string
  components: string
  classes: string[]
  description: string
}

// Full spell details
interface Spell {
  name: string
  source: string
  level: number
  school: string
  time: Array<{ number: number; unit: string }>
  range: { type: string; distance?: { type: string; amount?: number } }
  components: { v?: boolean; s?: boolean; m?: string | { text: string } }
  duration: Array<{ type: string; duration?: { type: string; amount?: number }; concentration?: boolean }>
  entries: Array<string | object>
}

// Full item details from catalog
interface ItemDetails {
  name: string
  source: string
  type?: string
  rarity?: string
  weight?: number
  value?: number
  ac?: number
  dmg1?: string
  dmgType?: string
  property?: string[]
  range?: string
  entries?: Array<string | object>
}

const route = useRoute()
const router = useRouter()
const characterStore = useCharacterStore()

// Tab navigation
const activeTab = ref<'character' | 'spells'>('character')

// Spell details cache and loading state
const spellDetails = ref<Record<string, Spell>>({})
const loadingSpellDetails = ref<Set<string>>(new Set())
const expandedSpells = ref<Set<string>>(new Set())

// Item details cache and loading state
const itemDetails = ref<Record<string, ItemDetails>>({})
const loadingItemDetails = ref<Set<string>>(new Set())
const expandedItems = ref<Set<string>>(new Set())

// Edit mode
const isEditing = ref(false)
const editData = ref<{
  character_name: string
  alignment: string
  current_hp: number
  personality: {
    traits: string
    ideals: string
    bonds: string
    flaws: string
  }
} | null>(null)

const characterId = computed(() => Number(route.params.id))
const character = computed(() => characterStore.currentCharacter)
const data = computed(() => character.value?.data as CharacterData)

onMounted(async () => {
  await characterStore.getCharacter(characterId.value)
})

// Ability score helpers
const getModifier = (score: number): number => Math.floor((score - 10) / 2)
const formatModifier = (mod: number): string => mod >= 0 ? `+${mod}` : `${mod}`

// Proficiency bonus based on level
const proficiencyBonus = computed(() => {
  if (!data.value) return 2
  return Math.ceil(data.value.level / 4) + 1
})

// Armor AC calculation
const getArmorAC = (armorName: string, dexMod: number): number => {
  const name = armorName.toLowerCase()

  // Extract magic bonus (+1, +2, +3)
  const magicMatch = name.match(/\+(\d)/)
  const magicBonus = magicMatch ? parseInt(magicMatch[1]) : 0

  // Light armor (full DEX)
  if (name.includes('padded') || (name.includes('leather') && !name.includes('studded'))) {
    return 11 + dexMod + magicBonus
  }
  if (name.includes('studded leather')) {
    return 12 + dexMod + magicBonus
  }

  // Medium armor (DEX max +2)
  const cappedDex = Math.min(dexMod, 2)
  if (name.includes('hide')) {
    return 12 + cappedDex + magicBonus
  }
  if (name.includes('chain shirt')) {
    return 13 + cappedDex + magicBonus
  }
  if (name.includes('scale mail') || name.includes('scale')) {
    return 14 + cappedDex + magicBonus
  }
  if (name.includes('breastplate')) {
    return 14 + cappedDex + magicBonus
  }
  if (name.includes('half plate')) {
    return 15 + cappedDex + magicBonus
  }

  // Heavy armor (no DEX)
  if (name.includes('ring mail')) {
    return 14 + magicBonus
  }
  if (name.includes('chain mail')) {
    return 16 + magicBonus
  }
  if (name.includes('splint')) {
    return 17 + magicBonus
  }
  if (name.includes('plate')) {
    return 18 + magicBonus
  }

  // Default: treat as light armor
  return 11 + dexMod + magicBonus
}

// Base AC calculation
const baseAC = computed(() => {
  if (!data.value) return 10
  const dexMod = getModifier(data.value.abilities.dexterity)

  // If armor is equipped, calculate based on armor type
  if (data.value.equipped.armor) {
    return getArmorAC(data.value.equipped.armor, dexMod)
  }

  // Unarmored: 10 + DEX
  return 10 + dexMod
})

// Check if character has a shield equipped
const hasShield = computed(() => {
  if (!data.value) return false
  return !!data.value.equipped.shield
})

// Skills with their associated abilities
const allSkills = [
  { name: 'Acrobatics', ability: 'dexterity' },
  { name: 'Animal Handling', ability: 'wisdom' },
  { name: 'Arcana', ability: 'intelligence' },
  { name: 'Athletics', ability: 'strength' },
  { name: 'Deception', ability: 'charisma' },
  { name: 'History', ability: 'intelligence' },
  { name: 'Insight', ability: 'wisdom' },
  { name: 'Intimidation', ability: 'charisma' },
  { name: 'Investigation', ability: 'intelligence' },
  { name: 'Medicine', ability: 'wisdom' },
  { name: 'Nature', ability: 'intelligence' },
  { name: 'Perception', ability: 'wisdom' },
  { name: 'Performance', ability: 'charisma' },
  { name: 'Persuasion', ability: 'charisma' },
  { name: 'Religion', ability: 'intelligence' },
  { name: 'Sleight of Hand', ability: 'dexterity' },
  { name: 'Stealth', ability: 'dexterity' },
  { name: 'Survival', ability: 'wisdom' }
]

// Check if proficient in a skill
const isProficientSkill = (skillName: string): boolean => {
  if (!data.value) return false
  return data.value.proficiencies.skills.some(
    s => s.toLowerCase() === skillName.toLowerCase()
  )
}

// Check if proficient in a save
const isProficientSave = (ability: string): boolean => {
  if (!data.value) return false
  return data.value.proficiencies.saves.some(
    s => s.toLowerCase() === ability.toLowerCase()
  )
}

// Get skill bonus
const getSkillBonus = (skill: { name: string; ability: string }): number => {
  if (!data.value) return 0
  const abilityScore = data.value.abilities[skill.ability as keyof typeof data.value.abilities]
  const mod = getModifier(abilityScore)
  const prof = isProficientSkill(skill.name) ? proficiencyBonus.value : 0
  return mod + prof
}

// Get save bonus
const getSaveBonus = (ability: string, score: number): number => {
  const mod = getModifier(score)
  const prof = isProficientSave(ability) ? proficiencyBonus.value : 0
  return mod + prof
}

// Passive Perception
const passivePerception = computed(() => {
  if (!data.value) return 10
  const wisMod = getModifier(data.value.abilities.wisdom)
  const prof = isProficientSkill('Perception') ? proficiencyBonus.value : 0
  return 10 + wisMod + prof
})

// Attacks
const hasAttacks = computed(() => {
  if (!data.value) return false
  return data.value.equipped.main_hand || data.value.equipped.off_hand
})

const attacks = computed(() => {
  if (!data.value) return []

  const result: { name: string; attackBonus: number; damage: string }[] = []
  const strMod = getModifier(data.value.abilities.strength)
  const dexMod = getModifier(data.value.abilities.dexterity)
  const prof = proficiencyBonus.value

  const getWeaponDamage = (weapon: string, abilityMod: number): string => {
    const w = weapon.toLowerCase()
    if (w.includes('greatsword') || w.includes('maul')) return `2d6${abilityMod >= 0 ? '+' : ''}${abilityMod}`
    if (w.includes('greataxe')) return `1d12${abilityMod >= 0 ? '+' : ''}${abilityMod}`
    if (w.includes('longsword') || w.includes('warhammer') || w.includes('battleaxe')) return `1d8${abilityMod >= 0 ? '+' : ''}${abilityMod}`
    if (w.includes('rapier')) return `1d8${abilityMod >= 0 ? '+' : ''}${abilityMod}`
    if (w.includes('shortsword') || w.includes('scimitar')) return `1d6${abilityMod >= 0 ? '+' : ''}${abilityMod}`
    if (w.includes('dagger')) return `1d4${abilityMod >= 0 ? '+' : ''}${abilityMod}`
    if (w.includes('quarterstaff') || w.includes('spear')) return `1d6${abilityMod >= 0 ? '+' : ''}${abilityMod}`
    if (w.includes('longbow')) return `1d8${abilityMod >= 0 ? '+' : ''}${abilityMod}`
    if (w.includes('shortbow') || w.includes('light crossbow')) return `1d6${abilityMod >= 0 ? '+' : ''}${abilityMod}`
    if (w.includes('heavy crossbow')) return `1d10${abilityMod >= 0 ? '+' : ''}${abilityMod}`
    return `1d6${abilityMod >= 0 ? '+' : ''}${abilityMod}`
  }

  const isFinesse = (weapon: string): boolean => {
    const w = weapon.toLowerCase()
    return w.includes('rapier') || w.includes('dagger') || w.includes('shortsword') ||
           w.includes('scimitar') || w.includes('whip')
  }

  const isRanged = (weapon: string): boolean => {
    const w = weapon.toLowerCase()
    return w.includes('bow') || w.includes('crossbow') || w.includes('dart') || w.includes('sling')
  }

  // Main hand
  if (data.value.equipped.main_hand) {
    const weapon = data.value.equipped.main_hand
    let abilityMod = strMod
    if (isRanged(weapon)) {
      abilityMod = dexMod
    } else if (isFinesse(weapon) && dexMod > strMod) {
      abilityMod = dexMod
    }
    result.push({
      name: weapon,
      attackBonus: prof + abilityMod,
      damage: getWeaponDamage(weapon, abilityMod)
    })
  }

  // Off hand (if not a shield)
  if (data.value.equipped.off_hand && !data.value.equipped.off_hand.toLowerCase().includes('shield')) {
    const weapon = data.value.equipped.off_hand
    result.push({
      name: weapon,
      attackBonus: prof + strMod,
      damage: getWeaponDamage(weapon, 0) // Off-hand doesn't add ability mod to damage
    })
  }

  return result
})

// Equipment
const hasEquipment = computed(() => {
  if (!data.value) return false
  const e = data.value.equipped
  return e.armor || e.shield || e.main_hand || e.off_hand
})

// Personality
const hasPersonality = computed(() => {
  if (!data.value) return false
  const p = data.value.personality
  return p.traits || p.ideals || p.bonds || p.flaws
})

// Spellcasting
const hasSpells = computed(() => {
  if (!data.value) return false
  return data.value.spells.cantrips.length > 0 ||
         data.value.spells.known_spells.length > 0 ||
         data.value.spells.prepared_spells.length > 0 ||
         Object.keys(data.value.spells.spell_slots).length > 0
})

// Sorted spell slot levels
const sortedSpellLevels = computed(() => {
  if (!data.value) return []
  return Object.keys(data.value.spells.spell_slots)
    .map(k => parseInt(k))
    .sort((a, b) => a - b)
})

// Full list casters (prepare from full class list)
const fullListCasters = ['wizard', 'cleric', 'druid', 'paladin']

// Known spells casters (select specific spells at level up)
const knownSpellsCasters = ['bard', 'sorcerer', 'warlock', 'ranger']

// All spellcasting classes (for showing available spells)
const allSpellcasters = [...fullListCasters, ...knownSpellsCasters]

// Check if character has a spellcasting class
const isSpellcaster = computed(() => {
  if (!data.value) return false
  return data.value.classes.some(c =>
    allSpellcasters.includes(c.class_name.toLowerCase())
  )
})

// Check if character has a full list caster class
const isFullListCaster = computed(() => {
  if (!data.value) return false
  return data.value.classes.some(c =>
    fullListCasters.includes(c.class_name.toLowerCase())
  )
})

// Get the primary spellcasting class
const spellcastingClass = computed(() => {
  if (!data.value) return null
  // Find first spellcasting class
  for (const cls of data.value.classes) {
    const name = cls.class_name.toLowerCase()
    if (fullListCasters.includes(name) || knownSpellsCasters.includes(name)) {
      return cls
    }
  }
  return null
})

// Calculate total caster level for multiclass
const totalCasterLevel = computed(() => {
  if (!data.value) return 0

  let casterLevel = 0
  for (const cls of data.value.classes) {
    const className = cls.class_name.toLowerCase()
    const level = cls.level

    // Full casters
    if (['bard', 'cleric', 'druid', 'sorcerer', 'wizard'].includes(className)) {
      casterLevel += level
    }
    // Half casters
    else if (['paladin', 'ranger'].includes(className)) {
      casterLevel += Math.floor(level / 2)
    }
    // Third casters (check subclass)
    else if (className === 'fighter' || className === 'rogue') {
      if (cls.subclass) {
        const subLower = cls.subclass.toLowerCase()
        if (subLower.includes('eldritch knight') || subLower.includes('arcane trickster')) {
          casterLevel += Math.floor(level / 3)
        }
      }
    }
  }

  return casterLevel
})

// Calculate max spell level based on total caster level
const maxSpellLevel = computed(() => {
  const level = totalCasterLevel.value
  if (level === 0) return 0

  if (level >= 17) return 9
  if (level >= 15) return 8
  if (level >= 13) return 7
  if (level >= 11) return 6
  if (level >= 9) return 5
  if (level >= 7) return 4
  if (level >= 5) return 3
  if (level >= 3) return 2
  return 1
})

// Available class spells from catalog
const availableSpells = ref<SpellSummary[]>([])
const loadingSpells = ref(false)

// Calculated spell slots from rules
const calculatedSpellSlots = ref<Record<number, { max: number; current: number }>>({})

// Fetch spell slots from class rules
const fetchSpellSlots = async () => {
  if (!isSpellcaster.value) {
    calculatedSpellSlots.value = {}
    return
  }

  try {
    const slots = await invoke<Record<number, { max: number; current: number }>>('get_character_spell_slots', {
      characterId: characterId.value
    })
    calculatedSpellSlots.value = slots
  } catch (e) {
    console.error('Failed to fetch spell slots:', e)
    calculatedSpellSlots.value = {}
  }
}

// Fetch spells when character loads (only for full list casters)
const fetchClassSpells = async () => {
  if (!isFullListCaster.value || !spellcastingClass.value) {
    availableSpells.value = []
    return
  }

  loadingSpells.value = true
  try {
    // Fetch spells for levels 0 through maxSpellLevel
    const levels = Array.from({ length: maxSpellLevel.value + 1 }, (_, i) => i)

    const spells = await invoke<SpellSummary[]>('search_spells', {
      levels,
      limit: 500
    })

    // Filter by class
    const className = spellcastingClass.value.class_name
    availableSpells.value = spells.filter(spell =>
      spell.classes.some(c => c.toLowerCase() === className.toLowerCase())
    )
  } catch (e) {
    console.error('Failed to fetch class spells:', e)
    availableSpells.value = []
  } finally {
    loadingSpells.value = false
  }
}

// Spells grouped by level
const spellsByLevel = computed(() => {
  const grouped: Record<number, SpellSummary[]> = {}

  for (const spell of availableSpells.value) {
    if (!grouped[spell.level]) {
      grouped[spell.level] = []
    }
    grouped[spell.level].push(spell)
  }

  // Sort spells within each level
  for (const level in grouped) {
    grouped[level].sort((a, b) => a.name.localeCompare(b.name))
  }

  return grouped
})

// Watch for character changes to fetch spells
watch(() => data.value, () => {
  if (data.value) {
    fetchClassSpells()
    fetchSpellSlots()
  }
}, { immediate: true })

// Helper to get primary class name
const primaryClassName = computed(() => {
  if (!data.value || !data.value.classes.length) return ''
  return data.value.classes[0].class_name
})

// Helper to get class string display (e.g., "Fighter 3 / Wizard 2")
const classString = computed(() => {
  if (!data.value || !data.value.classes.length) return ''
  return data.value.classes.map(c => `${c.class_name} ${c.level}`).join(' / ')
})

// Helper to get hit dice string
const hitDiceString = computed(() => {
  if (!data.value || !data.value.classes.length) return ''
  return data.value.classes.map(c => `${c.hit_dice_remaining}${c.hit_dice_type}`).join(', ')
})

// Get spellcasting ability based on class
const spellcastingAbility = computed(() => {
  if (!data.value) return 'intelligence'
  const cls = primaryClassName.value.toLowerCase()
  if (['wizard'].includes(cls)) return 'intelligence'
  if (['cleric', 'druid', 'ranger'].includes(cls)) return 'wisdom'
  if (['bard', 'paladin', 'sorcerer', 'warlock'].includes(cls)) return 'charisma'
  return 'intelligence'
})

const spellSaveDC = computed(() => {
  if (!data.value) return 10
  const ability = data.value.abilities[spellcastingAbility.value as keyof typeof data.value.abilities]
  return 8 + proficiencyBonus.value + getModifier(ability)
})

const spellAttackBonus = computed(() => {
  if (!data.value) return 0
  const ability = data.value.abilities[spellcastingAbility.value as keyof typeof data.value.abilities]
  return proficiencyBonus.value + getModifier(ability)
})

// Fetch spell details for rule text
const fetchSpellDetails = async (spellName: string) => {
  if (spellDetails.value[spellName] || loadingSpellDetails.value.has(spellName)) {
    return
  }

  loadingSpellDetails.value.add(spellName)
  try {
    const spell = await invoke<Spell | null>('get_spell_details', {
      name: spellName,
      source: 'PHB'
    })
    if (spell) {
      spellDetails.value[spellName] = spell
    }
  } catch (e) {
    console.error(`Failed to fetch spell details for ${spellName}:`, e)
  } finally {
    loadingSpellDetails.value.delete(spellName)
  }
}

// Toggle spell expansion
const toggleSpellExpansion = async (spellName: string) => {
  if (expandedSpells.value.has(spellName)) {
    expandedSpells.value.delete(spellName)
  } else {
    expandedSpells.value.add(spellName)
    await fetchSpellDetails(spellName)
  }
}

// Fetch item details for stat blocks
const fetchItemDetails = async (itemName: string, itemSource: string | null) => {
  const key = `${itemName}:${itemSource || 'PHB'}`
  if (itemDetails.value[key] || loadingItemDetails.value.has(key)) {
    return
  }

  loadingItemDetails.value.add(key)
  try {
    const item = await invoke<ItemDetails | null>('get_item_details', {
      itemName: itemName,
      itemSource: itemSource || 'PHB'
    })
    if (item) {
      itemDetails.value[key] = item
    }
  } catch (e) {
    console.error(`Failed to fetch item details for ${itemName}:`, e)
  } finally {
    loadingItemDetails.value.delete(key)
  }
}

// Toggle item expansion
const toggleItemExpansion = async (itemName: string, itemSource: string | null) => {
  const key = `${itemName}:${itemSource || 'PHB'}`
  if (expandedItems.value.has(key)) {
    expandedItems.value.delete(key)
  } else {
    expandedItems.value.add(key)
    await fetchItemDetails(itemName, itemSource)
  }
}

// Format item entry to string
const formatItemEntry = (entry: string | object): string => {
  if (typeof entry === 'string') {
    return entry
  }
  if (typeof entry === 'object' && entry !== null) {
    const obj = entry as Record<string, unknown>
    if (obj.type === 'entries' && Array.isArray(obj.entries)) {
      return (obj.entries as Array<string | object>).map(formatItemEntry).join('\n')
    }
    if (obj.type === 'list' && Array.isArray(obj.items)) {
      return (obj.items as string[]).map(item => `  - ${item}`).join('\n')
    }
  }
  return ''
}

// Format spell entry to string
const formatSpellEntry = (entry: string | object): string => {
  if (typeof entry === 'string') {
    return entry
  }
  if (typeof entry === 'object' && entry !== null) {
    const obj = entry as Record<string, unknown>
    if (obj.type === 'entries' && Array.isArray(obj.entries)) {
      return (obj.entries as Array<string | object>).map(formatSpellEntry).join('\n')
    }
    if (obj.type === 'list' && Array.isArray(obj.items)) {
      return (obj.items as string[]).map(item => `  - ${item}`).join('\n')
    }
  }
  return ''
}

// Get all character spells for display
const characterSpellNames = computed(() => {
  if (!data.value) return []
  const names = new Set<string>()

  data.value.spells.cantrips.forEach(s => names.add(s))
  data.value.spells.known_spells.forEach(s => names.add(s))
  data.value.spells.prepared_spells.forEach(s => names.add(s))

  return Array.from(names).sort()
})

// Get spells organized by level for the spell sheet
const spellsForSheet = computed(() => {
  if (!data.value) return {}

  const result: Record<number, string[]> = { 0: [] }

  // Add cantrips
  result[0] = [...data.value.spells.cantrips].sort()

  // For known spells casters, organize by fetched spell data
  // For now, put all known spells in a "Known" section
  const knownSpells = [...data.value.spells.known_spells].sort()

  // Try to organize by level using availableSpells data
  for (const spellName of knownSpells) {
    const spellInfo = availableSpells.value.find(s => s.name === spellName)
    const level = spellInfo?.level ?? 1
    if (!result[level]) result[level] = []
    result[level].push(spellName)
  }

  return result
})

// Actions
const goBack = () => {
  router.push('/characters')
}

// Level up dialog
const showLevelUpDialog = ref(false)

const levelUp = () => {
  showLevelUpDialog.value = true
}

const handleLevelUpCompleted = async () => {
  // Reload character data after level up
  await characterStore.getCharacter(characterId.value)
}

// Inventory manager
const showInventoryManager = ref(false)

const handleInventoryUpdated = async () => {
  // Reload character data after inventory changes
  await characterStore.getCharacter(characterId.value)
}

// PDF printing
const showPdfPreview = ref(false)
const isPrintingPdf = ref(false)
const pdfPreviewRef = ref<InstanceType<typeof PdfPreviewModal> | null>(null)

const pdfPreviewTitle = computed(() => {
  if (!data.value) return 'Character Sheet'
  return `${data.value.character_name} - Character Sheet`
})

const pdfFileName = computed(() => {
  if (!data.value) return 'character-sheet.pdf'
  const charName = data.value.character_name.replace(/\s+/g, '_')
  const classStr = data.value.classes
    .map(c => `${c.class_name}${c.level}`)
    .join('_')
  return `${charName}_${classStr}.pdf`
})

const printCharacter = async () => {
  if (!data.value) return

  isPrintingPdf.value = true
  showPdfPreview.value = true
  pdfPreviewRef.value?.setLoading(true)

  try {
    const result = await PrintService.generateCharacterSheet(characterId.value, 'sheet')
    pdfPreviewRef.value?.setPdfResult(result)
  } catch (e) {
    console.error('Failed to generate character PDF:', e)
    pdfPreviewRef.value?.setError(e instanceof Error ? e.message : 'Failed to generate PDF')
  } finally {
    isPrintingPdf.value = false
  }
}

// Edit mode functions
const startEditing = () => {
  if (!data.value) return
  editData.value = {
    character_name: data.value.character_name,
    alignment: data.value.alignment || '',
    current_hp: data.value.current_hp,
    personality: {
      traits: data.value.personality.traits || '',
      ideals: data.value.personality.ideals || '',
      bonds: data.value.personality.bonds || '',
      flaws: data.value.personality.flaws || ''
    }
  }
  isEditing.value = true
}

const cancelEditing = () => {
  isEditing.value = false
  editData.value = null
}

const saveEdits = async () => {
  if (!data.value || !editData.value) return

  try {
    // Create updated character data
    const updatedData = {
      ...data.value,
      character_name: editData.value.character_name,
      alignment: editData.value.alignment || null,
      current_hp: editData.value.current_hp,
      personality: {
        traits: editData.value.personality.traits || null,
        ideals: editData.value.personality.ideals || null,
        bonds: editData.value.personality.bonds || null,
        flaws: editData.value.personality.flaws || null
      }
    }

    await invoke('update_character', {
      characterId: characterId.value,
      characterData: updatedData,
      snapshotReason: 'Manual edit'
    })

    // Reload character data
    await characterStore.getCharacter(characterId.value)
    isEditing.value = false
    editData.value = null
  } catch (e) {
    console.error('Failed to save character:', e)
    alert('Failed to save character: ' + e)
  }
}

// Export character sheet as markdown
const exportCharacter = async () => {
  try {
    const markdown = await invoke<string>('render_character_sheet', {
      characterId: characterId.value
    })

    // Use Tauri's save dialog
    const { save } = await import('@tauri-apps/plugin-dialog')

    // Build filename: CharacterName_Class1Level_Class2Level.md
    const charName = data.value?.character_name || 'character'
    const classStr = data.value?.classes
      .map(c => `${c.class_name}${c.level}`)
      .join('_') || 'Unknown'
    const filename = `${charName}_${classStr}.md`

    const filePath = await save({
      defaultPath: filename,
      filters: [{ name: 'Markdown', extensions: ['md'] }]
    })

    if (filePath) {
      // Write file via backend command
      await invoke('write_text_file', {
        path: filePath,
        contents: markdown
      })
    }
  } catch (e) {
    console.error('Failed to export character:', e)
    alert('Failed to export character: ' + e)
  }
}

const deleteCharacter = async () => {
  if (!confirm('Are you sure you want to delete this character?')) return

  try {
    await characterStore.deleteCharacter(characterId.value)
    router.push('/characters')
  } catch (e) {
    console.error('Failed to delete character:', e)
  }
}
</script>

<style scoped>
.character-sheet-view {
  @apply space-y-6;
}

.loading,
.error-message {
  text-align: center;
  padding: var(--spacing-xl) 0;
  color: var(--color-text-secondary);
}

.error-message {
  color: var(--color-error);
}

/* Header */
.sheet-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding-bottom: var(--spacing-lg);
  border-bottom: 2px solid var(--color-border);
}

.header-main {
  @apply space-y-1;
}

.btn-back {
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  color: var(--color-text-secondary);
  cursor: pointer;
  font-size: 0.875rem;
  margin-bottom: var(--spacing-sm);
}

.btn-back:hover {
  background: var(--color-surface);
}

.character-name {
  font-size: 2rem;
  font-weight: 700;
  color: var(--color-text);
}

.character-subtitle {
  font-size: 1.25rem;
  color: var(--color-primary-500);
  font-weight: 500;
}

.character-background {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

/* Tab Navigation */
.tab-navigation {
  display: flex;
  gap: var(--spacing-sm);
  margin: var(--spacing-md) 0;
  border-bottom: 2px solid var(--color-border);
  padding-bottom: var(--spacing-sm);
}

.tab-button {
  padding: var(--spacing-sm) var(--spacing-lg);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm) var(--radius-sm) 0 0;
  color: var(--color-text-secondary);
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.tab-button:hover {
  color: var(--color-text);
  background: var(--color-surface-variant);
}

.tab-button.active {
  color: var(--color-primary-500);
  background: var(--color-surface-variant);
  border-bottom: 2px solid var(--color-primary-500);
  margin-bottom: -2px;
}

.spell-note {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  font-style: italic;
  margin-top: var(--spacing-sm);
}

.header-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.btn-secondary,
.btn-danger {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  border: none;
  font-weight: 500;
  cursor: pointer;
}

.btn-secondary {
  background: var(--color-surface-variant);
  color: var(--color-text);
}

.btn-secondary:hover {
  background: var(--color-primary-100);
}

.btn-danger {
  background: var(--color-error);
  color: white;
}

.btn-danger:hover {
  opacity: 0.9;
}

.btn-primary {
  background: var(--color-primary);
  color: white;
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  border: none;
  font-weight: 500;
  cursor: pointer;
}

.btn-primary:hover {
  opacity: 0.9;
}

/* Edit mode inputs */
.edit-input {
  background: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  color: var(--color-text);
  font-family: inherit;
}

.edit-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.edit-name {
  font-size: 1.5rem;
  font-weight: 700;
  width: 100%;
  margin-bottom: var(--spacing-xs);
}

.edit-alignment {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.edit-alignment-input {
  width: 150px;
  background: var(--color-surface);
  border: 2px solid var(--color-primary-100);
}

.edit-hp {
  width: 60px;
  text-align: center;
}

.edit-textarea {
  width: 100%;
  background: var(--color-surface-variant);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  padding: var(--spacing-sm);
  color: var(--color-text);
  font-family: inherit;
  font-size: 0.875rem;
  resize: vertical;
  margin-top: var(--spacing-xs);
}

.edit-textarea:focus {
  outline: none;
  border-color: var(--color-primary);
}

/* Main Content */
.sheet-content {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--spacing-lg);
}

@media (max-width: 1024px) {
  .sheet-content {
    grid-template-columns: 1fr;
  }
}

.sheet-column {
  @apply space-y-4;
}

.sheet-section {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--spacing-md);
}

.section-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: var(--spacing-md);
  padding-bottom: var(--spacing-sm);
  border-bottom: 1px solid var(--color-border);
}

/* Ability Scores */
.ability-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--spacing-sm);
}

.ability-box {
  text-align: center;
  padding: var(--spacing-sm);
  background: var(--color-surface-variant);
  border-radius: var(--radius-md);
}

.ability-name {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.ability-score {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-text);
}

.ability-modifier {
  font-size: 0.875rem;
  color: var(--color-primary-500);
}

/* Combat Stats */
.combat-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--spacing-sm);
}

.combat-stat {
  display: flex;
  flex-direction: column;
  padding: var(--spacing-sm);
  background: var(--color-surface-variant);
  border-radius: var(--radius-sm);
}

.combat-stat .stat-label {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
}

.combat-stat .stat-value {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text);
}

.combat-stat .stat-note {
  font-size: 0.7rem;
  color: var(--color-text-secondary);
}

.ac-shield {
  color: var(--color-primary-500);
}

.hp-stat {
  grid-column: span 2;
}

/* Saves & Skills */
.saves-list,
.skills-list {
  @apply space-y-1;
}

.save-item,
.skill-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs) 0;
  font-size: 0.875rem;
}

.save-proficient,
.skill-proficient {
  width: 1rem;
  color: var(--color-text-secondary);
  opacity: 0.3;
}

.save-proficient.active,
.skill-proficient.active {
  color: var(--color-primary-500);
  opacity: 1;
}

.save-name,
.skill-name {
  flex: 1;
  color: var(--color-text);
  text-transform: capitalize;
}

.skill-ability {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  text-transform: uppercase;
}

.save-bonus,
.skill-bonus {
  font-weight: 500;
  color: var(--color-primary-500);
  min-width: 2rem;
  text-align: right;
}

/* Attacks */
.attacks-list {
  @apply space-y-2;
}

.attack-item {
  display: grid;
  grid-template-columns: 1fr auto auto;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background: var(--color-surface-variant);
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
}

.attack-name {
  font-weight: 500;
  color: var(--color-text);
}

.attack-bonus {
  font-weight: 600;
  color: var(--color-primary-500);
}

.attack-damage {
  color: var(--color-text-secondary);
}

/* Proficiencies */
.proficiency-group {
  font-size: 0.875rem;
  color: var(--color-text);
  margin-bottom: var(--spacing-sm);
}

.proficiency-group strong {
  color: var(--color-text-secondary);
}

/* Features & Equipment */
.feature-list,
.equipment-list {
  @apply space-y-1;
  padding-left: var(--spacing-md);
  font-size: 0.875rem;
  color: var(--color-text);
}

.equipped-items {
  margin-bottom: var(--spacing-md);
}

.equipped-item {
  font-size: 0.875rem;
  padding: var(--spacing-xs) 0;
}

.equipped-slot {
  color: var(--color-text-secondary);
  font-weight: 500;
}

.inventory-section {
  border-top: 1px solid var(--color-border);
  padding-top: var(--spacing-sm);
}

.inventory-label {
  display: block;
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-xs);
}

/* Spells */
.spell-stats {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
}

.spell-stat {
  display: flex;
  flex-direction: column;
  padding: var(--spacing-sm);
  background: var(--color-surface-variant);
  border-radius: var(--radius-sm);
}

.spell-stat .stat-label {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
}

.spell-stat .stat-value {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text);
}

.spell-slots {
  margin-bottom: var(--spacing-md);
  font-size: 0.875rem;
}

.spell-slots strong {
  color: var(--color-text-secondary);
  display: block;
  margin-bottom: var(--spacing-xs);
}

.slots-grid {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-sm);
}

.slot-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--color-surface-variant);
  border-radius: var(--radius-sm);
  min-width: 3rem;
}

.slot-level {
  font-size: 0.7rem;
  color: var(--color-text-secondary);
}

.slot-count {
  font-weight: 600;
  color: var(--color-primary-500);
}

.spell-group {
  font-size: 0.875rem;
  margin-bottom: var(--spacing-sm);
}

.spell-group strong {
  color: var(--color-text-secondary);
}

/* Available spells for full list casters */
.available-spells {
  margin-top: var(--spacing-md);
}

.loading-spells {
  color: var(--color-text-secondary);
  font-style: italic;
  font-size: 0.875rem;
}

.spell-level-group {
  margin-bottom: var(--spacing-md);
}

.level-header {
  display: block;
  font-size: 0.875rem;
  color: var(--color-primary-500);
  margin-bottom: var(--spacing-xs);
}

.spell-names {
  font-size: 0.8rem;
  color: var(--color-text);
  line-height: 1.4;
}

.spell-tag {
  font-size: 0.6rem;
  vertical-align: super;
  color: var(--color-text-secondary);
  margin-left: 1px;
}

.no-spells {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

/* Currency */
.currency-grid {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md);
  font-size: 0.875rem;
}

.currency-item strong {
  color: var(--color-text-secondary);
}

/* Personality */
.personality-item {
  font-size: 0.875rem;
  margin-bottom: var(--spacing-sm);
  line-height: 1.4;
}

.personality-item strong {
  color: var(--color-text-secondary);
  display: block;
  margin-bottom: 2px;
}

/* Spells Sheet Tab */
.spells-sheet {
  @apply space-y-6;
}

.spells-header {
  background: var(--color-surface-variant);
  border-radius: var(--radius-md);
  padding: var(--spacing-lg);
}

.spellcasting-info {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-lg);
}

.spell-stat-box {
  text-align: center;
  padding: var(--spacing-sm);
  background: var(--color-surface);
  border-radius: var(--radius-sm);
  border: 1px solid var(--color-border);
}

.spell-stat-box .stat-label {
  font-size: 0.7rem;
  text-transform: uppercase;
  color: var(--color-text-secondary);
  display: block;
  margin-bottom: var(--spacing-xs);
}

.spell-stat-box .stat-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-text);
}

.spell-slots-row {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md);
  justify-content: center;
}

.spell-slot-box {
  text-align: center;
  padding: var(--spacing-sm);
  min-width: 4rem;
}

.slot-level-label {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--color-text-secondary);
  display: block;
  margin-bottom: var(--spacing-xs);
}

.slot-circles {
  display: flex;
  gap: 4px;
  justify-content: center;
}

.slot-circle {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: 2px solid var(--color-primary-500);
  background: var(--color-primary-500);
}

.slot-circle.used {
  background: transparent;
}

.spell-levels-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--spacing-lg);
}

.spell-level-section {
  background: var(--color-surface-variant);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.level-header-box {
  background: var(--color-primary-700);
  color: white;
  padding: var(--spacing-sm) var(--spacing-md);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.level-title {
  font-weight: 600;
  font-size: 0.9rem;
}

.slots-remaining {
  font-size: 0.8rem;
  opacity: 0.8;
}

.level-slots-info {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 4px;
}

.slots-total {
  font-size: 0.75rem;
  opacity: 0.9;
}

.slots-circles-header {
  display: flex;
  gap: 3px;
}

.slot-circle-sm {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  border: 1px solid rgba(255, 255, 255, 0.8);
  background: rgba(255, 255, 255, 0.8);
}

.slot-circle-sm.used {
  background: transparent;
}

.spell-list {
  padding: var(--spacing-sm);
  max-height: 400px;
  overflow-y: auto;
}

.spell-item {
  border-bottom: 1px solid var(--color-border);
}

.spell-item:last-child {
  border-bottom: none;
}

.spell-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-sm);
  cursor: pointer;
  transition: background 0.2s;
}

.spell-row:hover {
  background: var(--color-surface);
}

.spell-item .spell-name {
  font-size: 0.875rem;
  color: var(--color-text);
}

.expand-icon {
  font-size: 1rem;
  color: var(--color-text-secondary);
  font-weight: bold;
}

.spell-details {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
}

.loading-details {
  font-size: 0.8rem;
  color: var(--color-text-secondary);
  font-style: italic;
}

.spell-meta {
  display: flex;
  gap: var(--spacing-md);
  align-items: center;
  margin-bottom: var(--spacing-sm);
  font-size: 0.8rem;
  color: var(--color-primary-500);
  font-style: italic;
}

.concentration-tag {
  background: var(--color-warning-100);
  color: var(--color-warning-700);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  font-size: 0.7rem;
  font-style: normal;
}

.spell-properties {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--spacing-xs);
  margin-bottom: var(--spacing-sm);
  font-size: 0.8rem;
}

.spell-properties div {
  color: var(--color-text-secondary);
}

.spell-properties strong {
  color: var(--color-text);
}

.spell-description {
  font-size: 0.8rem;
  line-height: 1.5;
  color: var(--color-text);
}

.spell-description p {
  margin-bottom: var(--spacing-sm);
}

.spell-description p:last-child {
  margin-bottom: 0;
}

.no-spells-message {
  padding: var(--spacing-sm);
  text-align: center;
  font-size: 0.8rem;
  color: var(--color-text-secondary);
}

/* Item styles */
.item-list {
  padding: 0;
}

.item-entry {
  border-bottom: 1px solid var(--color-border);
}

.item-entry:last-child {
  border-bottom: none;
}

.item-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-sm) 0;
  cursor: pointer;
  transition: background 0.2s;
}

.item-row:hover {
  background: var(--color-surface);
}

.item-name {
  font-size: 0.875rem;
  color: var(--color-text);
}

.item-details {
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
}

.item-notes {
  padding: var(--spacing-sm);
  margin-bottom: var(--spacing-sm);
  background: var(--color-warning-100);
  border-radius: var(--radius-sm);
  font-size: 0.8rem;
  color: var(--color-text);
}

.item-meta {
  display: flex;
  gap: var(--spacing-md);
  align-items: center;
  margin-bottom: var(--spacing-sm);
  font-size: 0.8rem;
  color: var(--color-primary-500);
  font-style: italic;
}

.item-rarity {
  text-transform: capitalize;
}

.item-properties {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--spacing-xs);
  margin-bottom: var(--spacing-sm);
  font-size: 0.8rem;
}

.item-properties div {
  color: var(--color-text-secondary);
}

.item-properties strong {
  color: var(--color-text);
}

.item-description {
  font-size: 0.8rem;
  line-height: 1.5;
  color: var(--color-text);
}

.item-description p {
  margin-bottom: var(--spacing-sm);
}

.item-description p:last-child {
  margin-bottom: 0;
}
</style>
