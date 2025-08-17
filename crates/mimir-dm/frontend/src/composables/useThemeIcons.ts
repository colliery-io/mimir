import { computed } from 'vue'
import { useThemeStore } from '../stores/theme'
import type { Theme, IconMap, ThemeIcons } from '../types'

// Import all icon images
import lightEditIcon from '../assets/images/light-edit.png'
import lightLockedIcon from '../assets/images/light-locked.png'
import darkEditIcon from '../assets/images/dark-edit.png'
import darkLockedIcon from '../assets/images/dark-locked.png'
import hyperEditIcon from '../assets/images/hyper-edit.png'
import hyperLockedIcon from '../assets/images/hyper-locked.png'

// Import theme-specific icons
import lightGearIcon from '../assets/images/themes/light/gear.png'
import lightMimirIcon from '../assets/images/themes/light/mimir.png'
import lightNewIcon from '../assets/images/themes/light/new.png'
import darkGearIcon from '../assets/images/themes/dark/gear.png'
import darkMimirIcon from '../assets/images/themes/dark/mimir.png'
import darkNewIcon from '../assets/images/themes/dark/new.png'
import hyperGearIcon from '../assets/images/themes/hyper/gear.png'
import hyperMimirIcon from '../assets/images/themes/hyper/mimir.png'
import hyperNewIcon from '../assets/images/themes/hyper/new.png'

const iconMap: IconMap = {
  light: {
    edit: lightEditIcon,
    locked: lightLockedIcon,
    gear: lightGearIcon,
    mimir: lightMimirIcon,
    new: lightNewIcon
  },
  dark: {
    edit: darkEditIcon,
    locked: darkLockedIcon,
    gear: darkGearIcon,
    mimir: darkMimirIcon,
    new: darkNewIcon
  },
  hyper: {
    edit: hyperEditIcon,
    locked: hyperLockedIcon,
    gear: hyperGearIcon,
    mimir: hyperMimirIcon,
    new: hyperNewIcon
  }
}

export function useThemeIcons() {
  const themeStore = useThemeStore()
  
  const currentTheme = computed<Theme>(() => {
    return (themeStore.currentTheme as Theme) || 'light'
  })
  
  const currentIcons = computed<ThemeIcons>(() => {
    return iconMap[currentTheme.value]
  })
  
  const getIcon = (iconName: keyof ThemeIcons): string => {
    return currentIcons.value[iconName] || iconMap.light[iconName] || ''
  }
  
  const getEditIcon = computed(() => currentIcons.value.edit)
  const getLockedIcon = computed(() => currentIcons.value.locked)
  const getGearIcon = computed(() => currentIcons.value.gear || '')
  const getMimirIcon = computed(() => currentIcons.value.mimir || '')
  const getNewIcon = computed(() => currentIcons.value.new || '')
  
  return {
    currentTheme,
    currentIcons,
    getIcon,
    getEditIcon,
    getLockedIcon,
    getGearIcon,
    getMimirIcon,
    getNewIcon
  }
}